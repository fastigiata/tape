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
    NumPadEnter,
    NumPadDot,
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
    // Alphabet keys -- 48
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
    Tilde,
    Minus,
    Equal,
    LBracket,
    RBracket,
    Backslash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
    Space,
}