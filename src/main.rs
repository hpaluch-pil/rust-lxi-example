extern crate clap;

use std::ffi::{CStr, CString};
use std::fmt;
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

extern "C" {
    // Functions from:
    // - C:\Program Files\Pickering Interfaces Ltd\ClientBridge\Include\Picmlx.h
    // Build Time:
    // - C:\Program Files\Pickering Interfaces Ltd\ClientBridge\Lib\MSC\Picmlx_w64.lib
    // Run Time
    // C:\Windows\System32\Picmlx_w64.dll

    // DWORD PICMLX_API PICMLX_GetVersion(void);
    fn PICMLX_GetVersion() -> u32;
    // DWORD PICMLX_API PICMLX_Connect(DWORD Board,const LPCHAR Address,DWORD Port,DWORD Timeout,LPSESSION SID);
    fn PICMLX_Connect(board: u32, address: *const c_char, port: u32,
                      timeout: u32, sid:*mut c_long) -> u32;
    // DWORD PICMLX_API PICMLX_Disconnect(SESSION SID);
    fn PICMLX_Disconnect(sid:c_long) -> u32;
    // DWORD PICMLX_API PICMLX_ErrorCodeToMessage(DWORD ErrorCode,LPCHAR ErrorMsg,DWORD Length);
    fn PICMLX_ErrorCodeToMessage(error_code:u32,error_msg:*mut c_char,msg_len:u32) -> u32;

    // Functions from:
    // - C:\Program Files\Pickering Interfaces Ltd\ClientBridge\Include\Piplx.h
    // Build Time:
    // - C:\Program Files\Pickering Interfaces Ltd\ClientBridge\Lib\MSC\Piplx_w64.lib
    // Run Time
    // C:\Windows\System32\Piplx_w64.dll

    // DWORD PIPLX_API PIPLX_OpenSpecifiedCard(SESSION Sid,DWORD Bus,DWORD Slot,DWORD *CardNum);
    fn PIPLX_OpenSpecifiedCard(sid:c_long,bus:u32,slot:u32,card_num:*mut u32) -> u32;
    // DWORD PIPLX_API PIPLX_CloseSpecifiedCard(SESSION Sid,DWORD CardNum);
    fn PIPLX_CloseSpecifiedCard(sid:c_long,card_num:u32) -> u32;
    // DWORD PIPLX_API PIPLX_CardId(SESSION Sid,DWORD CardNum,LPCHAR Str,DWORD StrLen);
    fn PIPLX_CardId(sid:c_long,card_num:u32,str:*mut c_char,str_len:u32) -> u32;
}

// *** PICMLX wrappers ***
#[derive(Debug)]
struct PicmlxError {
    err_num: u32,
}

impl fmt::Display for PicmlxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"PICMLX{}: ",self.err_num)?;
        if let Ok(err_msg) = pil_picmlx_error_code_to_message(self.err_num) {
            write!(f,"{}",err_msg)
        } else {
            write!(f,"Unknown error code")
        }
    }
}

// Wrapper for DWORD PICMLX_API PICMLX_GetVersion(void);
fn pil_picmlx_get_version() -> u32 {
    unsafe { PICMLX_GetVersion() }
}

fn pil_picmlx_connect(board: u32, address:String, port: u32, timeout: u32)
    -> Result<c_long,PicmlxError> {
    let c_address = CString::new(address)
        .expect("Unable to create CString from specified address");
    let mut sid:c_long = 0;

    let err_code = unsafe {
        PICMLX_Connect(board, c_address.as_ptr(), port,
                                  timeout, &mut sid) };
    if err_code == 0 {
        Ok(sid)
    } else {
        Err(PicmlxError { err_num: err_code})
    }
}

fn pil_picmlx_disconnect (sid:c_long) -> Result<(),PicmlxError> {
    let err_code = unsafe { PICMLX_Disconnect(sid) };
    if err_code == 0 {
        Ok(())
    } else {
        Err(PicmlxError { err_num: err_code})
    }
}

// Wrapper for:
// fn PICMLX_ErrorCodeToMessage(error_code:u32,error_msg:*mut c_char,msg_len:u32) -> u32;
fn pil_picmlx_error_code_to_message(error_code:u32) -> Result<String,u32> {

    // output string handling from:
    // dns-lookup-master\src\hostname.rs

    let mut c_name = [0 as c_char; 256 as usize];
    let res = unsafe {
        PICMLX_ErrorCodeToMessage(error_code,
                                  c_name.as_mut_ptr(), c_name.len() as u32)
    };
    if res != 0 {
        return Err(res);
    }
    let err_msg = unsafe {
        CStr::from_ptr(c_name.as_ptr())
    };
    // TODO: Proper error handling...
    let err_msg = std::str::from_utf8(err_msg.to_bytes()).unwrap().to_owned();
    Ok(err_msg)
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

fn pil_piplx_card_id(sid:c_long,card_num:u32) -> Result<String,u32> {

    // output string handling from:
    // dns-lookup-master\src\hostname.rs

    let mut c_name = [0 as c_char; 256 as usize];
    let res = unsafe {
        PIPLX_CardId(sid,card_num,c_name.as_mut_ptr(), c_name.len() as _)
    };
    if res != 0 {
        return Err(res);
    }
    let card_id = unsafe {
        CStr::from_ptr(c_name.as_ptr())
    };
    // TODO: Proper error handling...
    let card_id = std::str::from_utf8(card_id.to_bytes()).unwrap().to_owned();
    Ok(card_id)
}


fn main() {
    let lxi_app_args = LxiAppArgs::parse();
    let picmlx_ver = pil_picmlx_get_version();
    const LXI_PORT: u32 = 1024;

    // How to detect Debug/Release: https://devtip.in/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
    #[cfg(debug_assertions)]
    let build_type = "Debug";

    #[cfg(not(debug_assertions))]
    let build_type = "Release";

    // sizeof pointer from: https://stackoverflow.com/a/64982586
    println!("Program version: {}-bit {}",8*std::mem::size_of::<*const u32>(),build_type);
    println!("Picmlx Raw Version is: {}",picmlx_ver);
    println!("Connecting to LXI on {}:{}...",lxi_app_args.lxi_address,LXI_PORT);

    let sid = pil_picmlx_connect(0,lxi_app_args.lxi_address,
                                 LXI_PORT,3000)
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
    let card_id = pil_piplx_card_id(sid,card_num);
    if card_id.is_ok() {
        println!("Card ID is '{}'", card_id.unwrap());
    } else {
        eprintln!("Error {} getting card id",card_id.unwrap_err());
    }
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
