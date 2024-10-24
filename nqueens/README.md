# An nqueens puzzle solver in Valida

This is a simple program that solves the n-queens problem in Valida.

As zkVMs need to run and prove all branches, this program may actually be unprovable due to n-queens heavy reliance on backtracking.

## System requirements

This example supports x86-64 Linux. [`rustup`](https://www.rust-lang.org/tools/install) is required. Arch Linux and Ubuntu are specifically supported, with other platforms possibly requiring some tinkering to make work.

## Toolchain installation

To run this project in the Valida VM, you need the Valida toolchain installed. Go to [LLVM Valida releases](https://github.com/lita-xyz/llvm-valida-releases/releases) to find the latest release. Download the release tarball, extract it, `cd` into the extracted folder, and run `sudo ./install.sh`.

## Entering the Valida shell

To put the Valida toolchain on your PATH, you can enter the Valida shell by running `valida-shell` in your shell. The above installation process should have resulted in `valida-shell` being on your `PATH`.

## Usage

Build the project, from the root directory of this repo:

```
cargo +valida build --release
```

To run the program, in the Valida shell, from the root directory of this repo:

```
valida> valida run ./target/delendum-unknown-baremetal-gnu/release/nqueens log
```

The `run` command runs the program, prompting for an input, and print the output to the console and the file `log` in the current directory.
