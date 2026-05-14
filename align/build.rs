fn main() {
    // Don't compile main.cpp here — it's the test driver, not a library
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=tests/include/aligned_simd.h");
    println!("cargo:rerun-if-changed=main.cpp");
}