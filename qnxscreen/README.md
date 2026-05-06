# qnxscreen

This is a high-level wrapper for the QNX Screen API. You can make a screen
context object, then create window objects using that, then call various methods
on your window objects.

```rust
let ctx = qnxscreen::Context::new(
    qnxscreen::ContextType::APPLICATION | qnxscreen::ContextType::INPUT_MANAGER,
)?;

let window = ctx.create_window()?;
window.set_swap_interval(1)?;
window.set_usage(qnxscreen::UsageType::NATIVE | qnxscreen::UsageType::WRITE)?;
window.create_buffers(2)?;
window.set_screen_format(qnxscreen::FormatType::RGBA8888)?;
let window_rect = window.get_rect()?;
println!("Screen size: {}", window_rect);
let buffers = window.get_buffers()?;
```

This crate was developed using the `aarch64-unknown-nto-qnx710` target in
Ferrocene, but may also work with the other Ferrocene QNX targets and with the
Tier 3 QNX targets in upstream Rust.

This crate uses `qnxscreen-sys` and that will require you to have sourced your QNX
SDP development environment (e.g. `source ~/qnx710/qnxsdp-env.sh`) before you
build it. You must also have the relevant Screen components installed using the
QNX Software Manager.

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
