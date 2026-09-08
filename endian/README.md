# boostinrust-endian

Rust reimplementation of selected Boost.Endian concepts, exposed to C++ through a type-safe `cxx` bridge.

## What it covers

The crate provides three layers:

1. Conversion functions for explicit native, big-endian, and little-endian conversions.
2. Buffer types that store values in a fixed byte order and convert explicitly with `get` and `set`.
3. Arithmetic types that preserve a fixed byte order while providing arithmetic operators.

The bridge also exposes byte-slice load/store functions and shared buffer structs for C++ callers. The C++ demo exercises conversions, buffers, arithmetic wrappers, and a mixed-endian binary record.

## Build and run

Run these commands from `endian/`:

```bash
cargo test
cargo build --release
g++ -std=c++11 -O2 main.cpp \
    -I target/cxxbridge \
    -L target/release \
    -l endian -l endian_bridge \
    -lpthread -ldl \
    -o endian_demo
./endian_demo
```

`cargo test` runs the Rust unit tests and verifies that the benchmark source and binary can be built. The build script generates the bridge headers under `target/cxxbridge/` and copies `libendian_bridge.a` into the active target profile. The generated declarations are in `boost-endian/src/ffi.rs.h`; the `rust/cxx.h` header supplies the `cxx` runtime types such as `rust::Slice`.

## Benchmark

The FFI benchmark compares a native C++ implementation of big-endian 64-bit load/store with the Rust implementation called through `cxx`:

- C++ native: manual byte shifts and stores.
- Rust FFI: `load_big_u64` and `store_big_u64` through `rust::Slice`.
- Four input sizes: 256, 16,384, 1,048,576, and 16,777,216 elements.
- 50 iterations per size.

Build the benchmark after building the crate so the generated headers and bridge library exist. The `find` command supplies the generated `cxx` include directory used by the bridge:

```bash
BRIDGE_INCLUDE=$(find target/release/build target/debug/build \
    -type d -path '*/out/cxxbridge/include' -print -quit)
g++ -std=c++11 -O2 tests/ffi_overhead_benchmark.cpp \
    -I target/cxxbridge \
    -I "$BRIDGE_INCLUDE" \
    -L target/release \
    -l endian -l endian_bridge \
    -lpthread -ldl \
    -o target/ffi_overhead_benchmark
./target/ffi_overhead_benchmark
```

The Rust integration tests in `tests/ffi_overhead_tests.rs` perform the same build check automatically. Run only those checks with `cargo test --test ffi_overhead_tests`.

## C++ compatibility

The bridge is compiled as C++11. C++ callers should treat the generated header as a build artifact and include `rust/cxx.h` before using `rust::Slice`. Rust panics must not cross the FFI boundary; callers should provide slices of the documented size for load/store operations.

## Scope

This crate aims to provide the endian behavior needed by the project rather than reproduce every Boost.Endian API or naming convention. The Rust modules are the source of truth for the native API, while `src/ffi.rs` defines the C++ integration surface.