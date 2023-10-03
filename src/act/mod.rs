use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use device_query::{DeviceEvents, DeviceState};
use enigo::{KeyboardControllable, MouseControllable};
use crate::canonicalize::{Action, ActionSense, Script};
use crate::canonicalize::declaration::{ActionType, CanonicalAction, CanonicalKey};

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
pub struct Actor {
    /// The type of the action to be acted
    act_type: ActionSense,
    /// The key that stops the acting
    stop_signal: Option<CanonicalKey>,
    /// Whether the actor is acting cyclically
    cyclic: Arc<Mutex<bool>>,
    /// A flag indicating whether the actor is working
    mission_guard: Arc<Mutex<bool>>,
    /// The script being acted
    script: Script,
}

impl Actor {
    /// Create a new actor
    pub fn new(script: Script, cyclic: bool, act_type: ActionSense, stop_signal: Option<CanonicalKey>) -> Self {
        Actor {
            act_type,
            stop_signal,
            cyclic: Arc::new(Mutex::new(cyclic)),
            mission_guard: Arc::new(Mutex::new(false)),
            script,
        }
    }

    /// Set the type of the action to be acted.
    ///
    /// This has no effect on the current acting. (The script is copied once [act](#method.act) is called)
    pub fn set_act_type(&mut self, act_type: ActionSense) {
        self.act_type = act_type;
    }

    /// Set whether the actor is acting cyclically.
    ///
    /// Can affect the current acting cause 'cyclic' is checked before every loop.
    pub fn set_cyclic(&mut self, cyclic: bool) {
        *self.cyclic.lock().unwrap() = cyclic;
    }

    /// Start acting.
    /// This will work in a new thread, so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the acting to finish.
    pub fn act(&mut self) {
        // set the working flag
        *self.mission_guard.lock().unwrap() = true;

        let cyclic_flag = Arc::clone(&self.cyclic);
        let stop_signal = self.stop_signal.clone();
        let mission_guard = Arc::clone(&self.mission_guard);
        let mut script_copy = self.script.to_filtered(self.act_type.clone());

        thread::spawn(move || {
            let mut eg = enigo::Enigo::new();

            // register a listener for the stop signal if there is one
            let _guard_quit;
            if let Some(quit_key) = stop_signal {
                let tmp1 = Arc::clone(&mission_guard);
                _guard_quit = DeviceState::new().on_key_down(move |key| {
                    // if the stop signal is pressed, stop the acting
                    if key == &quit_key {
                        *tmp1.lock().unwrap() = false;
                        return;
                    }
                })
            }

            // use 'Instant' to record the beginning time
            let mut begin_time = Instant::now();

            // do acting until the mission is finished
            while *mission_guard.lock().unwrap() {
                // get the elapsed time, calculate the frame on the 'timeline'
                let elapsed_ms = begin_time.elapsed().as_millis() as i64;

                // get the next action if there is one, otherwise check whether the script is cyclic or not
                if let Some(next_action) = script_copy.next_action() {
                    let wait_time = next_action.timeline - elapsed_ms;
                    if wait_time > 0 { thread::sleep(Duration::from_millis(wait_time as u64)); }

                    match next_action.action {
                        CanonicalAction::Keyboard(t, k) => match t {
                            ActionType::Press => if let Ok(enigo_key) = k.try_into() {
                                eg.key_down(enigo_key);
                            },
                            ActionType::Release => if let Ok(enigo_key) = k.try_into() {
                                eg.key_up(enigo_key);
                            },
                            // ignore ActionType::Move cause it is not a  valid keyboard action type
                            _ => {}
                        }
                        CanonicalAction::Mouse(t, b, p) => match t {
                            ActionType::Press => if let Ok(enigo_button) = b.try_into() {
                                eg.mouse_down(enigo_button);
                            },
                            ActionType::Release => if let Ok(enigo_button) = b.try_into() {
                                eg.mouse_up(enigo_button);
                            },
                            ActionType::Move => eg.mouse_move_to(p.0, p.1),
                        }
                    }
                } else {
                    // 1 - check whether the script is finished even if there is no next action
                    if elapsed_ms < script_copy.duration {
                        thread::sleep(Duration::from_millis((script_copy.duration - elapsed_ms) as u64));
                    }

                    // 2 - check whether it is cyclic
                    if *cyclic_flag.lock().unwrap() {
                        // 2.1 - if cyclic, reset the cursor and the beginning time
                        script_copy.reset_cursor();
                        begin_time = Instant::now();
                    } else {
                        // 2.2 - if not, finish the mission
                        *mission_guard.lock().unwrap() = false;
                        return;
                    }
                }
            }
        });
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
    use std::time::Duration;
    use device_query::Keycode;
    use enigo::{Key, KeyboardControllable, MouseButton, MouseControllable};
    use crate::act::Actor;
    use crate::canonicalize::declaration::{ActionType, CanonicalKey};
    use crate::canonicalize::{ActionSense, Script};

    #[test]
    fn act() {
        // region script raw
        let script_raw = r##"
name = "2023-09-26T14:52:04.720709900+00:00"
ctime = 1695739924720
duration = 7000

[[actions]]
ctime = 1695739927090
timeline = 2370

[actions.action]
Keyboard = ["Press", "Enter"]

[[actions]]
ctime = 1695739927110
timeline = 2390

[actions.action]
Keyboard = ["Release", "Enter"]

[[actions]]
ctime = 1695739927210
timeline = 2490

[actions.action]
Keyboard = ["Press", "Enter"]

[[actions]]
ctime = 1695739927270
timeline = 2550

[actions.action]
Keyboard = ["Release", "Enter"]

[[actions]]
ctime = 1695739927407
timeline = 2687

[actions.action]
Keyboard = ["Press", "Slash"]

[[actions]]
ctime = 1695739927469
timeline = 2749

[actions.action]
Keyboard = ["Release", "Slash"]

[[actions]]
ctime = 1695739927529
timeline = 2809

[actions.action]
Keyboard = ["Press", "Slash"]

[[actions]]
ctime = 1695739927592
timeline = 2872

[actions.action]
Keyboard = ["Release", "Slash"]

[[actions]]
ctime = 1695739927670
timeline = 2950

[actions.action]
Keyboard = ["Press", "Space"]

[[actions]]
ctime = 1695739927764
timeline = 3044

[actions.action]
Keyboard = ["Release", "Space"]

[[actions]]
ctime = 1695739927905
timeline = 3185

[actions.action]
Keyboard = ["Press", "KeyH"]

[[actions]]
ctime = 1695739927967
timeline = 3247

[actions.action]
Keyboard = ["Release", "KeyH"]

[[actions]]
ctime = 1695739928061
timeline = 3341

[actions.action]
Keyboard = ["Press", "KeyE"]

[[actions]]
ctime = 1695739928124
timeline = 3404

[actions.action]
Keyboard = ["Release", "KeyE"]

[[actions]]
ctime = 1695739928156
timeline = 3436

[actions.action]
Keyboard = ["Press", "KeyL"]

[[actions]]
ctime = 1695739928202
timeline = 3482

[actions.action]
Keyboard = ["Release", "KeyL"]

[[actions]]
ctime = 1695739928249
timeline = 3529

[actions.action]
Keyboard = ["Press", "KeyL"]

[[actions]]
ctime = 1695739928312
timeline = 3592

[actions.action]
Keyboard = ["Release", "KeyL"]

[[actions]]
ctime = 1695739928463
timeline = 3743

[actions.action]
Keyboard = ["Press", "KeyO"]

[[actions]]
ctime = 1695739928526
timeline = 3806

[actions.action]
Keyboard = ["Release", "KeyO"]

[[actions]]
ctime = 1695739929186
timeline = 4466

[actions.action]
Keyboard = ["Press", "Comma"]

[[actions]]
ctime = 1695739929254
timeline = 4534

[actions.action]
Keyboard = ["Release", "Comma"]

[[actions]]
ctime = 1695739929302
timeline = 4582

[actions.action]
Keyboard = ["Press", "Space"]

[[actions]]
ctime = 1695739929395
timeline = 4675

[actions.action]
Keyboard = ["Release", "Space"]

[[actions]]
ctime = 1695739929686
timeline = 4966

[actions.action]
Keyboard = ["Press", "KeyI"]
        "##;
        // endregion

        let script = Script::load(script_raw).unwrap();
        let mut actor = Actor::new(script, false, ActionSense::Keyboard, Some(CanonicalKey::Escape));

        // sleep 3 seconds for the user to prepare
        thread::sleep(Duration::from_secs(3));

        actor.act();

        // sleep 8 seconds for the actor to preform
        thread::sleep(Duration::from_secs(8));

        println!("finish!");
    }

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