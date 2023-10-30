use tape_core::canonicalize::{Action, Script};
use tape_core::canonicalize::declaration::{ActionType, CanonicalAction};

#[napi(object)]
pub struct FFISafeAction {
    /// The timestamp of the happening of the action
    pub ctime: i64,
    /// The time since the beginning of the script
    pub timeline: i64,
    /// The type of the action
    #[napi(ts_type = "'KeyPress' | 'KeyRelease' | 'MousePress' | 'MouseRelease' | 'MouseMove'")]
    pub action_type: String,
    /// The key of the action
    pub action_key: String,
    /// The position of the action (only for mouse action)
    #[napi(ts_type = "[x: number, y: number] | null")]
    pub action_position: Option<Vec<i32>>,
}

impl From<Action> for FFISafeAction {
    fn from(value: Action) -> Self {
        FFISafeAction {
            ctime: value.ctime,
            timeline: value.timeline,
            action_type: match value.action {
                CanonicalAction::Keyboard(kev, _) => match kev {
                    ActionType::Press => "KeyPress",
                    ActionType::Release => "KeyRelease",
                    // this branch should never be reached
                    ActionType::Move => "NEVER",
                },
                CanonicalAction::Mouse(mev, _, _) => match mev {
                    ActionType::Press => "MousePress",
                    ActionType::Release => "MouseRelease",
                    ActionType::Move => "MouseMove",
                },
            }.to_string(),
            action_key: match value.action {
                CanonicalAction::Keyboard(_, kkey) => kkey.to_string(),
                CanonicalAction::Mouse(_, mkey, _) => mkey.to_string(),
            },
            action_position: None,
        }
    }
}

#[napi(object)]
pub struct FFISafeScript {
    /// The name of the script, default to the timestamp of creation
    pub name: String,
    /// The timestamp of creation of the script
    pub ctime: i64,
    /// The duration of the script in milliseconds
    pub duration: i64,
    /// The actions to perform
    pub actions: Vec<FFISafeAction>,
}

impl From<Script> for FFISafeScript {
    fn from(value: Script) -> Self {
        FFISafeScript {
            name: value.name,
            ctime: value.ctime,
            duration: value.duration,
            actions: value.actions.into_iter().map(|action| action.into()).collect(),
        }
    }
}

/// valid key for keyboard action, used in ts-override
pub const TS_TYPE_KEY: &'static str = r###""escape" | "esc" | "f1" | "f2" | "f3" | "f4" | "f5" | "f6" | "f7" | "f8" | "f9" | "f10" | "f11" | "f12" | "prtsc" | "scrlk" | "pause" | "numpad0" | "numpad1" | "numpad2" | "numpad3" | "numpad4" | "numpad5" | "numpad6" | "numpad7" | "numpad8" | "numpad9" | "numlock" | "numpaddivide" | "numpadmultiply" | "numpadminus" | "numpadplus" | "numpadseparator" | "numpaddecimal" | "insert" | "delete" | "home" | "end" | "pageup" | "pagedown" | "arrowup" | "arrowdown" | "arrowleft" | "arrowright" | "tab" | "capslock" | "lshift" | "rshift" | "lctrl" | "rctrl" | "lalt" | "ralt" | "lmeta" | "rmeta" | "menu" | "enter" | "backspace" | "keya" | "a" | "keyb" | "b" | "keyc" | "c" | "keyd" | "d" | "keye" | "e" | "keyf" | "f" | "keyg" | "g" | "keyh" | "h" | "keyi" | "i" | "keyj" | "j" | "keyk" | "k" | "keyl" | "l" | "keym" | "m" | "keyn" | "n" | "keyo" | "o" | "keyp" | "p" | "keyq" | "q" | "keyr" | "r" | "keys" | "s" | "keyt" | "t" | "keyu" | "u" | "keyv" | "v" | "keyw" | "w" | "keyx" | "x" | "keyy" | "y" | "keyz" | "z" | "num0" | "0" | "num1" | "1" | "num2" | "2" | "num3" | "3" | "num4" | "4" | "num5" | "5" | "num6" | "6" | "num7" | "7" | "num8" | "8" | "num9" | "9" | "backquote" | "`" | "minus" | "-" | "equal" | "=" | "lbracket" | "[" | "rbracket" | "]" | "backslash" | "\\" | "semicolon" | ";" | "quote" | "'" | "\"" | "comma" | "," | "period" | "." | "slash" | "/" | "space" | " ""###;