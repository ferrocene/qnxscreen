# bouncing-ferris

This is a very basic QNX Screen application. It places a bunch of Ferris graphics
on the screen and moves them around.

This crate was developed using the `aarch64-unknown-nto-qnx710` target in
Ferrocene, but may also work with the other Ferrocene QNX targets and with the
Tier 3 QNX targets in upstream Rust.

This crate uses `qnxscreen-sys` and that will require you to have sourced your QNX
SDP development environment (e.g. `source ~/qnx710/qnxsdp-env.sh`) before you
build it. You must also have the relevant Screen components installed using the
QNX Software Manager.

## Building for AArch64 QNX Neutrino 7.1.0 with Ferrocene

```console
$ source ~/qnx710/qnxsdp-env.sh 
$ criticalup install
$ criticalup run cargo build --target aarch64-unknown-nto-qnx710
```

You then need to copy `../target/aarch64-unknown-nto-qnx710/debug/bouncing-ferris` to a running AArch64 QNX system

## Building with Rust

To build this binary using upstream Rust, please refer to the [target documentation](https://doc.rust-lang.org/stable/rustc/platform-support/nto-qnx.html).

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
