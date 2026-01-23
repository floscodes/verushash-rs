fn main() {
    cc::Build::new()
        .cpp(true) // C++ aktivieren
        .files([
            "native/crypto/verus_hash.cpp",
            "native/crypto/verus_clhash.cpp",
            "native/crypto/haraka.c",
        ])
        .include("native/src/crypto")
        .flag_if_supported("-std=c++17")
        .compile("veruscrypto");
}
