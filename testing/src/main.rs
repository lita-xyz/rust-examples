//! This example demonstrates how to set up a simple unit tests to run with the custom test framework.
//! The custom test framework will run the tests in both on both the host and the valida vm.
//! To run the tests on on both architectures all you need to do is run `cargo test`
//! To run the tests on just the host run `VALIDA_TEST=0 cargo test`
//!
//! For integration tests or a library you will not need the entrypoint macro or no_main attributes.
//! For a full example of integration tests see the `tests/test.rs` file.
#![feature(custom_test_frameworks, test)]
#![test_runner(valida_rs::test_utils::test_runner)]
#![cfg_attr(not(test), no_main)]
valida_rs::entrypoint!(main);

fn main() {
    println!("Hello world!");
}

#[test]
fn test_pass() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic]
fn test_fail() {
    assert_eq!(2 + 2, 5);
}

#[test]
#[ignore]
fn test_ignore() {
    assert_eq!(2 + 2, 4);
}
