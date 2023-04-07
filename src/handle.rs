#![allow(drop_bounds)]

pub mod linux_handle;
pub mod win32_handle;

use crate::window::event::WindowEvent;

pub trait Handle : Sized + Drop
{
	fn new(title: String, width: u32, height: u32) -> Result<Self, String>;
	
	fn confine_pointer(&self);
	fn release_pointer(&self);
	fn center_pointer(&self);
	fn hide_pointer(&self);
	fn show_pointer(&self);

	fn get_events(&self) -> Vec<WindowEvent>;
	fn get_size(&self) -> (u32, u32);
	fn get_pointer_location(&self) -> (u32, u32);
	fn get_window_origin(&self) -> (u32, u32);

	fn set_size(&self, width: u32, height: u32);
	fn set_pointer(&self, x_rel: u32, y_rel: u32); // Relative to the window origin
	fn set_title<T: ToString>(&self, title: T);

	fn destroy(&mut self);
}