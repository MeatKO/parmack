#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::handle::linux_handle::structs::*;

pub type xcb_connection_t = xcb_connection_t_struct;
pub type xcb_setup_t = xcb_setup_t_struct;
pub type xcb_screen_iterator_t = xcb_screen_iterator_t_struct;
pub type xcb_void_cookie_t = xcb_void_cookie_t_struct;
pub type xcb_screen_t = xcb_screen_t_struct;
pub type xcb_intern_atom_cookie_t = xcb_intern_atom_cookie_t_struct;
pub type xcb_generic_error_t = xcb_generic_error_t_struct;
pub type xcb_intern_atom_reply_t = xcb_intern_atom_reply_t_struct;
pub type xcb_prop_mode_t = xcb_prop_mode_t_struct;

pub type xcb_window_t = u32;
pub type xcb_atom_t = u32;
pub type xcb_colormap_t = u32;
pub type xcb_visualid_t = u32;

pub type xcb_keycode_t = u8;