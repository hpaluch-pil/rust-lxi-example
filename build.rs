// build.rs - required because we have links = "picmlx" in Cargo.toml
use std::env;

fn build_windows() {
    let platform_bits = env::var("CARGO_CFG_TARGET_POINTER_WIDTH")
        .expect("Env CARGO_CFG_TARGET_POINTER_WIDTH is not set");
    let platform_bits: u32 = platform_bits.parse().unwrap_or_else(|platform_bits| {
        panic!(
            "Env {}='{}' is not valid u32",
            "CARGO_CFG_TARGET_POINTER_WIDTH", platform_bits
        );
    });
    let cb_env_var = match platform_bits {
        64 => "ClientBridge64",
        32 => "ClientBridge",
        _ => panic!(
            "Unexpected pointer width: {} (variable {})",
            platform_bits, "CARGO_CFG_TARGET_POINTER_WIDTH"
        ),
    };

    let cb_base_dir = env::var(cb_env_var).unwrap_or_else(|cb_env_var| {
        panic!(
            "Env Variable '{}' not set. Ensure that you have ClientBridge/C++ installed.",
            cb_env_var
        );
    });
    let cb_lib_path = format!("{}\\Lib\\MSC", cb_base_dir);
    // this magic is required to link picmlx_w64.lib
    println!("cargo:rustc-link-lib=picmlx_w{}", platform_bits);
    println!("cargo:rustc-link-lib=piplx_w{}", platform_bits);
    println!("cargo:rustc-link-search=native={}", cb_lib_path);
}

fn build_unix() {
    println!("cargo:rustc-link-lib=picmlx");
    println!("cargo:rustc-link-lib=piplx");
}

fn main() {
    let os_family =
        env::var("CARGO_CFG_TARGET_FAMILY").expect("The CARGO_CFG_TARGET_FAMILY is not set");

    match os_family.as_str() {
        "windows" => build_windows(),
        "unix" => build_unix(),
        unk_family => panic!("Unknown OS Family '{}'", unk_family),
    }

    // this just causes rebuild when this script changes, see
    // https://doc.rust-lang.org/cargo/reference/build-script-examples.html
    println!("cargo:rerun-if-changed=build.rs");
}
