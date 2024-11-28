#![no_main]

use ed25519_dalek::*;

use rand::rngs::OsRng;

fn sign_verify() {
    // TestSignVerify

    let good: &[u8] = "test message".as_bytes();
    let bad: &[u8] = "wrong message".as_bytes();

    let mut csprng = OsRng;

    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    let good_sig: Signature = signing_key.sign(good);
    let bad_sig: Signature = signing_key.sign(bad);

    // Check that an honestly generated public key is not weak
    if verifying_key.is_weak() {
        valida_rs::io::println("Key is weak");
    }

    if !signing_key.verify(good, &good_sig).is_ok() {
        valida_rs::io::println("Verification of a valid signature failed!");
    }
    
    if !verifying_key.verify_strict(good, &good_sig).is_ok() {
        valida_rs::io::println("Strict verification of a valid signature failed!");
    }
    
    if !signing_key.verify(good, &bad_sig).is_err() {
        valida_rs::io::println("Verification of a signature on a different message passed!");
    }
    
    if !verifying_key.verify_strict(good, &bad_sig).is_err() {
        valida_rs::io::println("Strict verification of a signature on a different message passed!");
    }
    
    if !signing_key.verify(bad, &good_sig).is_err() {
        valida_rs::io::println("Verification of a signature on a different message passed!");
    }
    
    if !verifying_key.verify_strict(bad, &good_sig).is_err() {
        valida_rs::io::println("Strict verification of a signature on a different message passed!");
    }
}

valida_rs::entrypoint!(main);

pub fn main() {
    sign_verify();
}
