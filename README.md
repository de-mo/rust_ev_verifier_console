# E-Voting Verifier Console in Rust

## Introduction

This crate is the console application for the E-Voting system of Swiss Post.

On Linux, the application uses the library gmpmee. It is actually not the case on Windows.

## Build

The build on Windows must be done with MSYS2 (see [Crypto Primitives](https://github.com/de-mo/rust_ev_crypto_primitives) for details)

## Installation

Openssl must be installed on the system.

Create a file `.env` and configure it according to the following table

| Variable                  | Description                                            | Required | default |
| ------------------------- | ------------------------------------------------------ | :------: | ------- |
| VERIFIER_DATASET_PASSWORD | The password of the encrypted zip files                | X        | n/a |
| RUST_LOG                  | The log leven of  the logs (`info`, `debug`, `trace`)  |          | `info` |
| TXT_TAB_SIZE              | The tab size for the text reports and logs             |          | 2 |
| REPORT_FORMAT_DATE        | The format of the date in the report                   |          | `%d.%m.%Y %H:%M:%S.%3f` |
| DIRECT_TRUST_DIR_PATH     | The path to the direct trust keystore for the verifier |          | The path `./direct-trust` where `.` is the installation directory |
| REPORT_EXPORT_PDF         | true if the pdf report must be extracted, false else   |          | False
| REPORT_EXPORT_HTML        | true if the html report must be extracted, false else. If pdf is true, the html report will be also extracted   |          | False
| REPORT_EXPORT_TXT         | true if the pdf report must be extracted, false else   |          | False (if no report format is specified, the text report will be extracted) |
| REPORT_ELECTORAL_BOARD_MEMBERS | List of person in the admin board (signature)     |          | Empty (then 2 places for the signature will be generated) |
| REPORT_LOGO               | Path to the logo as png (emtpy = no logo)              |          | Empty (= no logo)     |
| REPORT_BROWSER_PATH       | Path to the browser executable.(e.g. msedge.exe)       |          | Empty (error if pdf report is expected)     |
| REPORT_BROWSER_SANDBOX    | Run the browser in sandbox mode                        |         | False                  |

## Usage

Create a file `.env` using the delivered `env.example`:
- Modify the password (`VERIFIER_DATASET_PASSWORD`) according to the used password to encrypt the zip files

Create a directory `log` 

Copy the direct trust certificate for  the verification on the specified (in `.env`) directory or `./direct-trust`.

Launch `rust_ev_verifier_console -h` to see the help

Launch the setup verification with the command:
```bash
rust_ev_verifier_console setup --context-zip <path_to_dataset>
```

Launch the tally verification with the command:
```bash
rust_ev_verifier_console tally --context-zip <path_to_dataset> --tally-zip <path_to_dataset>
```

If an error occurs during the generation of the report (e.g. browser not found for pdf report), the verification is not impacted and considered as successful. The application generates a json file with the report information in the `report` directory. You can adapt the file `.env` and relaunch the generation of the report with the command (without redoing the verification):
```bash
rust_ev_verifier_console report --input <path_to_json_file>
```

## Licence

rust_ev_verifier_console is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

See [LICENSE](LICENSE)

