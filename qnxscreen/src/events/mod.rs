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

impl<'ctx> std::fmt::Debug for EventType<'ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let label = match self {
            EventType::Create(_inner) => "EventType::Create",
            EventType::Property(_inner) => "EventType::Property",
            EventType::Close(_inner) => "EventType::Close",
            EventType::Input(_inner) => "EventType::Input",
            EventType::Jog(_inner) => "EventType::Jog",
            EventType::Pointer(_inner) => "EventType::Pointer",
            EventType::Keyboard(_inner) => "EventType::Keyboard",
            EventType::User(_inner) => "EventType::User",
            EventType::Post(_inner) => "EventType::Post",
            EventType::Display(_inner) => "EventType::Display",
            EventType::Idle(_inner) => "EventType::Idle",
            EventType::Unrealize(_inner) => "EventType::Unrealize",
            EventType::Gamepad(_inner) => "EventType::Gamepad",
            EventType::Joystick(_inner) => "EventType::Joystick",
            EventType::Device(_inner) => "EventType::Device",
            EventType::InputControl(_inner) => "EventType::InputControl",
            EventType::Gesture(_inner) => "EventType::Gesture",
            EventType::Manager(_inner) => "EventType::Manager",
            EventType::MultiTouchPretouch(_inner) => "EventType::MultiTouchPretouch",
            EventType::MultiTouchTouch(_inner) => "EventType::MultiTouchTouch",
            EventType::MultiTouchMove(_inner) => "EventType::MultiTouchMove",
            EventType::MultiTouchRelease(_inner) => "EventType::MultiTouchRelease",
        };
        write!(f, "{}", label)
    }
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

impl<'ctx> EventKeyboard<'ctx> {
    pub const KEYCODE_PAUSE: u32 = sys::KEYCODE_PAUSE as u32;
    pub const KEYCODE_SCROLL_LOCK: u32 = sys::KEYCODE_SCROLL_LOCK as u32;
    pub const KEYCODE_PRINT: u32 = sys::KEYCODE_PRINT as u32;
    pub const KEYCODE_SYSREQ: u32 = sys::KEYCODE_SYSREQ as u32;
    pub const KEYCODE_BREAK: u32 = sys::KEYCODE_BREAK as u32;

    pub const KEYCODE_ESCAPE: u32 = sys::KEYCODE_ESCAPE as u32;
    pub const KEYCODE_BACKSPACE: u32 = sys::KEYCODE_BACKSPACE as u32;
    pub const KEYCODE_TAB: u32 = sys::KEYCODE_TAB as u32;
    pub const KEYCODE_BACK_TAB: u32 = sys::KEYCODE_BACK_TAB as u32;
    pub const KEYCODE_RETURN: u32 = sys::KEYCODE_RETURN as u32;
    pub const KEYCODE_CAPS_LOCK: u32 = sys::KEYCODE_CAPS_LOCK as u32;
    pub const KEYCODE_LEFT_SHIFT: u32 = sys::KEYCODE_LEFT_SHIFT as u32;
    pub const KEYCODE_RIGHT_SHIFT: u32 = sys::KEYCODE_RIGHT_SHIFT as u32;
    pub const KEYCODE_LEFT_CTRL: u32 = sys::KEYCODE_LEFT_CTRL as u32;
    pub const KEYCODE_RIGHT_CTRL: u32 = sys::KEYCODE_RIGHT_CTRL as u32;
    pub const KEYCODE_LEFT_ALT: u32 = sys::KEYCODE_LEFT_ALT as u32;
    pub const KEYCODE_RIGHT_ALT: u32 = sys::KEYCODE_RIGHT_ALT as u32;
    pub const KEYCODE_MENU: u32 = sys::KEYCODE_MENU as u32;
    pub const KEYCODE_LEFT_HYPER: u32 = sys::KEYCODE_LEFT_HYPER as u32;
    pub const KEYCODE_RIGHT_HYPER: u32 = sys::KEYCODE_RIGHT_HYPER as u32;

    pub const KEYCODE_INSERT: u32 = sys::KEYCODE_INSERT as u32;
    pub const KEYCODE_HOME: u32 = sys::KEYCODE_HOME as u32;
    pub const KEYCODE_PG_UP: u32 = sys::KEYCODE_PG_UP as u32;
    pub const KEYCODE_DELETE: u32 = sys::KEYCODE_DELETE as u32;
    pub const KEYCODE_END: u32 = sys::KEYCODE_END as u32;
    pub const KEYCODE_PG_DOWN: u32 = sys::KEYCODE_PG_DOWN as u32;
    pub const KEYCODE_LEFT: u32 = sys::KEYCODE_LEFT as u32;
    pub const KEYCODE_RIGHT: u32 = sys::KEYCODE_RIGHT as u32;
    pub const KEYCODE_UP: u32 = sys::KEYCODE_UP as u32;
    pub const KEYCODE_DOWN: u32 = sys::KEYCODE_DOWN as u32;

    pub const KEYCODE_NUM_LOCK: u32 = sys::KEYCODE_NUM_LOCK as u32;
    pub const KEYCODE_KP_PLUS: u32 = sys::KEYCODE_KP_PLUS as u32;
    pub const KEYCODE_KP_MINUS: u32 = sys::KEYCODE_KP_MINUS as u32;
    pub const KEYCODE_KP_MULTIPLY: u32 = sys::KEYCODE_KP_MULTIPLY as u32;
    pub const KEYCODE_KP_DIVIDE: u32 = sys::KEYCODE_KP_DIVIDE as u32;
    pub const KEYCODE_KP_ENTER: u32 = sys::KEYCODE_KP_ENTER as u32;
    pub const KEYCODE_KP_HOME: u32 = sys::KEYCODE_KP_HOME as u32;
    pub const KEYCODE_KP_UP: u32 = sys::KEYCODE_KP_UP as u32;
    pub const KEYCODE_KP_PG_UP: u32 = sys::KEYCODE_KP_PG_UP as u32;
    pub const KEYCODE_KP_LEFT: u32 = sys::KEYCODE_KP_LEFT as u32;
    pub const KEYCODE_KP_FIVE: u32 = sys::KEYCODE_KP_FIVE as u32;
    pub const KEYCODE_KP_RIGHT: u32 = sys::KEYCODE_KP_RIGHT as u32;
    pub const KEYCODE_KP_END: u32 = sys::KEYCODE_KP_END as u32;
    pub const KEYCODE_KP_DOWN: u32 = sys::KEYCODE_KP_DOWN as u32;
    pub const KEYCODE_KP_PG_DOWN: u32 = sys::KEYCODE_KP_PG_DOWN as u32;
    pub const KEYCODE_KP_INSERT: u32 = sys::KEYCODE_KP_INSERT as u32;
    pub const KEYCODE_KP_DELETE: u32 = sys::KEYCODE_KP_DELETE as u32;

    pub const KEYCODE_F1: u32 = sys::KEYCODE_F1 as u32;
    pub const KEYCODE_F2: u32 = sys::KEYCODE_F2 as u32;
    pub const KEYCODE_F3: u32 = sys::KEYCODE_F3 as u32;
    pub const KEYCODE_F4: u32 = sys::KEYCODE_F4 as u32;
    pub const KEYCODE_F5: u32 = sys::KEYCODE_F5 as u32;
    pub const KEYCODE_F6: u32 = sys::KEYCODE_F6 as u32;
    pub const KEYCODE_F7: u32 = sys::KEYCODE_F7 as u32;
    pub const KEYCODE_F8: u32 = sys::KEYCODE_F8 as u32;
    pub const KEYCODE_F9: u32 = sys::KEYCODE_F9 as u32;
    pub const KEYCODE_F10: u32 = sys::KEYCODE_F10 as u32;
    pub const KEYCODE_F11: u32 = sys::KEYCODE_F11 as u32;
    pub const KEYCODE_F12: u32 = sys::KEYCODE_F12 as u32;

    /// The input device index for this Keyboard Event.
    pub fn get_device(&self) -> Result<i32, Error> {
        self.0.get_integer(sys::SCREEN_PROPERTY_DEVICE as i32)
    }

    /// A single integer that indicates the keyboard cap associated with an
    /// event.
    ///
    /// If you press the `Q` key on a US layout keyboard you will get `0x0071`
    /// which is the Unicode character *Latin Small Letter Q*. If you hold down
    /// Shift and press the `Q` key, you still get `0x0071`.
    ///
    /// Special keys, like *Up Arrow* or *Page Down* get a value in the `0xF000`
    /// *Private Use Area* range. See [`Self::KEYCODE_F1`], etc.
    pub fn get_keycap(&self) -> Result<u32, Error> {
        self.0
            .get_integer(sys::SCREEN_PROPERTY_KEY_CAP as i32)
            .map(|val| val as u32)
    }

    /// Get the bitmask of keyboard modifiers (Shift, Control, etc) currently
    /// set.
    pub fn get_modifiers(&self) -> Result<KeyboardModifiers, Error> {
        self.0
            .get_integer(sys::SCREEN_PROPERTY_MODIFIERS as i32)
            .map(|val| unsafe { KeyboardModifiers::from_bits_unchecked(val) })
    }

    /// A single integer that indicates the scan code associated with an event.
    pub fn get_scan(&self) -> Result<i32, Error> {
        self.0.get_integer(sys::SCREEN_PROPERTY_SCAN as i32)
    }

    /// A single integer that indicates the symbol associated with an event.
    ///
    /// This differs from [`Self::get_keycap`] in that if reflects the state of
    /// your current modifiers.
    ///  
    /// If you press the `Q` key you will get `0x0071` which is the Unicode
    /// character *Latin Small Letter Q*. If you hold down *Shift*, or turn on
    /// *Caps Lock*, and then press the `Q` key, you get `0x0051` which is *Latin
    /// Capital Letter Q*.
    ///
    /// Special keys, like *Up Arrow* or *Page Down* get a value in the `0xF000`
    /// *Private Use Area* range. See [`Self::KEYCODE_F1`], etc.
    pub fn get_sym(&self) -> Result<Option<char>, Error> {
        Ok(char::from_u32(
            self.0.get_integer(sys::SCREEN_PROPERTY_SYM as i32)? as u32,
        ))
    }
}

bitflags! {
    /// Represents the modifier keys on a keyboard.
    ///
    /// Values are from the QNX header file `keycodes.h`.
    pub struct KeyboardModifiers: i32 {
        /// Shift is pressed
        const SHIFT = sys::KEYMOD_SHIFT as i32;
        /// Ctrl is pressed
        const CTRL = sys::KEYMOD_CTRL as i32;
        /// Alt is pressed
        const ALT = sys::KEYMOD_ALT as i32;
        /// Altgr is pressed
        const ALTGR = sys::KEYMOD_ALTGR as i32;
        /// Shl3 is pressed
        const SHL3 = sys::KEYMOD_SHL3 as i32;
        /// Mod6 is pressed
        const MOD6 = sys::KEYMOD_MOD6 as i32;
        /// Mod7 is pressed
        const MOD7 = sys::KEYMOD_MOD7 as i32;
        /// Mod8 is pressed
        const MOD8 = sys::KEYMOD_MOD8 as i32;
        /// Shift Lock is pressed
        const SHIFT_LOCK = sys::KEYMOD_SHIFT_LOCK as i32;
        /// Control Lock is pressed
        const CTRL_LOCK = sys::KEYMOD_CTRL_LOCK as i32;
        /// Alt Lock is pressed
        const ALT_LOCK = sys::KEYMOD_ALT_LOCK as i32;
        /// AltGr Lock is pressed
        const ALTGR_LOCK = sys::KEYMOD_ALTGR_LOCK as i32;
        /// SHL3_LOCK is pressed?
        const SHL3_LOCK = sys::KEYMOD_SHL3_LOCK as i32;
        /// MOD6_LOCK is pressed?
        const MOD6_LOCK = sys::KEYMOD_MOD6_LOCK as i32;
        /// MOD7_LOCK is pressed?
        const MOD7_LOCK = sys::KEYMOD_MOD7_LOCK as i32;
        /// MOD8_LOCK is pressed?
        const MOD8_LOCK = sys::KEYMOD_MOD8_LOCK as i32;
        /// Caps Lock is pressed
        const CAPS_LOCK = sys::KEYMOD_CAPS_LOCK as i32;
        /// Num Lock is pressed
        const NUM_LOCK = sys::KEYMOD_NUM_LOCK as i32;
        /// Scroll Lock is pressed
        const SCROLL_LOCK = sys::KEYMOD_SCROLL_LOCK as i32;
    }
}

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

impl<'ctx> EventPost<'ctx> {
    /// Get the window associated with this event.
    ///
    /// Gives you a pointer to an existing Window, so we do not want to free the
    /// window object when the object goes out of scope.
    pub fn get_window(&self) -> Result<crate::Window, Error> {
        let ptr = self.0.get_pointer(sys::SCREEN_PROPERTY_WINDOW as i32)?;
        let Some(nn_ptr) = std::ptr::NonNull::new(ptr as *mut sys::_screen_window) else {
            return Err(Error::NullPointer);
        };
        Ok(crate::Window {
            ptr: nn_ptr,
            owned: false,
            _phantom: PhantomData,
        })
    }
}

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
        let Some(nn_ptr) = std::ptr::NonNull::new(ptr as *mut sys::_screen_window) else {
            return Err(Error::NullPointer);
        };
        Ok(crate::Window {
            ptr: nn_ptr,
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
