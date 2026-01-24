mod verus;

pub fn verus_hash_init() {
    unsafe {
        verus::verus_hash_init();
    }
}

pub fn verus_hash_v2_init() {
    unsafe {
        verus::verus_hash_v2_init();
    }
}

pub fn verus_hash(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe {
        verus::verus_hash(out.as_mut_ptr(), data.as_ptr(), data.len());
    }
    out
}

pub fn verus_hash_v2(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe {
        verus::verus_hash_v2(out.as_mut_ptr(), data.as_ptr(), data.len());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashes() {
        verus_hash_init();
        verus_hash_v2_init();
        let data = b"hello world";
        let h1 = verus_hash(data);
        let h2 = verus_hash_v2(data);
        assert_ne!(h1, h2); // v1 und v2 sollten unterschiedlich sein
    }

    #[test]
    fn test_verus_block_3911208() {
        verus_hash_v2_init();
        let header_hex = concat!(
            "04000100", // Version (Little-Endian)
            "ab7bf1dc7f25aae6fe440ee23a63d3b9fd21bc52ff66a1a30865c4d81f415585", // PrevHash (umgedreht)
            "2bf83fbefa8e542622f6a34261d231efbb69c6dc7658698898ed8d57269a5c8b", // MerkleRoot (umgedreht)
            "89b2b9ca4421153d45baf99f7a9d565b32679ae14361546da394b5e583fea72c", // FinalSaplingRoot (umgedreht)
            "f581d269",                                                         // Time: 1769165301
            "0562021b",                                                         // Bits: 1b026205
            "a517ed768aa6922644c776243346b78eb7ece943fa56f57ac3814222c2f40000"  // Nonce (umgedreht)
        );

        let header_bytes = hex::decode(header_hex).unwrap();

        // Das ist der "hash" aus deinem JSON (Ziel-Ergebnis)
        let expected_hash_hex = "0000000000005f1a85389d4671aae324ab32757dc1b537160e74c015a3571599";

        // Berechnung
        let mut result = verus_hash_v2(&header_bytes);

        // WICHTIG: Für die Anzeige/Explorer muss das Byte-Array umgedreht werden
        result.reverse();

        let result_hex = hex::encode(result);

        assert_eq!(result_hex, expected_hash_hex);
    }
}
