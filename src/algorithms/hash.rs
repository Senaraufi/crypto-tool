// This file implements hashing functions, including SHA-256.

use sha2::{Sha256, Digest};

pub fn hash_sha256(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

// Additional hashing functions can be added here.