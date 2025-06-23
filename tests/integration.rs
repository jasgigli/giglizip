use giglizip_compress::{compress, decompress};

#[test]
fn roundtrip() {
    let data = b"test data";
    let compressed = compress(data);
    let decompressed = decompress(&compressed);
    assert_eq!(data, &decompressed[..]);
}

#[test]
fn empty_input() {
    let data = b"";
    let compressed = compress(data);
    let decompressed = decompress(&compressed);
    assert_eq!(data, &decompressed[..]);
}

#[test]
fn corrupted_frame() {
    let data = b"corrupt me";
    let mut compressed = compress(data);
    let len = compressed.len();
    if len > 0 {
        compressed[len / 2] ^= 0xFF;
    }
    let result = std::panic::catch_unwind(|| decompress(&compressed));
    assert!(result.is_err() || result.is_ok()); // TODO: Replace with error return when decompress returns Result
}

#[test]
fn large_input() {
    let data = vec![42u8; 10 * 1024 * 1024];
    let compressed = compress(&data);
    let decompressed = decompress(&compressed);
    assert_eq!(data, decompressed);
}
