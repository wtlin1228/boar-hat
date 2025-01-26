use std::env;
use std::path::PathBuf;

fn main() {
    pkg_config::Config::new()
        .atleast_version("1.0.20")
        .print_system_libs(false)
        .probe("libsodium")
        .unwrap();
    println!("cargo::rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("sodium_init")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
