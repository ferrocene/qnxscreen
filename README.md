# Rust support for the QNX Screen API

This repository contains work-in-progress support for writing Rust programs that
use the QNX Screen API.

It is targeted at QNX SDP 7.1, and is tested using the Raspberry Pi 4 BSP.

## Components

### qnxscreen

This is a high-level wrapper for the QNX Screen API. You can make a screen
context object, then create window objects using that, then call various methods
on your window objects.

### qnxscreen-sys

This is a low-level wrapper, created by running
[bindgen](https://crates.io/crates/bindgen) against the QNX Screen API header
file.

### qnx-egl-sys

This is a low-level wrapper, created by running
[gl_generator](https://crates.io/crates/gl_generator) to produce an EGL 1.5 Rust API.

### bouncing-ferris

This is an example application which moves a Ferris image smoothly around the
screen.

### win-vsync

A Rust port of the QNX C demo of the same name. Shows multiple moving windows on-screen.

### gles-triangle

Draws a triangle on-screen using `qnxscreen` and `qnx-egl-sys`.

## qnxscreen Changelog

### Unreleased Changes

* Add [`KEYCODE_xx`] to `EventKeyboard` along with `get_sym()`, `get_keycap()`, etc.
* `Window`, `Buffer` and `Context` now hold `std::ptr::NonNull<T>`.
* Add child window support.
* Add support for setting window position and colour.
* Add API for getting *Display* information

### v0.5.0

* First release.
   * Supports basic window operations, and pumping events.

## qnxscreen-sys Changelog

### Unreleased Changes

* None

### v0.5.0

* First release.

## qnx-egl-sys Changelog

### Unrelease Changes

* First version

## Licence

* Copyright (c) Ferrous Systems, 2023
* Copyright (c) AdaCore, 2023
* Copyright (c) SixtyFPS GmbH, 2023

These libraries and examples are distributed under the terms of both the [MIT
license](./LICENSE-MIT) and the [Apache License (Version
2.0)](./LICENCE-APACHE). However portions are derived from the QNX SDP 7.1
header files and so you must also refer to your licence agreement with QNX.



