#![no_main]

valida_rs::entrypoint!(main);

pub fn main() {
    valida_rs::io::println("Please enter a number from 0 to 50:");
    let n = loop {
        match valida_rs::io::read_line::<u8>() {
            Ok(num) => break num,
            Err(e) => {
                valida_rs::io::println(&format!("Error reading input: {}. Please try again:", e));
            }
        }
    };
    
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => valida_rs::io::println("FizzBuzz"),
            (0, _) => valida_rs::io::println("Fizz"),
            (_, 0) => valida_rs::io::println("Buzz"),
            _ => valida_rs::io::println(&format!("{}", i))
        }
    }
}