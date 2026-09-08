# boostinrust-align

Rust aligned-buffer utilities and AVX operations inspired by Boost.Align. The crate is built as a static library and exposes a small C-compatible surface for C++ callers.

## What it covers

- 32-byte-aligned allocation for `f32` buffers.
- Rust AVX kernels operating on aligned buffers.
- C++ integration through the generated header in `tests/include/`.
- Comparisons against `boost::alignment::aligned_alloc` and a native C++ AVX kernel.

## Build and test

Run these commands from `align/`:

```bash
cargo test --lib
cargo build --release
g++ -std=c++03 -O2 -mavx main.cpp \
	-Itests/include \
	-I/usr/include/boost \
	-Ltarget/release \
	-lalign -lpthread -ldl \
	-o bench
./bench
```

`cargo test --lib` runs the library unit tests. The repository's integration test currently expects an rlib crate target, while this crate is configured as a static library, so use the library test command above.

The C++ demo requires a C++03-compatible compiler, a CPU with AVX support, and Boost headers providing Boost.Align.

## FFI benchmark

Build and run the FFI-overhead harness from `align/`:

```bash
cargo build --release
g++ -std=c++11 -O2 -mavx tests/ffi_overhead_benchmark.cpp \
	-Itests/include \
	-I/usr/include/boost \
	-Ltarget/release \
	-lalign -lpthread -ldl \
	-o target/ffi_overhead_benchmark
./target/ffi_overhead_benchmark
```

It runs four buffer sizes for 50 iterations and compares native C++, Rust through `AlignedBuffer`, and Rust through a raw pointer. Its captured output is recorded in [tests/ffi_overhead_results.md](tests/ffi_overhead_results.md).

---

## Benchmark Report
**Boost.Align (C++ AVX)** vs **Rust AlignedBuffer (Rust AVX)**

**Buffer:** 16,777,216 floats (64 MB) | **Iterations:** 50

---

### Implementation

| | Boost.Align | Rust AlignedBuffer |
|---|---|---|
| Allocator | `boost::alignment::aligned_alloc` | `Rust AlignedBuffer<f32>` (Box + alloc) |
| AVX kernel | C++ AVX (`_mm256_mul_ps`) | Rust AVX (`aligned_buffer_double_avx`) |

---

### Correctness

| Check | Boost.Align | Rust AlignedBuffer |
|---|---|---|
| Result | ✅ PASS | ✅ PASS |
| 32-byte alignment | OK (rem=0) | OK (rem=0) |

---

### Timing

| Metric | Boost.Align | Rust AlignedBuffer | Winner |
|---|---|---|---|
| Total time | 1.0839s | 1.0351s | ⬅ Rust |
| Per-iteration | 21.6782ms | 20.7014ms | ⬅ Rust |
| Throughput | 6.19 GB/s | 6.48 GB/s | ⬅ Rust |

> **Speed ratio (Boost / Rust):** 1.047x — Rust is **4.7% faster**

---

### Memory — Allocation

| Metric | Boost.Align | Rust AlignedBuffer | Winner |
|---|---|---|---|
| RSS delta | +196 KB | +65,540 KB | ⬅ Boost |
| Virtual delta | +65,540 KB | +65,540 KB | ~ TIE |

---

### Memory — During Run

| Metric | Boost.Align | Rust AlignedBuffer |
|---|---|---|
| RSS delta (run) | +64 KB | +0 KB |

> Near-zero expected — pages committed during warm-up.

---

### Memory — Deallocation

| Metric | Boost.Align | Rust AlignedBuffer |
|---|---|---|
| RSS delta (free) | -65,540 KB | -65,540 KB |
| Pages returned to OS | Yes (munmap) | Yes (munmap) |

---

### Summary

| Metric | Boost.Align | Rust AlignedBuffer |
|---|---|---|
| Allocator | Boost | Rust |
| AVX kernel | C++ (`_mm256_mul_ps`) | Rust (FFI) |
| Correctness | ✅ PASS | ✅ PASS |
| 32-byte alignment | OK | OK |
| Avg iteration time | 21.6782 ms | 20.7014 ms |
| Throughput | 6.19 GB/s | 6.48 GB/s |
| RSS on alloc | +196 KB | +65,540 KB |
| Memory returned on free | Yes (munmap) | Yes (munmap) |

> **Overall speed ratio (Boost/Rust):** 1.047x - Rust wins on throughput, Boost wins on RSS footprint.

## FFI-overhead benchmark

This second benchmark compares three methods over 50 iterations:

- C++ native: Boost allocation and a C++ AVX kernel.
- Rust FFI with `AlignedBuffer`: Rust allocation, wrapper access, and the Rust AVX kernel.
- Rust FFI with a raw pointer: Boost allocation and a direct Rust kernel call.

The results below show the smallest and largest tested buffers. Negative percentages mean the Rust method completed faster than the C++ baseline; the fixed wrapper cost becomes less significant as the buffer grows.

| Buffer | C++ native | Rust FFI (`AlignedBuffer`) | Rust FFI (raw pointer) |
|---|---:|---:|---:|
| 256 floats | 148 ns / 13.86 GB/s | 146 ns / 14.02 GB/s (-1.1%) | 137 ns / 14.99 GB/s (-7.6%) |
| 16,777,216 floats | 7,110,144 ns / 18.88 GB/s | 7,103,436 ns / 18.89 GB/s (-0.1%) | 6,981,330 ns / 19.23 GB/s (-1.8%) |

The complete captured output, including 16,384- and 1,048,576-float runs, is in [tests/ffi_overhead_results.md](tests/ffi_overhead_results.md).

## Interpreting the results

The end-to-end benchmark measures allocation, initialization, the AVX operation, and deallocation. The FFI benchmark focuses on the call and wrapper cost while repeating the same buffer operation. These are complementary measurements and should not be treated as a single score.

Results are machine-specific. Repeat the benchmarks on the target hardware before using them as a migration decision.
