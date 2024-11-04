# Grep in Valida

This is a simple program which users can enter a query and a string of contents, and the program will return all the words in the contents that contain the query.

## System requirements

This template supports x86-64 Linux. [`rustup`](https://www.rust-lang.org/tools/install) is required. Arch Linux and Ubuntu are specifically supported, with other platforms possibly requiring some tinkering to make work.

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
valida> valida run ./target/delendum-unknown-baremetal-gnu/release/grep log
```

The `run` command runs the program and asks the user for a query and contents, then prints the results to the console and the file `log` in the current directory.
