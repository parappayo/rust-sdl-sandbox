# WSL Setup

I'm currently stuck on `"fatal error: 'X11/Xlib.h' file not found"` because, I think, my C++ dev setup is trying to use X11 binaries for SDL2 and generate a Linux desktop app. I need to figure out how to set up gcc to cross-compile a Windows binary and set up Rust to use that instead.

## Install WSL

See the [official instructions](https://learn.microsoft.com/en-us/windows/wsl/install).

Or if that link is dead, try `wsl --install` from PowerShell.

The rest of these instructions assume you are in WSL.

## Install C++

* `sudo apt update`
* `sudo apt install build-essential cmake llvm clang`

## Install Rust

See the [official instructions](https://www.rust-lang.org/tools/install).

Or if that link is dead, try `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Troubleshooting PATH

Restart your shell and run `which cargo`. You should see output like `/home/jason/.cargo/bin/cargo`.

Installation should have automatically added `. "$HOME/.cargo/env` to `~/.profile` but if cargo is not in your PATH env then try adding that line.

# Troubleshooting

## CMake Errors

Bundled sdl2 with cmake on Windows is known to have problems.

* [Github: Bundled gcc/libraries on windows may cause errors when compiling native code](https://github.com/rust-lang/rust/issues/19519)
* [Stack Overflow: Problems running and building a program with the SDL2 crate](https://stackoverflow.com/questions/60432515/problems-running-and-building-a-program-with-the-sdl2-crate)

I am not sure how to fix this but you can try un-bundling the sdl2 crate (remove "bundled", "static-link" from `Cargo.toml`.) Apparently there is some incompatibility with Rust's bundled gcc.exe and cmake.

# "Couldn't find pregenerated bindings!"

I added "use-bindgen" to the sdl2-sys crate settings.

# "fatal error: 'X11/Xlib.h' file not found"

I'm honestly not sure if this will ever work on WSL. 😂
