use std::path::Path;

fn main() {
    // Ensure git submodules are expanded
    if !Path::new("ext/vuk/ext/SPIRV-Cross/include").exists() {
        panic!("Could not verify submodules were correctly initialized -- Did you forget to `git submodule update --init --recursive`?");
    }
}
