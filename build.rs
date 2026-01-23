fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .include("native/crypto");

    if target_arch == "x86_64" {
        build.files([
            "native/crypto/verus_hash.cpp",
            "native/crypto/verus_clhash.cpp",
            "native/crypto/haraka.c",
        ]);
    } else {
        panic!("Unsupported architecture: {}", target_arch);
    }

    build.compile("veruscrypto");
}
