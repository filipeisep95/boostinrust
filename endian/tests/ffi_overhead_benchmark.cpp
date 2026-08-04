#include "boost-endian/src/ffi.rs.h"
#include "rust/cxx.h"

#include <cassert>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <vector>

using namespace boost_endian;

static double now_sec() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return static_cast<double>(ts.tv_sec) + static_cast<double>(ts.tv_nsec) * 1e-9;
}

struct BenchResult {
    const char* name;
    const char* method;
    double total_sec;
    double per_iter_ms;
    double per_iter_ns;
    double throughput_gbs;
    int result_ok;
};

static BenchResult benchmark_cpp_native(std::size_t N, int ITERS) {
    BenchResult r{};
    r.name = "C++ Native";
    r.method = "manual big-endian load/store";
    r.total_sec = 0.0;
    r.per_iter_ms = 0.0;
    r.per_iter_ns = 0.0;
    r.throughput_gbs = 0.0;
    r.result_ok = 0;

    std::vector<std::uint8_t> buf(N * 8, 0);
    std::uint64_t checksum = 0;

    for (std::size_t i = 0; i < N; ++i) {
        std::uint64_t value = 0x0102030405060708ULL + i;
        for (int j = 7; j >= 0; --j) {
            buf[i * 8 + (7 - j)] = static_cast<std::uint8_t>((value >> (j * 8)) & 0xFFu);
        }
    }

    double t0 = now_sec();
    for (int it = 0; it < ITERS; ++it) {
        for (std::size_t i = 0; i < N; ++i) {
            std::uint64_t v = 0;
            for (int j = 0; j < 8; ++j) {
                v = (v << 8) | buf[i * 8 + j];
            }
            std::uint64_t next = v + 1;
            for (int j = 7; j >= 0; --j) {
                buf[i * 8 + (7 - j)] = static_cast<std::uint8_t>((next >> (j * 8)) & 0xFFu);
            }
            checksum += v;
        }
    }
    double t1 = now_sec();

    r.result_ok = (checksum != 0) ? 1 : 0;
    r.total_sec = t1 - t0;
    r.per_iter_ms = (t1 - t0) / ITERS * 1000.0;
    r.per_iter_ns = (t1 - t0) / ITERS * 1e9;
    r.throughput_gbs = static_cast<double>(N * 8 * 2) * ITERS / (t1 - t0) / 1e9;
    return r;
}

static BenchResult benchmark_rust_ffi(std::size_t N, int ITERS) {
    BenchResult r{};
    r.name = "Rust FFI";
    r.method = "cxx bridge load/store";
    r.total_sec = 0.0;
    r.per_iter_ms = 0.0;
    r.per_iter_ns = 0.0;
    r.throughput_gbs = 0.0;
    r.result_ok = 0;

    std::vector<std::uint8_t> buf(N * 8, 0);
    std::uint64_t checksum = 0;

    for (std::size_t i = 0; i < N; ++i) {
        store_big_u64(rust::Slice<std::uint8_t>(buf.data() + i * 8, 8), 0x0102030405060708ULL + i);
    }

    double t0 = now_sec();
    for (int it = 0; it < ITERS; ++it) {
        for (std::size_t i = 0; i < N; ++i) {
            std::uint64_t v = load_big_u64(rust::Slice<const std::uint8_t>(buf.data() + i * 8, 8));
            store_big_u64(rust::Slice<std::uint8_t>(buf.data() + i * 8, 8), v + 1);
            checksum += v;
        }
    }
    double t1 = now_sec();

    r.result_ok = (checksum != 0) ? 1 : 0;
    r.total_sec = t1 - t0;
    r.per_iter_ms = (t1 - t0) / ITERS * 1000.0;
    r.per_iter_ns = (t1 - t0) / ITERS * 1e9;
    r.throughput_gbs = static_cast<double>(N * 8 * 2) * ITERS / (t1 - t0) / 1e9;
    return r;
}

static void sep(char c, int n) {
    for (int i = 0; i < n; ++i) {
        std::putchar(c);
    }
    std::putchar('\n');
}

static void print_report(const BenchResult* cpp_native, const BenchResult* rust_ffi, std::size_t N, int ITERS) {
    const int W = 90;
    double overhead = (rust_ffi->per_iter_ns / cpp_native->per_iter_ns - 1.0) * 100.0;

    sep('=', W);
    std::printf("  FFI OVERHEAD ANALYSIS  --  endianv3 (C++ vs Rust FFI)\n");
    std::printf("  Elements : %zu (about %zu bytes)     Iterations : %d\n", N, N * 8, ITERS);
    sep('=', W);

    std::printf("\n  IMPLEMENTATION\n");
    sep('-', W);
    std::printf("  %-40s  %-48s\n", "Method", "Details");
    sep('-', W);
    std::printf("  %-40s  %-48s\n", "C++ Native", "manual big-endian byte shifts");
    std::printf("  %-40s  %-48s\n", "Rust FFI", "cxx bridge load/store via rust::Slice");

    std::printf("\n  CORRECTNESS\n");
    sep('-', W);
    std::printf("  %-40s  %s\n", "Method", "Result");
    sep('-', W);
    std::printf("  %-40s  %s\n", cpp_native->name, cpp_native->result_ok ? "PASS" : "FAIL");
    std::printf("  %-40s  %s\n", rust_ffi->name, rust_ffi->result_ok ? "PASS" : "FAIL");

    std::printf("\n  TIMING (Per-iteration)\n");
    sep('-', W);
    std::printf("  %-40s  %14s  %14s  %10s\n", "Method", "Time (ms)", "Time (ns)", "Throughput");
    sep('-', W);
    std::printf("  %-40s  %13.4fms  %13.0fns  %9.2f GB/s\n",
                cpp_native->name, cpp_native->per_iter_ms, cpp_native->per_iter_ns, cpp_native->throughput_gbs);
    std::printf("  %-40s  %13.4fms  %13.0fns  %9.2f GB/s  %+.1f%%\n",
                rust_ffi->name, rust_ffi->per_iter_ms, rust_ffi->per_iter_ns, rust_ffi->throughput_gbs, overhead);

    std::printf("\n");
    sep('=', W);
}

int main() {
    const int ITERS = 50;
    std::size_t sizes[] = {1 << 8, 1 << 14, 1 << 20, 1 << 24};

    for (std::size_t size : sizes) {
        std::printf("Running benchmarks for %zu elements (%zu bytes)...\n", size, size * 8);
        BenchResult cpp = benchmark_cpp_native(size, ITERS);
        BenchResult rust = benchmark_rust_ffi(size, ITERS);
        print_report(&cpp, &rust, size, ITERS);
        std::printf("\n\n");
    }

    return 0;
}
