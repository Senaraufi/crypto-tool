// This file implements the Advanced Encryption Standard (AES) algorithm.
// It provides functions for encryption and decryption.

use aes::{Aes128, Aes192, Aes256};
use cbc::{Decryptor, Encryptor};
use cipher::{
    block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
use rand::RngCore;

pub fn encrypt_aes128(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    // encrypt_padded_vec_mut is provided by BlockEncryptMut + Pkcs7
    Encryptor::<Aes128>::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data)
}

pub fn decrypt_aes128(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    Decryptor::<Aes128>::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .expect("Failed to decrypt")
}

pub fn encrypt_aes192(key: &[u8; 24], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    Encryptor::<Aes192>::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data)
}

pub fn decrypt_aes192(key: &[u8; 24], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    Decryptor::<Aes192>::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .expect("Failed to decrypt")
}

pub fn encrypt_aes256(key: &[u8; 32], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    Encryptor::<Aes256>::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data)
}

pub fn decrypt_aes256(key: &[u8; 32], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    Decryptor::<Aes256>::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .expect("Failed to decrypt")
}

pub fn generate_random_iv() -> [u8; 16] {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    iv
}