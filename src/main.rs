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

fn main() {
    let lxi_app_args = LxiAppArgs::parse();
    println!("Mock: Connecting to LXI on {}...",lxi_app_args.lxi_address);
    println!("Mock: Opening Card at Bus={} Slot={}",
             lxi_app_args.card_bus,lxi_app_args.card_slot);
    println!("Mock: Card ID is {}", "Fake Card ID");
    println!("Mock: Closing card");
    println!("Mock: Done, exiting...")
}
