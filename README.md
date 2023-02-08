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

### bouncing-ferris

This is an example application which moves a Ferris image smoothly around the
screen.

## qnxscreen Changelog

### Unreleased Changes

* None

### v0.5.1

* Add [`KEYCODE_xx`] to `EventKeyboard` along with `get_sym()`, `get_keycap()`, etc.
* `Window`, `Buffer` and `Context` now hold `std::ptr::NonNull<T>`.

### v0.5.0

* First release.
   * Supports basic window operations, and pumping events.

## qnxscreen-sys Changelog

### Unreleased Changes

* None

### v0.5.0

* First release.

## Licence

* Copyright (c) Ferrous Systems, 2023
* Copyright (c) AdaCore, 2023

These libraries are distributed under the terms of both the [MIT
license](./LICENSE-MIT) and the [Apache License (Version
2.0)](./LICENCE-APACHE). However portions are derived from the QNX SDP 7.1
header files and so you must also refer to your licence agreement with QNX.
