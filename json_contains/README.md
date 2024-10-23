# Prove a JsonPath contains a value

## System requirements

This template supports x86-64 Linux. [`rustup`](https://www.rust-lang.org/tools/install) is required. Arch Linux and Ubuntu are specifically supported, with other platforms possibly requiring some tinkering to make work.

## Toolchain installation

To run this project in the Valida VM, you need the Valida toolchain installed. Go to [LLVM Valida releases](https://github.com/lita-xyz/llvm-valida-releases/releases) to find the latest release. Download the release tarball, extract it, `cd` into the extracted folder, and run `sudo ./install.sh`.

## Entering the Valida shell

To put the Valida toolchain on your PATH, you can enter the Valida shell by running `valida-shell` in your shell. The above installation process should have resulted in `valida-shell` being on your `PATH`.

## Usage

Build the project, from the root directory of this repo:

```bash
valida> cargo +valida build
```

Run the program in native code:

```bash
$ echo '{"path":"$.foo.bar","expected":{"baz":1},"json":{"foo":{"bar":{"baz":1}}}}' | cargo run
path: $.foo.bar
contains json: {"baz":1}
```

Run the program in Valida:

```bash
valida> 
valida> echo '{"path":"$.foo.bar","expected":{"baz":1},"json":{"foo":{"bar":{"baz":1}}}}' | valida run target/delendum-unknown-baremetal-gnu/debug/json_contains log
path: $.foo.bar
contains json: {"baz":1}
```
