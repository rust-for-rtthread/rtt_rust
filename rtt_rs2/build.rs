extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let rtt_path = env::var("RTT_PATH");
    let rtt_path = match rtt_path {
        Ok(x) => x,
        Err(_) => return,
    };

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .ctypes_prefix("c_types")
        .clang_arg(format!("-I{}/", rtt_path))
        .clang_arg(format!("-I{}/rt-thread/{}", rtt_path, "include"))
        .clang_arg(format!("-I{}/rt-thread/{}", rtt_path, "components/finsh/"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
