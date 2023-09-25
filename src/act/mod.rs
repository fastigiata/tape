use std::sync::{Arc, Mutex};
use crate::canonicalize::{Action, ActionSense, Script};
use crate::canonicalize::declaration::{CanonicalAction, CanonicalKey};

impl Script {
    /// Get the next action to be performed
    pub fn next_action(&mut self) -> Option<Action> {
        let total: usize = self.actions.len();
        while self.cursor < total {
            let action = self.actions[self.cursor].clone();
            self.cursor += 1;
            return Some(action);
        }

        None
    }

    /// Get the next keyboard action to be performed
    pub fn next_keyboard_action(&mut self) -> Option<Action> {
        let total: usize = self.actions.len();

        while self.cursor < total {
            let action = &self.actions[self.cursor];
            self.cursor += 1;

            match action.action {
                CanonicalAction::Keyboard(..) => return Some(action.clone()),
                _ => continue,
            }
        }

        None
    }

    /// Get the next mouse action to be performed
    pub fn next_mouse_action(&mut self) -> Option<Action> {
        let total: usize = self.actions.len();

        while self.cursor < total {
            let action = &self.actions[self.cursor];
            self.cursor += 1;

            match action.action {
                CanonicalAction::Mouse(..) => return Some(action.clone()),
                _ => continue,
            }
        }

        None
    }
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
        // set the working flag to false
        *self.mission_guard.lock().unwrap() = false;
    }
}

#[cfg(test)]
mod unit_test {
    use std::thread;
    use std::thread::Thread;
    use std::time::Duration;
    use enigo::{Key, KeyboardControllable, MouseButton, MouseControllable};

    #[test]
    fn enigo_key() {
        // sleep 3 seconds for the user to prepare
        thread::sleep(Duration::from_secs(3));

        let mut actor = enigo::Enigo::new();
        actor.key_click(Key::A);
        actor.key_click(Key::B);
        actor.key_click(Key::C);
    }

    #[test]
    fn enigo_mouse() {
        // sleep 3 seconds for the user to prepare
        thread::sleep(Duration::from_secs(3));

        let mut actor = enigo::Enigo::new();
        actor.mouse_click(MouseButton::Right);
        actor.mouse_move_relative(-30, -30);
        thread::sleep(Duration::from_secs(2));
        actor.mouse_click(MouseButton::Left);
    }
}