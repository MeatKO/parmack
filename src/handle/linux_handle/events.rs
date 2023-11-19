use crate::handle::linux_handle::types::*;
use crate::handle::linux_handle::keymap::*;
use crate::handle::linux_handle::consts::*;

use crate::window::event::WindowEvent;
use crate::window::event::WindowActions;

use super::LinuxHandle;

impl LinuxHandle
{
	pub fn convert_generic_event(&self, event: *mut xcb_generic_event_t) -> WindowEvent
	{
		unsafe
		{
			// we clear the most significant bit of the 8 bit response_type
			// for WHATEVER reason...
			match ((*event).response_type & 0x7F) as u32
			{
				XCB_KEY_RELEASE =>
				{
					let key_code = *(event as *mut xcb_key_release_event_t);
					return WindowEvent::KeyRelease(convert_key_code(key_code.detail))
				}
				XCB_KEY_PRESS => 
				{
					let key_code = *(event as *mut xcb_key_press_event_t);
					return WindowEvent::KeyPress(convert_key_code(key_code.detail))
				}
				XCB_BUTTON_PRESS =>
				{
					let button_event = *(event as *mut xcb_button_press_event_t);
					// println!("pressed {}", button_event.detail);
					return WindowEvent::MousePress(convert_mouse_code(button_event.detail), button_event.event_x as i32, button_event.event_y as i32)
				}
				XCB_BUTTON_RELEASE =>
				{
					let button_event = *(event as *mut xcb_button_release_event_t);
					// println!("released {}", button_event.detail);
					return WindowEvent::MouseRelease(convert_mouse_code(button_event.detail), button_event.event_x as i32, button_event.event_y as i32)
				}
				XCB_CLIENT_MESSAGE =>
				{
					let key_code = *(event as *mut xcb_client_message_event_t);
	
					if key_code.data.data32[0] == self.atom_wm_delete_window
					{
						return WindowEvent::WindowAction(WindowActions::Close)
					}
	
					return WindowEvent::WindowAction(WindowActions::Expose)
				}
				XCB_MOTION_NOTIFY => 
				{
					let key_code = *(event as *mut xcb_motion_notify_event_t);
					return WindowEvent::WindowAction(
								WindowActions::Motion{
									x: key_code.event_x as i32, 
									y: key_code.event_y as i32
								}
							)
				}
				XCB_CONFIGURE_NOTIFY =>
				{
					let key_code = *(event as *mut xcb_configure_notify_event_t);
					return WindowEvent::WindowAction(
								WindowActions::Configure{ 
									width: key_code.height as i32,
									height: key_code.width as i32
								}
							)
				}
				XCB_FOCUS_IN =>
				{
					return WindowEvent::WindowAction(WindowActions::FocusIn)
				}
				XCB_FOCUS_OUT =>
				{
					return WindowEvent::WindowAction(WindowActions::FocusOut)
				}
				any => { return WindowEvent::Unknown(any) }
			}
		}
	}
}
