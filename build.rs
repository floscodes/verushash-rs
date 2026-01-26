fn main() {
    //let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let mut build = cc::Build::new();

    //if target_arch == "x86_64" {
    build
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .flag("-fno-lto")
        .flag("-mmacosx-version-min=15.5")
        .flag_if_supported("-maes")
        .flag_if_supported("-msse4.1")
        .flag_if_supported("-mpclmul")
        .include("native/crypto")
        .files([
            "native/crypto/verus_hash.cpp",
            "native/crypto/verus_clhash.cpp",
            "native/crypto/verus_clhash_portable.cpp",
            "native/crypto/haraka.c",
            "native/crypto/haraka_portable.c",
            "native/verushash.cc",
        ]);
    //} else {
    //panic!("Unsupported architecture: {}", target_arch);
    //}
    println!("cargo:rustc-link-lib=System");
    build.compile("verushash");
}
