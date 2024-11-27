//! These are unit tests in a library.
#![feature(custom_test_frameworks, test)]
#![test_runner(valida_rs::test_utils::test_runner)]

#[test]
fn test_integration() {
    assert_eq!(3 + 3, 6);
}
