# Rust support for the QNX Screen API

This repository contains work-in-progress support for writing Rust programs that
use the QNX Screen API.

It is targeted at QNX SDP 7.1, and is tested using the Raspberry Pi 4 BSP.

## Components

### [qnxscreen](./qnxscreen/)

This is a high-level wrapper for the QNX Screen API. You can make a screen
context object, then create window objects using that, then call various methods
on your window objects.

### [qnxscreen-sys](./qnxscreen-sys/)

This is a low-level wrapper, created by running
[bindgen](https://crates.io/crates/bindgen) against the QNX Screen API header
file.

### [qnx-egl-sys](./qnx-egl-sys)

This is a low-level wrapper, created by running
[gl_generator](https://crates.io/crates/gl_generator) to produce an EGL 1.5 Rust API.

### [bouncing-ferris](./bouncing-ferris)

This is an example application which moves a Ferris image smoothly around the
screen.

### [win-vsync](./win-vsync)

A Rust port of the QNX C demo of the same name. Shows multiple moving windows on-screen.

### [gles-triangle](./gles-triangle)

Draws a triangle on-screen using `qnxscreen` and `qnx-egl-sys`.

## License

* Copyright (c) Ferrous Systems
* Copyright (c) SixtyFPS GmbH

Unless otherwise stated, the software in this repository is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
    <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
