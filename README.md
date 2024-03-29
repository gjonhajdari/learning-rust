# Learning Rust

Files and projects created while learning Rust from [The Rust Programming Language](https://doc.rust-lang.org/book/) book by Steve Klabnik and Carol Nichols.

## Prerequisites

The Rust language needs to be installed in order to compile and run the code. You can find info on how to install Rust on your machine [here](https://www.rust-lang.org/tools/install).

## Getting started

Clone the repository

```bash
$ git clone https://github.com/gjonhajdari/learning-rust
```

Go inside of any module and if it has a binary crate, run `cargo run` to build and compile. If it has library crates, run `cargo test` to run unit/integration tests.

```bash
$ cd module-name/submodule-name

$ cargo run   # binary crate
$ cargo test  # library crate
```
