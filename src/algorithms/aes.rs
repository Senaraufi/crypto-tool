// This file implements the Advanced Encryption Standard (AES) algorithm.
// It provides functions for encryption and decryption.

use aes::{Aes128, Aes192, Aes256};
use cbc::{Decryptor, Encryptor};
use cipher::{
    block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
use rand::RngCore;

pub fn encrypt_aes128(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut buffer = data.to_vec();
    let len = data.len();
    buffer.resize(len + 16, 0);
    let encrypted_len = Encryptor::<Aes128>::new(key.into(), iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, len)
        .expect("Failed to encrypt")
        .len();
    buffer.truncate(encrypted_len);
    buffer
}

pub fn decrypt_aes128(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut buffer = data.to_vec();
    let decrypted = Decryptor::<Aes128>::new(key.into(), iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .expect("Failed to decrypt");
    decrypted.to_vec()
}

pub fn encrypt_aes192(key: &[u8; 24], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut buffer = data.to_vec();
    let len = data.len();
    buffer.resize(len + 16, 0);
    let encrypted_len = Encryptor::<Aes192>::new(key.into(), iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, len)
        .expect("Failed to encrypt")
        .len();
    buffer.truncate(encrypted_len);
    buffer
}

pub fn decrypt_aes192(key: &[u8; 24], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut buffer = data.to_vec();
    let decrypted = Decryptor::<Aes192>::new(key.into(), iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .expect("Failed to decrypt");
    decrypted.to_vec()
}

pub fn encrypt_aes256(key: &[u8; 32], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut buffer = data.to_vec();
    let len = data.len();
    buffer.resize(len + 16, 0);
    let encrypted_len = Encryptor::<Aes256>::new(key.into(), iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, len)
        .expect("Failed to encrypt")
        .len();
    buffer.truncate(encrypted_len);
    buffer
}

pub fn decrypt_aes256(key: &[u8; 32], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut buffer = data.to_vec();
    let decrypted = Decryptor::<Aes256>::new(key.into(), iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .expect("Failed to decrypt");
    decrypted.to_vec()
}

pub fn generate_random_iv() -> [u8; 16] {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    iv
}