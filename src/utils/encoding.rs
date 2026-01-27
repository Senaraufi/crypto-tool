// This file contains functions for encoding and decoding data, such as Base64 encoding.

use base64::{encode, decode};
use std::error::Error;

pub fn base64_encode(data: &[u8]) -> String {
    encode(data)
}

pub fn base64_decode(encoded: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded = decode(encoded)?;
    Ok(decoded)
}