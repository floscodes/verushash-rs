mod verus;

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
    fn test_verus_block_3911208() {
        // Der konstruierte Header für Block 3911208 (140 Bytes)
        // Format: Version + PrevHash + MerkleRoot + SaplingRoot + Time + Bits + Nonce
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
        let mut result = verus_hash(&header_bytes);

        // WICHTIG: Für die Anzeige/Explorer muss das Byte-Array umgedreht werden
        result.reverse();

        let result_hex = hex::encode(result);

        assert_eq!(result_hex, expected_hash_hex);
    }

    #[test]
    fn test_2_verus_block_3911208() {
        // Baue 1487 Bytes (little-endian):
        let mut header = Vec::new();

        // Version LE
        header.extend_from_slice(&65540u32.to_le_bytes());

        // Prevhash reversed
        header.extend(
            hex::decode("8555411fd8c46508a3a166ff52bc21fdb9d3633ae20ee4fee6aa257fdcf17bab")
                .unwrap(),
        );

        // Merkleroot reversed
        header.extend(
            hex::decode("8b5c9a26578ded9888695876dcc669bbef31d26142a3f6222654be8abef3f82b")
                .unwrap(),
        );

        // Finalsaplingroot als reserved reversed
        header.extend(
            hex::decode("2ca7fe83e5b594a36d546143e19a67325b569d7a9ffdba453d152144cab9b289")
                .unwrap(),
        );

        // Time LE
        header.extend(1769165301u32.to_le_bytes());

        // Bits LE reversed
        header.extend(hex::decode("0562021b").unwrap());

        // Nonce reversed (32 Bytes)
        header.extend(
            hex::decode("0000f4c2224281c37af556fa4396eceb8eb746332476c7442692a68a76ed17a5")
                .unwrap(),
        );

        // Solution (1347 Bytes): fd 05 40 (VarInt 1344) + 1344 Bytes data
        let solution_hex = "07000000000104009d7632dc0efb7a0aeb36dc8a110f1e4db9791ae79596a73a\
                        65a2a01a6b06106db306c7ce7f2a44fadb2ad985c536db08dd27efc7a0f65267\
                        fcb6258d97e21e6a1af5b8015c64d39ab44c60ead8317f9f5a9b6c4c5cbfbbb\
                        98aa2ed55c8706319353e3e33e19bbaa918b58fcbaecf7eb04dbed545010000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        0000000000000000000000000000000000000000000000000000000000000000\
                        37ebae01ff090000000000798a49"; // Dein voller solution-String, Länge genau 1347 Bytes checken!

        let solution = hex::decode(solution_hex).unwrap();
        assert_eq!(solution.len(), 1347);
        header.extend(solution);

        assert_eq!(header.len(), 1487);

        // Jetzt hashen
        let result = verus_hash(&header);
        let mut result_reversed = result;
        result_reversed.reverse();
        let result_hex = hex::encode(result_reversed);

        let expected = "0000000000005f1a85389d4671aae324ab32757dc1b537160e74c015a3571599";
        assert_eq!(result_hex, expected);
    }
}
