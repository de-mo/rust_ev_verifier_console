# E-Voting Verifier Console in Rust

## Introduction

This crate is the console application for the E-Voting system of Swiss Post.

On Linux, the application uses the library gmpmee. It is actually not the case on Windows.

## Build

The build on Windows must be done with MSYS2 (see [Crypto Primitives](https://github.com/de-mo/rust_ev_crypto_primitives) for details)

## Usage

Create a file `.env` using the delivered `env.example`:
- Modify the password (`VERIFIER_DATASET_PASSWORD`) according to the used password of the zip files

Create a directory `log` 

Copy the direct trust certificate for  the verification on the directors `./direct-trust`.

Launch `rust_ev_verifier_console -h` to see the help

## Licence

Open source License Apache 2.0

See [LICENSE](LICENSE)

