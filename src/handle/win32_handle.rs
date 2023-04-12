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
	fn new(title: String, width: u32, height: u32) -> Result<Self, String>
	{
		println!("created win32 window");
		return Ok(Win32Handle {});
	}
	
	fn confine_pointer(&self, active: bool) {}
	fn center_pointer(&self, active: bool) {}
	fn show_pointer(&self, active: bool) {}

	fn get_events(&self) -> Vec<WindowEvent> { return vec![]; }
	fn get_size(&self) -> (u32, u32) { return ( 0u32, 0u32) }
	fn get_pointer_location(&self) -> (u32, u32)  { return (0u32, 0u32) }
	fn get_window_origin(&self) -> (u32, u32)  { return (0u32, 0u32) }

	fn set_size(&self, width: u32, height: u32) {}
	fn set_title<T: ToString>(&self, title: T) {}
	fn set_pointer(&self, x_rel: u32, y_rel: u32) {}
	
	fn destroy(&mut self) {}
}