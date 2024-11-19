#![no_main]

#[no_mangle]
pub fn main() {
    println!("Normal message");    // Goes to stdout/output tape
    eprintln!("Error message");    // Goes to stdout/output tape
}