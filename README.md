# boostinrust

**boostinrust** is a repository of idiomatic Rust reimplementations of selected Boost C++ libraries.

The goal of the project is to preserve the behavior, correctness, and performance characteristics of the original Boost components while expressing them in Rust with safer ownership, clearer structure, and maintainable APIs.

## Overview

This repository is part of an ongoing effort to evaluate and migrate Boost-based C++ systems toward Rust, with emphasis on systems programming, interoperability, and benchmark-driven validation.

The current `align` module is a Rust rewrite of Boost.Align-style memory-alignment utilities and AVX-oriented buffer handling. It focuses on maintaining alignment guarantees and performance parity while using idiomatic Rust abstractions.

## Goals

- Recreate Boost library functionality in Rust.
- Keep the implementation idiomatic, safe, and maintainable.
- Preserve correctness, alignment, and performance characteristics.
- Support Rust-C++ interoperability where needed.
- Provide benchmark-backed evidence for migration decisions.

## Current Module

### `align`

The `align` module implements aligned buffer handling and AVX-oriented operations inspired by Boost.Align. It is designed to validate whether a Rust implementation can match or exceed the original C++ version in real workloads.

### Benchmark Summary

A benchmark comparing Boost.Align against the Rust implementation showed:

- Correctness: pass in both versions.
- 32-byte alignment: pass in both versions.
- Throughput: Rust was slightly faster.
- Memory behavior: both versions returned pages to the OS on free.

| Metric | Boost.Align | Rust AlignedBuffer |
|---|---:|---:|
| Avg iteration time | 21.6782 ms | 20.7014 ms |
| Throughput | 6.19 GB/s | 6.48 GB/s |
| Speed ratio | 1.047x | Rust faster by 4.7% |

## Build align

Build the Rust library:

```bash
cargo build --release
```

Build and run the C++ benchmark example:

```bash
g++ -std=c++03 -O2 -mavx main.cpp \
    -Itests/include \
    -I/usr/include/boost \
    -Ltarget/release \
    -lalign \
    -lpthread -ldl \
    -o bench && ./bench
```

## Thesis Material

The `MESCCFilipeFerreiraDissertation` folder contains the LaTeX sources for the thesis and related report material supporting this work.

## Scope

This repository is not a direct translation of Boost code line by line. It is a Rust-native reinterpretation of selected Boost components, designed to be practical for research, benchmarking, and long-term maintenance.

## License

Add the appropriate license information for this repository here.