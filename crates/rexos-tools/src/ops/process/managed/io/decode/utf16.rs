use super::detect::nul_positions;

pub(super) fn decode_utf16_lossy(bytes: Vec<u8>) -> String {
    let (even_nuls, odd_nuls) = nul_positions(&bytes);
    let pairs = (bytes.len() / 2).max(1) as f32;
    let even_ratio = even_nuls as f32 / pairs;
    let odd_ratio = odd_nuls as f32 / pairs;
    let is_likely_be = even_ratio > 0.60 && odd_ratio < 0.40;

    let mut u16s = Vec::with_capacity(bytes.len() / 2);
    let mut iter = bytes.chunks_exact(2);
    for chunk in &mut iter {
        let value = if is_likely_be {
            u16::from_be_bytes([chunk[0], chunk[1]])
        } else {
            u16::from_le_bytes([chunk[0], chunk[1]])
        };
        u16s.push(value);
    }

    if u16s.first() == Some(&0xFEFF) {
        u16s.remove(0);
    }

    String::from_utf16_lossy(&u16s)
}
