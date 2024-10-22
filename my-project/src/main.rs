#![no_main]

entrypoint::entrypoint!(main);

pub fn main() {
    entrypoint::io::println("Hello, world!");
}
