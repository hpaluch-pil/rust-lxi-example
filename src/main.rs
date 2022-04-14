extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author="Henryk Paluch of Pickering Interfaces Ltd.",
        version="0.1", about="Rust Example how to connect to LXI",
        long_about = "How to connect to LXI using Pickering ClientBridge C/C++ library")]
struct LxiAppArgs {
    #[clap(short='l', long="lxi-address")]
    lxi_address: String,

    #[clap(short='b', long="bus")]
    card_bus: u32,

    #[clap(short='s', long="slot")]
    card_slot: u32,
}

// Wrapper for:
// Build Time:
// - C:\Program Files\Pickering Interfaces Ltd\ClientBridge\Include\Picmlx.h
// - C:\Program Files\Pickering Interfaces Ltd\ClientBridge\Lib\MSC\Picmlx_w64.lib
// Run Time
// C:\Windows\System32\Picmlx_w64.dll

extern "C" {
    //   DWORD PICMLX_API PICMLX_GetVersion(void);
    fn PICMLX_GetVersion() -> u32;
}

// Wrapper for DWORD PICMLX_API PICMLX_GetVersion(void);
fn pil_picmlx_get_version() -> u32 {
    unsafe { PICMLX_GetVersion() }
}

fn main() {
    let lxi_app_args = LxiAppArgs::parse();
    let picmlx_ver = pil_picmlx_get_version();
    println!("Picmlx Raw Version is: {}",picmlx_ver);
    println!("Mock: Connecting to LXI on {}...",lxi_app_args.lxi_address);
    println!("Mock: Opening Card at Bus={} Slot={}",
             lxi_app_args.card_bus,lxi_app_args.card_slot);
    println!("Mock: Card ID is {}", "Fake Card ID");
    println!("Mock: Closing card");
    println!("Mock: Done, exiting...")
}
