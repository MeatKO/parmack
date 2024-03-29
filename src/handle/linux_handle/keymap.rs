use crate::window::event::*;

pub fn convert_key_code(key_code: u8) -> KeyCode 
{
    match key_code 
	{
        24 => KeyCode::Q,
        25 => KeyCode::W,
        26 => KeyCode::E,
        27 => KeyCode::R,
        28 => KeyCode::T,
        29 => KeyCode::Y,
        30 => KeyCode::U,
        31 => KeyCode::I,
        32 => KeyCode::O,
        33 => KeyCode::P,
        38 => KeyCode::A,
        39 => KeyCode::S,
        40 => KeyCode::D,
        41 => KeyCode::F,
        42 => KeyCode::G,
        43 => KeyCode::H,
        44 => KeyCode::J,
        45 => KeyCode::K,
        46 => KeyCode::L,
        52 => KeyCode::Z,
        53 => KeyCode::X,
        54 => KeyCode::C,
        55 => KeyCode::V,
        56 => KeyCode::B,
        57 => KeyCode::N,
        58 => KeyCode::M,
        10 => KeyCode::One,
        11 => KeyCode::Two,
        12 => KeyCode::Three,
        13 => KeyCode::Four,
        14 => KeyCode::Five,
        15 => KeyCode::Six,
        16 => KeyCode::Seven,
        17 => KeyCode::Eight,
        18 => KeyCode::Nine,
        19 => KeyCode::Zero,
        67 => KeyCode::F1,
        68 => KeyCode::F2,
        69 => KeyCode::F3,
        70 => KeyCode::F4,
        71 => KeyCode::F5,
        72 => KeyCode::F6,
        73 => KeyCode::F7,
        74 => KeyCode::F8,
        75 => KeyCode::F9,
        76 => KeyCode::F10,
        95 => KeyCode::F11,
        96 => KeyCode::F12,
        36 => KeyCode::Enter,
        50 => KeyCode::ShiftLeft,
        62 => KeyCode::ShiftRight,
        9 => KeyCode::Escape,
        65 => KeyCode::Space,
        22 => KeyCode::Backspace,
        23 => KeyCode::Tab,
        37 => KeyCode::CtrlLeft,
        105 => KeyCode::CtrlRight,
        64 => KeyCode::AltLeft,
        108 => KeyCode::AltRight,
        66 => KeyCode::CapsLock,
        77 => KeyCode::NumLock,
        78 => KeyCode::ScrollLock,
        111 => KeyCode::ArrowUp,
        116 => KeyCode::ArrowDown,
        113 => KeyCode::ArrowLeft,
        114 => KeyCode::ArrowRight,
        110 => KeyCode::Home,
        115 => KeyCode::End,
        112 => KeyCode::PageUp,
        117 => KeyCode::PageDown,
        118 => KeyCode::Insert,
        119 => KeyCode::Delete,
        133 => KeyCode::Windows,
        134 => KeyCode::Application,
        107 => KeyCode::PrintScreen,
        127 => KeyCode::PauseBreak,
        _ => KeyCode::Unknown,
    }
}

pub fn convert_mouse_code(mouse_code: u8) -> MouseCode
{
	match mouse_code
	{
		1 => MouseCode::Left,
        2 => MouseCode::Middle,
        3 => MouseCode::Right,
        4 => MouseCode::ScrollUp,
        5 => MouseCode::ScrollDown,
        8 => MouseCode::SideBack,
        9 => MouseCode::SideFront,
        _ => MouseCode::Unknown,
	}
}