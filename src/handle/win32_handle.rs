use crate::handle::Handle;
use crate::window::event::WindowEvent;

pub struct Win32Handle
{

}

impl Handle for Win32Handle
{
	fn new(window_title: &Option<String>, width: u32, height: u32) -> Result<Self, String>
	{
		return Err("couldn't create win32 window".to_owned());
	}
	
	fn lock_pointer(&self) {}
	fn unlock_pointer(&self) {}
	fn center_pointer(&self) {}
	fn hide_pointer(&self) {}
	fn show_pointer(&self) {}

	fn get_event(&self) -> Option<WindowEvent>
	{
		return None;
	}

	fn destroy(&mut self) {}
}