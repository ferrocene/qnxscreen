# CHANGELOG

This is the CHANGELOG for the published libraries in this repository. Changes for
the example binaries are not tracked.

## qnxscreen Changelog

### Unreleased Changes

### v0.6.0

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

### Unreleased Changes

* First version
