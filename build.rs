// build.rs - required because we have links = "picmlx" in Cargo.toml
use std::env;

fn main() {
    let cb_base_dir = env::var("ClientBridge64").unwrap();
    let cb_lib_path = format!("{}\\Lib\\MSC",cb_base_dir);
    // this magic is required to link picmlx_w64.lib
    println!("cargo:rustc-link-lib=picmlx_w64");
    println!("cargo:rustc-link-lib=piplx_w64");
    println!("cargo:rustc-link-search=native={}",cb_lib_path);
    // this just causes rebuild when this script changes, see
    // https://doc.rust-lang.org/cargo/reference/build-script-examples.html
    println!("cargo:rerun-if-changed=build.rs");
}
