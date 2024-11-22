use std::io::{self};
// Tests for read
// 1. Direct read examples
pub fn demonstrate_direct_read() {
    // Empty buffer read
    println!("Reading empty buffer (this should return 0 bytes)...");
    let mut buf = [];
    match io::stdin().read(&mut buf) {
        Ok(empty_result) => println!("Empty buffer read: {} bytes", empty_result),
        Err(e) => println!("Error reading empty buffer: {}", e)
    }
    
    // Single byte read
    println!("Reading single byte (press any key)...");
    let mut buf = [0u8; 1];
    match io::stdin().read(&mut buf) {
        Ok(single_byte) => println!("Single byte read: {} bytes, value: {}", single_byte, String::from_utf8_lossy(&buf)),
        Err(e) => println!("Error reading single byte: {}", e)
    }
    
    // Multiple bytes read
    println!("Reading up to 10 bytes (type some characters and press Enter)...");
    let mut buf = [0u8; 10];
    match io::stdin().read(&mut buf) {
        Ok(bytes_read) => println!("Multiple bytes read: {} bytes, values: {}", bytes_read, String::from_utf8_lossy(&buf[..bytes_read])),
        Err(e) => println!("Error reading multiple bytes: {}", e)
    }
    
    // Read until EOF
    println!("Reading up to 100 bytes (type some text and press Ctrl+D for EOF)...");
    let mut buf = [0u8; 100];
    match io::stdin().read(&mut buf) {
        Ok(bytes_read) => println!("Read until EOF: {} bytes, values: {}", bytes_read, String::from_utf8_lossy(&buf[..bytes_read])),
        Err(e) => println!("Error reading until EOF: {}", e)
    }
}

// 2. Buffered reading examples
pub fn demonstrate_buffered_read() {
    // Read line
    println!("Reading a line (type a line of text and press Enter)...");
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(bytes_read) => println!("Line read: {} bytes, content: {}", bytes_read, line),
        Err(e) => println!("Error reading line: {}", e)
    }
    
    // Read until
    let mut buf = Vec::new();
    match io::stdin().read_until(b'\n', &mut buf) {
        Ok(bytes_read) => println!("Read until newline: {} bytes, content: {}", bytes_read, String::from_utf8_lossy(&buf)),
        Err(e) => println!("Error reading until newline: {}", e)
    }

    // Line iterator
    for line in stdin.lines() {
        println!("{}", line?);
    }
    
    // Read to string
    println!("Reading entire input into string (type multiple lines, press Ctrl+D when done)...");
    let mut content = String::new();
    match io::stdin().read_to_string(&mut content) {
        Ok(bytes_read) => println!("String read: {} bytes, content: {}", bytes_read, content),
        Err(e) => println!("Error reading to string: {}", e)
    }
}