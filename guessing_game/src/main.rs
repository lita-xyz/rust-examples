#![no_main]

use rand::Rng;
use std::cmp::Ordering;

entrypoint::entrypoint!(main);

fn main() {
    entrypoint::io::println("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
      entrypoint::io::println("Please input your guess between 1 and 100.");

        let guess = match entrypoint::io::read_line::<u8>() {
            Ok(num) => num,
            Err(e) => {
                entrypoint::io::println(&format!("Error reading input: {}", e));
                continue;
            }
        };

        entrypoint::io::println(&format!("You guessed: {guess}"));

        match guess.cmp(&secret_number) {
            Ordering::Less => entrypoint::io::println("Too small!"),
            Ordering::Greater => entrypoint::io::println("Too big!"),
            Ordering::Equal => {
                entrypoint::io::println("You win!");
                break;
            }
        }
    }
}
