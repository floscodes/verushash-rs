// (C) 2026 flopetautschnig (floscodes)
// Distributed under the MIT software license, see the accompanying
// file COPYING or http://www.opensource.org/licenses/mit-license.php.

mod verushash;

pub fn verus_hash_v1(data: &[u8]) -> [u8; 32] {
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
    fn test_verus_hash_v1() {
        let data = b"hello world";
        let h = verus_hash_v1(data);
        println!("h: {}", hex::encode(h));

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2() {
        let data = b"hello world";
        let h = verus_hash_v2(data);
        println!("h: {}", hex::encode(h));

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_3x() {
        for _ in 0..3 {
            let data = b"hello world";
            let h = verus_hash_v2(data);
            println!("h: {}", hex::encode(h));
        }
    }

    #[test]
    fn test_verus_hash_v2_1() {
        let data = b"hello world";
        let h = verus_hash_v2_2(data);
        println!("h: {}", hex::encode(h));

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_1_3x() {
        for _ in 0..3 {
            let data = b"hello world";
            let h = verus_hash_v2_2(data);
            println!("h: {}", hex::encode(h));
        }

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_2() {
        let data = b"hello world";
        let h = verus_hash_v2_2(data);
        println!("h: {}", hex::encode(h));

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_2_3x() {
        for _ in 0..3 {
            let data = b"hello world";
            let h = verus_hash_v2_2(data);
            println!("h: {}", hex::encode(h));
        }
        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_2_1000x() {
        for _ in 0..1000 {
            let data = b"hello world";
            let h = verus_hash_v2_2(data);
            println!("h: {}", hex::encode(h));
        }

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_2_standard() {
        let version: u32 = 4;
        let prev_block = hex!("0000000000000000000000000000000000000000000000000000000000000000");
        let merkle_root = hex!("1111111111111111111111111111111111111111111111111111111111111111");
        let sapling_root = hex!("2222222222222222222222222222222222222222222222222222222222222222");
        let time: u32 = 1600000000;
        let bits: u32 = 0x1d00ffff;
        let nonce = hex!("3333333333333333333333333333333333333333333333333333333333333333");

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

        #[cfg(windows)]
        std::process::exit(0);
    }
}
