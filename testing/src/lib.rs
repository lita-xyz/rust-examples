//! These are unit tests in a library.
#![feature(custom_test_frameworks, test)]
#![test_runner(valida_rs::test_utils::test_runner)]

#[test]
fn test_add() {
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
