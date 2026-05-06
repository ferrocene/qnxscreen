//! Auto-generated bindings for QNX's OpenGL APIs.
//!
//! Copyright (c) SixtyFPS GmbH

#![allow(non_camel_case_types)]

use std::os::raw;

pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_uint64_t = u64;
pub type khronos_ssize_t = isize;
pub type EGLint = i32;
pub type EGLenum = raw::c_uint;
pub type EGLNativeDisplayType = raw::c_int;
pub type EGLNativePixmapType = *const raw::c_void; /* FIXME, should be screen pixmap */
pub type EGLNativeWindowType = *const qnxscreen_sys::_screen_window;

#[allow(unused_imports)]
pub mod egl {
    pub type khronos_utime_nanoseconds_t = super::khronos_utime_nanoseconds_t;
    pub type khronos_uint64_t = super::khronos_uint64_t;
    pub type khronos_ssize_t = super::khronos_ssize_t;
    pub type EGLNativeDisplayType = super::EGLNativeDisplayType;
    pub type EGLNativePixmapType = super::EGLNativePixmapType;
    pub type EGLNativeWindowType = super::EGLNativeWindowType;
    pub type EGLint = super::EGLint;
    pub type NativeDisplayType = super::EGLNativeDisplayType;
    pub type NativePixmapType = super::EGLNativePixmapType;
    pub type NativeWindowType = super::EGLNativeWindowType;

    include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));
}

pub use egl::types::EGLContext;
pub use egl::types::EGLDisplay;

#[link(name = "EGL")]
unsafe extern "C" {}
