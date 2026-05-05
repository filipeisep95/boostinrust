fn main() {
    cxx_build::bridge("src/ffi.rs")
        .std("c++17")
        .compile("aligned-rs");

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=include/aligned.h");
}