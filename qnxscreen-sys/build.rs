//! qnxscreen-sys build script.
//!
//! 1. Tells cargo we need to link against `libscreen.so`, which is the QNX C
//!    library containing this API.
//! 2. Runs bindgen.
//!
//! Copyright (c) Ferrous Systems, 2023

use std::env;
use std::path::PathBuf;

fn main() {
    // Path to QNX
    let qnx_target = std::env::var("QNX_TARGET").expect("QNX_TARGET not set");
    let cargo_target = std::env::var("TARGET").expect("TARGET is not set");
    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH not set")
        .as_str()
    {
        "aarch64" => "aarch64le",
        unknown => {
            panic!("Unhandled arch {:?}", unknown);
        }
    };

    let lib_path: std::path::PathBuf = [&qnx_target, arch, "lib"].iter().collect();
    let usr_lib_path: std::path::PathBuf = [&qnx_target, arch, "usr", "lib"].iter().collect();
    let include_path: std::path::PathBuf = [&qnx_target, "usr", "include"].iter().collect();

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-search={}", usr_lib_path.display());
    println!("cargo:rustc-link-search={}", include_path.display());

    // Tell cargo to tell rustc to link the system `libscreen.so` library.
    println!("cargo:rustc-link-lib=screen");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell it where the QNX source lives
        .clang_arg(format!("-I{}", include_path.display()))
        // Tell it about the CPU architecture
        .clang_arg(format!("--target={}", cargo_target))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Only include the screen API
        .allowlist_type("screen_.*")
        .allowlist_function("screen_.*")
        .allowlist_var("SCREEN_.*")
        .allowlist_var("KEYMOD.*")
        .allowlist_var("KEY_.*")
        .allowlist_var("KEYCODE_.*")
        .allowlist_var("KEYIND_.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
