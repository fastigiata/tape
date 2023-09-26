use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
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
}

/// The gap between two loops
const LOOP_GAP: u64 = 100;

/// A **recorder** is a person who records your [action](../canonicalize/struct.Action.html)s into a [script](../canonicalize/struct.Script.html) for an [actor](../act/struct.Actor.html) to perform
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

impl Recorder {
    /// Create a new recorder
    pub fn new(record_type: ActionSense, stop_signal: Option<CanonicalKey>) -> Self {
        Recorder {
            record_type,
            stop_signal,
            mission_guard: Arc::new(Mutex::new(false)),
            script: Arc::new(Mutex::new(Script::empty())),
        }
    }

    /// Start recording.
    /// This will work in a new thread, so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the recording to finish.
    pub fn record(&self) {
        // set the working flag
        *self.mission_guard.lock().unwrap() = true;

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
        });
    }

    /// Finish recording and return the script
    pub fn finish(&self) -> Script {
        // set the working flag to false
        *self.mission_guard.lock().unwrap() = false;

        // return the script
        self.script.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn record() {
        let recorder = Recorder::new(ActionSense::Mouse, Some(CanonicalKey::Escape));
        recorder.record();

        thread::sleep(Duration::from_secs(5));

        match recorder.finish().publish() {
            Ok(v) => println!("ok! {}", v),
            Err(e) => println!("err! {}", e),
        }
    }
}