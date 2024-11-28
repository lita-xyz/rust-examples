#![no_main]

use sha2::{Sha256, Digest};

#[no_mangle]
pub fn main() {
    // Read a 32-byte input
    let input = match valida_rs::io::read_line::<u32>() {
        Ok(w) => w,
        Err(e) => {
            valida_rs::io::println(&format!("Error reading input: {}", e));
            return;
        }
    };
    // Create SHA-256 hasher and compute hash
    let mut hasher = Sha256::new();
    hasher.update(&input.to_be_bytes());
    let result = hasher.finalize();

    // Output the hash
    println!("{:?}", result);
}
