#!/bin/bash

# Requires you to build Rust from source with the new QNX libstd target
# and link it to rustup as `+qnx`.

# Also requires QNX SDP 7.1 installed in ~/qnx710

set -euo pipefail

source ~/qnx710/qnxsdp-env.sh
cargo +qnx build --release --target=aarch64-unknown-nto-qnx710

