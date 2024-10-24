# Fibonacci: A template project for Valida

This is a simple program that calculates the n-th fibonacci number and proves the execution of the computation in Valida. You can use this as a template for your projects which create Valida proofs of execution of Rust code.

## System requirements

This template supports x86-64 Linux. [`rustup`](https://www.rust-lang.org/tools/install) is required. Arch Linux and Ubuntu are specifically supported, with other platforms possibly requiring some tinkering to make work.

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
valida> valida run ./target/delendum-unknown-baremetal-gnu/debug/fibonacci log
```

The `run` command runs the program, prompting for an input, and print the output to the console and the file `log` in the current directory.

The log file should contain:

```
25
-th fibonacci number is:
75025
```

## Writing your own Valida project

For projects with dependencies on `io` or `rand`, make sure your `main` and `Cargo.toml` include the code in this template. Also, make sure you have the same `.cargo/config.toml` in your project. If you want to build the project not targeting Valida, remove the `[build]` section in `.cargo/config.toml` and `cargo` will build the project targeting the host machine, unless otherwise specified.

We edited some functions to make them compatible with the Valida VM. When using these, the default Rust functions won't work. We call the Valida version with the `entrypoint::` prefix.

- `io`: Valida only supports standard `io` to the extent of `stdin` and `stdout`. To use `println` in Valida, one needs to call `entrypoint::io::println` as in `my-project`. A better `io` library will be added later.
- `rand`: to ensure the VM can prove the calculation of a given random number, we use our own function to generate a random byte with a specific seed.

These implementations are in `valida-rs/src/io.rs` and `valida-rs/src/rand.rs`.
