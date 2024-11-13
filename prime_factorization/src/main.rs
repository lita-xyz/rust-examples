#![no_main]

use prime_factorization::{read_number, check_prime_factorization};

valida_rs::entrypoint!(main);

pub fn main() {
    valida_rs::io::println("Please enter a 32-bit number:");
    // Read a line from stdin and parse it as an u8.
    let x = read_number();

    valida_rs::io::println("Please enter the number of prime factors (with multiplicity):");
    let n = read_number();

    let mut ys = vec![];
    for _i in 0..n {
        valida_rs::io::println("Please enter the next prime factor:");
        ys.push(read_number());
    }

    if check_prime_factorization(x, ys) {
        valida_rs::io::println("Verified prime factorization of:");
        valida_rs::io::println(&x.to_string());
    } else {
        valida_rs::io::println("Failed to verify prime factorization");
    }
}
