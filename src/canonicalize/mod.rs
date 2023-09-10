use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::canonicalize::declaration::{CanonicalAction, CanonicalButton, CanonicalKey, KeyboardAction, MouseAction};

pub mod declaration;
pub mod convert_enigo;
pub mod convert_dq;

/// An **action** is a single event that can be performed by an [actor](../act/struct.Actor.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    /// The timestamp of the happening of the action
    pub ctime: i64,
    /// The time since the beginning of the script
    pub timeline: i64,
    /// The action to perform
    pub action: CanonicalAction,
}

impl Action {
    pub fn from_keyboard(ev: KeyboardAction, target: CanonicalKey) -> Action {
        Action {
            ctime: Utc::now().timestamp_millis(),
            timeline: 0,
            action: CanonicalAction::Keyboard(ev, target),
        }
    }

    pub fn from_mouse(ev: MouseAction, target: CanonicalButton, pos: (i32, i32)) -> Action {
        Action {
            ctime: Utc::now().timestamp_millis(),
            timeline: 0,
            action: CanonicalAction::Mouse(ev, target, pos),
        }
    }
}

/// A **script** is a sequence of [action](struct.Action.html)s recorded by a [recorder](../rec/struct.Recorder.html) for an [actor](../act/struct.Actor.html) to perform
#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    /// The timestamp of creation of the script
    pub ctime: i64,
    /// The duration of the script in milliseconds
    pub duration: i64,
    /// The actions to perform
    pub actions: Vec<Action>,
}

impl Script {
    /// Check if the script is valid
    /// Rule:
    /// 1. The actions are sorted by their timeline
    /// 2. The duration of the script should not be less than the timeline of the last action
    fn self_check(&self) -> Result<(), String> {
        // check if the actions are sorted by their timeline
        let mut prev = 0;
        for action in &self.actions {
            if action.timeline < prev {
                return Err(format!("The actions are not sorted by their timeline!"));
            }
            prev = action.timeline;
        }

        // check if the duration of the script is not less than the timeline of the last action
        if self.duration < prev {
            return Err(format!("The duration of the script is less than the timeline of the last action!"));
        }

        Ok(())
    }

    /// Create a empty script
    pub fn new() -> Script {
        let t = Utc::now();
        Script {
            ctime: t.timestamp_millis(),
            duration: 0,
            actions: Vec::new(),
        }
    }

    /// Parse a script from a TOML string
    pub fn parse(config: String) {
        // match toml::from_str(&config) {
        //     Ok(v) => println!("ok! {:?}", v),
        //     Err(e) => println!("err! {}", e),
        // }
    }

    /// Add an action to the script
    pub fn add_action(&mut self, mut action: Action) {
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
    pub fn add_keyboard_action(&mut self, ev: KeyboardAction, target: CanonicalKey) {
        self.add_action(Action::from_keyboard(ev, target));
    }

    /// Add a mouse action to the script
    pub fn add_mouse_action(&mut self, ev: MouseAction, target: CanonicalButton, pos: (i32, i32)) {
        self.add_action(Action::from_mouse(ev, target, pos));
    }
}


#[cfg(test)]
mod unit_test {
    use std::thread;
    use std::time::Duration;
    use toml::ser::Error;
    use super::*;
    use crate::canonicalize::declaration::CanonicalButton;

    #[test]
    fn serialize_script() {
        // let mut mv = Script::new();
        // thread::sleep(Duration::from_secs(1));
        // mv.add_keyboard_action(KeyboardAction::Press, CanonicalKey::KeyA);
        // thread::sleep(Duration::from_secs(2));
        // mv.add_keyboard_action(KeyboardAction::Release, CanonicalKey::KeyA);
        // thread::sleep(Duration::from_secs(1));
        // mv.add_mouse_action(MouseAction::Press, CanonicalButton::Left, (50, 50));
        // thread::sleep(Duration::from_secs(1));
        // mv.add_mouse_action(MouseAction::Release, CanonicalButton::Left, (50, 50));

        // match toml::to_string(&mv) {
        //     Ok(v) => println!("ok! {}", v),
        //     Err(e) => println!("err! {}", e),
        // }

        let actions = Action {
            ctime: 1694355501429,
            timeline: 1017,
            action: CanonicalAction::Keyboard(KeyboardAction::Press, CanonicalKey::KeyA),
        };

        match toml::to_string(&actions) {
            Ok(v) => println!("ok!\n{}", v),
            Err(e) => println!("err!\n{}", e),
        }
    }

    #[test]
    fn deserialize_script() {
        let config = r#"ctime = 1694355501429
timeline = 1017
action = ["Press", "KeyA"]"#;

        match toml::from_str::<Action>(&config) {
            Ok(v) => println!("ok! {:?}", v),
            Err(e) => println!("err! {}", e),
        }
    }


    #[derive(Debug, Serialize, Deserialize)]
    enum XYZ { X, Y, Z }

    #[derive(Debug, Serialize, Deserialize)]
    enum AB {
        A((XYZ, i32, i32)),
        B((XYZ, String, String)),
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Box {
        inner: Vec<AB>,
    }

    #[test]
    fn toml_test() {
        let my_box = Box {
            inner: vec![
                AB::A((XYZ::X, 1, 2)),
                AB::B((XYZ::Y, "hello".to_string(), "world".to_string())),
            ],
        };

        println!("original:\n{:?}", &my_box);

        println!("===== now serialize =====");

        // serialize my_box to a TOML string
        match toml::to_string(&my_box) {
            Ok(v) => {
                println!("ser ok!\n{}", &v);

                println!("===== now deserialize =====");

                // deserialize the TOML string back to my_box
                match toml::from_str::<Box>(&v) {
                    Ok(v) => println!("de ok!\n{:?}", v),
                    Err(e) => println!("de err!\n{}", e),
                }
            }
            Err(e) => println!("ser err!\n{}", e),
        }
    }
}