fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .include("native/crypto");

    if target_arch == "x86_64" {
        println!("cargo:warning=Building x86_64 (SIMD / Original VerusHash)");
        build.files([
            "native/crypto/verus_hash.cpp",
            "native/crypto/verus_clhash.cpp",
            "native/crypto/haraka.c",
        ]);
    } else if target_arch == "aarch64" {
        println!("cargo:warning=Building ARM64 CPU-only (Produktiv, korrekt)");
        build.files(["native/crypto/verus_hash_cpu.cpp"]);
    } else {
        panic!("Unsupported architecture: {}", target_arch);
    }

    build.compile("veruscrypto");
}
