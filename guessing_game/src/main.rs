#![no_main]

use rand::Rng;
use std::cmp::Ordering;

valida_rs::entrypoint!(main);

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
      println!("Please input your guess between 1 and 100.");

        let guess = match valida_rs::io::read_line::<u8>() {
            Ok(num) => num,
            Err(e) => {
                println!("Error reading input: {}", e);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
