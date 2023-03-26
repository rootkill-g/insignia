extern crate ed25519_dalek;
extern crate rand;

use ed25519_dalek::{Keypair, Signer};
use openssl::rsa::{Padding, Rsa};
use rand::rngs::OsRng;
use std::{
    fs::File,
    io::{Read, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the file to buffer
    let mut file = File::open("plaintext")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // generate a new ed25519 keypair
    let keypair = Keypair::generate(&mut OsRng {});

    // generate a new RSA keypair for encryption
    let rsa_keypair = Rsa::generate(4096)?;

    // encrypt the file using the RSA public key
    let mut encrypted_data = vec![0; rsa_keypair.size() as usize];
    let encrypted_len = rsa_keypair.public_encrypt(&buffer, &mut encrypted_data, Padding::PKCS1)?;
    encrypted_data.truncate(encrypted_len);

    // write the encrypted data and signature to a file
    let mut output_file = File::create("data.encrypted")?;
    output_file.write_all(&encrypted_data)?;

    // sign the encrypted data using ed25519 keypair
    let signature = keypair.sign(&encrypted_data);
    let mut signature_file = File::create("data.signature")?;
    signature_file.write_all(&signature.to_bytes())?;

    assert!(keypair.verify(&encrypted_data, &signature).is_ok());
    println!("The file was encrypted and it's signature is verified!");
    Ok(())
}
