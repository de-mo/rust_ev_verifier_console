# E-Voting Verifier Console in Rust

## Introduction

This crate is the console application for the E-Voting system of Swiss Post.

On Linux, the application uses the library gmpmee. It is actually not the case on Windows.

## Build

The build on Windows must be done with MSYS2 (see [Crypto Primitives](https://github.com/de-mo/rust_ev_crypto_primitives) for details)

## Installation

Create a file `.env` and configure it according to the following table

| Variable                  | Description                                            | Required | default |
| ------------------------- | ------------------------------------------------------ | :------: | ------- |
| VERIFIER_DATASET_PASSWORD | The password of the encrypted zip files                | X        | n/a |
| RUST_LOG                  | The log leven of  the logs (`info`, `debug`, `trace`)  |          | `info` |
| TXT_REPORT_TAB_SIZE       | The tab size for the text reports                      |          | 2 |
| DIRECT_TRUST_DIR_PATH     | The path to the direct trust keystore for the verifier |          | The path `./direct-trust` where `.` is the installation directory |



## Usage

Create a file `.env` using the delivered `env.example`:
- Modify the password (`VERIFIER_DATASET_PASSWORD`) according to the used password to encrypt the zip files

Create a directory `log` 

Copy the direct trust certificate for  the verification on the specified (in `.env`) directory or `./direct-trust`.

Launch `rust_ev_verifier_console -h` to see the help

## Licence

Open source License Apache 2.0

See [LICENSE](LICENSE)

