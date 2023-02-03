//! QNX Screen API
//!
//! Ported to Rust by Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2023
//!
//! Copyright (c) AdaCore, 2023

use std::marker::PhantomData;

use bitflags::bitflags;
use qnxscreen_sys as sys;

pub mod events;

// ============================================================================
//
// Error
//
// ============================================================================

/// The errors this API can produce
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("QNX API error")]
    System(#[from] std::io::Error),
    #[error("Unknown Event Code")]
    UnknownEvent(i32),
    #[error("QNX provided a SCREEN_PROPERTY_X we didn't recognise")]
    UnknownPropertyName(u32),
    #[error("A bad string argument was supplied to the Rust API")]
    BadStringArgument,
}

// ============================================================================
//
// Screen Context
//
// ============================================================================

/// Represents a Screen Context.
///
/// The C type is `screen_context_t`.
#[derive(Debug)]
pub struct Context(sys::screen_context_t);

impl Context {
    /// Create a new Screen Application Context.
    pub fn new(context_type: ContextType) -> Result<Context, Error> {
        let mut ptr = std::ptr::null_mut();
        let rc = unsafe { sys::screen_create_context(&mut ptr, context_type.bits()) };
        map_error(rc, || Ok(Context(ptr)))
    }

    /// Get the number of displays.
    ///
    /// Fetches the [`sys::SCREEN_PROPERTY_DISPLAY_COUNT`] property.
    pub fn num_displays(&self) -> Result<i32, Error> {
        let mut num_displays = 0;
        let rc = unsafe {
            sys::screen_get_context_property_iv(
                self.0,
                sys::SCREEN_PROPERTY_DISPLAY_COUNT as i32,
                &mut num_displays,
            )
        };
        map_error(rc, || Ok(num_displays))
    }

    /// Get a window onto this screen.
    pub fn create_window<'ctx>(&'ctx self) -> Result<Window<'ctx>, Error> {
        let mut ptr = std::ptr::null_mut();
        let rc = unsafe { sys::screen_create_window(&mut ptr, self.0) };
        map_error(rc, || {
            Ok(Window {
                ptr: ptr as sys::screen_window_t,
                owned: true,
                _phantom: PhantomData,
            })
        })
    }

    /// Get an event from the event queue, work out what type it is, and store
    /// it in a new [`events::EventType`] object.
    ///
    /// If there are no events, this function will return `Ok(None)`.
    pub fn get_event<'ctx>(
        &'ctx self,
        timeout: Option<std::time::Duration>,
    ) -> Result<Option<events::EventType<'ctx>>, Error> {
        let timeout_ns = timeout.map(|t| t.as_nanos()).unwrap_or(0) as u64;
        let ev = self.create_event();
        let rc = unsafe { sys::screen_get_event(self.0, ev.ptr, timeout_ns) };
        if rc == 0 {
            ev.get_type()
        } else {
            Err(std::io::Error::last_os_error().into())
        }
    }

    /// Create a new, empty, event.
    ///
    /// The event is associated with this context, to ensure the event is freed before the context is freed, to avoid
    /// dangling pointers. The C API does not enforce this rule but we do.
    fn create_event<'ctx>(&'ctx self) -> events::Event<'ctx> {
        let mut ptr: sys::screen_event_t = std::ptr::null_mut();
        let rc = unsafe { sys::screen_create_event(&mut ptr) };
        if rc == 0 {
            events::Event {
                ptr,
                _phantom: PhantomData,
            }
        } else {
            panic!("Failed to create event");
        }
    }
}

impl Drop for Context {
    /// Clean up the QNX object. We can assume the pointer is valid.
    fn drop(&mut self) {
        unsafe {
            sys::screen_destroy_context(self.0);
        }
    }
}

bitflags! {
    /// The permissions we can give to the [`Context::new`] function.
    ///
    /// ```rust
    /// # use qnxscreen::{Context, ContextType};
    /// let ctx = Context::new(ContextType::APPLICATION | ContextType::INPUT_MANGER)?;
    /// ```
    pub struct ContextType: i32 {
        const APPLICATION = sys::SCREEN_APPLICATION_CONTEXT as i32;
        const PROPERTY = sys::SCREEN_PROPERTY_CONTEXT as i32;
        const OBJECT_TYPE = sys::SCREEN_OBJECT_TYPE_CONTEXT as i32;
        const INPUT_MANAGER = sys::SCREEN_INPUT_MANAGER_CONTEXT as i32;
        const POWER_MANAGER = sys::SCREEN_POWER_MANAGER_CONTEXT as i32;
        const INPUT_PROVIDER = sys::SCREEN_INPUT_PROVIDER_CONTEXT as i32;
        const WINDOW_MANAGER = sys::SCREEN_WINDOW_MANAGER_CONTEXT as i32;
        const BUFFER_PROVIDER = sys::SCREEN_BUFFER_PROVIDER_CONTEXT as i32;
        const DISPLAY_MANAGER = sys::SCREEN_DISPLAY_MANAGER_CONTEXT as i32;
    }
}

// ============================================================================
//
// Screen Window
//
// ============================================================================

/// Represents a Screen Window.
///
/// The C type is `screen_window_t`.
#[derive(Debug)]
pub struct Window<'window> {
    ptr: sys::screen_window_t,
    owned: bool,
    _phantom: PhantomData<&'window Context>,
}

impl<'window> Window<'window> {
    /// Set the swap interval.
    ///
    /// This is the number of Vertical Blanking Intervals that must pass between
    /// window post operations.
    ///
    /// Set to 1 to get a 1:1 frame sync (i.e. 60 frames/second on a 60 Hz
    /// display).
    pub fn set_swap_interval(&self, swap_interval: i32) -> Result<(), Error> {
        let rc = unsafe {
            sys::screen_set_window_property_iv(
                self.ptr,
                sys::SCREEN_PROPERTY_SWAP_INTERVAL as i32,
                &swap_interval,
            )
        };
        map_error(rc, || Ok(()))
    }

    /// Set the window usage.
    ///
    /// Sets the [`sys::SCREEN_PROPERTY_USAGE`] property.
    pub fn set_usage(&self, usage_type: UsageType) -> Result<(), Error> {
        let value: i32 = usage_type.bits();
        let rc = unsafe {
            sys::screen_set_window_property_iv(self.ptr, sys::SCREEN_PROPERTY_USAGE as i32, &value)
        };
        map_error(rc, || Ok(()))
    }

    /// Set the ID property.
    //
    /// This appears in /dev/screen.
    pub fn set_id(&self, id: &str) -> Result<(), Error> {
        let c_string = std::ffi::CString::new(id).map_err(|_| Error::BadStringArgument)?;
        let rc = unsafe {
            sys::screen_set_window_property_cv(
                self.ptr,
                sys::SCREEN_PROPERTY_ID_STRING as i32,
                id.len() as i32,
                c_string.as_ptr(),
            )
        };
        map_error(rc, || Ok(()))
    }

    /// Create some buffers.
    ///
    /// The buffers are internal to this object, but you can get a reference to
    /// them.
    pub fn create_buffers(&self, num_buffers: i32) -> Result<(), Error> {
        let rc = unsafe { sys::screen_create_window_buffers(self.ptr, num_buffers) };
        map_error(rc, || Ok(()))
    }

    /// Set the pixel format for this window.
    ///
    /// Sets the `SCREEN_PROPERTY_FORMAT` property.
    pub fn set_screen_format(&self, pixel_format: FormatType) -> Result<(), Error> {
        let value = pixel_format as i32;
        let rc = unsafe {
            sys::screen_set_window_property_iv(self.ptr, sys::SCREEN_PROPERTY_FORMAT as i32, &value)
        };
        map_error(rc, || Ok(()))
    }

    /// Gets the size of the window.
    pub fn get_size(&self) -> Result<Rect, Error> {
        let mut dimensions = [0i32; 2];
        let rc = unsafe {
            sys::screen_get_window_property_iv(
                self.ptr,
                sys::SCREEN_PROPERTY_SIZE as i32,
                dimensions.as_mut_ptr(),
            )
        };
        map_error(rc, || {
            Ok(Rect {
                x: 0,
                y: 0,
                width: dimensions[0],
                height: dimensions[1],
            })
        })
    }

    /// Make window content updates visible on the display
    pub fn post(
        &self,
        buffer: &Buffer,
        dirty_rects: &[Rect],
        flags: PostFlags,
    ) -> Result<(), Error> {
        let rc = unsafe {
            let rect_ptr = dirty_rects.as_ptr() as *const i32;
            sys::screen_post_window(
                self.ptr,
                buffer.ptr,
                dirty_rects.len() as i32,
                rect_ptr,
                flags.bits(),
            )
        };
        map_error(rc, || Ok(()))
    }

    /// Access the buffers associated with this screen.
    pub fn get_buffers(&'window self) -> Result<Vec<Buffer<'window>>, Error> {
        // The maximum number of buffers allowed is 35.
        let mut buffers: [sys::screen_buffer_t; 35] = [std::ptr::null_mut(); 35];
        let rc = unsafe {
            sys::screen_get_window_property_pv(
                self.ptr,
                sys::SCREEN_PROPERTY_RENDER_BUFFERS as i32,
                buffers.as_mut_ptr() as *mut *mut std::ffi::c_void,
            )
        };
        map_error(rc, || {
            let mut result = Vec::new();
            for buffer_ptr in buffers.iter() {
                if !buffer_ptr.is_null() {
                    result.push(Buffer {
                        ptr: *buffer_ptr,
                        _phantom: PhantomData,
                    })
                }
            }
            Ok(result)
        })
    }
}

impl<'window> Drop for Window<'window> {
    /// Clean up the QNX object. We can assume the pointer is valid.
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                sys::screen_destroy_window(self.ptr);
            }
        }
    }
}

bitflags! {
    /// The permissions we can give to the [`Window::set_usage`] function.
    ///
    /// ```rust
    /// # use qnxscreen::{Context, ContextType, UsageType};
    /// let ctx = Context::new(ContextType::APPLICATION)?;
    /// let win = ctx.get_window()?;
    /// win.set_usage(UsageType::DISPLAY | UsageType::WRITE)?;
    /// ```
    pub struct UsageType: i32 {
        /// Flag to indicate that the CPU can read from the buffer(s) associated
        /// with the API object.
        const READ = sys::SCREEN_USAGE_READ as i32;
        /// Flag to indicate that the CPU can write to the buffer(s) associated
        /// with the API object.
        const WRITE = sys::SCREEN_USAGE_WRITE as i32;
        /// Flag to indicate that buffer(s) associated with the API object can
        /// be used for native API operations. If using blits (`screen_blit()`)
        /// or fills (`screen_fill()`), this flag must be set on the buffer.
        const NATIVE = sys::SCREEN_USAGE_NATIVE as i32;
        /// Flag to indicate that OpenGL ES 1.X is used for rendering the buffer
        /// associated with the API object.
        const OPENGL_ES1 = sys::SCREEN_USAGE_OPENGL_ES1 as i32;
        /// Flag to indicate that OpenGL ES 2.X is used for rendering the buffer
        /// associated with the API object.
        const OPENGL_ES2 = sys::SCREEN_USAGE_OPENGL_ES2 as i32;
        /// Flag to indicate that OpenGL ES 3.X is used for rendering the buffer
        /// associated with the API object.
        const OPENGL_ES3 = sys::SCREEN_USAGE_OPENGL_ES3 as i32;
        /// Flag to indicate that OpenVG is used for rendering the buffer
        /// associated with the API object.
        const OPENVG = sys::SCREEN_USAGE_OPENVG as i32;
        /// Flag to indicate that the buffer can be written to by a video decoder.
        const VIDEO = sys::SCREEN_USAGE_VIDEO as i32;
        /// Flag to indicate that the buffer can be written to by capture
        /// devices (such as cameras, analog-to-digital-converters, ...), and
        /// read by a hardware video encoder.
        const CAPTURE = sys::SCREEN_USAGE_CAPTURE as i32;
        /// Flag to indicate that the buffer can be reconfigured from landscape
        /// to portrait orientation without reallocation.
        const ROTATION = sys::SCREEN_USAGE_ROTATION as i32;
        /// Flag to suggest the use of a display overlay. Screen won't use
        /// display overlays unless your application assigns a pipeline. The
        /// `SCREEN_USAGE_OVERLAY` flag suggests that Screen tries to post your
        /// buffer onto a pipeline that doesn't have a framebuffer. If unable
        /// to, Screen will use composition instead. Changing
        /// `SCREEN_USAGE_OVERLAY` may affect pipeline assignment. Set this flag
        /// when you are targeting a non-composited pipeline.
        const OVERLAY = sys::SCREEN_USAGE_OVERLAY as i32;
        /// Flag to indicate that a buffer is compressed.
        const COMPRESSION = sys::SCREEN_USAGE_COMPRESSION as i32;
        /// Flag to indicate that the buffer should be physically contiguous and
        /// the physical address should be provided. This flag is used only as a
        /// suggestion to Screen. Query the buffer's
        /// `SCREEN_PROPERTY_PHYSICALLY_CONTIGUOUS` property to confirm whether
        /// the buffer is actually contiguous. This flag is associated with the
        /// preprocessor definition `SCREEN_USAGE_PHYSICAL`.
        const PHYSICAL = sys::SCREEN_USAGE_PHYSICAL as i32;
        /// Flag to indicate that Vulkan is used for rendering the buffer
        /// associated with the API object.
        const VULKAN = sys::SCREEN_USAGE_VULKAN as i32;
    }
}

/// The size/format of pixels we can support in a given buffer.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum FormatType {
    BYTE = sys::SCREEN_FORMAT_BYTE as i32,
    RGBA4444 = sys::SCREEN_FORMAT_RGBA4444 as i32,
    RGBX4444 = sys::SCREEN_FORMAT_RGBX4444 as i32,
    RGBA5551 = sys::SCREEN_FORMAT_RGBA5551 as i32,
    RGBX5551 = sys::SCREEN_FORMAT_RGBX5551 as i32,
    RGB565 = sys::SCREEN_FORMAT_RGB565 as i32,
    RGB888 = sys::SCREEN_FORMAT_RGB888 as i32,
    RGBA8888 = sys::SCREEN_FORMAT_RGBA8888 as i32,
    RGBX8888 = sys::SCREEN_FORMAT_RGBX8888 as i32,
    YVU9 = sys::SCREEN_FORMAT_YVU9 as i32,
    YUV420 = sys::SCREEN_FORMAT_YUV420 as i32,
    NV12 = sys::SCREEN_FORMAT_NV12 as i32,
    YV12 = sys::SCREEN_FORMAT_YV12 as i32,
    UYVY = sys::SCREEN_FORMAT_UYVY as i32,
    YUY2 = sys::SCREEN_FORMAT_YUY2 as i32,
    YVYU = sys::SCREEN_FORMAT_YVYU as i32,
    V422 = sys::SCREEN_FORMAT_V422 as i32,
    AYUV = sys::SCREEN_FORMAT_AYUV as i32,
    NV16 = sys::SCREEN_FORMAT_NV16 as i32,
    P010 = sys::SCREEN_FORMAT_P010 as i32,
    BGRA8888 = sys::SCREEN_FORMAT_BGRA8888 as i32,
    BGRX8888 = sys::SCREEN_FORMAT_BGRX8888 as i32,
    RGBA1010102 = sys::SCREEN_FORMAT_RGBA1010102 as i32,
    RGBX1010102 = sys::SCREEN_FORMAT_RGBX1010102 as i32,
    BGRA1010102 = sys::SCREEN_FORMAT_BGRA1010102 as i32,
    BGRX1010102 = sys::SCREEN_FORMAT_BGRX1010102 as i32,
}

bitflags! {
    /// The flags we can give to the [`Window::post`] function.
    pub struct PostFlags: i32 {
        /// Indicator that the function blocks until the operation has completed.
        const IDLE = sys::SCREEN_WAIT_IDLE as i32;
    }
}

// ============================================================================
//
// Screen Buffer
//
// ============================================================================

/// Represents a piece of memory we can draw on.
///
/// This piece of memory is owned by someone else. It will not free when this
/// object is dropped.
///
/// The C type is `screen_buffer_t`.
#[derive(Debug)]
pub struct Buffer<'buffer> {
    ptr: sys::screen_buffer_t,
    _phantom: PhantomData<&'buffer Window<'buffer>>,
}

impl<'buffer> Buffer<'buffer> {
    /// Gets the stride-size of this buffer, in bytes.
    pub fn get_stride(&self) -> Result<usize, Error> {
        let mut stride = 0;
        let rc = unsafe {
            sys::screen_get_buffer_property_iv(
                self.ptr,
                sys::SCREEN_PROPERTY_STRIDE as i32,
                &mut stride,
            )
        };
        map_error(rc, || Ok(stride as usize))
    }

    /// Gets the pointer for this buffer.
    pub fn get_pointer(&self) -> Result<*mut u8, Error> {
        let mut ptr: *mut u8 = std::ptr::null_mut();
        let rc = unsafe {
            sys::screen_get_buffer_property_pv(
                self.ptr,
                sys::SCREEN_PROPERTY_POINTER as i32,
                &mut ptr as *mut *mut u8 as *mut *mut _,
            )
        };
        map_error(rc, || Ok(ptr))
    }
}

// ============================================================================
//
// Rect
//
// ============================================================================

/// Describes a rectangular region of the screen.
///
/// Must be laid out as four integers in memory.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Rect {
    /// The X position of the top-left corner, in pixels
    x: i32,
    /// The Y position of the top-left corner, in pixels
    y: i32,
    /// The width in pixels
    width: i32,
    /// The height in pixels
    height: i32,
}

impl Rect {
    /// Make a new rectangle.
    ///
    /// The rectangle has the given position (of the top-left corner), and the
    /// given width and height.
    pub fn new(top_left: (i32, i32), width: i32, height: i32) -> Self {
        Rect {
            x: top_left.0,
            y: top_left.1,
            width,
            height,
        }
    }

    /// Get the x-coord of the left hand edge
    pub fn x1(&self) -> i32 {
        self.x
    }

    /// Get the x-coord of the right hand edge
    pub fn x2(&self) -> i32 {
        self.x + self.width
    }

    /// Get the y-coord of the top edge
    pub fn y1(&self) -> i32 {
        self.y
    }

    /// Get the y-coord of the bottom edge
    pub fn y2(&self) -> i32 {
        self.y + self.height
    }

    // Get the width of the rectangle
    pub fn width(&self) -> i32 {
        self.width
    }

    // Get the height of the rectangle
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Calculate the bottom right corner position.
    pub fn bottom_right(&self) -> (i32, i32) {
        (self.x2(), self.y2())
    }

    /// Move the rectangle to the given position.
    pub fn set_position(&mut self, top_left: (i32, i32)) {
        self.x = top_left.0;
        self.y = top_left.1;
    }

    /// Move the rectangle by the relative amount given.
    pub fn adjust_position(&mut self, delta_x: i32, delta_y: i32) {
        self.x += delta_x;
        self.y += delta_y;
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x{}@{}x{}", self.width, self.height, self.x, self.y)
    }
}

// ============================================================================
//
// Helper Functions
//
// ============================================================================

/// Check the result of a QNX API call and create a `Result`.
///
/// If the QNX API was succesful you get `Ok(ok_fn())`.
///
/// If the QNX API was not successful you get `Err(...)` where the error comes
/// from `std::error::last_os_error()`.
fn map_error<F, T>(code: i32, ok_fn: F) -> Result<T, Error>
where
    F: FnOnce() -> Result<T, Error>,
{
    if code == 0 {
        ok_fn()
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}

// ============================================================================
//
// End of file
//
// ============================================================================
