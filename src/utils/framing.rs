/// Block framing: prepend 4-byte length, then compressed block
pub fn frame_block(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + data.len());
    let len = data.len() as u32;
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(data);
    out
}

pub fn deframe_block(data: &[u8]) -> Option<&[u8]> {
    if data.len() < 4 { return None; }
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + len { return None; }
    Some(&data[4..4+len])
}
