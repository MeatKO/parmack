pub mod window;
pub mod handle;

#[cfg(target_os = "linux")]
pub mod window_handle 
{
	pub use crate::handle::linux_handle::*;
}

#[cfg(target_os = "windows")]
pub mod window_handle 
{
	pub use crate::handle::win32_handle::*;
}

#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))] 
pub fn unsupported_target_os()
{
	compile_error!("Unsupported Target OS.");
}	