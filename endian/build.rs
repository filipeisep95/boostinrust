use std::{env, fs, path::PathBuf};

fn main() {
    // ── Build the cxx bridge ──────────────────────────────────────────────────
    // cxx_build processes src/ffi.rs, generates C++ glue code, compiles it into
    // libendian_bridge.a, and exports headers under target/cxxbridge/:
    //   target/cxxbridge/rust/cxx.h                 ← cxx runtime
    //   target/cxxbridge/boost-endian/src/ffi.rs.h  ← generated bridge header
    cxx_build::bridge("src/ffi.rs")
        .std("c++11")
        .compile("endian_bridge");

    // ── Copy the bridge library to target/{profile}/ ──────────────────────────
    // Allows self-contained manual compilation (no need to hunt in $OUT_DIR):
    //
    //   cargo build --release
    //   g++ -std=c++11 -O2 main.cpp              \
    //       -I target/cxxbridge                   \
    //       -L target/release                     \
    //       -l endian -l endian_bridge            \
    //       -lpthread -ldl -o endian_demo
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());
    let target_dir = PathBuf::from("target").join(&profile);

    if target_dir.exists() {
        let src = out_dir.join("libendian_bridge.a");
        let dst = target_dir.join("libendian_bridge.a");
        if src.exists() {
            fs::copy(src, dst).ok();
        }
    }

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=main.cpp");
}
