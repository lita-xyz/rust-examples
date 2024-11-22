#![no_main]

#[no_mangle]
pub fn main() {
    // Print without newline
    print!("Hello");

    // Print with newline
    println!("Hello");

    // Print errors to stdout. This is tested to work in Valida when the test was added.
    // Commented out because they are not printed to stdout in host and the test output fails with the test script.
    // eprintln!("Error message"); 
    // eprint!("Error without newline");
    // Test format specifiers
    let value = "test";
    let number = 42;
    let float_num = 3.14159;

    println!("{}", value);           // Display trait -> "test"
    // commented out because the test script fails because this is different in host.
    // println!("{:p}", &value);        // Pointer address -> "0x7fff..." 
    println!("{:b}", number);        // Binary -> "101010"
    println!("{:x}", number);        // Hexadecimal (lowercase) -> "2a"
    println!("{:X}", number);        // Hexadecimal (uppercase) -> "2A"
    println!("{:o}", number);        // Octal -> "52"
    println!("{:e}", float_num);     // Scientific notation -> "3.14159e0"
    println!("{:.2}", float_num);    // Float with 2 decimal places -> "3.14"
    println!("{:>10}", value);       // Right-align with width 10 -> "      test"
    println!("{:<10}", value);       // Left-align with width 10 -> "test      "
    println!("{:^10}", value);       // Center-align with width 10 -> "   test   "
    println!("{:0>5}", value);       // Pad with zeros on left -> "0test"

    // Test panic
    panic!("Test that this panic message is shown");
}
