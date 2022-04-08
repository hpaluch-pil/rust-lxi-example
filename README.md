# Rust to LXI example

Simple example written in Rust that connects to LXI and prints `Card_ID` of specified card.

## Setup

Tested on Windows 10.

You need to have installed:
- latest Pickering ClientBridge/C++ library from: https://downloads.pickeringtest.info/downloads/drivers/Sys60/
- MSVC 2019 Toolchain + Windows 10 SDK as shown on: https://rust-lang.github.io/rustup/installation/windows.html

### Building application:
Open CMD and issue these commands:

```cmd
rem have to intialize MSVC 2019 64-bit toolset environment
"c:\Program Files (x86)\Microsoft Visual Studio\2019\Professional\VC\Auxiliary\Build\vcvars64.bat"

rem build command
cargo build
```



= Resources

* Clap Argument parser based on demo:
  * https://github.com/clap-rs/clap/blob/v3.1.8/examples/demo.rs
