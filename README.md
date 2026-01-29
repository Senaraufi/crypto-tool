# Crypto Tool

[![Rust](https://img.shields.io/badge/rust-1.91.1-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-blue.svg)](https://crates.io/)

> A lightweight, high-performance command-line cryptographic toolkit built in Rust. Crypto Tool provides secure implementations of industry-standard encryption algorithms (AES-128/192/256, RSA) and cryptographic hash functions (SHA-256), designed for developers who need reliable encryption, decryption, and data integrity verification in their workflows.

## Overview
The Crypto Tool is a command-line application designed for performing various cryptographic operations, including encryption, decryption, and hashing. It implements popular algorithms such as AES and RSA, providing a user-friendly interface for secure data handling.

## Features
- **Encryption and Decryption**: Supports AES and RSA algorithms.
- **Hashing**: Implements various hash functions, including SHA-256.
- **Encoding**: Provides utility functions for encoding and decoding data (e.g., Base64).
- **Command-Line Interface**: Easy to use CLI for user interaction.

## Getting Started

### Prerequisites
- Rust programming language installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation
1. Clone the repository:
   ```
   git clone https://github.com/yourusername/crypto-tool.git
   ```
2. Navigate to the project directory:
   ```
   cd crypto-tool
   ```
3. Build the project:
   ```
   cargo build
   ```

### Usage
To run the application, use the following command:
```
cargo run -- [options]
```
Replace `[options]` with the desired commands for encryption, decryption, or hashing.

### Contributing
Contributions are welcome! Please submit a pull request or open an issue for any suggestions or improvements.

### License
This project is licensed under the MIT License. See the LICENSE file for more details.