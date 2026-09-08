use std::fs;
use std::process::Command;

#[test]
fn benchmark_source_is_present() {
    let path = "tests/ffi_overhead_benchmark.cpp";
    let content = fs::read_to_string(path).expect("benchmark source should exist");
    assert!(content.contains("FFI OVERHEAD ANALYSIS"), "benchmark harness missing report header");
    assert!(content.contains("Rust FFI"), "benchmark harness missing Rust FFI section");
}

#[test]
fn benchmark_binary_can_be_built() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = format!("{}/target", manifest_dir);
    let out = format!("{}/ffi_overhead_benchmark", target_dir);

    let include_dirs = [
        format!("{}/target/cxxbridge", manifest_dir),
        format!("{}/target/debug/build", manifest_dir),
        format!("{}/target/release/build", manifest_dir),
    ];
    let mut cmd = Command::new("g++");
    cmd.arg("-std=c++11")
        .arg("-O2")
        .arg("tests/ffi_overhead_benchmark.cpp");

    for dir in &include_dirs {
        if std::path::Path::new(dir).exists() {
            cmd.arg("-I").arg(dir);
        }
    }

    let mut include_candidates = vec![];
    for root in ["target/debug/build", "target/release/build"] {
        let root_path = std::path::Path::new(&manifest_dir).join(root);
        if let Ok(entries) = std::fs::read_dir(root_path) {
            for entry in entries.flatten() {
                let include_dir = entry.path().join("out/cxxbridge/include");
                if include_dir.exists() {
                    include_candidates.push(include_dir);
                }
            }
        }
    }

    for include_dir in include_candidates {
        cmd.arg("-I").arg(include_dir);
    }

    let status = cmd
        .arg("-L")
        .arg(format!("{}/target/debug", manifest_dir))
        .arg("-lendian")
        .arg("-lendian_bridge")
        .arg("-lpthread")
        .arg("-ldl")
        .arg("-o")
        .arg(&out)
        .status()
        .expect("failed to invoke g++");

    assert!(status.success(), "benchmark binary compilation failed");
    assert!(std::path::Path::new(&out).exists(), "benchmark binary was not produced");
}
