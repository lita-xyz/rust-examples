#![no_main]

#[no_mangle]
pub fn main() {
    println!("Please enter a number:");
    let n = loop {
        match valida_rs::io::read_line::<u32>() {
            Ok(num) => break num,
            Err(e) => {
                println!("Error reading input: {}. Please try again:", e);
            }
        }
    };
    
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i)
        }
    }
}