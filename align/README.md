# boostinrust

# Report 17-05-2026

## Commands

To build the Rust library:

`cargo build --release`

To build the test example in C++:
`g++ -std=c++03 -O2 -mavx main.cpp     -Itests/include     -I/usr/include/boost     -Ltarget/release     -lalign     -lpthread -ldl     -o bench && ./bench`

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

> **Overall speed ratio (Boost/Rust):** 1.047x — Rust wins on throughput, Boost wins on RSS footprint.
