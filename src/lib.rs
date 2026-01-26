mod verushash;

pub fn verus_hash(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe {
        verushash::verus_v1_hash(out.as_mut_ptr(), data.as_ptr(), data.len());
    }
    out
}

pub fn verus_hash_v2(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe {
        verushash::verus_v2_hash(out.as_mut_ptr(), data.as_ptr(), data.len());
    }
    out
}

pub fn verus_hash_v2_1(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe {
        verushash::verus_v2_1_hash(out.as_mut_ptr(), data.as_ptr(), data.len());
    }
    out
}

pub fn verus_hash_v2_2(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe {
        verushash::verus_v2_2_hash(out.as_mut_ptr(), data.as_ptr(), data.len());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_hashes() {
        let data = b"hello world";
        let h1 = verus_hash(data);
        let h2 = verus_hash_v2(data);
        assert_ne!(h1, h2); // v1 und v2 should be different
    }

    #[test]
    fn test_verus_hash_v2_2_standard() {
        let version: u32 = 4;
        let mut prev_block =
            hex!("0000000000000000000000000000000000000000000000000000000000000000");
        let mut merkle_root =
            hex!("1111111111111111111111111111111111111111111111111111111111111111");
        let mut sapling_root =
            hex!("2222222222222222222222222222222222222222222222222222222222222222");
        let time: u32 = 1600000000;
        let bits: u32 = 0x1d00ffff; // nBits als u32
        let mut nonce = hex!("3333333333333333333333333333333333333333333333333333333333333333");

        let mut header = Vec::with_capacity(140);
        header.extend_from_slice(&version.to_le_bytes());
        header.extend_from_slice(&prev_block);
        header.extend_from_slice(&merkle_root);
        header.extend_from_slice(&sapling_root);
        header.extend_from_slice(&time.to_le_bytes());
        header.extend_from_slice(&bits.to_le_bytes());
        header.extend_from_slice(&nonce);

        let result = verus_hash_v2_2(&header);

        assert_ne!(result, [0u8; 32]); // check whether the result varies for different nonces.
    }
}
