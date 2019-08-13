#![allow(bad_style, improper_ctypes)]

extern crate libc;
extern crate libnghttp2_sys;

use libnghttp2_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
