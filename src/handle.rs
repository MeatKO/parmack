pub mod linux_handle;
pub mod win32_handle;

use crate::window::event::WindowEvent;

pub trait Handle : Sized
{
	fn new(window_title: &Option<String>, width: u32, height: u32) -> Result<Self, String>;
	
	fn lock_pointer(&self);
	fn unlock_pointer(&self);
	fn center_pointer(&self);
	fn hide_pointer(&self);
	fn show_pointer(&self);

	fn get_event(&self) -> Option<WindowEvent>;
	fn destroy(&mut self);
}