use crate::handle::linux_handle::types::*;
use crate::handle::linux_handle::externs::*;
use crate::handle::linux_handle::consts::*;

use std::ptr::null_mut as nullptr;
use std::ffi::CString;

pub unsafe fn get_atom<T: ToString>(conn: *mut xcb_connection_t, name: T) -> xcb_atom_t
{
	let c_name = CString::new(name.to_string()).unwrap();
	let cookie = xcb_intern_atom(conn, 0, name.to_string().bytes().len() as u16, c_name.as_bytes_with_nul().as_ptr() as _);
	let reply = xcb_intern_atom_reply(conn, cookie, nullptr());

	return 
		match reply.as_mut()
		{
			Some(reply) => { reply.atom }
			None => { XCB_NONE }
		};
}