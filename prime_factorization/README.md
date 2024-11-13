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

To run the program with a file input, you can use the following commands:

```
valida> echo -ne '\x19' > 25.bin
valida> valida run ./target/delendum-unknown-baremetal-gnu/debug/fibonacci log 25.bin
```

The file 25.bin is a binary file containing the number 25. This is the input to the `fibonacci` program.

The `run` command will load the binary, and execute the program. The program will then run, and print the output to the console and the file `log` in the current directory.

The log file should contain:

```
25
-th fibonacci number is:
75025
```
