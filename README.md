# rust-sdl-sandbox
Sandbox project for learning Rust + SDL2 + OpenGL

# Setup

## Rust Environment

You'll need Rust to build the project. Fair warning: these instructions will go out of date quickly.

Setup is easiest with a Linux-like env. Windows users have various options including Visual Studio, [WSL](https://learn.microsoft.com/en-us/windows/wsl/install), [MSYS2](https://www.msys2.org/), [Cygwin](https://www.cygwin.com/), and [PowerShell](https://learn.microsoft.com/en-us/powershell/). Using the sdl2-sys crate on Windows may require troubleshooting effort.

I have tried to document some setup instructions for [MSYS](./docs/msys-setup.md) and [WSL](./docs/wsl-setup.md) users. As of this writing the instructions for WSL are incomplete because I could not get this project to build on Windows yet. `¯\_(ツ)_/¯` The instructions for MSYS did work for me but without the ability to static link SDL2.

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
