use giglizip_compress::{compress, decompress};

#[test]
fn roundtrip() {
    let data = b"test data";
    let compressed = compress(data);
    let decompressed = decompress(&compressed);
    assert_eq!(data, &decompressed[..]);
}
