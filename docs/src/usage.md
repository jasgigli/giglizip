# Usage

## CLI

```
giglizip --input file.txt --output file.gz --level 5 --method zstd
```

## Library

```rust
let compressed = giglizip_compress::compress(b"data");
let decompressed = giglizip_compress::decompress(&compressed);
```
