# qnx-egl-sys

This is a low-level wrapper over the QNX OpenGL APIs, created by running
[gl_generator](https://crates.io/crates/gl_generator) against the QNX OpenGL API
implementation.

This crate was developed using the `aarch64-unknown-nto-qnx710` target in
Ferrocene, but may also work with the other Ferrocene QNX targets and with the
Tier 3 QNX targets in upstream Rust.

This crate uses the QNX OpenGL libraries and that will require you to have
sourced your QNX SDP development environment (e.g. `source
~/qnx710/qnxsdp-env.sh`) before you build it. You must also have the relevant
OpenGL components installed using the QNX Software Manager.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
    <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
