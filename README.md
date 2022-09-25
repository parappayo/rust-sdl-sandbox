# rust-sdl-sandbox
Sandbox project for learning Rust + SDL2 + OpenGL

# Setup

## Rust Environment
You'll need Rust to build the project. Fair warning: these instructions will go out of date quickly.

Setup is easiest with a Linux-like env, on Windows there are various options ([WSL](https://learn.microsoft.com/en-us/windows/wsl/install), [MSYS2](https://www.msys2.org/), [Cygwin](https://www.cygwin.com/), PowerShell.) Generally trying to use the sdl2-sys crate on Windows seems to be a mess.

WSL
* `sudo apt update`
* `sudo apt install build-essential cmake llvm clang`
* `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Build and Run
[Cargo](https://doc.rust-lang.org/cargo/) makes this simple.

* `cargo run`

If you just want to compile without running, do `cargo build` instead.

# Goals
To demo rendering the following:
* To get this thing to compile on various environments
* Tile layers, possibly using [Tiled editor](https://www.mapeditor.org/) data
* Lines, grids (possibly as a shader)
* Animated sprites
* Text

# References

## General
* [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2)
* [rustup](https://rustup.rs/)
* [The Rust Book](https://doc.rust-lang.org/book/)
* [Learn OpenGL](https://learnopengl.com/)

## Tutorials
* [Lazy Foo's SDL2 Tutorials, in Rust](https://github.com/ysgard/rust-lazy-foo)
* [Rust and OpenGL from Scratch](http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html) - Ironic Blog, Nerijus Arlauskas
* [Learn Game Dev in Rust](https://sunjay.dev/learn-game-dev/intro.html) - Sunjay Varma

## Books
* [Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/) - Herbert Wolverson
* [Rust in Action](https://www.manning.com/books/rust-in-action) - Tim McNamara
* [The Rust Programming Language](https://nostarch.com/Rust2018) - Steve Klabnik, Carol Nichols

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
