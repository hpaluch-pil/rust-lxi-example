# Rust to LXI example

Simple example written in Rust that connects to LXI and prints `Card_ID` of specified card.

Status:
- successfully connects/disconnects LXI
- and opens specified card

## Setup

Tested on Windows 10.

You need to have installed:
- latest Pickering ClientBridge/C++ library from: https://downloads.pickeringtest.info/downloads/drivers/Sys60/
- MSVC 2019 Toolchain + Windows 10 SDK as shown on: https://rust-lang.github.io/rustup/installation/windows.html
- Running LXI - you can use LXI Simulator that can be downloaded
  freely from https://downloads.pickeringtest.info/downloads/LXI_Simulator/

Following components were used:
```cmd
C:\> rustup -V

rustup 1.24.3 (ce5817a94 2021-05-31)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.60.0 (7737e0b5c 2022-04-04)`

C:\> rustup toolchain list

stable-x86_64-pc-windows-msvc (default)

C:\> cargo -V
 
cargo 1.60.0 (d1fd9fe2c 2022-03-01)
```

### Building application:
Open CMD and issue these commands:

```cmd
rem if build failes with weird w2_sock32.lib error try this:
"c:\Program Files (x86)\Microsoft Visual Studio\2019\Professional\VC\Auxiliary\Build\vcvars64.bat"

rem build command
cargo build
```

To run this program invoke (replace `192.168.56.101` with
IP Address of your LXI Simulator):
```cmd
cargo run -- -l 192.168.56.101 -b 1 -s 15
```
Example output:
```
Picmlx Raw Version is: 1183
Connecting to LXI on 192.168.56.101:1024...
Got Session: 22448
Opening Card at Bus=1 Slot=15
Got CardNum=1
Mock: Card ID is Fake Card ID
Closing card with CardNum=1
Disconnecting from LXI...
Done, exiting...
```

= Resources

* Clap Argument parser based on demo:
  * https://github.com/clap-rs/clap/blob/v3.1.8/examples/demo.rs
* DLL linking:
  * https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search
  * https://doc.rust-lang.org/cargo/reference/build-script-examples.html#linking-to-system-libraries
  * https://github.com/rust-lang/cargo/issues/4533
