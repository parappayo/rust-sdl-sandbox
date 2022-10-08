# MSYS2 MinGW Setup

I got the project to build with gcc in an MSYS shell environment without using static linking for SDL2.

The usual caveats apply about checking paths to see what makes sense for your environment. For example my `echo ${USERPROFILE}` is `C:\Users\jason` and my MSYS MingGW path is `C:\msys64\mingw64`. Your system paths are unlikely to match, so make the appropriate substitutions where necessary.

## Install mingw64

See the [official instructions](https://www.mingw-w64.org/)

## Install C++

* `pacman -Fy`
* `pacman -S gcc cmake llvm clang`

## Install Rust

Download `rustup-init.exe` from the [official site](https://www.rust-lang.org/tools/install).

Windows terminal programs often have trouble running in a mingw shell if they prompt for input, which rustup-init.exe does. To get around this you can either use winpty or run rustup with the `-y` flag. Since we need to customize our installation, let's use winpty:

* `pacman -S winpty`
* `winpty ./rustup-init.exe`
* When prompted choose "2) Customize installation"
* Give the following compiler option: `x86_64-pc-windows-gnu`

## Fix PATH

I added the following to my `~/.bash_profile`

`PATH="${PATH}:/c/Users/jason/.cargo/bin"`

I wanted to use the more generic `PATH="${PATH}:${USERPROFILE}/.cargo/bin` instead but when I used that and ran `which cargo` the output looked weird and I got errors like the following when running cargo: `bash: \Users\jason/.cargo/bin/cargo: No such file or directory`. So yeah, eff that noise.

## Configure Cargo to Use gcc

I added this to `${USERPROFILE}/.cargo/config`

```
[target.x86_64-pc-windows-gnu]
linker = "C:\\msys64\\mingw64\\bin\\gcc.exe"
ar = "C:\\msys64\\mingw64\\bin\\ar.exe"
```

You may prefer to add that to the local config for your project instead of making that a global config for your Rust environment.

## Install SDL2 Libs

Download SDL2 libs: [SDL2-devel-2.0.22-mingw.zip](https://github.com/libsdl-org/SDL/releases/tag/release-2.0.22)

Copy all contents

* from `SDL2-devel-2.0.22-mingw.zip\SDL2-2.0.22\x86_64-w64-mingw32\lib`
* to `C:\msys64\mingw64\x86_64-w64-mingw32\lib`

## Troubleshooting cmake

I ran into an error when running `cargo build`

```
error: failed to run custom build command for `sdl2-sys v0.35.2`
...
CMake Error: Could not create named generator MSYS Makefiles
```

I fixed it by removing `"use-bindgen", "bundled", "static-link"` from the SDL2 crate features in `Cargo.toml`.
