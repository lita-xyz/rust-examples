# Fibonacci: A template project for Valida

This is a simple program that calculates the n-th fibonacci number and proves the execution of the computation in Valida. You can use this as a template for your projects which create Valida proofs of execution of Rust code.

## Usage

Build the project, from the root directory of this repo:

```
cargo +valida build --release
```

To run the program, in the Valida shell, from the root directory of this repo:

```
valida> valida run ./target/valida-unknown-baremetal-gnu/release/fibonacci log
```

The `run` command runs the program, prompting for an input, and print the output to the console and the file `log` in the current directory.

The log file should contain:

```
25
-th fibonacci number is:
75025
```
