use std::{env, path};

fn main() {
    let root = path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let opus_path = root.join("opus");

    let bindings = bindgen::Builder::default()
        .header(opus_path.join("include/opus_custom.h").to_string_lossy().into_owned())
        .header(opus_path.join("include/opus_multistream.h").to_string_lossy().into_owned())
        .header(opus_path.join("include/opus_projection.h").to_string_lossy().into_owned())
        .clang_arg("-I").clang_arg(opus_path.join("include").to_string_lossy())
        .clang_arg("-I").clang_arg(opus_path.join("src").to_string_lossy())
        .derive_debug(true)
        .derive_default(true)
        // .allowlist_recursively(false)
        .generate()
        .expect("Unable to generate bindings");

    let output_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(output_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
