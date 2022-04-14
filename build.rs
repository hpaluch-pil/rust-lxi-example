// build.rs - required because we have links = "picmlx" in Cargo.toml
use std::env;

fn build_windows() {
    let cb_base_dir = env::var("ClientBridge64").unwrap();
    let cb_lib_path = format!("{}\\Lib\\MSC",cb_base_dir);
    // this magic is required to link picmlx_w64.lib
    println!("cargo:rustc-link-lib=picmlx_w64");
    println!("cargo:rustc-link-lib=piplx_w64");
    println!("cargo:rustc-link-search=native={}",cb_lib_path);
}

fn build_unix() {
    println!("cargo:rustc-link-lib=picmlx");
    println!("cargo:rustc-link-lib=piplx");
}

fn main() {
    let os_family = env::var("CARGO_CFG_TARGET_FAMILY")
        .expect("The CARGO_CFG_TARGET_FAMILY is not set");

    if os_family == "windows" {
        build_windows();
    } else if os_family == "unix" {
        build_unix();
    } else {
        panic!("Unknown OS Family '{}'",os_family);
    }

    // this just causes rebuild when this script changes, see
    // https://doc.rust-lang.org/cargo/reference/build-script-examples.html
    println!("cargo:rerun-if-changed=build.rs");
}
