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