# Fibonacci: A template project for Valida

This is a simple program that calculates the n-th fibonacci number and proves the execution of the computation in Valida. You can use this as a template for your projects which create Valida proofs of execution of Rust code.

## System requirements

This template supports x86-64 Linux. [`rustup`](https://www.rust-lang.org/tools/install) is required. Arch Linux and Ubuntu are specifically supported, with other platforms possibly requiring some tinkering to make work.

<!-- TODO add more details. -->
Mac is also supported via a docker container. 

## Toolchain installation

To run this template project in the Valida VM, you need the Valida toolchain installed. Go to [LLVM Valida releases](https://github.com/lita-xyz/llvm-valida-releases/releases) to find the latest release. Download the release tarball, extract it, `cd` into the extracted folder, and run `sudo ./install.sh`.

## Entering the Valida shell

To put the Valida toolchain on your PATH, you can enter the Valida shell by running `valida-shell` in your shell. The above installation process should have resulted in `valida-shell` being on your `PATH`.

## Usage

Build the project, from the root directory of this repo, in the Valida shell:

```
valida> cargo +valida build
```

To run the program, in the Valida shell, from the root directory of this repo:

```
valida> valida run ./target/valida-unknown-baremetal-gnu/debug/fibonacci log
```

The `run` command runs the program, prompting for an input, and print the output to the console and the file `log` in the current directory.

To run the program with a file input, you can use the following commands:

```
valida> echo -ne '\x19' > 25.bin
valida> valida run ./target/valida-unknown-baremetal-gnu/debug/fibonacci log 25.bin
```

The file 25.bin is a binary file containing the number 25. This is the input to the `fibonacci` program.

The `run` command will load the binary, and execute the program. The program will then run, and print the output to the console and the file `log` in the current directory.

The log file should contain:

```
The 25-th fibonacci number is: 75025
```

## Writing your own Valida project

We aim to make it so that your native Rust code works on Valida with minimal changes. There are constant improvements in this regard so please check this section for each new version of Valida.

### Changes required for your program to work on Valida and differences from native Rust

As of now, one can use all `stdio` printing functions normally, including `println!`, `eprintln!`, `print!`, and `eprint!`. However, because Valida only has one output tape which prints to `stdout`, all errors are also printed to `stdout`, as opposed to the usual case of `stderr`. Relatedly, [`panic!`](https://doc.rust-lang.org/std/macro.panic.html) is supported, and the error messages when panicking are printed to `stdout`(as opposed to `stderr`) as well.

For reading, one should use the `entrypoint::io` library. The standard read functions like `stdin().read_line(&mut line)` are not yet supported. Support will be added in future versions. See below for more details.

For projects with dependencies on randomness (`rand`), you will need to use the `main` branch of `valida-rs` and call the Valida version of the randomness functions. See below for more details.

### Using `valida-rs`

`valida-rs` is to be used as a dependency in a Valida project. It provides an IO library that works on Valida and the entry point for programs that require randomness.

#### The `IO` library

This library provides common IO functions that work on Valida. See [io.rs](https://github.com/lita-xyz/valida-rs/blob/main/src/io.rs) for the full list of available functions. To use this library, add `valida-rs` to your `Cargo.toml` as shown below. Then call the functions with the `valida_rs::io::` prefix.

To use our IO library functions, you will want to use the `no-deps` branch. Unless you need randomness and/or serialization/deserialization of data, in which case you should use the `main` branch. See below for more details.

To use the "no-deps" branch, add the following to your `Cargo.toml`:

```toml
[dependencies]
valida-rs = { git = "https://github.com/lita-xyz/valida-rs.git", branch = "no-deps" }
```

Also, in your `src/main.rs`, add the following:

```rust
#![no_main]

#[no_mangle]  
pub fn main() {
    // your code here
}
```

The `#[no_mangle]` attribute tells the compiler not to mangle (rename) the function name during compilation. We need this because the Valida runtime looks for a function specifically named "main".

#### For projects that require `rand`

Projects that require `rand` should use the `main` branch of `valida-rs` as a dependency. Add the following to your `Cargo.toml`:

```toml
[dependencies]
valida-rs = { git = "https://github.com/lita-xyz/valida-rs.git", branch = "main" }
getrandom = "0.2.15" # or the current version
rand = "0.8.5" # or the current version
```

Also, add the following to your `src/main.rs`:

```rust
#![no_main]

valida_rs::entrypoint!(main);
```

The `entrypoint!` macro:

- Sets up a deterministic random number generator: It ensures that when `rand` functions are called, they are fixed to a specified seed and thus are deterministic. This is required for the program to be provable.
- Creates a new entry point that wraps the user's main function: This is required because we need to make Rust call this `main` function, the standard Rust `main` function does not work on Valida.


The `#![no_main]` (with `!`) is an inner attribute that applies to the entire crate, telling the Rust compiler not to look for a standard main function entry point. We need this because we are providing a custom entry point.