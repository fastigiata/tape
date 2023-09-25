use std::sync::{Arc, Mutex};
use crate::canonicalize::{Action, ActionSense, Script};
use crate::canonicalize::declaration::{CanonicalKey};

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

    /// Reset the cursor to the beginning of the script
    pub fn reset_cursor(&mut self) {
        self.cursor = 0;
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
    use device_query::Keycode;
    use enigo::{Key, KeyboardControllable, MouseButton, MouseControllable};
    use crate::canonicalize::declaration::ActionType;
    use crate::canonicalize::Script;

    #[test]
    fn next_action_test() {
        let mut mv = Script::empty();
        thread::sleep(Duration::from_secs(1));
        mv.add_keyboard_action(ActionType::Press, Keycode::A);
        thread::sleep(Duration::from_secs(2));
        mv.add_keyboard_action(ActionType::Release, Keycode::K);
        thread::sleep(Duration::from_secs(1));
        mv.add_mouse_action(ActionType::Press, 1, (50, 50));
        thread::sleep(Duration::from_secs(1));
        mv.add_mouse_action(ActionType::Release, 0, (50, 50));

        while let Some(action) = mv.next_action() {
            println!("next action is [{:?}]", action);
            thread::sleep(Duration::from_secs(1));
        }
    }

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