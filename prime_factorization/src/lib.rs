pub fn read_number() -> u32 {
    loop {
        match valida_rs::io::read_line::<u32>() {
            Ok(num) => break num,
            Err(e) => {
                valida_rs::io::println(&format!("Error reading input: {}. Please try again:", e));
            }
        }
    }
}

pub fn check_prime_factorization(x: u32, ys: Vec<u32>) -> bool {
    let mut z = 1;
    for y in ys {
        z *= y;
    }
    z == x
}
