# boostinrust

Rust reimplementations of selected Boost components, with C++ interoperability and benchmark-driven validation.

The repository currently contains two independent Rust crates:

| Crate | Purpose | Documentation |
|---|---|---|
| `align` | 32-byte-aligned buffers and AVX operations | [align/README.md](align/README.md) |
| `endian` | Endian conversion, buffer, and arithmetic types | [endian/README.md](endian/README.md) |

Each crate has its own `Cargo.toml`, build script, C++ example, tests, and benchmark notes. Run Cargo commands from the crate directory you are working on.

## Prerequisites

- Rust toolchain with Cargo.
- A C++ compiler with C++11 support.
- Boost headers, including Boost.Align, for the `align` C++ comparisons.
- An AVX-capable CPU for the align examples and benchmarks.

The `endian` crate builds its C++ bridge with the `cxx-build` dependency during `cargo build`.

## Repository layout

- `align/` - Boost.Align-inspired allocation and SIMD experiments.
- `endian/` - Boost.Endian-inspired Rust implementation exposed to C++ through `cxx`.
- `MESCCFilipeFerreiraDissertation/` - dissertation sources and generated thesis material.
- `benchmarktests.md` - possible extensions to the benchmark suite.
- `endian_comparison.md` - comparison notes for the endian implementation.

## Design goals

- Preserve the relevant behavior and correctness guarantees of the Boost APIs.
- Use Rust ownership and type-safety where they improve the implementation.
- Keep C++ integration explicit and reproducible.
- Support performance comparisons with native C++ baselines.

## Quick start

```bash
cd align
cargo test --lib
cargo build --release
g++ -std=c++03 -O2 -mavx main.cpp \
	-Itests/include -I/usr/include/boost -Ltarget/release \
	-lalign -lpthread -ldl -o bench
./bench

cd ../endian
cargo test
cargo build --release
g++ -std=c++11 -O2 main.cpp \
	-I target/cxxbridge -L target/release \
	-l endian -l endian_bridge -lpthread -ldl -o endian_demo
./endian_demo
```

For the FFI benchmark commands and troubleshooting notes, see the linked README files above.

## Thesis Material

The `MESCCFilipeFerreiraDissertation` folder contains the LaTeX sources for the thesis and related report material supporting this work.

## Scope

This is not a line-by-line translation of Boost. It is a Rust-native reinterpretation of selected components for systems programming research, C++ integration, and maintainable production experiments.

## License

License information has not yet been added.