# gles-triangle

This is a very basic QNX Screen application that draws a triangle using EGL.

This crate was developed using the `aarch64-unknown-nto-qnx710` target in
Ferrocene, but may also work with the other Ferrocene QNX targets and with the
Tier 3 QNX targets in upstream Rust.

This crate uses `qnxscreen-sys` and that will require you to have sourced your QNX
SDP development environment (e.g. `source ~/qnx710/qnxsdp-env.sh`) before you
build it. You must also have the relevant Screen components installed using the
QNX Software Manager.

## Building with Ferrocene

```console
$ source ~/qnx710/qnxsdp-env.sh 
$ criticalup install
$ criticalup run cargo build --target aarch64-unknown-nto-qnx710
```

You then need to copy `../target/aarch64-unknown-nto-qnx710/debug/gles-triangle` to a running AArch64 QNX system

## Building with Rust

```console
$ source ~/qnx710/qnxsdp-env.sh 
$ cargo +nightly build --target aarch64-unknown-nto-qnx710 -Zbuild-std
```

Running the binary is as per the Ferrocene instructions.

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
