use giglizip_compress::{compress, decompress};

fn main() {
    let data = b"Hello, world!";
    let compressed = compress(data);
    let decompressed = decompress(&compressed);
    assert_eq!(data, &decompressed[..]);
    println!("Compression and decompression succeeded.");
}
