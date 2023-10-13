use crate::handle::linux_handle::externs::free;

#[repr(transparent)]
pub struct XCBWrapper<T>(pub *mut T);

impl<T> Drop for XCBWrapper<T>
{
	fn drop(&mut self) 
	{
		unsafe 
		{
			free(self.0 as _);
		}
	}
}