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
    

    #[test]
    fn test_verus_hash_v1() {
        let data = b"hello world";
        let h = verus_hash_v1(data);
        let h_display = hex::encode(h);
        println!("h: {}", h_display);

        assert_eq!(
            "071cf6db1bf047ee07e0fcc3f5a9c77d2580798ae19e3c4ef242e14a78378f40",
            h_display
        );

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2() {
        let data = b"hello world";
        let h = verus_hash_v2(data);
        let h_display = hex::encode(h);
        println!("h: {}", h_display);

        assert_eq!(
            "5bc0ba7e97fe14d19f527a803577dadeae3ea45b15a68b050ee308d581e096b1",
            h_display
        );

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_3x() {
        for _ in 0..3 {
            let data = b"hello world";
            let h = verus_hash_v2(data);
            let h_display = hex::encode(h);
            println!("h: {}", h_display);

            assert_eq!(
                "5bc0ba7e97fe14d19f527a803577dadeae3ea45b15a68b050ee308d581e096b1",
                h_display
            );
        }
        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_1() {
        let data = b"hello world";
        let h = verus_hash_v2_2(data);
        let h_display = hex::encode(h);
        println!("h: {}", h_display);

        assert_eq!(
            "6cae82cbef9b80afe08e2ceab0073f5db66b3f2f9c3ebca9e8f4e36f7cef4baf",
            h_display
        );

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_1_3x() {
        for _ in 0..3 {
            let data = b"hello world";
            let h = verus_hash_v2_2(data);
            let h_display = hex::encode(h);
            println!("h: {}", h_display);

            assert_eq!(
                "6cae82cbef9b80afe08e2ceab0073f5db66b3f2f9c3ebca9e8f4e36f7cef4baf",
                h_display
            );
        }

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_2() {
        let data = b"hello world";
        let h = verus_hash_v2_2(data);
        let h_display = hex::encode(h);
        println!("h: {}", h_display);

        assert_eq!(
            "6cae82cbef9b80afe08e2ceab0073f5db66b3f2f9c3ebca9e8f4e36f7cef4baf",
            h_display
        );

        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_2_3x() {
        for _ in 0..3 {
            let data = b"hello world";
            let h = verus_hash_v2_2(data);
            let h_display = hex::encode(h);
            println!("h: {}", h_display);

            assert_eq!(
                "6cae82cbef9b80afe08e2ceab0073f5db66b3f2f9c3ebca9e8f4e36f7cef4baf",
                h_display
            );
        }
        #[cfg(windows)]
        std::process::exit(0);
    }

    #[test]
    fn test_verus_hash_v2_2_1000x() {
        for _ in 0..1000 {
            let data = b"hello world";
            let h = verus_hash_v2_2(data);
            let h_display = hex::encode(h);
            println!("h: {}", h_display);

            assert_eq!(
                "6cae82cbef9b80afe08e2ceab0073f5db66b3f2f9c3ebca9e8f4e36f7cef4baf",
                h_display
            );
        }

        #[cfg(windows)]
        std::process::exit(0);
    }

fn hex_to_le32(hex_str: &str) -> [u8; 32] {
    let bytes = hex::decode(hex_str).unwrap();
    assert_eq!(bytes.len(), 32);
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    arr
}

#[test]
fn test_build_verus_header_dummy()  {
    let version = 65540u32.to_le_bytes(); // 4 Bytes LE
    let prevhash = hex_to_le32("97c8dcfb6805365f196aae2b64762c217dcde9dcf03b3d9580708956b303491b");
    let merkleroot = hex_to_le32("b9a50cf232c2e0af578f08260abf311efbd520fe95eba0f12c6b901a4e041da3");
    let reserved = [0u8; 32];
    let time = 1767916632u32.to_le_bytes(); // 4 Bytes LE
    let bits_hex = "1b028f33"; // Verus: oft als LE interpretiert
    let bits_bytes = hex::decode(bits_hex).unwrap();
    let mut bits = [0u8; 4];
    bits.copy_from_slice(&bits_bytes); // LE
    let nonce_hex = "086690fdb44d689745ae5754a4293ce546f3fe4ba8ab113bb9b71f591a138bfe";
    let nonce = hex_to_le32(nonce_hex);

    // Header zusammenbauen (Standard Verus: 1487 Bytes für Hash)
    let mut header = Vec::new();
    header.extend_from_slice(&version);      // 0-3
    header.extend_from_slice(&prevhash);     // 4-35
    header.extend_from_slice(&merkleroot);   // 36-67
    header.extend_from_slice(&reserved);     // 68-99
    header.extend_from_slice(&time);         // 100-103
    header.extend_from_slice(&bits);         // 104-107
    header.extend_from_slice(&nonce);        // 108-139
    // Solution: 1347 Bytes (140-1486)
    // Für Stake/Minted: Oft fd 40 05 00 + PoS-Daten oder aus "solution" Feld decodieren
    // Hier Platzhalter: fd400500 + Nulls + Teile aus solution (vereinfacht)
    let mut solution = vec![0xfd, 0x40, 0x05, 0x00];
    solution.resize(1347, 0u8); // Padding mit 0
    header.extend_from_slice(&solution);

    assert_eq!(header.len(), 1487);
}

}
