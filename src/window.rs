#[cfg(target_os = "linux")]
pub mod window_handle 
{
	use crate::handle::linux_handle::*;
	pub type WindowHandle = LinuxHandle;
}

#[cfg(target_os = "windows")]
pub mod window_handle
{
	use crate::handle::win32_handle::*;
	pub type WindowHandle = Win32Handle;
}

pub mod event;
pub mod consts;

use crate::handle::Handle;

use self::window_handle::WindowHandle;

pub struct WindowBuilder
{
	title: Option<String>,
	width: u32,
	height: u32,
}

impl WindowBuilder
{
	pub fn new() -> Self
	{
		return Self {
			title: None,
			width: 150,
			height: 150,
		}
	}

	pub fn with_title<T: ToString>(mut self, title: T) -> Self
	{
		self.title = Some(title.to_string());
		self
	}

	pub fn with_dimensions(mut self, width: u32, height: u32) -> Self
	{
		self.width = width;
		self.height = height;
		self
	}

	pub fn build(self) -> Result<WindowHandle, String>
	{
		Ok(WindowHandle::new(&self.title, self.width, self.height)?)
	}
}