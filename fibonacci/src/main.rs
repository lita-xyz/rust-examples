#![no_main]

entrypoint::entrypoint!(main);

pub fn main() {
    // NOTE: values of n larger than 46 will overflow the u32 type.
    let n = unsafe { entrypoint::io::getchar() as u8 };
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut sum: u32;
    for _ in 1..n {
            sum = a + b;
            a = b;
            b = sum;
        }

    entrypoint::io::println(&n.to_string());
    entrypoint::io::println("-th fibonacci number is:");
    entrypoint::io::println(&b.to_string());
}
