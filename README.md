# Fibonacci: A template project for Valida

This is a simple program that calculates the n-th fibonacci number and proves the execution of the computation in Valida. You can use this as a template for your projects which create Valida proofs of execution of Rust code.

## Installing the Valida toolchain

Using this template requires the Valida toolchain, version 0.6.0-alpha to be installed on your system. There are two ways to install this toolchain: via Docker, or via the Linux release bundle.

### Docker-based installation

We provide a Docker container with the Valida LLVM and Rust toolchains already installed. Docker is the only supported method of running on platforms other than x86 Linux.

```bash
# Download the container
docker pull ghcr.io/lita-xyz/llvm-valida-releases/valida-build-container:v0.6.0-alpha

cd your-valida-project

# Enter the container:
docker run --platform linux/amd64 -it --rm -v $(realpath .):/src ghcr.io/lita-xyz/llvm-valida-releases/valida-build-container:v0.6.0-alpha

# You are now in a shell with the valida rust toolchain installed!
```

### Linux-based installation

#### System requirements

 * This toolchain supports x86-64 Linux based on `glibc-2.9` or newer `glibc`.
 * [`rustup`](https://www.rust-lang.org/tools/install) is required.
 * Arch Linux and Ubuntu 24.04 LTS are specifically supported, with other platforms possibly requiring some tinkering to make work.

#### Download

To download the Linux-based release bundle:

```
wget https://github.com/lita-xyz/llvm-valida-releases/releases/download/v0.6.0-alpha/llvm-valida-v0.6.0-alpha-linux-x86_64.tar.xz
```

#### Installation

From the untarred release bundle, in the directory called `valida-toolchain`, the same directory containing these release notes, run:

```bash
sudo ./install.sh
```

#### Entering the Valida shell

Upon having installed the toolchain, the Valida shell should be on your `PATH`, and if you run `which valida-shell`, you should see something like:

```bash
/home/morgan/.local/bin/valida-shell
```

If the result is very different from this, then either the installation did not complete successfully, or you had another executable named `valida-shell` somewhere on your `PATH`.

If you run `valida-shell`, then you should see a shell prompt that reads `valida> `. You should then have on your `PATH` all of the executables from the Valida toolchain needed to follow the usage instructions below.

## Usage

Build the project, from the root directory of this repo, in the Valida shell:

```bash
cargo +valida build
```

To run the program, in the Valida shell, from the root directory of this repo:

```bash
valida run ./target/valida-unknown-baremetal-gnu/debug/fibonacci log
```

The `run` command runs the program, prompting for an input, and print the output to the console and the file `log` in the current directory.

To run the program with a file input, you can use the following commands:

```bash
echo -ne '\x19' > 25.bin
valida run ./target/valida-unknown-baremetal-gnu/debug/fibonacci log 25.bin
```

The file 25.bin is a binary file containing the number 25. This is the input to the `fibonacci` program.

The `run` command will load the binary, and execute the program. The program will then run, and print the output to the console and the file `log` in the current directory.

The log file should contain:

```
25
-th fibonacci number is:
75025
```
