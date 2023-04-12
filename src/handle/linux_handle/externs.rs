#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::handle::linux_handle::types::*;

extern "C" 
{
    pub fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;

    pub fn xcb_connect(
        displayname: *const ::std::os::raw::c_char,
        screenp: *mut ::std::os::raw::c_int,
    ) -> *mut xcb_connection_t;

    pub fn xcb_generate_id(c: *mut xcb_connection_t) -> u32;

    pub fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;

	pub fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;

	pub fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;

	pub fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_intern_atom_cookie_t;

    pub fn xcb_intern_atom_unchecked(
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_intern_atom_cookie_t;

    pub fn xcb_intern_atom_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t;

	pub fn xcb_change_property(
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;

	pub fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;

    pub fn xcb_unmap_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;

	pub fn xcb_flush(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;

	pub fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;

	pub fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;

	pub fn xcb_grab_pointer(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t;

	pub fn xcb_ungrab_pointer(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t;

	pub fn xcb_get_geometry(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;

	pub fn xcb_get_geometry_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t;
}