use secp256k1::ecdsa::Signature;
use secp256k1::hashes::{sha256, Hash};
use secp256k1::{Message, PublicKey, Secp256k1};

#[cfg(not(target_arch = "valida"))]
use secp256k1::rand::rngs::OsRng;

const MSG: &str = "Hello World!";

#[cfg(not(target_arch = "valida"))]
fn print_fresh_keypair_and_signature() {
    use std::io::Write;

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    let digest = sha256::Hash::hash(MSG.as_bytes());
    let message = Message::from_digest(digest.to_byte_array());

    let serialized_public_key = public_key.serialize();
    let signature = secp.sign_ecdsa(&message, &secret_key);

    std::io::stdout().write(&serialized_public_key).unwrap(); // 33 bytes
    std::io::stdout().write(secret_key.as_ref()).unwrap(); // 32 bytes
    std::io::stdout()
        .write(&signature.serialize_compact())
        .unwrap(); // 64 bytes

    // use
    // xxd -p -c1 output | sed 's/^/0x/' | paste -sd ',' -
    // to dump concatenated keys and signature on host
}

fn verify_signature_based_on_hardcoded_signature_and_public_key() {
    // -- secret key was
    //   ce e986 baaf a29f ac83 4743 d6ae ea32
    // 42e2 7062 3a22 e090 9c3d ee16 3bf0 7016
    // e3

    const PUBLIC_KEY_SERIALIZED: [u8; 33] = [
        0x03, 0xce, 0x00, 0xba, 0xd2, 0xd8, 0xf1, 0x46, 0xe1, 0xd6, 0x70, 0x79, 0xb7, 0xfb, 0x40,
        0xf2, 0xff, 0xe7, 0x2d, 0xe9, 0xcd, 0x15, 0xe0, 0xd2, 0xfb, 0xd0, 0x01, 0xf8, 0xe8, 0x5a,
        0xab, 0xb9, 0xfa,
    ];

    const SIGNATURE_SERIALIZED: [u8; 64] = [
        0xe7, 0x69, 0x93, 0xac, 0x34, 0x45, 0xa4, 0x95, 0x5f, 0x89, 0x18, 0xf7, 0xec, 0xfb, 0x5c,
        0xd9, 0xf2, 0x4c, 0xc9, 0x51, 0x07, 0xfe, 0x92, 0x21, 0xd4, 0x5c, 0xa6, 0xc9, 0x11, 0x47,
        0x12, 0xd3, 0x3a, 0xe8, 0x8e, 0x2d, 0x8c, 0xc8, 0xfe, 0x44, 0xdd, 0xfb, 0xfd, 0xcc, 0xba,
        0xfe, 0x1a, 0x0f, 0x9b, 0xb0, 0xa9, 0x05, 0x40, 0x35, 0xd7, 0x17, 0x86, 0xd2, 0x5b, 0xcf,
        0x9e, 0xa2, 0x7e, 0x25,
    ];

    let secp = Secp256k1::new();

    let digest = sha256::Hash::hash(MSG.as_bytes());
    let message = Message::from_digest(digest.to_byte_array());

    let public_key = PublicKey::from_slice(&PUBLIC_KEY_SERIALIZED).unwrap();
    let sig = Signature::from_compact(&SIGNATURE_SERIALIZED).unwrap();

    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
}

fn main() {
    #[cfg(not(target_arch = "valida"))]
    print_fresh_keypair_and_signature();

    // just to make sure that verification with harcoded data passes on host
    verify_signature_based_on_hardcoded_signature_and_public_key();
}
