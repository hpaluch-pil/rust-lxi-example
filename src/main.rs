extern crate anyhow;
extern crate clap;

use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char,c_long};

use clap::Parser;
//use anyhow::Context; // can't use for single borrow data...

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

// C structure returned by PICMLX_GetVersionEx() and PIPLX_GetVersionEx()
#[repr(C)]
#[derive(Clone,Copy,Debug)]
struct CbVersionInfo {
    major:u32,
    minor:u32,
    patch:u32,
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
    // VERSION_INFO PICMLX_API PICMLX_GetVersionEx();
    fn PICMLX_GetVersionEx() -> CbVersionInfo;

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

    // DWORD PIPLX_API PIPLX_GetVersion(void);
    fn PIPLX_GetVersion() -> u32;
    // VERSION_INFO PIPLX_API PIPLX_GetVersionEx();
    fn PIPLX_GetVersionEx() -> CbVersionInfo;
    // DWORD PIPLX_API PIPLX_OpenSpecifiedCard(SESSION Sid,DWORD Bus,DWORD Slot,DWORD *CardNum);
    fn PIPLX_OpenSpecifiedCard(sid:c_long,bus:u32,slot:u32,card_num:*mut u32) -> u32;
    // DWORD PIPLX_API PIPLX_CloseSpecifiedCard(SESSION Sid,DWORD CardNum);
    fn PIPLX_CloseSpecifiedCard(sid:c_long,card_num:u32) -> u32;
    // DWORD PIPLX_API PIPLX_CardId(SESSION Sid,DWORD CardNum,LPCHAR Str,DWORD StrLen);
    fn PIPLX_CardId(sid:c_long,card_num:u32,str:*mut c_char,str_len:u32) -> u32;
    // DWORD PIPLX_API PIPLX_ErrorCodeToMessage(DWORD ErrorCode,LPCHAR ErrorMsg,DWORD Length);
    fn PIPLX_ErrorCodeToMessage(error_code:u32,error_msg:*mut c_char,msg_len:u32) -> u32;
}

// *** PICMLX wrappers ***
#[derive(Debug)]
struct PicmlxError {
    err_num: u32,
}

// required by anyhow
impl std::error::Error for PicmlxError {}

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

#[derive(Debug)]
struct PicmlxHandle {
    sid: c_long,
}

impl Drop for PicmlxHandle {
    fn drop(&mut self) {
        println!("Cleanup: Closing session `{:?}`...", self);
        pil_picmlx_disconnect(self.sid).unwrap_or_else(|err|{
            eprintln!("Cleanup: ERROR: LXI Disconnect returned error {}",err);
        });
        println!("Cleanup: Done. Session {} closed.",self.sid);
    }
}

fn picmlx_error_wrapper(err_code: u32) -> Result<(),PicmlxError> {
    if err_code != 0 {
        Err(PicmlxError { err_num: err_code})
    } else {
        Ok(())
    }
}

fn pil_picmlx_connect(board: u32, address:String, port: u32, timeout: u32)
    -> Result<PicmlxHandle,PicmlxError> {
    let c_address = CString::new(address)
        .expect("Unable to create CString from specified address");
    let mut sid:c_long = 0;

    let err_code = unsafe {
        PICMLX_Connect(board, c_address.as_ptr(), port,
                                  timeout, &mut sid) };
    picmlx_error_wrapper(err_code)?;
    Ok(PicmlxHandle { sid})
}

fn pil_picmlx_disconnect (sid:c_long) -> Result<(),PicmlxError> {
    let err_code = unsafe { PICMLX_Disconnect(sid) };
    picmlx_error_wrapper(err_code)?;
    Ok(())
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

// *** PIPLX wrappers ***
#[derive(Debug)]
struct PiplxError {
    err_num: u32,
}
// required by anyhow
impl std::error::Error for PiplxError {}

impl fmt::Display for PiplxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"PIPLX{}: ",self.err_num)?;
        if let Ok(err_msg) = pil_piplx_error_code_to_message(self.err_num) {
            write!(f,"{}",err_msg)
        } else {
            write!(f,"Unknown error code")
        }
    }
}

#[derive(Debug)]
struct PiplxHandle<'a> {
    card_num: u32,
    picmlx_handle: &'a PicmlxHandle,
}

impl<'a> Drop for PiplxHandle<'a> {
    fn drop(&mut self) {
        println!("Cleanup: Closing card `{:?}`...", self);
        pil_piplx_close_specified_card(self.picmlx_handle, self.card_num).unwrap_or_else(|err| {
            eprintln!("Cleanup: Error closing card: {}", err);
        });
        println!("Cleanup: Done. Card with CardNum={} closed.",self.card_num);
    }
}

fn piplx_error_wrapper(err_code: u32) -> Result<(),PiplxError> {
    if err_code != 0 {
        Err(PiplxError { err_num: err_code})
    } else {
        Ok(())
    }
}

// fn PIPLX_OpenSpecifiedCard(sid:c_long,bus:u32,slot:u32,card_num:*mut u32) -> u32;
fn pil_piplx_open_specified_card(picmlx_handle: &PicmlxHandle,bus:u32,slot:u32)
    -> Result<PiplxHandle,PiplxError> {
    let mut card_num:u32 = 0;
    let err_code = unsafe { PIPLX_OpenSpecifiedCard(picmlx_handle.sid,bus,slot,&mut card_num) } ;
    piplx_error_wrapper(err_code)?;
    Ok(PiplxHandle{ picmlx_handle: &picmlx_handle, card_num: card_num  })
}

// fn PIPLX_CloseSpecifiedCard(sid:c_long,card_num:u32) -> u32;
fn pil_piplx_close_specified_card(picmlx_handle: &PicmlxHandle,card_num:u32) -> Result<(),PiplxError> {
    let err_code = unsafe { PIPLX_CloseSpecifiedCard(picmlx_handle.sid,card_num) };
    piplx_error_wrapper(err_code)?;
    Ok(())
}

fn pil_piplx_card_id(picmlx_handle: &PicmlxHandle,piplx_handle: &PiplxHandle) -> Result<String,PiplxError> {
    // output string handling from:
    // dns-lookup-master\src\hostname.rs
    let mut c_name = [0 as c_char; 256 as usize];
    let res = unsafe {
        PIPLX_CardId(picmlx_handle.sid,piplx_handle.card_num,c_name.as_mut_ptr(), c_name.len() as _)
    };
    piplx_error_wrapper(res)?;

    let card_id = unsafe {
        CStr::from_ptr(c_name.as_ptr())
    };
    // TODO: Proper error handling...
    let card_id = std::str::from_utf8(card_id.to_bytes()).unwrap().to_owned();
    Ok(card_id)
}

// Wrapper for:
// fn PIPLX_ErrorCodeToMessage(error_code:u32,error_msg:*mut c_char,msg_len:u32) -> u32;
fn pil_piplx_error_code_to_message(error_code:u32) -> Result<String,u32> {

    // output string handling from:
    // dns-lookup-master\src\hostname.rs

    let mut c_name = [0 as c_char; 256 as usize];
    let res = unsafe {
        PIPLX_ErrorCodeToMessage(error_code,
                                  c_name.as_mut_ptr(), c_name.len() as u32)
    };
    // println!("error_code='{}', res='{}' cname={:?}",error_code,res,c_name);
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

fn main() -> anyhow::Result<()> {
    let lxi_app_args = LxiAppArgs::parse();
    const LXI_PORT: u32 = 1024;

    // How to detect Debug/Release: https://devtip.in/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
    #[cfg(debug_assertions)]
    let build_type = "Debug";

    #[cfg(not(debug_assertions))]
    let build_type = "Release";

    // sizeof pointer from: https://stackoverflow.com/a/64982586
    println!("Program version: {}-bit {}",8*std::mem::size_of::<*const u32>(),build_type);
    let picmlx_ver = unsafe { PICMLX_GetVersion() };
    println!("Picmlx Raw Version is: {}",picmlx_ver);
    let picmlx_ex_ver = unsafe {  PICMLX_GetVersionEx() };
    println!("Picmlx Ex Version: {:?}",picmlx_ex_ver);

    let piplx_ver = unsafe { PIPLX_GetVersion() };
    println!("Piplx Raw Version is: {}",piplx_ver);
    let piplx_ex_ver = unsafe {  PIPLX_GetVersionEx() };
    println!("Piplx Ex Version: {:?}",piplx_ex_ver);

    println!("Connecting to LXI on {}:{}...",lxi_app_args.lxi_address,LXI_PORT);
    // created block to close card/session before exiting main
    {
        let picmlx_handle = pil_picmlx_connect(0, lxi_app_args.lxi_address,
                                               LXI_PORT, 3000)?;
        println!("Got Session: {}", picmlx_handle.sid);
        println!("Opening Card at Bus={} Slot={}",
                 lxi_app_args.card_bus, lxi_app_args.card_slot);
        let piplx_handle = pil_piplx_open_specified_card(
            &picmlx_handle, lxi_app_args.card_bus, lxi_app_args.card_slot)?;
        println!("Got CardNum={}", piplx_handle.card_num);
        let card_id = pil_piplx_card_id(&picmlx_handle, &piplx_handle)?;
        println!("Card ID is '{}'", card_id);
    }
    println!("Done, exiting...");

    Ok(())
}
