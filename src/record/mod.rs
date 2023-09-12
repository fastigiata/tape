use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use device_query::{DeviceEvents, DeviceState, Keycode};
use crate::canonicalize::declaration::CanonicalKey;
use crate::canonicalize::Script;

/// The gap between two loops
const LOOP_GAP: u64 = 100;

/// The type of a record
#[derive(Debug, Clone, PartialEq)]
enum RecordType { Keyboard, Mouse, Both }

impl RecordType {
    pub fn with_keyboard(&self) -> bool {
        self != &RecordType::Mouse
    }

    pub fn with_mouse(&self) -> bool {
        self != &RecordType::Keyboard
    }
}

/// A **recorder** is a person who records your [action](../canonicalize/struct.Action.html)s into a [script](../canonicalize/struct.Script.html) for an [actor](../act/struct.Actor.html) to perform
struct Recorder {
    /// The type of the record
    record_type: RecordType,
    /// The key that stops the recording
    stop_signal: Option<CanonicalKey>,

    /// Whether the recorder is recording
    is_working: Arc<Mutex<bool>>,
    /// The script being recorded
    script: Arc<Mutex<Script>>,
}

impl Recorder {
    pub fn listen(&self) {
        *self.is_working.lock().unwrap() = true;

        let flag = Arc::clone(&self.is_working);
        thread::spawn(move || {
            let device_state = DeviceState::new();

            let flag_clone = Arc::clone(&flag);
            let _guard = device_state.on_key_down(move |key| {
                if key == &Keycode::Escape {
                    *flag_clone.lock().unwrap() = false;
                }

                println!("{} is pressed!", key);
            });

            let _guard = 1;

            println!("on_key_down is set at {}", Utc::now().timestamp_millis());

            while *flag.lock().unwrap() {
                thread::sleep(Duration::from_millis(LOOP_GAP));
            };

            println!("listen finished at {}", Utc::now().timestamp_millis())
        });
    }

    /// Create a new recorder
    pub fn new(record_type: RecordType, stop_signal: Option<CanonicalKey>) -> Self {
        Recorder {
            record_type,
            stop_signal,
            is_working: Arc::new(Mutex::new(false)),
            script: Arc::new(Mutex::new(Script::empty())),
        }
    }

    /// Start recording.
    /// This will work in a new thread, so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the recording to finish.
    pub fn work(&self) {
        // set the working flag
        *self.is_working.lock().unwrap() = true;

        // start the recording thread
        thread::spawn({
            let record_type = self.record_type.clone();
            let stop_signal = self.stop_signal.clone();
            let is_working = Arc::clone(&self.is_working);
            let script = Arc::clone(&self.script);

            move || {
                let listener = DeviceState::new();

                // need to record 'keyboard'
                if record_type.with_keyboard() {
                    println!("need to record keyboard");

                    let work_lock = Arc::clone(&is_working);
                    listener.on_key_down(move |key| {
                        println!("{} is pressed!", key);

                        if key == &CanonicalKey::Escape {
                            *work_lock.lock().unwrap() = false;
                        }
                    });

                    //
                    listener.on_key_up(|key| {
                        println!("{} is released!", key);
                    });
                }

                // need to record 'mouse'
                // if record_type.with_mouse() {
                //     println!("need to record mouse");
                //
                //     listener.on_mouse_move(|pos| {
                //         println!("Mouse moved to {:?}", pos);
                //     });
                //     listener.on_mouse_down(|btn| {
                //         println!("{:?} is pressed!", btn);
                //     });
                //     listener.on_mouse_up(|btn| {
                //         println!("{:?} is released!", btn);
                //     });
                // }

                // wait for the stop signal
                while *is_working.lock().unwrap() {
                    thread::sleep(Duration::from_millis(LOOP_GAP));
                };

                println!("stop working");
            }
        });
    }

    /// Finish recording and return the script
    pub fn finish(&self) -> Script {
        todo!()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn ppp() {
        let recorder = Recorder::new(RecordType::Both, Some(CanonicalKey::Escape));
        recorder.listen();

        loop {}
    }

    #[test]
    fn record() {
        let recorder = Recorder::new(RecordType::Both, Some(CanonicalKey::Escape));
        recorder.work();

        thread::sleep(Duration::from_secs(5));

        println!("done");
    }
}