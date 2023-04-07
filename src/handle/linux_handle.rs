pub mod types;
pub mod enums;
pub mod structs;
pub mod externs;
pub mod functions;
pub mod consts;
pub mod keymap;

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

	fn get_event(&self) -> Option<WindowEvent> 
	{ 
		unsafe 
		{
			if xcb_connection_has_error(self.xcb_conn) > 0
			{
				todo!();
			}

			let event = xcb_poll_for_event(self.xcb_conn).as_mut()? as *mut xcb_generic_event_t;

			// we clear the most significant bit of the 8 bit response_type
			// for WHATEVER reason...
			match ((*event).response_type & 0x7F) as u32
			{
				XCB_KEY_RELEASE =>
				{
					let key_code = *(event as *mut xcb_key_press_event_t);
					return Some(WindowEvent::KeyRelease(convert_key_code(key_code.detail)))
				}
				XCB_KEY_PRESS => 
				{
					let key_code = *(event as *mut xcb_key_press_event_t);
					return Some(WindowEvent::KeyPress(convert_key_code(key_code.detail)))
				}
				XCB_CLIENT_MESSAGE =>
				{
					let key_code = *(event as *mut xcb_client_message_event_t);

					if key_code.data.data32[0] == self.atom_wm_delete_window
					{
						return Some(WindowEvent::WindowAction(WindowActions::Close))
					}

					return Some(WindowEvent::WindowAction(WindowActions::Expose))
				}
				XCB_MOTION_NOTIFY => 
				{
					let key_code = *(event as *mut xcb_motion_notify_event_t);
					return Some(
						WindowEvent::WindowAction(
							WindowActions::Motion{
								x: key_code.event_x as i32, 
								y: key_code.event_y as i32
							}
						)
					);
				}
				XCB_CONFIGURE_NOTIFY =>
				{
					let key_code = *(event as *mut xcb_configure_notify_event_t);
					return Some(
						WindowEvent::WindowAction(
							WindowActions::Configure{ 
								width: key_code.height as i32,
								height: key_code.width as i32
							}
						)
					);
				}
				XCB_FOCUS_IN =>
				{
					return Some(WindowEvent::WindowAction(WindowActions::FocusIn));
				}
				XCB_FOCUS_OUT =>
				{
					return Some(WindowEvent::WindowAction(WindowActions::FocusOut));
				}
				any => { println!("unknown event {}", any); return None }
			}
		}
	}

	fn get_size(&self) -> (u32, u32) { return (0u32, 0u32) }

	fn set_size(&self, width: u32, height: u32) {}
	fn set_title<T: ToString>(&self, title: T) {}

	fn destroy(&mut self) {}
}