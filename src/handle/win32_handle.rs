use crate::handle::Handle;
use crate::window::event::WindowEvent;

pub struct Win32Handle
{

}

impl Drop for Win32Handle
{
	fn drop(&mut self) 
	{ 
		todo!() 
	}
}

impl Handle for Win32Handle
{
	fn new(window_title: &Option<String>, width: u32, height: u32) -> Result<Self, String>
	{
		println!("created win32 window");
		return Ok(Win32Handle {});
	}
	
	fn lock_pointer(&self) {}
	fn unlock_pointer(&self) {}
	fn center_pointer(&self) {}
	fn hide_pointer(&self) {}
	fn show_pointer(&self) {}

	fn get_event(&self) -> Option<WindowEvent> { return None; }
	fn get_size(&self) -> (u32, u32) { return ( 0u32, 0u32) }

	fn set_size(&self, width: u32, height: u32) {}
	fn set_title<T: ToString>(&self, title: T) {}
	
	fn destroy(&mut self) {}
}