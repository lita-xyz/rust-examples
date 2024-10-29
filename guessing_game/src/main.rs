#![no_main]

use rand::Rng;
use std::cmp::Ordering;

valida_rs::entrypoint!(main);

fn main() {
    valida_rs::io::println("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
      valida_rs::io::println("Please input your guess between 1 and 100.");

        let guess = match valida_rs::io::read_line::<u8>() {
            Ok(num) => num,
            Err(e) => {
                valida_rs::io::println(&format!("Error reading input: {}", e));
                continue;
            }
        };

        valida_rs::io::println(&format!("You guessed: {guess}"));

        match guess.cmp(&secret_number) {
            Ordering::Less => valida_rs::io::println("Too small!"),
            Ordering::Greater => valida_rs::io::println("Too big!"),
            Ordering::Equal => {
                valida_rs::io::println("You win!");
                break;
            }
        }
    }
}
