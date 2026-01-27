// This file implements the Advanced Encryption Standard (AES) algorithm.
// It provides functions for encryption and decryption.

use aes::{Aes128, Aes192, Aes256};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use std::convert::TryInto;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
type Aes192Cbc = Cbc<Aes192, Pkcs7>;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt_aes128(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

pub fn decrypt_aes128(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(data).unwrap()
}

pub fn encrypt_aes192(key: &[u8; 24], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes192Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

pub fn decrypt_aes192(key: &[u8; 24], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes192Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(data).unwrap()
}

pub fn encrypt_aes256(key: &[u8; 32], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

pub fn decrypt_aes256(key: &[u8; 32], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(data).unwrap()
}

pub fn generate_random_iv() -> [u8; 16] {
    rand::thread_rng().gen::<[u8; 16]>()
}