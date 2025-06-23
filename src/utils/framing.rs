use crc32fast::Hasher;

/// Block framing: prepend 4-byte length, then 4-byte CRC32, then compressed block
pub fn frame_block(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + data.len());
    let len = data.len() as u32;
    let mut hasher = Hasher::new();
    hasher.update(data);
    let crc = hasher.finalize();
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&crc.to_le_bytes());
    out.extend_from_slice(data);
    out
}

/// Returns Ok(payload) or Err if frame is corrupted or incomplete
pub fn deframe_block(data: &[u8]) -> Result<&[u8], &'static str> {
    if data.len() < 8 { return Err("Frame too short"); }
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let crc = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    if data.len() < 8 + len { return Err("Frame incomplete"); }
    let payload = &data[8..8+len];
    let mut hasher = Hasher::new();
    hasher.update(payload);
    if hasher.finalize() != crc {
        return Err("CRC mismatch");
    }
    Ok(payload)
}
