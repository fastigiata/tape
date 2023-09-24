use std::sync::{Arc, Mutex};
use crate::canonicalize::{ActionSense, Script};
use crate::canonicalize::declaration::CanonicalKey;

impl Script {
    // TODO: 'next_action' implementation
}

/// An **actor** is a person who performs a [script](../canonicalize/struct.Script.html)  of [action](../act/struct.Action.html)s recorded by a [recorder](../record/struct.Recorder.html)
struct Actor {
    /// The type of the action to be acted
    act_type: ActionSense,
    /// The key that stops the acting
    stop_signal: Option<CanonicalKey>,
    /// Whether the actor is acting cyclically
    cyclic: Arc<Mutex<bool>>,
    /// A flag indicating whether the actor is working
    mission_guard: Arc<Mutex<bool>>,
    /// The script being acted
    script: Arc<Mutex<Script>>,
}

impl Actor {
    /// Create a new actor
    pub fn new(script: Script, cyclic: bool, act_type: ActionSense, stop_signal: Option<CanonicalKey>) -> Self {
        Actor {
            act_type,
            stop_signal,
            cyclic: Arc::new(Mutex::new(cyclic)),
            mission_guard: Arc::new(Mutex::new(false)),
            script: Arc::new(Mutex::new(script)),
        }
    }

    /// Set whether the actor is acting cyclically
    pub fn set_cyclic(&mut self, cyclic: bool) {
        *self.cyclic.lock().unwrap() = cyclic;
    }

    /// Start acting.
    /// This will work in a new thread, so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the acting to finish.
    pub fn act(&mut self) {
        todo!("Actor::act()")
    }

    /// Interrupt the actor from acting (it will do nothing if the actor is not acting)
    pub fn finish(&mut self) {
        /// set the working flag to false
        *self.mission_guard.lock().unwrap() = false;
    }
}