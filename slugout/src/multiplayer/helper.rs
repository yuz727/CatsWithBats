use aes::{
    cipher::{
        generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit, StreamCipher,
        StreamCipherSeek,
    },
    Aes128,
};
use bevy::{prelude::*, window::ReceivedCharacter};
use rand::rngs::OsRng;
use rand::RngCore;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use typenum::U16;

pub fn decrypt_rsa(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    private_key
        .decrypt(Pkcs1v15Encrypt, &ciphertext)
        .map_err(|e| format!("Failed to decrypt: {}", e))
}

pub fn encrypt_rsa(
    public_key: &RsaPublicKey,
    plaintext: &[u8],
) -> Result<Vec<u8>, rsa::errors::Error> {
    let mut rng = OsRng;
    public_key.encrypt(&mut rng, Pkcs1v15Encrypt, plaintext)
}

pub fn generate_server_rsa_keypair(mut server_info: ResMut<super::ServerSocket>) {
    let mut rng = rand::thread_rng();
    let bits = 1024;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let pub_key = RsaPublicKey::from(&priv_key);
    println!("Public key: {:?}", pub_key);
    server_info.private_key = Some(priv_key);
    server_info.public_key = Some(pub_key.clone());
}

pub fn encrypt_aes(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut ciphertext = Vec::new();
    let cipher = Aes128::new(key.into());

    let mut iter = plaintext.chunks(16);

    // Process all full 16-byte chunks
    for chunk in iter {
        let mut block: GenericArray<u8, U16> = GenericArray::default();
        for (i, &byte) in chunk.iter().enumerate() {
            block[i] = byte;
        }
        cipher.encrypt_block(&mut block);
        ciphertext.extend_from_slice(block.as_slice());
    }

    ciphertext
}

pub fn decrypt_aes(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut decrypted_plaintext = Vec::new();
    let cipher = Aes128::new(key.into());

    for chunk in ciphertext.chunks(16) {
        let mut block: GenericArray<u8, U16> = GenericArray::default();
        for (i, &byte) in chunk.iter().enumerate() {
            block[i] = byte;
        }

        cipher.decrypt_block(&mut block);
        decrypted_plaintext.extend_from_slice(block.as_slice());
    }

    // Remove padding
    while decrypted_plaintext.last() == Some(&0) {
        decrypted_plaintext.pop();
    }

    decrypted_plaintext
}