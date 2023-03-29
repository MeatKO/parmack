pub mod window;
pub mod handle;

#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))] 
pub fn unsupported_target_os()
{
	compile_error!("Unsupported Target OS.");
}