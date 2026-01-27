// RSA algorithm implementation in Rust

use rand::Rng;
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use sha2::{Sha256, Digest};

pub struct RSA {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl RSA {
    pub fn new(bits: usize) -> Self {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        RSA { private_key, public_key }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        self.public_key.encrypt(&mut rand::thread_rng(), padding, data).expect("Failed to encrypt")
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8> {
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        self.private_key.decrypt(padding, encrypted_data).expect("Failed to decrypt")
    }

    pub fn get_public_key(&self) -> RsaPublicKey {
        self.public_key.clone()
    }

    pub fn get_private_key(&self) -> RsaPrivateKey {
        self.private_key.clone()
    }
}