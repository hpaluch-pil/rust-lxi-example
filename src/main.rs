extern crate clap;

use std::ffi::CString;
use std::os::raw::{c_char,c_long};
use std::process;

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
    // Functions from:
    // - C:\Program Files\Pickering Interfaces Ltd\ClientBridge\Include\Picmlx.h
    // DWORD PICMLX_API PICMLX_GetVersion(void);
    fn PICMLX_GetVersion() -> u32;
    // DWORD PICMLX_API PICMLX_Connect(DWORD Board,const LPCHAR Address,DWORD Port,DWORD Timeout,LPSESSION SID);
    fn PICMLX_Connect(board: u32, address: *const c_char, port: u32,
                      timeout: u32, sid:*mut c_long) -> u32;
    // DWORD PICMLX_API PICMLX_Disconnect(SESSION SID);
    fn PICMLX_Disconnect(sid:c_long) -> u32;

    // Functions from:
    // - C:\Program Files\Pickering Interfaces Ltd\ClientBridge\Include\Piplx.h
    // DWORD PIPLX_API PIPLX_OpenSpecifiedCard(SESSION Sid,DWORD Bus,DWORD Slot,DWORD *CardNum);
    fn PIPLX_OpenSpecifiedCard(sid:c_long,bus:u32,slot:u32,card_num:*mut u32) -> u32;
    // DWORD PIPLX_API PIPLX_CloseSpecifiedCard(SESSION Sid,DWORD CardNum);
    fn PIPLX_CloseSpecifiedCard(sid:c_long,card_num:u32) -> u32;
}

// Wrapper for DWORD PICMLX_API PICMLX_GetVersion(void);
fn pil_picmlx_get_version() -> u32 {
    unsafe { PICMLX_GetVersion() }
}

fn pil_picmlx_connect(board: u32, address:String, port: u32, timeout: u32)
    -> Result<c_long,u32> {
    let c_address = CString::new(address)
        .expect("Unable to create CString from specified address");
    let mut sid:c_long = 0;

    let err_code = unsafe {
        PICMLX_Connect(board, c_address.as_ptr(), port,
                                  timeout, &mut sid) };
    if err_code == 0 {
        Ok(sid)
    } else {
        Err(err_code)
    }
}

fn pil_picmlx_disconnect (sid:c_long) -> Result<(),u32> {
    let err_code = unsafe { PICMLX_Disconnect(sid) };
    if err_code == 0 {
        Ok(())
    } else {
        Err(err_code)
    }
}

// fn PIPLX_OpenSpecifiedCard(sid:c_long,bus:u32,slot:u32,card_num:*mut u32) -> u32;
fn pil_piplx_open_specified_card(sid:c_long,bus:u32,slot:u32) -> Result<u32,u32> {
    let mut card_num:u32 = 0;
    let res = unsafe { PIPLX_OpenSpecifiedCard(sid,bus,slot,&mut card_num) } ;
    if res == 0 {
        Ok(card_num)
    } else {
        Err(res)
    }
}

// fn PIPLX_CloseSpecifiedCard(sid:c_long,card_num:u32) -> u32;
fn pil_piplx_close_specified_card(sid:c_long,card_num:u32) -> Result<(),u32> {
    let err_code = unsafe { PIPLX_CloseSpecifiedCard(sid,card_num) };
    if err_code == 0 {
        Ok(())
    } else {
        Err(err_code)
    }

}

fn main() {
    let lxi_app_args = LxiAppArgs::parse();
    let picmlx_ver = pil_picmlx_get_version();
    const LXI_PORT: u32 = 1024;

    println!("Picmlx Raw Version is: {}",picmlx_ver);
    println!("Connecting to LXI on {}:{}...",lxi_app_args.lxi_address,LXI_PORT);

    let sid = pil_picmlx_connect(0,lxi_app_args.lxi_address,
                                 LXI_PORT,10000)
        .unwrap_or_else(|err|{
            eprintln!("LXI Connect returned error {}",err);
            process::exit(1);
        });
    println!("Got Session: {}",sid);
    println!("Opening Card at Bus={} Slot={}",
             lxi_app_args.card_bus,lxi_app_args.card_slot);
    let card_num = pil_piplx_open_specified_card(
        sid,lxi_app_args.card_bus,lxi_app_args.card_slot).unwrap_or_else(|err|{
        eprintln!("Unable to open card - error code: {}",err);
        process::exit(1);
    });
    println!("Got CardNum={}",card_num);
    println!("Mock: Card ID is {}", "Fake Card ID");
    println!("Closing card with CardNum={}",card_num);
    pil_piplx_close_specified_card(sid,card_num).unwrap_or_else(|err|{
        eprintln!("Error closing card: {}",err);
    });
    println!("Disconnecting from LXI...");
    pil_picmlx_disconnect(sid).unwrap_or_else(|err|{
        eprintln!("LXI Disconnect returned error {}",err);
    });
    println!("Done, exiting...")
}
