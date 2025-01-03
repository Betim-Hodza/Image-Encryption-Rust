# Image Encryption System

Rust project built to encrypt and decrypt images using AES256Cbc 

## Getting Started

If you have rust already installed, then just clone this repo and get to encrypting!

Note when encrypting, please save your secret keys in safe spot! (or don't if you don't wanna)

### Prerequisites

Requirements for the software and other tools to build, test and push 
- [rust](https://www.rust-lang.org/)

### Installing

Instead of me telling you how to install rust, i'll direct you to them since they've done so.
- [rust installation](https://www.rust-lang.org/learn/get-started)


## Built With
```
[package]
name = "Image-Encryption-System"
version = "0.1.0"
edition = "2021"

[dependencies]
aes = "0.7"
block-modes = "0.8"
rand = "0.8"
anyhow = "1.0"
clap = "4.0"
hex = "0.4"
```

## Author

- **Betim Hodza** - [Betim-Hodza](https://github.com/Betim-Hodza)

## License

This project is licensed under the [MIT License](LICENSE)
