fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch == "wasm32" {
        return;
    }

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::fs::create_dir_all(format!("{crate_dir}/include")).unwrap();

    // Parse ffi.rs directly so cbindgen finds the extern "C" functions
    // without needing to expand cfg-gated module declarations in lib.rs.
    cbindgen::Builder::new()
        .with_src(format!("{crate_dir}/src/ffi.rs"))
        .with_language(cbindgen::Language::C)
        .with_include_guard("IS_EVEN_H")
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(format!("{crate_dir}/include/is_even.h"));
}
