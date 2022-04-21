# Rust to LXI example

Simple example written in Rust that connects to LXI and prints `Card_ID` of specified card.

Status:
- finished:
  - connects to LXI
  - opens specified card
  - prints Card ID
  - we use [Drp Trait](https://doc.rust-lang.org/std/ops/trait.Drop.html) to:
    - close Card
    - disconnect LXI
- uses [anyhow](https://rust-cli.github.io/book/tutorial/errors.html#providing-context) crate for
  better error handling...
  - also there is alternative `error-chain` (deprecated?)
    - https://rust-lang-nursery.github.io/rust-cookbook/about.html#a-note-about-error-handling
- internals using buffers for C calls:
  - https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/os.rs

## Setup

Tested on Windows 10 and openSUSE LEAP 15.3

### Setup: Windows

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

C:\> rustc -V
rustc 1.60.0 (7737e0b5c 2022-04-04)

C:\> cargo -V
 
cargo 1.60.0 (d1fd9fe2c 2022-03-01)
```

### Windows: Building application:
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
Program version: 64-bit Debug
Picmlx Raw Version is: 1183
Picmlx Ex Version: CbVersionInfo { major: 1, minor: 18, patch: 3 }
Piplx Raw Version is: 1172
Piplx Ex Version: CbVersionInfo { major: 1, minor: 17, patch: 2 }
Connecting to LXI on 192.168.56.101:1024...
Got Session: 23867
Opening Card at Bus=1 Slot=15
Got CardNum=1
Card ID is '40-160-001,1000000,1.01'
Cleanup: Closing card `PiplxHandle { card_num: 1, picmlx_handle: PicmlxHandle { sid: 23867 } }`...
Cleanup: Done. Card with CardNum=1 closed.
Cleanup: Closing session `PicmlxHandle { sid: 23867 }`...
Cleanup: Done. Session 23867 closed.
Done, exiting...
```

### Windows: 32-bit build

> It does not work - on build
> I get mismatch of 32 and 64 bit target
> for link.exe. Also see: https://users.rust-lang.org/t/how-to-build-both-32-and-64-bit-app-on-windows/26365
> 
> So please stick to 64-bit MSVC target.

Experimental support. You need likely to
add 32-bit target first using (from https://users.rust-lang.org/t/how-to-build-both-32-and-64-bit-app-on-windows/26365):
```cmd
rustup target add i686-pc-windows-msvc
```

And then invoke cargo with target parameter
```cmd
cargo build --target=i686-pc-windows-msvc
```

### Setup: Linux

Tested on openSUSE LEAP 15.3:

You need:
- Running LXI - you can use LXI Simulator that can be downloaded
  freely from https://downloads.pickeringtest.info/downloads/LXI_Simulator/
- install cargo (will also install rustc etc.):

```bash
sudo zypper in cargo
```

- install ClientBridge/C++ library for Linux:
  - Download latest ClientBridge for Linux from (used "Debian" for OpenSUSE - more up to date than RedHat versions):
  - https://downloads.pickeringtest.info/downloads/drivers/Sys60/Linux/ClientBridge/Debian/
  - unpack downloaded version and create necessary symlinks, for example:

    ```bash
    cd
    curl -fLO https://downloads.pickeringtest.info/downloads/drivers/Sys60/Linux/ClientBridge/\
    Debian/ClientBridge-1.64.0.0-amd64_deb.tar.gz
    tar xvzf ClientBridge-1.64.0.0-amd64_deb.tar.gz ./usr/lib
    sudo cp usr/lib/libpi*.so* /usr/local/lib/64
    cd /usr/local/lib64/
    sudo ln -s libpicmlx.so.1.18.2 libpicmlx.so
    sudo ln -s libpiplx.so.1.17.1 libpiplx.so
    ```
WARNING! It is less than ideal to (mis)use Debian builds
for openSUSE but we have no binary packages for SUSE yet.
Here we prefere Debian builds over Redhat builds because
they are up-to-date.


Now back in this project directory use
command `cargo build` to build example binary

You can run this binary using command like:
```bash
cargo run -- -l IP_OF_LXI_SIMULATOR -b 1 -s 15
```
Example output:
```
cargo run -- -l 192.168.100.192 -b 1 -s 15
   Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/rust-lxi-example -l 192.168.100.192 -b 1 -s 15`
Program version: 64-bit Debug
Picmlx Raw Version is: 1182
Picmlx Ex Version: CbVersionInfo { major: 1, minor: 18, patch: 2 }
Piplx Raw Version is: 1171
Piplx Ex Version: CbVersionInfo { major: 1, minor: 17, patch: 1 }
Connecting to LXI on 192.168.100.192:1024...
Got Session: 332792694
Opening Card at Bus=1 Slot=15
Got CardNum=1
Card ID is '40-160-001,1000000,1.01'
Cleanup: Closing card `PiplxHandle { card_num: 1, picmlx_handle: PicmlxHandle { sid: 332792694 } }`...
Cleanup: Done. Card with CardNum=1 closed.
Cleanup: Closing session `PicmlxHandle { sid: 332792694 }`...
Cleanup: Done. Session 332792694 closed.
Done, exiting...
```


= Resources

* Clap Argument parser based on demo:
  * https://github.com/clap-rs/clap/blob/v3.1.8/examples/demo.rs
* DLL linking:
  * https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search
  * https://doc.rust-lang.org/cargo/reference/build-script-examples.html#linking-to-system-libraries
  * https://github.com/rust-lang/cargo/issues/4533
* Building 32-bit MSVC Rust binary (unresolved):
  * https://users.rust-lang.org/t/how-to-build-both-32-and-64-bit-app-on-windows/26365
* Getting sizeof pointer:
  * https://stackoverflow.com/questions/64982138/how-to-print-the-size-of-raw-pointer
* How do detect Debug/Release build:
  * https://devtip.in/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
