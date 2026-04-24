# Liberato Address Scheme

## Description

`liberato-address-scheme` is a library for address schemes based on `OpenInternetCryptographyKeys` and `libslug20`.

Addresses use BLAKE2B at variable digest lengths with other information attached to it with keys being PEM-Encoded.

### Algorithms Supported

- [X] ShulginSigning (SPHINCS+ & ED25519)
- [X] EsphandSigning (FALCON1024 & ED25519)
- [X] AbsolveSigning (ML-DSA3 & ED25519)