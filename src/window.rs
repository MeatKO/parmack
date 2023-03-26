use crate::handle::Handle;

use self::window_handle::WindowHandle;

#[cfg(target_os = "linux")]
pub mod window_handle 
{
	pub use crate::handle::linux_handle::*;
	pub type WindowHandle = LinuxHandle;
}

#[cfg(target_os = "windows")]
pub mod window_handle
{
	pub use crate::handle::win32_handle::*;
	pub type WindowHandle = Win32Handle;
}

pub mod event;

pub struct Window
{
	handle: Option<Box<WindowHandle>>,
	title: Option<String>,
	pub width: u32,
	pub height: u32,
	pub is_focused: bool,
}

impl Window
{
	pub fn new() -> Self
	{
		return Window { 
			handle: None,
			title: None,
			width: 150,
			height: 150,
			is_focused: true
		}
	}

	pub fn with_title<T>(mut self, title: T) -> Self
	where T: ToString
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

	pub fn build(mut self) -> Result<Self, String>
	{
		self.handle = Some(Box::new(WindowHandle::new(&self.title, self.width, self.height)?));

		Ok(self)
	}
}