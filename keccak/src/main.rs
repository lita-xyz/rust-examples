#![no_main]

use base16ct; // Constant-time hex encoding with no branches (branches need to be fully visited/proven anyway in VVMs/zkVMs and no std dependencies)
use tiny_keccak::{Keccak, Hasher};

entrypoint::entrypoint!(main);

pub fn main() {
    // TODO: read binary files
    let input = entrypoint::io::read_line::<String>().unwrap();

    let mut digest = [0u8; 32];
    let mut keccak = Keccak::v256();
    keccak.update(input.as_bytes());
    keccak.finalize(&mut digest);

    let mut hex_buf = [0u8; 32];
    let hex = base16ct::lower::encode_str(&digest, &mut hex_buf).unwrap();
    entrypoint::io::println(&format!("keccak digest: 0x{}", hex));
}
