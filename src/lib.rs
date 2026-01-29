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
}
