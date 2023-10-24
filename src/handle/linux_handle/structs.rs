#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::handle::linux_handle::types::*;

#[repr(C)]
pub struct xcb_connection_t_struct {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct xcb_setup_t_struct {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [u8; 4usize],
}

#[repr(C)]
pub struct xcb_screen_iterator_t_struct {
    pub data: *mut xcb_screen_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct xcb_screen_t_struct {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}

#[repr(C)]
pub struct xcb_void_cookie_t_struct {
    pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct xcb_intern_atom_cookie_t_struct {
    pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct xcb_generic_error_t_struct {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub resource_id: u32,
    pub minor_code: u16,
    pub major_code: u8,
    pub pad0: u8,
    pub pad: [u32; 5usize],
    pub full_sequence: u32,
}

#[repr(C)]
pub struct xcb_intern_atom_reply_t_struct {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xcb_atom_t,
}

#[repr(u32)]
pub enum xcb_prop_mode_t_struct {
    XCB_PROP_MODE_REPLACE = 0,
    XCB_PROP_MODE_PREPEND = 1,
    XCB_PROP_MODE_APPEND = 2,
}

#[repr(C)]
pub struct xcb_generic_event_t_struct {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7usize],
    pub full_sequence: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_key_press_event_t_struct {
    pub response_type: u8,
    pub detail: xcb_keycode_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_button_press_event_t_struct {
    pub response_type: u8,
    pub detail: xcb_button_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcb_client_message_event_t_struct {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub type_: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xcb_client_message_data_t_struct {
    pub data8: [u8; 20usize],
    pub data16: [u16; 10usize],
    pub data32: [u32; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_motion_notify_event_t_struct {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_notify_event_t_struct {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_pointer_cookie_t_struct {
    pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_geometry_cookie_t_struct {
    pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_geometry_reply_t_struct {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_cookie_t_struct {
    pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_request_t_struct {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_reply_t_struct {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub mask: u16,
    pub pad0: [u8; 2usize],
}