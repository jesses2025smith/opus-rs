use std::{env, path, fs};

fn main() {
    let root = path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let opus_path = root.join("opus");

    build_opus_from_source(&opus_path);

    let bindings = bindgen::Builder::default()
        .header(opus_path.join("include/opus.h").to_string_lossy().into_owned())
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

fn build_opus_from_source(opus_path: &path::Path) {
    let mut build = cc::Build::new();

    build.include(opus_path.join("include"))
        .include(opus_path.join("src"))
        .include(opus_path.join("celt"))
        .include(opus_path.join("silk"))
        .include(opus_path.join("silk/float"))
        .define("OPUS_BUILD", None)
        .define("USE_ALLOCA", None)
        .define("HAVE_LRINT", None)
        .define("HAVE_LRINTF", None)
        .opt_level(2)
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-maybe-uninitialized")
        .flag("-Wno-sign-compare")
        .flag("-Wno-pragmas")
        ;
 
    add_c_files(&mut build, &opus_path.join("src"));
    add_c_files(&mut build, &opus_path.join("celt"));
    add_c_files(&mut build, &opus_path.join("silk"));
    add_c_files(&mut build, &opus_path.join("silk/float"));
    
    build.compile("opus");
}

fn add_c_files(build: &mut cc::Build, dir: &path::Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "c") {
                build.file(path);
            }
        }
    }
}