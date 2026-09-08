// align/tests/ffi_overhead_bench.cpp
// FFI Overhead Analysis: C++ Native vs Rust FFI
// Measures the cost of calling Rust code from C++ across language boundaries

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <cmath>
#include <immintrin.h>

#include <boost/align/aligned_alloc.hpp>
#include <boost/align/is_aligned.hpp>

#include "aligned_simd.h"

// ============================================================
//  Timing utilities
// ============================================================

static double now_sec() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec + (double)ts.tv_nsec * 1e-9;
}

struct MemSnapshot { long rss_kb; long virt_kb; };

static MemSnapshot read_mem() {
    MemSnapshot s = {0, 0};
    FILE* f = fopen("/proc/self/status", "r");
    if (!f) return s;
    char line[128];
    while (fgets(line, sizeof(line), f)) {
        if (strncmp(line, "VmRSS:",  6) == 0) sscanf(line + 6, "%ld", &s.rss_kb);
        if (strncmp(line, "VmSize:", 7) == 0) sscanf(line + 7, "%ld", &s.virt_kb);
    }
    fclose(f);
    return s;
}

// ============================================================
//  C++ Native AVX kernel (baseline)
// ============================================================

static void double_f32_avx_cpp(float* ptr, std::size_t len) {
    __m256 two = _mm256_set1_ps(2.0f);
    for (std::size_t i = 0; i < len; i += 8) {
        __m256 v = _mm256_load_ps(ptr + i);
        v = _mm256_mul_ps(v, two);
        _mm256_store_ps(ptr + i, v);
    }
}

// ============================================================
//  Result struct for single benchmark
// ============================================================

struct BenchResult {
    const char* name;
    const char* method;         // "C++ Native", "Rust FFI (AlignedBuffer)", "Rust FFI (raw ptr)"
    double total_sec;
    double per_iter_ms;
    double per_iter_ns;
    double throughput_gbs;
    int result_ok;
};

// ============================================================
//  Benchmark runners
// ============================================================

static BenchResult benchmark_cpp_native(std::size_t N, int ITERS) {
    BenchResult r;
    r.name = "C++ Native";
    r.method = "C++ (_mm256_mul_ps)";
    r.total_sec = 0;
    r.per_iter_ms = 0;
    r.per_iter_ns = 0;
    r.throughput_gbs = 0;
    r.result_ok = 0;
    
    const std::size_t ALIGN = 32;
    float* buf = static_cast<float*>(
        boost::alignment::aligned_alloc(ALIGN, N * sizeof(float)));
    if (!buf) { fprintf(stderr, "alloc failed\n"); exit(1); }
    
    // Warm-up
    for (std::size_t i = 0; i < N; ++i) buf[i] = 1.0f;
    double_f32_avx_cpp(buf, N);
    
    // Benchmark
    double t0 = now_sec();
    for (int it = 0; it < ITERS; ++it) {
        for (std::size_t i = 0; i < N; ++i) buf[i] = 1.0f;
        double_f32_avx_cpp(buf, N);
    }
    double t1 = now_sec();
    
    r.result_ok = (buf[0] == 2.0f) ? 1 : 0;
    r.total_sec = t1 - t0;
    r.per_iter_ms = (t1 - t0) / ITERS * 1000.0;
    r.per_iter_ns = (t1 - t0) / ITERS * 1e9;
    r.throughput_gbs = (double)(N * sizeof(float) * 2) * ITERS / (t1 - t0) / 1e9;
    
    boost::alignment::aligned_free(buf);
    return r;
}

static BenchResult benchmark_rust_ffi_aligned_buffer(std::size_t N, int ITERS) {
    BenchResult r;
    r.name = "Rust FFI (AlignedBuffer)";
    r.method = "Rust FFI (Box wrapper)";
    r.total_sec = 0;
    r.per_iter_ms = 0;
    r.per_iter_ns = 0;
    r.throughput_gbs = 0;
    r.result_ok = 0;
    
    AlignedBufferF32* rbuf = aligned_buffer_create(N, 1.0f);
    if (!rbuf) { fprintf(stderr, "rust alloc failed\n"); exit(1); }
    
    // Warm-up
    aligned_buffer_double_avx(rbuf);
    
    // Benchmark
    double t0 = now_sec();
    for (int it = 0; it < ITERS; ++it) {
        float* p = aligned_buffer_as_mut_ptr(rbuf);
        for (std::size_t i = 0; i < N; ++i) p[i] = 1.0f;
        aligned_buffer_double_avx(rbuf);
    }
    double t1 = now_sec();
    
    const float* rptr = aligned_buffer_as_ptr(rbuf);
    r.result_ok = (rptr[0] == 2.0f) ? 1 : 0;
    r.total_sec = t1 - t0;
    r.per_iter_ms = (t1 - t0) / ITERS * 1000.0;
    r.per_iter_ns = (t1 - t0) / ITERS * 1e9;
    r.throughput_gbs = (double)(N * sizeof(float) * 2) * ITERS / (t1 - t0) / 1e9;
    
    aligned_buffer_destroy(rbuf);
    return r;
}

static BenchResult benchmark_rust_ffi_raw_ptr(std::size_t N, int ITERS) {
    BenchResult r;
    r.name = "Rust FFI (raw ptr)";
    r.method = "Rust FFI (direct kernel)";
    r.total_sec = 0;
    r.per_iter_ms = 0;
    r.per_iter_ns = 0;
    r.throughput_gbs = 0;
    r.result_ok = 0;
    
    const std::size_t ALIGN = 32;
    float* buf = static_cast<float*>(
        boost::alignment::aligned_alloc(ALIGN, N * sizeof(float)));
    if (!buf) { fprintf(stderr, "alloc failed\n"); exit(1); }
    
    // Warm-up
    for (std::size_t i = 0; i < N; ++i) buf[i] = 1.0f;
    double_f32_avx_raw(buf, N);
    
    // Benchmark
    double t0 = now_sec();
    for (int it = 0; it < ITERS; ++it) {
        for (std::size_t i = 0; i < N; ++i) buf[i] = 1.0f;
        double_f32_avx_raw(buf, N);
    }
    double t1 = now_sec();
    
    r.result_ok = (buf[0] == 2.0f) ? 1 : 0;
    r.total_sec = t1 - t0;
    r.per_iter_ms = (t1 - t0) / ITERS * 1000.0;
    r.per_iter_ns = (t1 - t0) / ITERS * 1e9;
    r.throughput_gbs = (double)(N * sizeof(float) * 2) * ITERS / (t1 - t0) / 1e9;
    
    boost::alignment::aligned_free(buf);
    return r;
}

// ============================================================
//  Reporting
// ============================================================

static void sep(char c, int n) { 
    int i;
    for (i = 0; i < n; ++i) putchar(c); 
    putchar('\n'); 
}

static const char* timing_winner(double a, double b, int lower_is_better) {
    double eps = 0.02;
    double r = (b != 0.0) ? a / b : 1.0;
    if (lower_is_better) {
        if (r < 1.0 - eps) return "<-- FASTER";
        if (r > 1.0 + eps) return "           ";
        return "~ TIE";
    } else {
        if (r > 1.0 + eps) return "<-- BETTER";
        if (r < 1.0 - eps) return "           ";
        return "~ TIE";
    }
}

static void print_overhead_report(
    const BenchResult* cpp_native,
    const BenchResult* rust_ffi_buf,
    const BenchResult* rust_ffi_ptr,
    std::size_t N, int ITERS) {
    
    const int W = 90;
    
    double overhead_buf = (rust_ffi_buf->per_iter_ns / cpp_native->per_iter_ns - 1.0) * 100.0;
    double overhead_ptr = (rust_ffi_ptr->per_iter_ns / cpp_native->per_iter_ns - 1.0) * 100.0;
    
    sep('=', W);
    printf("  FFI OVERHEAD ANALYSIS  --  Boost.Align (C++ AVX) vs Rust (FFI)\n");
    printf("  Buffer : %zu floats (%zu MB)     Iterations : %d\n",
           N, (N * sizeof(float)) >> 20, ITERS);
    sep('=', W);
    
    // Implementation details
    printf("\n  IMPLEMENTATION\n");
    sep('-', W);
    printf("  %-40s  %-48s\n", "Method", "Details");
    sep('-', W);
    printf("  %-40s  %-48s\n", "C++ Native", "boost::aligned_alloc + _mm256_mul_ps");
    printf("  %-40s  %-48s\n", "Rust FFI (AlignedBuffer)", "Rust Box wrapper + FFI call + assertions");
    printf("  %-40s  %-48s\n", "Rust FFI (raw ptr)", "C++-alloc'd ptr + direct Rust kernel");
    
    // Correctness
    printf("\n  CORRECTNESS\n");
    sep('-', W);
    printf("  %-40s  %s\n", "Method", "Result");
    sep('-', W);
    printf("  %-40s  %s\n", cpp_native->name, cpp_native->result_ok ? "PASS" : "FAIL");
    printf("  %-40s  %s\n", rust_ffi_buf->name, rust_ffi_buf->result_ok ? "PASS" : "FAIL");
    printf("  %-40s  %s\n", rust_ffi_ptr->name, rust_ffi_ptr->result_ok ? "PASS" : "FAIL");
    
    // Timing
    printf("\n  TIMING (Per-iteration)\n");
    sep('-', W);
    printf("  %-40s  %14s  %14s  %10s\n", "Method", "Time (ms)", "Time (ns)", "Throughput");
    sep('-', W);
    printf("  %-40s  %13.4fms  %13.0fns  %9.2f GB/s\n",
           cpp_native->name, cpp_native->per_iter_ms, cpp_native->per_iter_ns, cpp_native->throughput_gbs);
    printf("  %-40s  %13.4fms  %13.0fns  %9.2f GB/s  %s\n",
           rust_ffi_buf->name, rust_ffi_buf->per_iter_ms, rust_ffi_buf->per_iter_ns, 
           rust_ffi_buf->throughput_gbs,
           timing_winner(rust_ffi_buf->per_iter_ns, cpp_native->per_iter_ns, 1));
    printf("  %-40s  %13.4fms  %13.0fns  %9.2f GB/s  %s\n",
           rust_ffi_ptr->name, rust_ffi_ptr->per_iter_ms, rust_ffi_ptr->per_iter_ns, 
           rust_ffi_ptr->throughput_gbs,
           timing_winner(rust_ffi_ptr->per_iter_ns, cpp_native->per_iter_ns, 1));
    
    // FFI Overhead Analysis
    printf("\n  FFI OVERHEAD ANALYSIS\n");
    sep('-', W);
    printf("  Method comparison vs C++ Native:\n\n");
    printf("    Rust FFI (AlignedBuffer): %+.2f%% overhead (%+.0f ns)\n",
           overhead_buf, rust_ffi_buf->per_iter_ns - cpp_native->per_iter_ns);
    printf("    Rust FFI (raw ptr):       %+.2f%% overhead (%+.0f ns)\n",
           overhead_ptr, rust_ffi_ptr->per_iter_ns - cpp_native->per_iter_ns);
    
    printf("\n    Breakdown:\n");
    printf("      AlignedBuffer wrapper overhead: %+.0f ns\n",
           rust_ffi_buf->per_iter_ns - rust_ffi_ptr->per_iter_ns);
    printf("      (Box dereferencing + assertions)\n");
    
    // Summary table
    printf("\n");
    sep('=', W);
    printf("  SUMMARY\n");
    sep('=', W);
    printf("  %-40s  %14s  %14s\n", "Method", "Per-iter (ns)", "Throughput");
    sep('-', W);
    printf("  %-40s  %13.0f  %13.2f GB/s\n", cpp_native->name, cpp_native->per_iter_ns, cpp_native->throughput_gbs);
    printf("  %-40s  %13.0f  %13.2f GB/s  (%+.1f%%)\n", 
           rust_ffi_buf->name, rust_ffi_buf->per_iter_ns, rust_ffi_buf->throughput_gbs, overhead_buf);
    printf("  %-40s  %13.0f  %13.2f GB/s  (%+.1f%%)\n", 
           rust_ffi_ptr->name, rust_ffi_ptr->per_iter_ns, rust_ffi_ptr->throughput_gbs, overhead_ptr);
    sep('=', W);
}

// ============================================================
//  main
// ============================================================

int main() {
    const int ITERS = 50;
    std::size_t test_sizes[4] = {
        1 << 8,    // 256 floats (1 KB)
        1 << 14,   // 16K floats (64 KB)
        1 << 20,   // 1M floats (4 MB)
        1 << 24,   // 16M floats (64 MB)
    };
    int size_idx;
    
    printf("\n");
    
    // Test with different buffer sizes
    for (size_idx = 0; size_idx < 4; ++size_idx) {
        std::size_t size = test_sizes[size_idx];
        BenchResult cpp;
        BenchResult rust_buf;
        BenchResult rust_ptr;
        
        printf("Running benchmarks for %zu floats (%zu MB)...\n", 
               size, (size * sizeof(float)) >> 20);
        
        cpp = benchmark_cpp_native(size, ITERS);
        rust_buf = benchmark_rust_ffi_aligned_buffer(size, ITERS);
        rust_ptr = benchmark_rust_ffi_raw_ptr(size, ITERS);
        
        print_overhead_report(&cpp, &rust_buf, &rust_ptr, size, ITERS);
        printf("\n\n");
    }
    
    printf("================================================================================\n");
    printf("Interpretation Guide:\n");
    printf("  * FFI overhead (positive %%): FFI call cost on top of work\n");
    printf("  * Overhead trend: decreases as buffer size increases (fixed cost amortized)\n");
    printf("  * AlignedBuffer vs raw ptr: wrapper overhead (Box, assertions, ptr derefs)\n");
    printf("================================================================================\n\n");
    
    return 0;
}
