#[derive(Debug)]
pub enum WindowEvent
{
	KeyPress(KeyCode),
	KeyRelease(KeyCode),
	MousePress(MouseCode, i32, i32),
	MouseRelease(MouseCode, i32, i32),
	WindowAction(WindowActions),
	Unknown(u32)
}

#[derive(Debug)]
pub enum WindowActions
{
	Expose,
	Leave,
	Close,
	Minimize,
	Maximize,
	Resize{ width: i32, height: i32 },
	FocusOut,
	FocusIn,
	Motion{ x: i32, y: i32 },
	Configure{ width: i32, height: i32 },
}

#[derive(Debug)]
#[repr(u32)]
pub enum MouseCode
{
	Left,
	Right,
	Middle,
	Unknown,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum KeyCode
{
    // Alphabet
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    
    // Numerals
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    
    // Function keys
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
    
    // Additional keys
    Enter,
    ShiftLeft,
    ShiftRight,
    Escape,
    Space,
    Backspace,
    Tab,
    CtrlLeft,
    CtrlRight,
    AltLeft,
    AltRight,
    CapsLock,
    NumLock,
    ScrollLock,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,
    Windows,
    Application,
    PrintScreen,
    PauseBreak,
    
    // Placeholder default value
    Unknown,
}