#![no_main]
#[macro_use]
mod atomics;
use atomics::*;

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
    
    // Test arithmetic
    let x = 10;
    let y = 20;
    // Test addition
    let z = x + y;
    println!("{}", z);
    // Test gti 
    let q = x > 9;
    println!("{}", q);
    // Test lti
    let r = x < 11;
    println!("{}", r);

    // Test vector
    let mut v = Vec::new();
    v.push(1);
    println!("{:?}", v);
    // Test panic. Commented out because it is not shown in host.
    // panic!("Test that this panic message is shown");

    // Run all atomics tests

    // Generate test functions for each atomic type
    test_store_load!(test_bool_store_load, AtomicBool, true);
    test_store_load!(test_i8_store_load, AtomicI8, 8);
    test_store_load!(test_i16_store_load, AtomicI16, 16);
    test_store_load!(test_i32_store_load, AtomicI32, 32);
    test_store_load!(test_i64_store_load, AtomicI64, 64);
    test_store_load!(test_isize_store_load, AtomicIsize, 100);
    test_store_load!(test_u8_store_load, AtomicU8, 8);
    test_store_load!(test_u16_store_load, AtomicU16, 16);
    test_store_load!(test_u32_store_load, AtomicU32, 32);
    test_store_load!(test_u64_store_load, AtomicU64, 64);
    test_store_load!(test_usize_store_load, AtomicUsize, 100);

    // Generate fetch_add test functions
    test_fetch_add!(test_i32_fetch_add, AtomicI32, 32, 1);
    test_fetch_add!(test_u64_fetch_add, AtomicU64, 64, 1);
    test_fetch_add!(test_usize_fetch_add, AtomicUsize, 100, 5);

    // Generate fetch_sub test functions
    test_fetch_sub!(test_i32_fetch_sub, AtomicI32, 32, 1);
    test_fetch_sub!(test_u64_fetch_sub, AtomicU64, 64, 1);
    test_fetch_sub!(test_usize_fetch_sub, AtomicUsize, 100, 5);

    // Generate fetch_or test functions
    test_fetch_or!(test_i32_fetch_or, AtomicI32, 0x0F, 0xF0);
    test_fetch_or!(test_u64_fetch_or, AtomicU64, 0x0F, 0xF0);
    test_fetch_or!(test_usize_fetch_or, AtomicUsize, 0x0F, 0xF0);

    // Run all store/load tests
    let store_load_tests = test_bool_store_load()
    && test_i8_store_load()
    && test_i16_store_load()
    && test_i32_store_load()
    && test_i64_store_load()
    && test_isize_store_load()
    && test_u8_store_load()
    && test_u16_store_load()
    && test_u32_store_load()
    && test_u64_store_load()
    && test_usize_store_load();

    // Run all fetch_add tests
    let fetch_add_tests = test_i32_fetch_add() && test_u64_fetch_add() && test_usize_fetch_add();

    // Run all fetch_sub tests
    let fetch_sub_tests = test_i32_fetch_sub() && test_u64_fetch_sub() && test_usize_fetch_sub();

    // Run all fetch_or tests
    let fetch_or_tests = test_i32_fetch_or() && test_u64_fetch_or() && test_usize_fetch_or();

    if store_load_tests && fetch_add_tests && fetch_sub_tests && fetch_or_tests {
        println!("All tests passed");
        } else {
        println!("Some test failed");
    }
}
