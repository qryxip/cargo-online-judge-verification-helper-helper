# cargo-online-judge-verification-helper-helper

[![CI](https://github.com/qryxip/cargo-online-judge-verification-helper-helper/workflows/CI/badge.svg)](https://github.com/qryxip/cargo-online-judge-verification-helper-helper/actions?workflow=CI)
[![dependency status](https://deps.rs/repo/github/qryxip/cargo-online-judge-verification-helper-helper/status.svg)](https://deps.rs/repo/github/qryxip/cargo-online-judge-verification-helper-helper)

## Installation

```console
❯ cargo install --git https://github.com/qryxip/cargo-online-judge-verification-helper-helper
```

## Usage

Note: The interface may change.

### `gen-doc rust`

```console
❯ cargo oj-verify-helper gen-doc rust -h
cargo-oj-verify-helper-gen-doc-rust 0.0.0
Collect crate-level documentation

USAGE:
    cargo oj-verify-helper gen-doc rust

OPTIONS:
        --crates-dir <PATH>       Crates directory. Defaults to { workspace_root }/crates
        --manifest-path <PATH>    Path to Cargo.toml
    -h, --help                    Prints help information
    -V, --version                 Prints version information
```

### `gen-doc oj-verify`

```console
❯ cargo oj-verify-helper gen-doc oj-verify -h
cargo-oj-verify-helper-gen-doc-oj-verify 0.0.0
Generate markdown files for oj-verify

USAGE:
    cargo oj-verify-helper gen-doc oj-verify

OPTIONS:
        --md-dir <PATH>           Markdown directory. Defaults to { workspace_root }/md
        --manifest-path <PATH>    Path to Cargo.toml
    -h, --help                    Prints help information
    -V, --version                 Prints version information
```
