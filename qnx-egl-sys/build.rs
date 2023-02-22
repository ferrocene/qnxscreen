fn main() {
    let dest = std::path::PathBuf::from(&std::env::var("OUT_DIR").unwrap());

    let mut file = std::fs::File::create(dest.join("egl_bindings.rs")).unwrap();
    let reg = gl_generator::Registry::new(
        gl_generator::Api::Egl,
        (1, 5),
        gl_generator::Profile::Core,
        gl_generator::Fallbacks::All,
        [],
    );

    reg.write_bindings(gl_generator::StaticGenerator, &mut file)
        .unwrap()
}
