use std::sync::{Arc, Mutex};
use crate::canonicalize::declaration::CanonicalKey;
use crate::canonicalize::Script;

/// The type of a record
enum RecordType { Keyboard, Mouse, Both }

/// A **recorder** is a person who records your [action](../canonicalize/struct.Action.html)s into a [script](../canonicalize/struct.Script.html) for an [actor](../act/struct.Actor.html) to perform
struct Recorder {
    /// The type of the record
    record_type: RecordType,
    /// The key that stops the recording
    stop_signal: Option<CanonicalKey>,

    /// Whether the recorder is recording
    is_recording: Arc<Mutex<bool>>,
    /// The script being recorded
    script: Arc<Mutex<Script>>,
}

impl Recorder {
    pub fn new(record_type: RecordType, stop_signal: Option<CanonicalKey>) -> Self {
        Recorder {
            record_type,
            stop_signal,
            is_recording: Arc::new(Mutex::new(false)),
            script: Arc::new(Mutex::new(Script::empty())),
        }
    }

    /// Start recording
    pub fn start() {
        todo!()
    }

    /// Finish recording and return the script
    pub fn finish(&self) -> Script {
        todo!()
    }
}