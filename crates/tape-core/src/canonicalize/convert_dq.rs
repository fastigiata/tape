use device_query::{MouseButton, Keycode};
use crate::canonicalize::declaration::{CanonicalButton, CanonicalKey};

// TODO: pr for missing keys in device_query
impl From<Keycode> for CanonicalKey {
    fn from(value: Keycode) -> Self {
        match value {
            // Function keys  -- 16
            Keycode::Escape => CanonicalKey::Escape,
            Keycode::F1 => CanonicalKey::F1,
            Keycode::F2 => CanonicalKey::F2,
            Keycode::F3 => CanonicalKey::F3,
            Keycode::F4 => CanonicalKey::F4,
            Keycode::F5 => CanonicalKey::F5,
            Keycode::F6 => CanonicalKey::F6,
            Keycode::F7 => CanonicalKey::F7,
            Keycode::F8 => CanonicalKey::F8,
            Keycode::F9 => CanonicalKey::F9,
            Keycode::F10 => CanonicalKey::F10,
            Keycode::F11 => CanonicalKey::F11,
            Keycode::F12 => CanonicalKey::F12,
            // Keycode::PrtSc => CanonicalKey::PrtSc,
            // Keycode::ScrLk => CanonicalKey::ScrLk,
            // Keycode::Pause => CanonicalKey::Pause,
            // NumPad keys -- 17
            Keycode::Numpad0 => CanonicalKey::NumPad0,
            Keycode::Numpad1 => CanonicalKey::NumPad1,
            Keycode::Numpad2 => CanonicalKey::NumPad2,
            Keycode::Numpad3 => CanonicalKey::NumPad3,
            Keycode::Numpad4 => CanonicalKey::NumPad4,
            Keycode::Numpad5 => CanonicalKey::NumPad5,
            Keycode::Numpad6 => CanonicalKey::NumPad6,
            Keycode::Numpad7 => CanonicalKey::NumPad7,
            Keycode::Numpad8 => CanonicalKey::NumPad8,
            Keycode::Numpad9 => CanonicalKey::NumPad9,
            // Keycode::Numlock => CanonicalKey::NumLock,
            Keycode::NumpadDivide => CanonicalKey::NumPadDivide,
            Keycode::NumpadMultiply => CanonicalKey::NumPadMultiply,
            Keycode::NumpadSubtract => CanonicalKey::NumPadMinus,
            Keycode::NumpadAdd => CanonicalKey::NumPadPlus,
            // Keycode::NumpadSeparator => CanonicalKey::NumPadSeparator,
            // Keycode::Decimal => CanonicalKey::NumPadDecimal,
            // Navigation & Arrow keys -- 10
            Keycode::Insert => CanonicalKey::Insert,
            Keycode::Delete => CanonicalKey::Delete,
            Keycode::Home => CanonicalKey::Home,
            Keycode::End => CanonicalKey::End,
            Keycode::PageUp => CanonicalKey::PageUp,
            Keycode::PageDown => CanonicalKey::PageDown,
            Keycode::Up => CanonicalKey::ArrowUp,
            Keycode::Down => CanonicalKey::ArrowDown,
            Keycode::Left => CanonicalKey::ArrowLeft,
            Keycode::Right => CanonicalKey::ArrowRight,
            // Modifier keys -- 13
            Keycode::Tab => CanonicalKey::Tab,
            Keycode::CapsLock => CanonicalKey::CapsLock,
            Keycode::LShift => CanonicalKey::LShift,
            Keycode::RShift => CanonicalKey::RShift,
            Keycode::LControl => CanonicalKey::LCtrl,
            Keycode::RControl => CanonicalKey::RCtrl,
            Keycode::LAlt => CanonicalKey::LAlt,
            Keycode::RAlt => CanonicalKey::RAlt,
            Keycode::Meta => CanonicalKey::LMeta,
            // Keycode::LMeta => CanonicalKey::RMeta,
            // Keycode::RMeta => CanonicalKey::RMeta,
            // Keycode::Menu => CanonicalKey::Menu,
            Keycode::Enter => CanonicalKey::Enter,
            Keycode::Backspace => CanonicalKey::Backspace,
            // Printable keys -- 48
            Keycode::A => CanonicalKey::KeyA,
            Keycode::B => CanonicalKey::KeyB,
            Keycode::C => CanonicalKey::KeyC,
            Keycode::D => CanonicalKey::KeyD,
            Keycode::E => CanonicalKey::KeyE,
            Keycode::F => CanonicalKey::KeyF,
            Keycode::G => CanonicalKey::KeyG,
            Keycode::H => CanonicalKey::KeyH,
            Keycode::I => CanonicalKey::KeyI,
            Keycode::J => CanonicalKey::KeyJ,
            Keycode::K => CanonicalKey::KeyK,
            Keycode::L => CanonicalKey::KeyL,
            Keycode::M => CanonicalKey::KeyM,
            Keycode::N => CanonicalKey::KeyN,
            Keycode::O => CanonicalKey::KeyO,
            Keycode::P => CanonicalKey::KeyP,
            Keycode::Q => CanonicalKey::KeyQ,
            Keycode::R => CanonicalKey::KeyR,
            Keycode::S => CanonicalKey::KeyS,
            Keycode::T => CanonicalKey::KeyT,
            Keycode::U => CanonicalKey::KeyU,
            Keycode::V => CanonicalKey::KeyV,
            Keycode::W => CanonicalKey::KeyW,
            Keycode::X => CanonicalKey::KeyX,
            Keycode::Y => CanonicalKey::KeyY,
            Keycode::Z => CanonicalKey::KeyZ,
            Keycode::Key0 => CanonicalKey::Num0,
            Keycode::Key1 => CanonicalKey::Num1,
            Keycode::Key2 => CanonicalKey::Num2,
            Keycode::Key3 => CanonicalKey::Num3,
            Keycode::Key4 => CanonicalKey::Num4,
            Keycode::Key5 => CanonicalKey::Num5,
            Keycode::Key6 => CanonicalKey::Num6,
            Keycode::Key7 => CanonicalKey::Num7,
            Keycode::Key8 => CanonicalKey::Num8,
            Keycode::Key9 => CanonicalKey::Num9,
            Keycode::Grave => CanonicalKey::BackQuote,
            Keycode::Minus => CanonicalKey::Minus,
            Keycode::Equal => CanonicalKey::Equal,
            Keycode::LeftBracket => CanonicalKey::LBracket,
            Keycode::RightBracket => CanonicalKey::RBracket,
            Keycode::BackSlash => CanonicalKey::BackSlash,
            Keycode::Semicolon => CanonicalKey::Semicolon,
            Keycode::Apostrophe => CanonicalKey::Quote,
            Keycode::Comma => CanonicalKey::Comma,
            Keycode::Dot => CanonicalKey::Period,
            Keycode::Slash => CanonicalKey::Slash,
            Keycode::Space => CanonicalKey::Space,
        }
    }
}

impl PartialEq<Keycode> for CanonicalKey {
    fn eq(&self, other: &Keycode) -> bool {
        CanonicalKey::from(*other) == *self
    }
}

impl PartialEq<CanonicalKey> for Keycode {
    fn eq(&self, other: &CanonicalKey) -> bool {
        CanonicalKey::from(*self) == *other
    }
}

impl TryFrom<CanonicalKey> for Keycode {
    type Error = &'static str;

    fn try_from(value: CanonicalKey) -> Result<Self, Self::Error> {
        match value {
// Function keys  -- 16
            CanonicalKey::Escape => Ok(Keycode::Escape),
            CanonicalKey::F1 => Ok(Keycode::F1),
            CanonicalKey::F2 => Ok(Keycode::F2),
            CanonicalKey::F3 => Ok(Keycode::F3),
            CanonicalKey::F4 => Ok(Keycode::F4),
            CanonicalKey::F5 => Ok(Keycode::F5),
            CanonicalKey::F6 => Ok(Keycode::F6),
            CanonicalKey::F7 => Ok(Keycode::F7),
            CanonicalKey::F8 => Ok(Keycode::F8),
            CanonicalKey::F9 => Ok(Keycode::F9),
            CanonicalKey::F10 => Ok(Keycode::F10),
            CanonicalKey::F11 => Ok(Keycode::F11),
            CanonicalKey::F12 => Ok(Keycode::F12),
            // CanonicalKey::PrtSc => Ok(Keycode::PrtSc),
            // CanonicalKey::ScrLk => Ok(Keycode::ScrLk),
            // CanonicalKey::Pause => Ok(Keycode::Pause),
            // NumPad keys -- 17
            CanonicalKey::NumPad0 => Ok(Keycode::Numpad0),
            CanonicalKey::NumPad1 => Ok(Keycode::Numpad1),
            CanonicalKey::NumPad2 => Ok(Keycode::Numpad2),
            CanonicalKey::NumPad3 => Ok(Keycode::Numpad3),
            CanonicalKey::NumPad4 => Ok(Keycode::Numpad4),
            CanonicalKey::NumPad5 => Ok(Keycode::Numpad5),
            CanonicalKey::NumPad6 => Ok(Keycode::Numpad6),
            CanonicalKey::NumPad7 => Ok(Keycode::Numpad7),
            CanonicalKey::NumPad8 => Ok(Keycode::Numpad8),
            CanonicalKey::NumPad9 => Ok(Keycode::Numpad9),
            // CanonicalKey::NumLock => Ok(Keycode::Numlock),
            CanonicalKey::NumPadDivide => Ok(Keycode::NumpadDivide),
            CanonicalKey::NumPadMultiply => Ok(Keycode::NumpadMultiply),
            CanonicalKey::NumPadMinus => Ok(Keycode::NumpadSubtract),
            CanonicalKey::NumPadPlus => Ok(Keycode::NumpadAdd),
            // CanonicalKey::NumPadSeparator => Ok(Keycode::NumpadSeparator),
            // CanonicalKey::NumPadDecimal => Ok(Keycode::Decimal),
            // Navigation & Arrow keys -- 10
            CanonicalKey::Insert => Ok(Keycode::Insert),
            CanonicalKey::Delete => Ok(Keycode::Delete),
            CanonicalKey::Home => Ok(Keycode::Home),
            CanonicalKey::End => Ok(Keycode::End),
            CanonicalKey::PageUp => Ok(Keycode::PageUp),
            CanonicalKey::PageDown => Ok(Keycode::PageDown),
            CanonicalKey::ArrowUp => Ok(Keycode::Up),
            CanonicalKey::ArrowDown => Ok(Keycode::Down),
            CanonicalKey::ArrowLeft => Ok(Keycode::Left),
            CanonicalKey::ArrowRight => Ok(Keycode::Right),
            // Modifier keys -- 13
            CanonicalKey::Tab => Ok(Keycode::Tab),
            CanonicalKey::CapsLock => Ok(Keycode::CapsLock),
            CanonicalKey::LShift => Ok(Keycode::LShift),
            CanonicalKey::RShift => Ok(Keycode::RShift),
            CanonicalKey::LCtrl => Ok(Keycode::LControl),
            CanonicalKey::RCtrl => Ok(Keycode::RControl),
            CanonicalKey::LAlt => Ok(Keycode::LAlt),
            CanonicalKey::RAlt => Ok(Keycode::RAlt),
            CanonicalKey::LMeta => Ok(Keycode::Meta),
            CanonicalKey::RMeta => Ok(Keycode::Meta),
            // CanonicalKey::Menu => Ok(Keycode::Menu),
            CanonicalKey::Enter => Ok(Keycode::Enter),
            CanonicalKey::Backspace => Ok(Keycode::Backspace),
            // Printable keys -- 48
            CanonicalKey::KeyA => Ok(Keycode::A),
            CanonicalKey::KeyB => Ok(Keycode::B),
            CanonicalKey::KeyC => Ok(Keycode::C),
            CanonicalKey::KeyD => Ok(Keycode::D),
            CanonicalKey::KeyE => Ok(Keycode::E),
            CanonicalKey::KeyF => Ok(Keycode::F),
            CanonicalKey::KeyG => Ok(Keycode::G),
            CanonicalKey::KeyH => Ok(Keycode::H),
            CanonicalKey::KeyI => Ok(Keycode::I),
            CanonicalKey::KeyJ => Ok(Keycode::J),
            CanonicalKey::KeyK => Ok(Keycode::K),
            CanonicalKey::KeyL => Ok(Keycode::L),
            CanonicalKey::KeyM => Ok(Keycode::M),
            CanonicalKey::KeyN => Ok(Keycode::N),
            CanonicalKey::KeyO => Ok(Keycode::O),
            CanonicalKey::KeyP => Ok(Keycode::P),
            CanonicalKey::KeyQ => Ok(Keycode::Q),
            CanonicalKey::KeyR => Ok(Keycode::R),
            CanonicalKey::KeyS => Ok(Keycode::S),
            CanonicalKey::KeyT => Ok(Keycode::T),
            CanonicalKey::KeyU => Ok(Keycode::U),
            CanonicalKey::KeyV => Ok(Keycode::V),
            CanonicalKey::KeyW => Ok(Keycode::W),
            CanonicalKey::KeyX => Ok(Keycode::X),
            CanonicalKey::KeyY => Ok(Keycode::Y),
            CanonicalKey::KeyZ => Ok(Keycode::Z),
            CanonicalKey::Num0 => Ok(Keycode::Key0),
            CanonicalKey::Num1 => Ok(Keycode::Key1),
            CanonicalKey::Num2 => Ok(Keycode::Key2),
            CanonicalKey::Num3 => Ok(Keycode::Key3),
            CanonicalKey::Num4 => Ok(Keycode::Key4),
            CanonicalKey::Num5 => Ok(Keycode::Key5),
            CanonicalKey::Num6 => Ok(Keycode::Key6),
            CanonicalKey::Num7 => Ok(Keycode::Key7),
            CanonicalKey::Num8 => Ok(Keycode::Key8),
            CanonicalKey::Num9 => Ok(Keycode::Key9),
            CanonicalKey::BackQuote => Ok(Keycode::Grave),
            CanonicalKey::Minus => Ok(Keycode::Minus),
            CanonicalKey::Equal => Ok(Keycode::Equal),
            CanonicalKey::LBracket => Ok(Keycode::LeftBracket),
            CanonicalKey::RBracket => Ok(Keycode::RightBracket),
            CanonicalKey::BackSlash => Ok(Keycode::BackSlash),
            CanonicalKey::Semicolon => Ok(Keycode::Semicolon),
            CanonicalKey::Quote => Ok(Keycode::Apostrophe),
            CanonicalKey::Comma => Ok(Keycode::Comma),
            CanonicalKey::Period => Ok(Keycode::Dot),
            CanonicalKey::Slash => Ok(Keycode::Slash),
            CanonicalKey::Space => Ok(Keycode::Space),
            _ => Err("Unknown key"),
        }
    }
}

impl From<MouseButton> for CanonicalButton {
    fn from(value: MouseButton) -> Self {
        match value {
            1 => CanonicalButton::Left,
            2 => CanonicalButton::Right,
            3 => CanonicalButton::Middle,
            4 => CanonicalButton::Back,
            5 => CanonicalButton::Forward,
            _ => CanonicalButton::Unknown,
        }
    }
}

impl TryFrom<CanonicalButton> for MouseButton {
    type Error = &'static str;

    fn try_from(value: CanonicalButton) -> Result<Self, Self::Error> {
        match value {
            CanonicalButton::Left => Ok(1),
            CanonicalButton::Right => Ok(2),
            CanonicalButton::Middle => Ok(3),
            CanonicalButton::Back => Ok(4),
            CanonicalButton::Forward => Ok(5),
            _ => Err("Unknown button"),
        }
    }
}