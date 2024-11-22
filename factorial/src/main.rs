#![no_main]

#[no_mangle]
pub fn main() {
    println!("Please enter a number from 0 to 12:");
    let num = loop {
        match valida_rs::io::read_line::<u32>() {
            Ok(num) => {
                if num > 12 {
                    println!("Number must be between 0 and 12. Please try again:");
                    continue;
                }
                break num
            },
            Err(e) => {
                println!("Error reading input: {}. Please try again:", e);
            }
        }
    };
    let mut factorial:u32 = 1;
    for i in 1..=num {
        factorial *= i;
    }
    println!("The factorial of {} is {}", num, factorial);
}
