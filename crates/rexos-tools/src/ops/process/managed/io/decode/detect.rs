pub(super) fn likely_utf16(bytes: &[u8]) -> bool {
    let nuls = bytes.iter().filter(|&&byte| byte == 0).count();
    let nul_ratio = nuls as f32 / bytes.len() as f32;
    if bytes.len() < 4 || nul_ratio < 0.20 {
        return false;
    }

    let (even_nuls, odd_nuls) = nul_positions(bytes);
    let pairs = (bytes.len() / 2).max(1) as f32;
    let even_ratio = even_nuls as f32 / pairs;
    let odd_ratio = odd_nuls as f32 / pairs;

    (odd_ratio > 0.60 && even_ratio < 0.40) || (even_ratio > 0.60 && odd_ratio < 0.40)
}

pub(super) fn nul_positions(bytes: &[u8]) -> (usize, usize) {
    let mut even_nuls = 0usize;
    let mut odd_nuls = 0usize;
    for (index, byte) in bytes.iter().enumerate() {
        if *byte == 0 {
            if index % 2 == 0 {
                even_nuls += 1;
            } else {
                odd_nuls += 1;
            }
        }
    }
    (even_nuls, odd_nuls)
}
