use crate::window::event::*;

pub fn convert_key_code(key_code: u8) -> KeyCode
{
	match key_code
	{
		9 => { KeyCode::ESC }
		25 => { KeyCode::W }
		38 => { KeyCode::A }
		39 => { KeyCode::S }
		40 => { KeyCode::D }
		_ => { KeyCode::Unknown }
	}
}

