// This is the input to bindgen.

// We need to define these macros so that the headers contain all the values
// we need.
#define __LITTLEENDIAN__
#define __QNXNTO__

// This is the QNX Screen API
#include <screen/screen.h>
// This contains KEYCODE_XX macros for decoding Keyboard Events
#include <sys/keycodes.h>
