//! # Screen Event handling

use std::marker::PhantomData;

use bitflags::bitflags;
use num_traits::FromPrimitive;

use crate::{map_error, sys, Error};

pub mod property;

// ============================================================================
//
// Event
//
// ============================================================================

/// A handle to a Screen event
pub struct Event<'ctx> {
    pub(crate) ptr: sys::screen_event_t,
    pub(crate) _phantom: PhantomData<&'ctx sys::screen_context_t>,
}

impl<'ctx> Event<'ctx> {
    /// Get the type of event.
    ///
    /// The QNX value `SCREEN_EVENT_NONE` is mapped to `Option::None`.
    pub fn get_type(self) -> Result<Option<EventType<'ctx>>, Error> {
        let mut ev_type = 0;
        let rc = unsafe {
            sys::screen_get_event_property_iv(
                self.ptr,
                sys::SCREEN_PROPERTY_TYPE as i32,
                &mut ev_type,
            )
        };
        map_error(rc, || Ok(()))?;
        Ok(match ev_type as u32 {
            sys::SCREEN_EVENT_NONE => None,
            sys::SCREEN_EVENT_CREATE => Some(EventType::Create(EventCreate(self))),
            sys::SCREEN_EVENT_PROPERTY => Some(EventType::Property(EventProperty(self))),
            sys::SCREEN_EVENT_CLOSE => Some(EventType::Close(EventClose(self))),
            sys::SCREEN_EVENT_INPUT => Some(EventType::Input(EventInput(self))),
            sys::SCREEN_EVENT_JOG => Some(EventType::Jog(EventJog(self))),
            sys::SCREEN_EVENT_POINTER => Some(EventType::Pointer(EventPointer(self))),
            sys::SCREEN_EVENT_KEYBOARD => Some(EventType::Keyboard(EventKeyboard(self))),
            sys::SCREEN_EVENT_USER => Some(EventType::User(EventUser(self))),
            sys::SCREEN_EVENT_POST => Some(EventType::Post(EventPost(self))),
            sys::SCREEN_EVENT_DISPLAY => Some(EventType::Display(EventDisplay(self))),
            sys::SCREEN_EVENT_IDLE => Some(EventType::Idle(EventIdle(self))),
            sys::SCREEN_EVENT_UNREALIZE => Some(EventType::Unrealize(EventUnrealize(self))),
            sys::SCREEN_EVENT_GAMEPAD => Some(EventType::Gamepad(EventGamepad(self))),
            sys::SCREEN_EVENT_JOYSTICK => Some(EventType::Joystick(EventJoystick(self))),
            sys::SCREEN_EVENT_DEVICE => Some(EventType::Device(EventDevice(self))),
            sys::SCREEN_EVENT_INPUT_CONTROL => {
                Some(EventType::InputControl(EventInputControl(self)))
            }
            sys::SCREEN_EVENT_GESTURE => Some(EventType::Gesture(EventGesture(self))),
            sys::SCREEN_EVENT_MANAGER => Some(EventType::Manager(EventManager(self))),
            sys::SCREEN_EVENT_MTOUCH_PRETOUCH => {
                Some(EventType::MultiTouchPretouch(EventMultiTouchPretouch(self)))
            }
            sys::SCREEN_EVENT_MTOUCH_TOUCH => {
                Some(EventType::MultiTouchTouch(EventMultiTouchTouch(self)))
            }
            sys::SCREEN_EVENT_MTOUCH_MOVE => {
                Some(EventType::MultiTouchMove(EventMultiTouchMove(self)))
            }
            sys::SCREEN_EVENT_MTOUCH_RELEASE => {
                Some(EventType::MultiTouchRelease(EventMultiTouchRelease(self)))
            }
            _ => return Err(Error::UnknownEvent(ev_type)),
        })
    }

    /// Get an integer property from this event.
    fn get_integer(&self, property: i32) -> Result<i32, Error> {
        let mut value = 0;
        let rc = unsafe { sys::screen_get_event_property_iv(self.ptr, property, &mut value) };
        map_error(rc, || Ok(value))
    }

    /// Get an 2-tuple property from this event.
    ///
    /// Typically used for X/Y positions.
    fn get_two_integers(&self, property: i32) -> Result<(i32, i32), Error> {
        let mut values = [0, 0];
        let rc =
            unsafe { sys::screen_get_event_property_iv(self.ptr, property, values.as_mut_ptr()) };
        map_error(rc, || Ok((values[0], values[1])))
    }

    /// Get an pointer property from this event.
    fn get_pointer(&self, property: i32) -> Result<*mut std::ffi::c_void, Error> {
        let mut ptr = std::ptr::null_mut();
        let rc = unsafe { sys::screen_get_event_property_pv(self.ptr, property, &mut ptr) };
        map_error(rc, || Ok(ptr))
    }
}

impl<'ctx> Drop for Event<'ctx> {
    fn drop(&mut self) {
        let rc = unsafe { sys::screen_destroy_event(self.ptr) };
        if rc != 0 {
            panic!("Failed to drop event");
        }
    }
}

// ============================================================================
//
// EventType
//
// ============================================================================

/// The set of types of events that we know about.
pub enum EventType<'ctx> {
    Create(EventCreate<'ctx>),
    Property(EventProperty<'ctx>),
    Close(EventClose<'ctx>),
    Input(EventInput<'ctx>),
    Jog(EventJog<'ctx>),
    Pointer(EventPointer<'ctx>),
    Keyboard(EventKeyboard<'ctx>),
    User(EventUser<'ctx>),
    Post(EventPost<'ctx>),
    Display(EventDisplay<'ctx>),
    Idle(EventIdle<'ctx>),
    Unrealize(EventUnrealize<'ctx>),
    Gamepad(EventGamepad<'ctx>),
    Joystick(EventJoystick<'ctx>),
    Device(EventDevice<'ctx>),
    InputControl(EventInputControl<'ctx>),
    Gesture(EventGesture<'ctx>),
    Manager(EventManager<'ctx>),
    MultiTouchPretouch(EventMultiTouchPretouch<'ctx>),
    MultiTouchTouch(EventMultiTouchTouch<'ctx>),
    MultiTouchMove(EventMultiTouchMove<'ctx>),
    MultiTouchRelease(EventMultiTouchRelease<'ctx>),
}

// ============================================================================
//
// EventCreate
//
// ============================================================================

/// Represents a `SCREEN_EVENT_CREATE`.
pub struct EventCreate<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventProperty
//
// ============================================================================

/// Represents a `SCREEN_EVENT_PROPERTY`.
pub struct EventProperty<'ctx>(Event<'ctx>);

impl<'ctx> EventProperty<'ctx> {
    pub fn get_name(&self) -> Result<property::Name, Error> {
        let int_value = self.0.get_integer(sys::SCREEN_PROPERTY_NAME as i32)? as u32;
        match property::Name::from_u32(int_value) {
            Some(epn) => Ok(epn),
            None => Err(Error::UnknownPropertyName(int_value)),
        }
    }
}

// ============================================================================
//
// EventClose
//
// ============================================================================

/// Represents a `SCREEN_EVENT_CLOSE`.
pub struct EventClose<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventInput
//
// ============================================================================

/// Represents a `SCREEN_EVENT_INPUT`.
pub struct EventInput<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventJog
//
// ============================================================================

/// Represents a `SCREEN_EVENT_JOG`.
pub struct EventJog<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventPointer
//
// ============================================================================

/// Represents a `SCREEN_EVENT_POINTER`.
pub struct EventPointer<'ctx>(Event<'ctx>);

impl<'ctx> EventPointer<'ctx> {
    /// A bitmask indicating which buttons are pressed.
    pub fn get_buttons(&self) -> Result<MouseButtons, Error> {
        self.0
            .get_integer(sys::SCREEN_PROPERTY_BUTTONS as i32)
            .map(|val| unsafe { MouseButtons::from_bits_unchecked(val) })
    }

    /// The input device index for this Pointer Event.
    pub fn get_device(&self) -> Result<i32, Error> {
        self.0.get_integer(sys::SCREEN_PROPERTY_DEVICE as i32)
    }

    /// A single integer that indicates the modifiers associated with this
    /// Pointer Event.
    pub fn get_modifiers(&self) -> Result<i32, Error> {
        self.0.get_integer(sys::SCREEN_PROPERTY_MODIFIERS as i32)
    }

    /// A single integer that indicates the number and direction of mouse wheel
    /// ticks in a horizontal direction. Wheel ticks toward the left are
    /// indicated by a negative (-) number, while those toward the right are
    /// indicated by a positive (+) one.
    pub fn get_mouse_horizontal_wheel(&self) -> Result<i32, Error> {
        self.0
            .get_integer(sys::SCREEN_PROPERTY_MOUSE_HORIZONTAL_WHEEL as i32)
    }

    /// A single integer that indicates the number and direction of mouse wheel
    /// ticks in a vertical direction. Wheel ticks in an upward direction are
    /// indicated by a negative (-) number, while those in the downward
    /// direction are indicated by a positive (+) one.
    pub fn get_mouse_wheel(&self) -> Result<i32, Error> {
        self.0.get_integer(sys::SCREEN_PROPERTY_MOUSE_WHEEL as i32)
    }

    /// Get the screen coordinates associated with the Pointer Event.
    ///
    /// Returns `(x, y)` position as a tuple.
    pub fn get_position(&self) -> Result<(i32, i32), Error> {
        self.0
            .get_two_integers(sys::SCREEN_PROPERTY_POSITION as i32)
    }

    /// Get the source viewport position associated with the Pointer Event.
    ///
    /// Returns `(x, y)` position as a tuple.
    pub fn get_source_position(&self) -> Result<(i32, i32), Error> {
        self.0
            .get_two_integers(sys::SCREEN_PROPERTY_SOURCE_POSITION as i32)
    }
}

bitflags! {
    /// Represents the buttons on a mouse.
    pub struct MouseButtons: i32 {
        /// Left mouse button is pressed.
        const LEFT = sys::SCREEN_LEFT_MOUSE_BUTTON as i32;
        /// Middle mouse button is pressed.
        const MIDDLE = sys::SCREEN_MIDDLE_MOUSE_BUTTON as i32;
        /// Right mouse button is pressed.
        const RIGHT = sys::SCREEN_RIGHT_MOUSE_BUTTON as i32;
    }
}

// ============================================================================
//
// EventKeyboard
//
// ============================================================================

/// Represents a `SCREEN_EVENT_KEYBOARD`.
pub struct EventKeyboard<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventUser
//
// ============================================================================

/// Represents a `SCREEN_EVENT_USER`.
pub struct EventUser<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventPost
//
// ============================================================================

/// Represents a `SCREEN_EVENT_POST`.
pub struct EventPost<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventDisplay
//
// ============================================================================

/// Represents a `SCREEN_EVENT_DISPLAY`.
pub struct EventDisplay<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventIdle
//
// ============================================================================

/// Represents a `SCREEN_EVENT_IDLE`.
pub struct EventIdle<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventUnrealize
//
// ============================================================================

/// Represents a `SCREEN_EVENT_UNREALIZE`.
pub struct EventUnrealize<'ctx>(Event<'ctx>);

impl<'ctx> EventUnrealize<'ctx> {
    /// Get the window associated with this event.
    ///
    /// Gives you a pointer to an existing Window, so we do not want to free the
    /// window object when the object goes out of scope.
    pub fn get_window(&self) -> Result<crate::Window, Error> {
        let ptr = self.0.get_pointer(sys::SCREEN_PROPERTY_WINDOW as i32)?;
        Ok(crate::Window {
            ptr: ptr as sys::screen_window_t,
            owned: false,
            _phantom: PhantomData,
        })
    }
}

// ============================================================================
//
// EventGamepad
//
// ============================================================================

/// Represents a `SCREEN_EVENT_GAMEPAD`.
pub struct EventGamepad<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventJoystick
//
// ============================================================================

/// Represents a `SCREEN_EVENT_JOYSTICK`.
pub struct EventJoystick<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventDevice
//
// ============================================================================

/// Represents a `SCREEN_EVENT_DEVICE`.
pub struct EventDevice<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventInputControl
//
// ============================================================================

/// Represents a `SCREEN_EVENT_INPUTCONTROL`.
pub struct EventInputControl<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventGesture
//
// ============================================================================

/// Represents a `SCREEN_EVENT_GESTURE`.
pub struct EventGesture<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventManager
//
// ============================================================================

/// Represents a `SCREEN_EVENT_MANAGER`.
pub struct EventManager<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventMultiTouchPretouch
//
// ============================================================================

/// Represents a `SCREEN_EVENT_MULTITOUCHPRETOUCH`.
pub struct EventMultiTouchPretouch<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventMultiTouchTouch
//
// ============================================================================

/// Represents a `SCREEN_EVENT_MULTITOUCHTOUCH`.
pub struct EventMultiTouchTouch<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventMultiTouchMove
//
// ============================================================================

/// Represents a `SCREEN_EVENT_MULTITOUCHMOVE`.
pub struct EventMultiTouchMove<'ctx>(Event<'ctx>);

// ============================================================================
//
// EventMultiTouchRelease
//
// ============================================================================

/// Represents a `SCREEN_EVENT_MULTITOUCHRELEASE`.
pub struct EventMultiTouchRelease<'ctx>(Event<'ctx>);

// ============================================================================
//
// End of file
//
// ============================================================================
