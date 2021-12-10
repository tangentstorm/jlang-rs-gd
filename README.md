# jlang-rs-gd
jlang--godot bridge, built in rust

## building
- install rust
- install [clang](https://clang.llvm.org/).
- set the `LIBCLANG_PATH` environment variable
- run `cargo test`, `cargo build`, etc.

ex:

    # powershell:
    $env:LIBCLANG_PATH="d:/program files/llvm/bin"; cargo test

    # git bash:
    LIBCLANG_PATH='d:/program files/llvm/bin' cargo test
