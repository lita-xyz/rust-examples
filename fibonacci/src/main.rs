#![no_main]

#[no_mangle]

pub fn main() {
    println!("Please enter a number from 0 to 46:");
    // Read a line from stdin and parse it as an u8.
    let n = loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_bytes_read) => {
                match input.trim().parse::<u8>() {
                    Ok(num) => break num,
                    Err(e) => {
                        println!("Error reading input: {}. Please try again:", e);
                        return;
                    }
                }
            },
            Err(e) => {
                println!("Error reading input: {}. Please try again:", e);
                return;
            }
        }
    };
    // n that is larger than 46 will overflow the u32 type.
    if n > 46 {
        println!("Error: n is too large. Please enter a number no more than 46.");
        return;
    }
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut sum: u32;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    println!("The {}-th fibonacci number is: {}", n, b);
}
