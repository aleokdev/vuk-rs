use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Ensure git submodules are expanded
    if !Path::new("ext/vuk/ext/SPIRV-Cross/include").exists() {
        panic!("Could not verify submodules were correctly initialized -- Did you forget to `git submodule update --init --recursive`?");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_args(&["-Iext/vuk/include/", "-std=c++20"])
        .allowlist_type("vuk::.*")
        .opaque_type("std::.*")
        .opaque_type("vuk::Result.*")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .enable_cxx_namespaces()
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
