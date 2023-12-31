use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode, MouseButton};
use crate::canonicalize::declaration::{ActionType, CanonicalKey};
use crate::canonicalize::{Action, ActionSense, Script};

// Collection of methods of Script on 'record'
impl Script {
    /// Add an action to the script
    fn add_action(&mut self, mut action: Action) {
        // calculate the elapsed time since the creation of the script
        let elapsed = action.ctime - self.ctime;
        // update the timeline of the action
        action.timeline = elapsed;
        // update the duration of the script
        self.duration = elapsed;
        // add the action to the script
        self.actions.push(action);
    }

    /// Add a keyboard action to the script
    pub fn add_keyboard_action(&mut self, ev: ActionType, target: Keycode) {
        self.add_action(Action::from_keyboard(ev, target.into()));
    }

    /// Add a mouse action to the script
    pub fn add_mouse_action(&mut self, ev: ActionType, target: MouseButton, pos: (i32, i32)) {
        self.add_action(Action::from_mouse(ev, target.into(), pos));
    }

    /// Bind actions into a script (use the creation time of the script to the current time as the duration)
    /// Return a copy of the script
    pub fn bound(&mut self) -> Script {
        self.duration = Utc::now().timestamp_millis() - self.ctime;
        self.clone()
    }
}

/// The gap between two loops
const LOOP_GAP: u64 = 100;

/// A **recorder** is a person who records your [action](../canonicalize/struct.Action.html)s into a [script](../canonicalize/struct.Script.html) for an [actor](../act/struct.Actor.html) to perform.
///
/// ---
/// - The recorder does not internally determine whether you call `record/record_sync` again on the working recorder. You need to judge by yourself to avoid unexpected behavior (use [is_working](#method.is_working)).
pub struct Recorder {
    /// The type of the action to be recorded
    record_type: ActionSense,
    /// The key that stops the recording
    stop_signal: Option<CanonicalKey>,
    /// A flag indicating whether the recorder is working
    mission_guard: Arc<Mutex<bool>>,
    /// The script being recorded
    script: Arc<Mutex<Script>>,
}

impl Default for Recorder {
    fn default() -> Self {
        Recorder {
            record_type: ActionSense::Keyboard,
            stop_signal: Some(CanonicalKey::Escape),
            mission_guard: Arc::new(Mutex::new(false)),
            script: Arc::new(Mutex::new(Script::empty())),
        }
    }
}

impl Recorder {
    /// Whether the recorder has a stop signal
    pub fn has_stop_signal(&self) -> bool {
        self.stop_signal.is_some()
    }

    /// Check whether the actor is working
    pub fn is_working(&self) -> bool {
        *self.mission_guard.lock().unwrap()
    }

    /// Create a new recorder
    pub fn new(record_type: ActionSense, stop_signal: Option<CanonicalKey>) -> Self {
        Recorder {
            record_type,
            stop_signal,
            mission_guard: Arc::new(Mutex::new(false)),
            script: Arc::new(Mutex::new(Script::empty())),
        }
    }

    /// Set the type of the action to be recorded
    ///
    /// This has no effect on the current recording. (The listener is set once [record](#method.record) is called.)
    pub fn set_record_type(&mut self, record_type: ActionSense) {
        self.record_type = record_type;
    }

    /// Set the key that stops the recording
    ///
    /// This has no effect on the current recording. (The signal is copied once [record](#method.record) is called.)
    pub fn set_stop_signal(&mut self, stop_signal: Option<CanonicalKey>) {
        self.stop_signal = stop_signal;
    }

    /// Start recording
    /// (asynchronous, you can use [finish](#method.finish) to interrupt the recording).
    ///
    /// This will run in a separate thread, so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the recording to finish.
    /// ---
    /// **on_finish**: a callback function that will be called when the recording is finished
    /// - If set to None, do nothing when the recording is finished
    /// - If set to Some(f), call f(script) when the recording is finished
    /// ---
    /// If you want to use synchronous recording, please call [record_sync](#method.record_sync).
    pub fn record(&self, on_finish: Option<Box<dyn FnOnce(Script) + Send>>) {
        // set the working flag
        *self.mission_guard.lock().unwrap() = true;
        self.script.lock().unwrap().reset();

        let record_type = self.record_type.clone();
        let stop_signal = self.stop_signal.clone();
        let mission_guard = Arc::clone(&self.mission_guard);
        let script = Arc::clone(&self.script);

        // start the recording thread
        thread::spawn(move || {
            let ds = DeviceState::new();

            // several guards
            let _guard_kd;
            let _guard_ku;
            let _guard_md;
            let _guard_mu;
            let _guard_mm;
            let _guard_quit;

            // keyboard events
            if record_type.with_keyboard() {
                // keydown listener
                let tmp1 = Arc::clone(&mission_guard);
                let tmp2 = stop_signal.clone();
                let tmp3 = Arc::clone(&script);
                _guard_kd = ds.on_key_down(move |key| {
                    // if the stop signal is pressed, stop the recording
                    if tmp2.is_some_and(|v| &v == key) {
                        *tmp1.lock().unwrap() = false;
                        return;
                    }

                    // push the action to the script
                    tmp3.lock().unwrap().add_keyboard_action(ActionType::Press, *key);
                });

                // keyup listener
                let tmp1 = Arc::clone(&mission_guard);
                let tmp2 = stop_signal.clone();
                let tmp3 = Arc::clone(&script);
                _guard_ku = ds.on_key_up(move |key| {
                    // if the stop signal is pressed, stop the recording
                    if tmp2.is_some_and(|v| &v == key) {
                        *tmp1.lock().unwrap() = false;
                        return;
                    }

                    // push the action to the script
                    tmp3.lock().unwrap().add_keyboard_action(ActionType::Release, *key);
                });
            } else {
                // if the recorder does not record keyboard events,
                // we still need to listen to the stop signal
                // keydown listener
                let tmp1 = Arc::clone(&mission_guard);
                let tmp2 = stop_signal.clone();
                _guard_quit = ds.on_key_down(move |key| {
                    // if the stop signal is pressed, stop the recording
                    if tmp2.is_some_and(|v| &v == key) {
                        *tmp1.lock().unwrap() = false;
                        return;
                    }
                });
            }

            // mouse events
            if record_type.with_mouse() {
                // mousedown listener
                let tmp1 = DeviceState::new();
                let tmp2 = Arc::clone(&script);
                _guard_md = ds.on_mouse_down(move |btn| {
                    tmp2.lock().unwrap().add_mouse_action(
                        ActionType::Press, *btn,
                        tmp1.get_mouse().coords,
                    );
                });

                // mouseup listener
                let tmp1 = DeviceState::new();
                let tmp2 = Arc::clone(&script);
                _guard_mu = ds.on_mouse_up(move |btn| {
                    tmp2.lock().unwrap().add_mouse_action(
                        ActionType::Release, *btn,
                        tmp1.get_mouse().coords,
                    );
                });

                // mousemove listener
                let tmp1 = Arc::clone(&script);
                _guard_mm = ds.on_mouse_move(move |pos| {
                    tmp1.lock().unwrap().add_mouse_action(
                        ActionType::Move, 0,
                        *pos,
                    );
                });
            }

            // do recording until the mission is finished
            while *mission_guard.lock().unwrap() {
                // sleep for a while to avoid too frequent checking
                thread::sleep(Duration::from_millis(LOOP_GAP));
            };

            // bind the script
            let script_copy = script.lock().unwrap().bound();

            // call the callback function if it is set
            if let Some(f) = on_finish {
                f(script_copy);
            };
        });
    }

    /// Interrupt the recording.
    ///
    /// If you want to manipulate the result, please set the **on_finish** when calling [record](#method.record).
    pub fn finish(&self) {
        // set the working flag to false
        *self.mission_guard.lock().unwrap() = false;
    }

    /// Start recording
    /// (synchronous, this will block until the recording is finished).
    /// ---
    /// return:
    /// - **Ok(Script)**: The script being recorded
    /// - **Err(())**: If you have not set the stop signal, this will return a Err(()).
    /// This is by design rather than a bug (image a situation where you have not set the stop signal, then the recorder will never stop).
    /// ---
    /// If you want to use asynchronous recording, please call [record](#method.record).
    pub fn record_sync(&self) -> Result<Script, ()> {
        if self.stop_signal.is_none() {
            return Err(());
        }

        // set the working flag
        *self.mission_guard.lock().unwrap() = true;
        self.script.lock().unwrap().reset();

        let record_type = self.record_type.clone();
        let stop_signal = self.stop_signal.clone().unwrap();
        let mission_guard = Arc::clone(&self.mission_guard);
        let script = Arc::clone(&self.script);

        let ds = DeviceState::new();

        // several guards
        let _guard_kd;
        let _guard_ku;
        let _guard_md;
        let _guard_mu;
        let _guard_mm;
        let _guard_quit;

        // keyboard events
        if record_type.with_keyboard() {
            // keydown listener
            let tmp1 = Arc::clone(&mission_guard);
            let tmp2 = stop_signal.clone();
            let tmp3 = Arc::clone(&script);
            _guard_kd = ds.on_key_down(move |key| {
                // if the stop signal is pressed, stop the recording
                if &tmp2 == key {
                    *tmp1.lock().unwrap() = false;
                    return;
                }

                // push the action to the script
                tmp3.lock().unwrap().add_keyboard_action(ActionType::Press, *key);
            });

            // keyup listener
            let tmp1 = Arc::clone(&mission_guard);
            let tmp2 = stop_signal.clone();
            let tmp3 = Arc::clone(&script);
            _guard_ku = ds.on_key_up(move |key| {
                // if the stop signal is pressed, stop the recording
                if &tmp2 == key {
                    *tmp1.lock().unwrap() = false;
                    return;
                }

                // push the action to the script
                tmp3.lock().unwrap().add_keyboard_action(ActionType::Release, *key);
            });
        } else {
            // if the recorder does not record keyboard events,
            // we still need to listen to the stop signal
            // keydown listener
            let tmp1 = Arc::clone(&mission_guard);
            let tmp2 = stop_signal.clone();
            _guard_quit = ds.on_key_down(move |key| {
                // if the stop signal is pressed, stop the recording
                if &tmp2 == key {
                    *tmp1.lock().unwrap() = false;
                    return;
                }
            });
        }

        // mouse events
        if record_type.with_mouse() {
            // mousedown listener
            let tmp1 = DeviceState::new();
            let tmp2 = Arc::clone(&script);
            _guard_md = ds.on_mouse_down(move |btn| {
                tmp2.lock().unwrap().add_mouse_action(
                    ActionType::Press, *btn,
                    tmp1.get_mouse().coords,
                );
            });

            // mouseup listener
            let tmp1 = DeviceState::new();
            let tmp2 = Arc::clone(&script);
            _guard_mu = ds.on_mouse_up(move |btn| {
                tmp2.lock().unwrap().add_mouse_action(
                    ActionType::Release, *btn,
                    tmp1.get_mouse().coords,
                );
            });

            // mousemove listener
            let tmp1 = Arc::clone(&script);
            _guard_mm = ds.on_mouse_move(move |pos| {
                tmp1.lock().unwrap().add_mouse_action(
                    ActionType::Move, 0,
                    *pos,
                );
            });
        }

        // do recording until the mission is finished
        while *mission_guard.lock().unwrap() {
            // sleep for a while to avoid too frequent checking
            thread::sleep(Duration::from_millis(LOOP_GAP));
        };

        // bind the script
        let r = Ok(script.lock().unwrap().bound());
        r
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn record() {
        let recorder = Recorder::new(ActionSense::Keyboard, Some(CanonicalKey::Escape));
        recorder.record(Some(Box::new(|script| {
            println!("script: {:?}", script.duration);
        })));

        thread::sleep(Duration::from_secs(5));

        // match recorder.finish().publish() {
        //     Ok(v) => println!("ok!\n{}", v),
        //     Err(e) => println!("err!\n{}", e),
        // }
        //
    }

    #[test]
    fn record_sync() {
        let recorder = Recorder::new(ActionSense::Keyboard, Some(CanonicalKey::Escape));

        // here, we use Box to create a pointer to the closure,
        // rather then capture the variable 'recorder'.
        // so that we can reuse the recorder later.
        let task: Box<dyn Fn() -> Result<Script, ()>> = Box::new(|| recorder.record_sync());

        // first use of recorder
        match task() {
            Ok(script) => {
                println!("script 1: {:?}", script.publish());
            }
            Err(_) => {
                println!("err 2!");
            }
        };

        thread::sleep(Duration::from_secs(3));

        println!("task done! 'record' is still here! {}", recorder.has_stop_signal());
        // // reusing the recorder
        // match recorder.record_sync() {
        //     Ok(script) => {
        //         println!("script 2: {:?}", script.publish());
        //     }
        //     Err(_) => {
        //         println!("err 2!");
        //     }
        // };
    }
}