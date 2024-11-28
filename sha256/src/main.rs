#![no_main]

use sha2::{Sha256, Digest};

#[no_mangle]
pub fn main() {
    // Read a string input
    let input = match valida_rs::io::read_line::<String>() {
        Ok(s) => s,
        Err(e) => {
            valida_rs::io::println(&format!("Error reading input: {}", e));
            return;
        }
    };
    // Create SHA-256 hasher and compute hash
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    // Output the hash
    println!("{:?}", result);
}
