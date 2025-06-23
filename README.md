# giglizip-compress

[![CI](https://github.com/your/repo/actions/workflows/ci.yml/badge.svg)](https://github.com/your/repo/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/giglizip-compress.svg)](https://crates.io/crates/giglizip-compress)

A fast, multi-algorithm compression CLI and library for Rust.

## Quickstart

```sh
cargo install giglizip-compress
```

## Example

```sh
giglizip --input file.txt --output file.gz --level 5 --method zstd
```

## Library Usage

```rust
let compressed = giglizip_compress::compress(b"data");
let decompressed = giglizip_compress::decompress(&compressed);
```
