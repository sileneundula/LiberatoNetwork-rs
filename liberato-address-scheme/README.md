# Liberato Address Scheme

![Static Badge](https://img.shields.io/badge/WeSpeakInCursive-purple?style=flat-square&label=Owner)
![Crates.io Version](https://img.shields.io/crates/v/liberato-address-scheme?style=flat-square)
![Crates.io Total Downloads](https://img.shields.io/crates/d/liberato-address-scheme?style=flat-square)
![Crates.io License](https://img.shields.io/crates/l/liberato-address-scheme?style=flat-square)


## Description

`liberato-address-scheme` is a library for address schemes based on `OpenInternetCryptographyKeys` and `libslug20`.

Addresses use BLAKE2B at variable digest lengths with other information attached to it with keys being PEM-Encoded.

### Algorithms Supported

- [X] ShulginSigning (SPHINCS+ & ED25519)
- [X] EsphandSigning (FALCON1024 & ED25519)
- [X] AbsolveSigning (ML-DSA3 & ED25519)
