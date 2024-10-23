# Conway's Game of Life for Valida

This is a simple program that plays Conway's Game of Life and proves the execution of the computation in Valida.

## System requirements

This template supports x86-64 Linux. [`rustup`](https://www.rust-lang.org/tools/install) is required. Arch Linux and Ubuntu are specifically supported, with other platforms possibly requiring some tinkering to make work.

## Toolchain installation

To run this template project in the Valida VM, you need the Valida toolchain installed. Go to [LLVM Valida releases](https://github.com/lita-xyz/llvm-valida-releases/releases) to find the latest release. Download the release tarball, then follow these steps, substituting `$path_to_release_tarball` with a fully qualified path to the release tarball:

```bash
sudo mkdir -p /valida-toolchain
sudo chown $(whoami):users /valida-toolchain
cd /
tar xf $path_to_release_tarball
sudo install /valida-toolchain/valida-shell /usr/local/bin
```

In the final `install` command, you can replace `/usr/local/bin` with any folder on your `PATH` environment variable (e.g., `~/.local/bin`). You can omit the `sudo` if your user has write permissions on the target folder.

## Entering the Valida shell

To put the Valida toolchain on your PATH, you can enter the Valida shell by running `valida-shell` in your shell. The above installation process should have resulted in `valida-shell` being on your `PATH`.

## Usage

Build the project for Valida, from the root directory of this repo:

```
cargo +valida build
```

To run the program in Valida, in the Valida shell, from the root directory of this repo:

```
valida> valida run ./target/delendum-unknown-baremetal-gnu/debug/conway log
```

The `run` command will load the binary, and execute the program. The program will then run, and print the output to the console and the file `log` in the current directory.
