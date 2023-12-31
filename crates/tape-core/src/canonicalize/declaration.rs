use serde::{Deserialize, Serialize};

// ========== ========== Keyboard/Mouse definitions

/// Keyboard keys
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CanonicalKey {
    // Function keys  -- 16
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrtSc,
    ScrLk,
    Pause,
    // NumPad keys -- 17
    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumLock,
    NumPadDivide,
    NumPadMultiply,
    NumPadMinus,
    NumPadPlus,
    NumPadSeparator,
    NumPadDecimal,
    // Navigation & Arrow keys -- 10
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    // Modifier keys -- 13
    Tab,
    CapsLock,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    /// note: 'alt' on windows and linux, 'option' on mac
    LAlt,
    /// note: 'alt' on windows and linux, 'option' on mac
    RAlt,
    /// note: 'command' on mac, 'windows' on windows, 'super' on linux
    LMeta,
    /// note: 'command' on mac, 'windows' on windows, 'super' on linux
    RMeta,
    /// note: 'menu' on windows
    Menu,
    Enter,
    Backspace,
    // Printable keys -- 48
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    BackQuote,
    Minus,
    Equal,
    LBracket,
    RBracket,
    BackSlash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
    Space,
    // Unknown keys -- 1
    Unknown,
}

/// Mouse buttons
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CanonicalButton {
    // Mouse buttons -- 5
    Left,
    Right,
    Middle,
    Back,
    Forward,
    // Unknown buttons -- 1
    Unknown,
}

impl From<String> for CanonicalKey {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            // Function keys  -- 16
            "escape" | "esc" => CanonicalKey::Escape,
            "f1" => CanonicalKey::F1,
            "f2" => CanonicalKey::F2,
            "f3" => CanonicalKey::F3,
            "f4" => CanonicalKey::F4,
            "f5" => CanonicalKey::F5,
            "f6" => CanonicalKey::F6,
            "f7" => CanonicalKey::F7,
            "f8" => CanonicalKey::F8,
            "f9" => CanonicalKey::F9,
            "f10" => CanonicalKey::F10,
            "f11" => CanonicalKey::F11,
            "f12" => CanonicalKey::F12,
            "prtsc" => CanonicalKey::PrtSc,
            "scrlk" => CanonicalKey::ScrLk,
            "pause" => CanonicalKey::Pause,
            // NumPad keys -- 17
            "numpad0" => CanonicalKey::NumPad0,
            "numpad1" => CanonicalKey::NumPad1,
            "numpad2" => CanonicalKey::NumPad2,
            "numpad3" => CanonicalKey::NumPad3,
            "numpad4" => CanonicalKey::NumPad4,
            "numpad5" => CanonicalKey::NumPad5,
            "numpad6" => CanonicalKey::NumPad6,
            "numpad7" => CanonicalKey::NumPad7,
            "numpad8" => CanonicalKey::NumPad8,
            "numpad9" => CanonicalKey::NumPad9,
            "numlock" => CanonicalKey::NumLock,
            "numpaddivide" => CanonicalKey::NumPadDivide,
            "numpadmultiply" => CanonicalKey::NumPadMultiply,
            "numpadminus" => CanonicalKey::NumPadMinus,
            "numpadplus" => CanonicalKey::NumPadPlus,
            "numpadseparator" => CanonicalKey::NumPadSeparator,
            "numpaddecimal" => CanonicalKey::NumPadDecimal,
            // Navigation & Arrow keys -- 10
            "insert" => CanonicalKey::Insert,
            "delete" => CanonicalKey::Delete,
            "home" => CanonicalKey::Home,
            "end" => CanonicalKey::End,
            "pageup" => CanonicalKey::PageUp,
            "pagedown" => CanonicalKey::PageDown,
            "arrowup" => CanonicalKey::ArrowUp,
            "arrowdown" => CanonicalKey::ArrowDown,
            "arrowleft" => CanonicalKey::ArrowLeft,
            "arrowright" => CanonicalKey::ArrowRight,
            // Modifier keys -- 13
            "tab" => CanonicalKey::Tab,
            "capslock" => CanonicalKey::CapsLock,
            "lshift" => CanonicalKey::LShift,
            "rshift" => CanonicalKey::RShift,
            "lctrl" => CanonicalKey::LCtrl,
            "rctrl" => CanonicalKey::RCtrl,
            // note: 'alt' on windows and linux, 'option' on mac
            "lalt" => CanonicalKey::LAlt,
            // note: 'alt' on windows and linux, 'option' on mac
            "ralt" => CanonicalKey::RAlt,
            // note: 'command' on mac, 'windows' on windows, 'super' on linux
            "lmeta" => CanonicalKey::LMeta,
            // note: 'command' on mac, 'windows' on windows, 'super' on linux
            "rmeta" => CanonicalKey::RMeta,
            // note: 'menu' on windows
            "menu" => CanonicalKey::Menu,
            "enter" => CanonicalKey::Enter,
            "backspace" => CanonicalKey::Backspace,
            // Printable keys -- 48
            "keya" | "a" => CanonicalKey::KeyA,
            "keyb" | "b" => CanonicalKey::KeyB,
            "keyc" | "c" => CanonicalKey::KeyC,
            "keyd" | "d" => CanonicalKey::KeyD,
            "keye" | "e" => CanonicalKey::KeyE,
            "keyf" | "f" => CanonicalKey::KeyF,
            "keyg" | "g" => CanonicalKey::KeyG,
            "keyh" | "h" => CanonicalKey::KeyH,
            "keyi" | "i" => CanonicalKey::KeyI,
            "keyj" | "j" => CanonicalKey::KeyJ,
            "keyk" | "k" => CanonicalKey::KeyK,
            "keyl" | "l" => CanonicalKey::KeyL,
            "keym" | "m" => CanonicalKey::KeyM,
            "keyn" | "n" => CanonicalKey::KeyN,
            "keyo" | "o" => CanonicalKey::KeyO,
            "keyp" | "p" => CanonicalKey::KeyP,
            "keyq" | "q" => CanonicalKey::KeyQ,
            "keyr" | "r" => CanonicalKey::KeyR,
            "keys" | "s" => CanonicalKey::KeyS,
            "keyt" | "t" => CanonicalKey::KeyT,
            "keyu" | "u" => CanonicalKey::KeyU,
            "keyv" | "v" => CanonicalKey::KeyV,
            "keyw" | "w" => CanonicalKey::KeyW,
            "keyx" | "x" => CanonicalKey::KeyX,
            "keyy" | "y" => CanonicalKey::KeyY,
            "keyz" | "z" => CanonicalKey::KeyZ,
            "num0" | "0" => CanonicalKey::Num0,
            "num1" | "1" => CanonicalKey::Num1,
            "num2" | "2" => CanonicalKey::Num2,
            "num3" | "3" => CanonicalKey::Num3,
            "num4" | "4" => CanonicalKey::Num4,
            "num5" | "5" => CanonicalKey::Num5,
            "num6" | "6" => CanonicalKey::Num6,
            "num7" | "7" => CanonicalKey::Num7,
            "num8" | "8" => CanonicalKey::Num8,
            "num9" | "9" => CanonicalKey::Num9,
            "backquote" | "`" => CanonicalKey::BackQuote,
            "minus" | "-" => CanonicalKey::Minus,
            "equal" | "=" => CanonicalKey::Equal,
            "lbracket" | "[" => CanonicalKey::LBracket,
            "rbracket" | "]" => CanonicalKey::RBracket,
            "backslash" | "\\" => CanonicalKey::BackSlash,
            "semicolon" | ";" => CanonicalKey::Semicolon,
            "quote" | "'" | "\"" => CanonicalKey::Quote,
            "comma" | "," => CanonicalKey::Comma,
            "period" | "." => CanonicalKey::Period,
            "slash" | "/" => CanonicalKey::Slash,
            "space" | " " => CanonicalKey::Space,
            // Unknown keys -- 1
            _ => CanonicalKey::Unknown,
        }
    }
}

impl From<String> for CanonicalButton {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "left" => CanonicalButton::Left,
            "right" => CanonicalButton::Right,
            "middle" => CanonicalButton::Middle,
            "back" => CanonicalButton::Back,
            "forward" => CanonicalButton::Forward,
            _ => CanonicalButton::Unknown,
        }
    }
}

impl ToString for CanonicalKey {
    fn to_string(&self) -> String {
        match self {
            CanonicalKey::Escape => "escape",
            CanonicalKey::F1 => "f1",
            CanonicalKey::F2 => "f2",
            CanonicalKey::F3 => "f3",
            CanonicalKey::F4 => "f4",
            CanonicalKey::F5 => "f5",
            CanonicalKey::F6 => "f6",
            CanonicalKey::F7 => "f7",
            CanonicalKey::F8 => "f8",
            CanonicalKey::F9 => "f9",
            CanonicalKey::F10 => "f10",
            CanonicalKey::F11 => "f11",
            CanonicalKey::F12 => "f12",
            CanonicalKey::PrtSc => "prtsc",
            CanonicalKey::ScrLk => "scrlk",
            CanonicalKey::Pause => "pause",
            CanonicalKey::NumPad0 => "numpad0",
            CanonicalKey::NumPad1 => "numpad1",
            CanonicalKey::NumPad2 => "numpad2",
            CanonicalKey::NumPad3 => "numpad3",
            CanonicalKey::NumPad4 => "numpad4",
            CanonicalKey::NumPad5 => "numpad5",
            CanonicalKey::NumPad6 => "numpad6",
            CanonicalKey::NumPad7 => "numpad7",
            CanonicalKey::NumPad8 => "numpad8",
            CanonicalKey::NumPad9 => "numpad9",
            CanonicalKey::NumLock => "numlock",
            CanonicalKey::NumPadDivide => "numpaddivide",
            CanonicalKey::NumPadMultiply => "numpadmultiply",
            CanonicalKey::NumPadMinus => "numpadminus",
            CanonicalKey::NumPadPlus => "numpadplus",
            CanonicalKey::NumPadSeparator => "numpadseparator",
            CanonicalKey::NumPadDecimal => "numpaddecimal",
            CanonicalKey::Insert => "insert",
            CanonicalKey::Delete => "delete",
            CanonicalKey::Home => "home",
            CanonicalKey::End => "end",
            CanonicalKey::PageUp => "pageup",
            CanonicalKey::PageDown => "pagedown",
            CanonicalKey::ArrowUp => "arrowup",
            CanonicalKey::ArrowDown => "arrowdown",
            CanonicalKey::ArrowLeft => "arrowleft",
            CanonicalKey::ArrowRight => "arrowright",
            CanonicalKey::Tab => "tab",
            CanonicalKey::CapsLock => "capslock",
            CanonicalKey::LShift => "lshift",
            CanonicalKey::RShift => "rshift",
            CanonicalKey::LCtrl => "lctrl",
            CanonicalKey::RCtrl => "rctrl",
            CanonicalKey::LAlt => "lalt",
            CanonicalKey::RAlt => "ralt",
            CanonicalKey::LMeta => "lmeta",
            CanonicalKey::RMeta => "rmeta",
            CanonicalKey::Menu => "menu",
            CanonicalKey::Enter => "enter",
            CanonicalKey::Backspace => "backspace",
            CanonicalKey::KeyA => "a",
            CanonicalKey::KeyB => "b",
            CanonicalKey::KeyC => "c",
            CanonicalKey::KeyD => "d",
            CanonicalKey::KeyE => "e",
            CanonicalKey::KeyF => "f",
            CanonicalKey::KeyG => "g",
            CanonicalKey::KeyH => "h",
            CanonicalKey::KeyI => "i",
            CanonicalKey::KeyJ => "j",
            CanonicalKey::KeyK => "k",
            CanonicalKey::KeyL => "l",
            CanonicalKey::KeyM => "m",
            CanonicalKey::KeyN => "n",
            CanonicalKey::KeyO => "o",
            CanonicalKey::KeyP => "p",
            CanonicalKey::KeyQ => "q",
            CanonicalKey::KeyR => "r",
            CanonicalKey::KeyS => "s",
            CanonicalKey::KeyT => "t",
            CanonicalKey::KeyU => "u",
            CanonicalKey::KeyV => "v",
            CanonicalKey::KeyW => "w",
            CanonicalKey::KeyX => "x",
            CanonicalKey::KeyY => "y",
            CanonicalKey::KeyZ => "z",
            CanonicalKey::Num0 => "0",
            CanonicalKey::Num1 => "1",
            CanonicalKey::Num2 => "2",
            CanonicalKey::Num3 => "3",
            CanonicalKey::Num4 => "4",
            CanonicalKey::Num5 => "5",
            CanonicalKey::Num6 => "6",
            CanonicalKey::Num7 => "7",
            CanonicalKey::Num8 => "8",
            CanonicalKey::Num9 => "9",
            CanonicalKey::BackQuote => "backquote",
            CanonicalKey::Minus => "minus",
            CanonicalKey::Equal => "equal",
            CanonicalKey::LBracket => "lbracket",
            CanonicalKey::RBracket => "rbracket",
            CanonicalKey::BackSlash => "backslash",
            CanonicalKey::Semicolon => "semicolon",
            CanonicalKey::Quote => "quote",
            CanonicalKey::Comma => "comma",
            CanonicalKey::Period => "period",
            CanonicalKey::Slash => "slash",
            CanonicalKey::Space => "space",
            CanonicalKey::Unknown => "UNKNOWN",
        }.to_string()
    }
}

impl ToString for CanonicalButton {
    fn to_string(&self) -> String {
        match self {
            CanonicalButton::Left => "left",
            CanonicalButton::Right => "right",
            CanonicalButton::Middle => "middle",
            CanonicalButton::Back => "back",
            CanonicalButton::Forward => "forward",
            CanonicalButton::Unknown => "UNKNOWN",
        }.to_string()
    }
}

// ========== ========== Action definitions

/// The type of an action
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActionType {
    Press,
    Release,
    /// note: only for mouse
    Move,
}

/// The information of an action
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CanonicalAction {
    /// A keyboard action -- Tuple(type, key)
    /// - type: press/release
    /// - key: the key to press/release
    Keyboard(ActionType, CanonicalKey),
    /// A mouse action -- Tuple(type, button, pos)
    /// - type: press/release/move (when move, the button is [CanonicalButton::Unknown])
    /// - button: the button to press/release/move
    /// - pos: the position to move to or the position of the mouse
    Mouse(ActionType, CanonicalButton, (i32, i32)),
}