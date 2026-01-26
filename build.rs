fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // 1. C-Dateien kompilieren (Haraka)
    let mut build_c = cc::Build::new();
    build_c
        .flag("-fno-lto")
        .include("native/crypto")
        .files(["native/crypto/haraka.c", "native/crypto/haraka_portable.c"]);

    // Architekturspezifische Flags für C-Teil
    if target_arch == "x86_64" {
        build_c.flag("-maes").flag("-msse4.1").flag("-mpclmul");
    } else if target_arch == "aarch64" {
        build_c.flag("-march=armv8-a+crypto");
    }
    build_c.compile("haraka");

    // 2. C++-Dateien kompilieren (VerusCore & Wrapper)
    let mut build_cpp = cc::Build::new();
    build_cpp
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .flag("-fno-lto")
        .flag("-mmacosx-version-min=15.5")
        .include("native/crypto")
        .include("/opt/homebrew/include")
        .files([
            "native/crypto/verus_hash.cpp",
            "native/crypto/verus_clhash.cpp",
            "native/crypto/verus_clhash_portable.cpp",
            "native/verushash.cc",
        ]);

    if target_arch == "x86_64" {
        build_cpp.flag("-maes").flag("-msse4.1").flag("-mpclmul");
    } else if target_arch == "aarch64" {
        build_cpp.flag("-march=armv8-a+crypto");
    }

    build_cpp.compile("verushash");

    // Rust mitteilen, dass wir gegen C++ linken müssen (besonders auf macOS wichtig)
    println!("cargo:rustc-link-lib=c++");
}
