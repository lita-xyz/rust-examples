#![no_main]

#[no_mangle]
pub fn main() {
    println!("Please enter a word or phrase:");
    let word = match valida_rs::io::read_line::<String>() {
        Ok(w) => w,
        Err(e) => {
            println!("Error reading input: {}", e);
            return;
        }
    };
    let word_no_spaces = word.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let is_palindrome = word_no_spaces.chars().eq(word_no_spaces.chars().rev());
    println!("Is '{}' a palindrome? {}", word, is_palindrome);
}
