pub fn compress_entropy(data: &[u8]) -> Vec<u8> {
    // TODO: Call fse::compress or zstd_safe::compress
    data.to_vec()
}

pub fn decompress_entropy(data: &[u8]) -> Vec<u8> {
    // TODO: Call fse::decompress or zstd_safe::decompress
    data.to_vec()
}
