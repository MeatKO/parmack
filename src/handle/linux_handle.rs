use crate::handle::Handle;
use crate::window::event::WindowEvent;

pub struct LinuxHandle
{

}

impl Handle for LinuxHandle
{
	fn new(window_title: &Option<String>, width: u32, height: u32) -> Result<Self, String>
	{
		return Err("couldn't create xcb window".to_owned());
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

	fn destroy(&mut self)
	{

	}
}