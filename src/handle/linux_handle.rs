mod types;
mod enums;
mod structs;
mod externs;
mod functions;
mod consts;

use crate::handle::linux_handle::types::*;
use crate::handle::linux_handle::enums::*;
use crate::handle::linux_handle::externs::*;
use crate::handle::linux_handle::functions::*;
use crate::handle::linux_handle::consts::*;

use crate::handle::Handle;

use crate::window::event::WindowEvent;
use crate::window::consts::*;

use std::ptr::null_mut as nullptr;

pub struct LinuxHandle
{
	xcb_conn: *mut xcb_connection_t,
	xcb_window: xcb_window_t,
	atom_wm_protocols: xcb_atom_t,
	atom_wm_delete_window: xcb_atom_t
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
	fn new(window_title: &Option<String>, width: u32, height: u32) -> Result<Self, String>
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

			let title = 
				match window_title
				{
					Some(title) => { title.clone() }
					None => { DEFAULT_WINDOW_NAME.to_owned() }
				};

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

			// let needed_extensions = vec![
			// 	"VK_KHR_xcb_surface",
			// 	"VK_KHR_surface"
			// ];

			// get_missing_extensions(&needed_extensions, &vk_handle.available_extensions);

			// match create_xcb_surface_function(&vk_handle.instance)
			// {
			// 	None => { panic!("This platform doesn't offer a 'vkCreateXcbSurfaceKHR' function.") }
			// 	Some(function) => 
			// 	{
			// 		let surface_create_info = VkXcbSurfaceCreateInfoKHR {
			// 			sType: VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
			// 			connection: handle.xcb_conn,
			// 			window: handle.xcb_window,
			// 			flags: 0,
			// 			pNext: nullptr()
			// 		};

			// 		let result = function(vk_handle.instance, &surface_create_info, nullptr(), &mut vk_handle.window_surface);
			// 		match result
			// 		{
			// 			VkResult::VK_SUCCESS => {}
			// 			res => { panic!("Vulkan is not supported on given X window. vkCreateXcbSurfaceKHR() resulted in {:?}", res) }
			// 		}
			// 	}
			// }
		}

		Ok(handle)
	}
	
	fn lock_pointer(&self) {}
	fn unlock_pointer(&self) {}
	fn center_pointer(&self) {}
	fn hide_pointer(&self) {}
	fn show_pointer(&self) {}

	fn get_event(&self) -> Option<WindowEvent> { return None; }
	fn get_size(&self) -> (u32, u32) { return ( 0u32, 0u32) }

	fn set_size(&self, width: u32, height: u32) {}
	fn set_title<T: ToString>(&self, title: T) {}

	fn destroy(&mut self) {}
}