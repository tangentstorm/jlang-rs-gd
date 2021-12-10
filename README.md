# jlang-rs-gd

[J](https://www.jsoftware.com/#/README) is an extremely high-level mathematical notation and programming language.

[Godot](https://godotengine.org/) is a game / gui / multimedia engine.

**jlang-rs-gd** lets you call J from godot, using a glue layer built in rust.

## requirements

- [rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (rust is a programming language, cargo is its build tool)
- [clang](https://clang.llvm.org/) (the godot bindings need this to build)
- [godot](https://godotengine.org/) (I use 3.3, but any 3.x should work)
- [j](https://www.jsoftware.com/#/README) (so you have a j dll to call)

## building the dll

The output of the build process is a dll that you need to copy to your Godot project.

Until [jlang-rs](https://github.com/tangentstorm/jlang-rs) is somewhat
stable, you need a local working copy in a sibling directory
(or uncomment the `{ git = ... }` line in `Cargo.toml`)

    cd /path/to/dev/area
    git clone https://github.com/tangentstorm/jlang-rs
    git clone https://github.com/tangentstorm/jlang-rs-gd
    cd jlang-rs-gd

to build `jlang_rs_gd.dll`:

- set the `LIBCLANG_PATH` environment variable
- run `cargo test`, `cargo build`, etc.

examples:

    # powershell:
    $env:LIBCLANG_PATH="d:/program files/llvm/bin"; cargo test

    # git bash:
    LIBCLANG_PATH='d:/program files/llvm/bin' cargo test

## Running the demo

Once you have a sucessful build:

    cp target/debug/jlang_rs_gd.dll demo/
    cp j.dll demo/   #  or .so on linux (untested)

The run godot and open the project in the `demo/` directory.

## Adding the DLL to your own Godot project

This is a slightly tedious, manual process:

- copy the two DLLs from the above section to your project directory.
- copy the `jlang-rs-gd.tres` file from the demo directory, or [recreate it from scratch](https://godot-rust.github.io/book/getting-started/hello-world.html#creating-the-nativescript-resource
)
- create a GdNative script in godot, using jlang-rs-gd.tres as the
