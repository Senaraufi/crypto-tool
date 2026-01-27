// RSA algorithm implementation in Rust

use rand::rngs::OsRng;
use rsa::{pkcs1v15::Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub struct RSA {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl RSA {
    pub fn new(bits: usize) -> Self {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        RSA {
            private_key,
            public_key,
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut rng = OsRng;
        self.public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, data)
            .expect("Failed to encrypt")
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8> {
        self.private_key
            .decrypt(Pkcs1v15Encrypt, encrypted_data)
            .expect("Failed to decrypt")
    }

    pub fn get_public_key(&self) -> RsaPublicKey {
        self.public_key.clone()
    }

    pub fn get_private_key(&self) -> RsaPrivateKey {
        self.private_key.clone()
    }
}