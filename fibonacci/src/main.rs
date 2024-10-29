#![no_main]

valida_rs::entrypoint!(main);

pub fn main() {
    valida_rs::io::println("Please enter a number from 0 to 46:");
    // Read a line from stdin and parse it as an u8.
    let n = loop {
        match valida_rs::io::read_line::<u8>() {
            Ok(num) => break num,
            Err(e) => {
                valida_rs::io::println(&format!("Error reading input: {}. Please try again:", e));
            }
        }
    };
    // n that is larger than 46 will overflow the u32 type.
    if n > 46 {
        valida_rs::io::println("Error: n is too large. Please enter a number no more than 46.");
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

    valida_rs::io::println(&n.to_string());
    valida_rs::io::println("-th fibonacci number is:");
    valida_rs::io::println(&b.to_string());
}
