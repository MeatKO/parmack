pub mod types;
pub mod enums;
pub mod structs;
pub mod externs;
pub mod functions;
pub mod consts;
pub mod keymap;
pub mod events;

use crate::handle::linux_handle::types::*;
use crate::handle::linux_handle::enums::*;
use crate::handle::linux_handle::externs::*;
use crate::handle::linux_handle::functions::*;
use crate::handle::linux_handle::consts::*;
use crate::handle::linux_handle::keymap::*;

use crate::handle::Handle;

use crate::window::event::WindowEvent;
use crate::window::event::WindowActions;
use crate::window::consts::*;

use std::ptr::null_mut as nullptr;

pub struct LinuxHandle
{
	pub xcb_conn: *mut xcb_connection_t,
	pub xcb_window: xcb_window_t,
	pub atom_wm_protocols: xcb_atom_t,
	pub atom_wm_delete_window: xcb_atom_t
}

impl Drop for LinuxHandle
{
	fn drop(&mut self) 
	{ 
		unsafe
		{
			xcb_destroy_window(self.xcb_conn, self.xcb_window);
			self.xcb_conn = nullptr();
		}
	}
}

impl Handle for LinuxHandle
{
	fn new(title: String, width: u32, height: u32) -> Result<Self, String>
	{
		let width = 
			match width
			{
				0 => DEFAULT_WIDTH,
				_ => width
			};

		let height = 
			match height
			{
				0 => DEFAULT_HEIGHT,
				_ => height
			};

		let mut handle = 
			LinuxHandle
			{
				xcb_conn: nullptr(),
				xcb_window: 0,
				atom_wm_protocols: 0,
				atom_wm_delete_window: 0
			};

		let window_values: [u32; 1] = [
			xcb_event_mask_t::XCB_EVENT_MASK_FOCUS_CHANGE as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_EXPOSURE as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_STRUCTURE_NOTIFY as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_KEY_PRESS as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_KEY_RELEASE as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_PRESS as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_RELEASE as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_POINTER_MOTION as u32
		];
		
		unsafe
		{
			handle.xcb_conn = xcb_connect(nullptr(), nullptr());

			if xcb_connection_has_error(handle.xcb_conn) != 0
			{
				return Err("couldn't connect to xcb server".to_owned());
			}
	
			handle.xcb_window = xcb_generate_id(handle.xcb_conn);

			let iterator = xcb_setup_roots_iterator(xcb_get_setup(handle.xcb_conn));

			xcb_create_window(
				handle.xcb_conn,
				XCB_COPY_FROM_PARENT as u8,
				handle.xcb_window,
				(*iterator.data).root,
				0,
				0,
				width as u16,
				height as u16,
				0,
				xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
				(*iterator.data).root_visual,
				xcb_cw_t::XCB_CW_EVENT_MASK as u32, 
				window_values.as_ptr() as _
			);

			handle.atom_wm_protocols = get_atom(handle.xcb_conn, "WM_PROTOCOLS");
			if handle.atom_wm_protocols == 0
			{
				return Err("xcb wm_protocols atom was not found".to_owned());
			}

			handle.atom_wm_delete_window = get_atom(handle.xcb_conn, "WM_DELETE_WINDOW");
			if handle.atom_wm_protocols == 0
			{
				return Err("xcb wm_delete_window atom was not found".to_owned());
			}

			xcb_change_property(
				handle.xcb_conn,
				xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
				handle.xcb_window,
				handle.atom_wm_protocols,
				xcb_atom_enum_t::XCB_ATOM_ATOM as u32,
				32,
				1, 
				(&mut handle.atom_wm_delete_window as *const xcb_atom_t) as _
			);

			xcb_change_property(
				handle.xcb_conn,
				xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
				handle.xcb_window,
				get_atom(handle.xcb_conn, "_NET_WM_NAME"),
				get_atom(handle.xcb_conn, "UTF8_STRING"),
				8,
				title.bytes().len() as u32, 
				title.as_ptr() as _
			);

			xcb_map_window(handle.xcb_conn, handle.xcb_window);
			xcb_flush(handle.xcb_conn);
		}

		Ok(handle)
	}
	
	fn confine_pointer(&self, active: bool)
	{
		unsafe 
		{
			match active 
			{
				true => 
				{
					xcb_grab_pointer(
						self.xcb_conn, 
						0, 
						self.xcb_window,
						xcb_event_mask_t::XCB_EVENT_MASK_FOCUS_CHANGE as u16 |
						xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_PRESS as u16 |
						xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_RELEASE as u16 |
						xcb_event_mask_t::XCB_EVENT_MASK_POINTER_MOTION as u16 |
						xcb_event_mask_t::XCB_EVENT_MASK_LEAVE_WINDOW as u16,
						xcb_grab_mode_t::XCB_GRAB_MODE_ASYNC as u8, 
						xcb_grab_mode_t::XCB_GRAB_MODE_ASYNC as u8, 
						self.xcb_window,
						0,
						XCB_CURRENT_TIME
					);
		
					xcb_flush(self.xcb_conn);
				}
				false => 
				{
					xcb_ungrab_pointer(
						self.xcb_conn,
						XCB_CURRENT_TIME
					);

					xcb_flush(self.xcb_conn);
				}
			}
		}
	}

	// this does not currently toggle... fix later
	fn center_pointer(&self, active: bool) 
	{
		unsafe 
		{
			let (current_x, current_y) = self.get_pointer_location();
			let (current_width, current_height) = self.get_size();

			if current_x == (current_width / 2) as i32 &&
				current_y == (current_height / 2) as i32
			{
				return;
			}
			

			xcb_warp_pointer(
				self.xcb_conn, 
				self.xcb_window, 
				self.xcb_window, 
				0, 
				0, 
				current_width as _, 
				current_height as _, 
				(current_width / 2) as _,  
				(current_height / 2) as _
			);

			xcb_flush(self.xcb_conn);
		}
	}

	fn show_pointer(&self, active: bool) 
	{
		unsafe 
		{
			let font = xcb_generate_id(self.xcb_conn);

			// ignoring the font cookie
			xcb_open_font_checked (
				self.xcb_conn,
				font,
				"cursor\0".len() as u16,
				"cursor\0".as_ptr() as _
			);

			let cursor = xcb_generate_id(self.xcb_conn);

			xcb_create_glyph_cursor(
				self.xcb_conn, 
				cursor, 
				font, 
				font, 
				cursor as u16, 
				(cursor + 1) as u16, 
				0, 
				0, 
				0, 
				0, 
				0, 
				0
			);

			xcb_flush(self.xcb_conn);
		}
	}

	fn get_events(&self) -> Vec<WindowEvent>
	{ 
		unsafe 
		{
			if xcb_connection_has_error(self.xcb_conn) > 0
			{
				todo!();
			}

			let mut event_vec = vec![];

			while let Some(event) = xcb_poll_for_event(self.xcb_conn).as_mut()
			{
				event_vec.push(self.convert_generic_event(event));
			}

			return event_vec;
		}
	}

	fn get_size(&self) -> (u32, u32) 
	{ 
		unsafe 
		{
			let geometry_cookie = xcb_get_geometry(self.xcb_conn, self.xcb_window);

			let mut err: *mut xcb_generic_error_t = &mut std::mem::zeroed::<xcb_generic_error_t>();
			let geometry_response = xcb_get_geometry_reply(self.xcb_conn, geometry_cookie, &mut err);

			match geometry_response.as_mut()
			{
				None => { return (0u32, 0u32) }
				Some(response) =>
				{
					return (response.width as u32, response.height as u32);
				}
			}
		}
	}
	fn get_pointer_location(&self) -> (i32, i32)  
	{
		unsafe 
		{
			let pointer_cookie = xcb_query_pointer(self.xcb_conn, self.xcb_window);

			let mut err: *mut xcb_generic_error_t = &mut std::mem::zeroed::<xcb_generic_error_t>();
			let pointer_response = xcb_query_pointer_reply(self.xcb_conn, pointer_cookie, &mut err);

			match pointer_response.as_mut()
			{
				None => { return (0i32, 0i32) }
				Some(response) =>
				{
					return (response.win_x as i32, response.win_y as i32)
				}
			}
		}
	}
	fn get_window_origin(&self) -> (u32, u32)  { return (0u32, 0u32) }

	fn set_size(&self, width: u32, height: u32) {}
	fn set_title<T: ToString>(&self, title: T) {}
	fn set_pointer(&self, x_rel: u32, y_rel: u32) {}

	fn destroy(&mut self) {}
}