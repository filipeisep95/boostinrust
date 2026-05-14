// main.cpp  —  Boost.Align + C++ AVX  vs  Rust AlignedBuffer + Rust AVX
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
//  Helpers
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
//  Pure C++ AVX kernel  (used by Boost side)
// ============================================================

static void double_f32_avx_cpp(float* ptr, std::size_t len) {
    __m256 two = _mm256_set1_ps(2.0f);
    for (std::size_t i = 0; i < len; i += 8) {
        __m256 v = _mm256_load_ps(ptr + i);   // requires 32-byte alignment
        v = _mm256_mul_ps(v, two);
        _mm256_store_ps(ptr + i, v);
    }
}

// ============================================================
//  Result struct
// ============================================================

struct BenchResult {
    const char* name;
    const char* allocator;
    const char* kernel;

    // alignment
    int         align_ok;
    std::size_t align_remainder;

    // memory — alloc
    long rss_before_alloc_kb;
    long rss_after_alloc_kb;
    long virt_before_alloc_kb;
    long virt_after_alloc_kb;

    // memory — run
    long rss_before_run_kb;
    long rss_after_run_kb;

    // memory — free
    long rss_before_free_kb;
    long rss_after_free_kb;

    // timing
    double total_sec;
    double per_iter_ms;
    double throughput_gbs;

    // correctness
    int result_ok;
};

// ============================================================
//  Benchmark runners
// ============================================================

static void run_boost(BenchResult* r, std::size_t N, int ITERS) {
    r->name      = "Boost.Align";
    r->allocator = "boost::alignment::aligned_alloc";
    r->kernel    = "C++ AVX (_mm256_mul_ps)";

    const std::size_t ALIGN = 32;

    MemSnapshot mb = read_mem();
    float* buf = static_cast<float*>(
        boost::alignment::aligned_alloc(ALIGN, N * sizeof(float)));
    if (!buf) { fprintf(stderr, "boost alloc failed\n"); exit(1); }
    MemSnapshot ma = read_mem();

    r->rss_before_alloc_kb  = mb.rss_kb;
    r->rss_after_alloc_kb   = ma.rss_kb;
    r->virt_before_alloc_kb = mb.virt_kb;
    r->virt_after_alloc_kb  = ma.virt_kb;

    r->align_ok        = (reinterpret_cast<std::size_t>(buf) % ALIGN == 0) ? 1 : 0;
    r->align_remainder = reinterpret_cast<std::size_t>(buf) % ALIGN;

    // warm-up — page in all memory
    for (std::size_t i = 0; i < N; ++i) buf[i] = 1.0f;
    double_f32_avx_cpp(buf, N);

    MemSnapshot mr0 = read_mem();
    double t0 = now_sec();
    for (int it = 0; it < ITERS; ++it) {
        for (std::size_t i = 0; i < N; ++i) buf[i] = 1.0f;
        double_f32_avx_cpp(buf, N);
    }
    double t1 = now_sec();
    MemSnapshot mr1 = read_mem();

    r->rss_before_run_kb = mr0.rss_kb;
    r->rss_after_run_kb  = mr1.rss_kb;
    r->total_sec         = t1 - t0;
    r->per_iter_ms       = (t1 - t0) / ITERS * 1000.0;
    r->throughput_gbs    = (double)(N * sizeof(float) * 2) * ITERS / (t1 - t0) / 1e9;

    r->result_ok = 1;
    for (std::size_t i = 0; i < 16; ++i)
        if (buf[i] != 2.0f) { r->result_ok = 0; break; }

    MemSnapshot mf0 = read_mem();
    boost::alignment::aligned_free(buf);
    MemSnapshot mf1 = read_mem();
    r->rss_before_free_kb = mf0.rss_kb;
    r->rss_after_free_kb  = mf1.rss_kb;
}

static void run_rust(BenchResult* r, std::size_t N, int ITERS) {
    r->name      = "Rust AlignedBuffer";
    r->allocator = "Rust AlignedBuffer<f32> (Box + alloc)";
    r->kernel    = "Rust AVX (aligned_buffer_double_avx)";

    const std::size_t ALIGN = 32;

    MemSnapshot mb = read_mem();
    AlignedBufferF32* rbuf = aligned_buffer_create(N, 1.0f);
    if (!rbuf) { fprintf(stderr, "rust alloc failed\n"); exit(1); }
    MemSnapshot ma = read_mem();

    r->rss_before_alloc_kb  = mb.rss_kb;
    r->rss_after_alloc_kb   = ma.rss_kb;
    r->virt_before_alloc_kb = mb.virt_kb;
    r->virt_after_alloc_kb  = ma.virt_kb;

    const float* rptr  = aligned_buffer_as_ptr(rbuf);
    r->align_ok        = (reinterpret_cast<std::size_t>(rptr) % ALIGN == 0) ? 1 : 0;
    r->align_remainder = reinterpret_cast<std::size_t>(rptr) % ALIGN;

    // warm-up
    aligned_buffer_double_avx(rbuf);

    MemSnapshot mr0 = read_mem();
    double t0 = now_sec();
    for (int it = 0; it < ITERS; ++it) {
        float* p = aligned_buffer_as_mut_ptr(rbuf);
        for (std::size_t i = 0; i < N; ++i) p[i] = 1.0f;
        aligned_buffer_double_avx(rbuf);
    }
    double t1 = now_sec();
    MemSnapshot mr1 = read_mem();

    r->rss_before_run_kb = mr0.rss_kb;
    r->rss_after_run_kb  = mr1.rss_kb;
    r->total_sec         = t1 - t0;
    r->per_iter_ms       = (t1 - t0) / ITERS * 1000.0;
    r->throughput_gbs    = (double)(N * sizeof(float) * 2) * ITERS / (t1 - t0) / 1e9;

    rptr = aligned_buffer_as_ptr(rbuf);
    r->result_ok = 1;
    for (std::size_t i = 0; i < 16; ++i)
        if (rptr[i] != 2.0f) { r->result_ok = 0; break; }

    MemSnapshot mf0 = read_mem();
    aligned_buffer_destroy(rbuf);
    MemSnapshot mf1 = read_mem();
    r->rss_before_free_kb = mf0.rss_kb;
    r->rss_after_free_kb  = mf1.rss_kb;
}

// ============================================================
//  Report
// ============================================================

static void sep(char c, int n) { for(int i=0;i<n;++i) putchar(c); putchar('\n'); }

static const char* timing_winner(double a, double b, int lower_is_better) {
    double eps = 0.02;
    double r   = (b != 0.0) ? a / b : 1.0;
    if (lower_is_better) {
        if (r < 1.0 - eps) return "<-- BETTER";
        if (r > 1.0 + eps) return "           ";
        return "~ TIE";
    } else {
        if (r > 1.0 + eps) return "<-- BETTER";
        if (r < 1.0 - eps) return "           ";
        return "~ TIE";
    }
}

static void print_report(const BenchResult* A, const BenchResult* B,
                         std::size_t N, int ITERS) {
    const int W = 76;

    long a_rss_alloc  = A->rss_after_alloc_kb  - A->rss_before_alloc_kb;
    long b_rss_alloc  = B->rss_after_alloc_kb  - B->rss_before_alloc_kb;
    long a_virt_alloc = A->virt_after_alloc_kb - A->virt_before_alloc_kb;
    long b_virt_alloc = B->virt_after_alloc_kb - B->virt_before_alloc_kb;
    long a_rss_run    = A->rss_after_run_kb    - A->rss_before_run_kb;
    long b_rss_run    = B->rss_after_run_kb    - B->rss_before_run_kb;
    long a_rss_free   = A->rss_after_free_kb   - A->rss_before_free_kb;
    long b_rss_free   = B->rss_after_free_kb   - B->rss_before_free_kb;
    double speed_ratio = (B->total_sec != 0.0) ? A->total_sec / B->total_sec : 0.0;

    sep('=', W);
    printf("  BENCHMARK REPORT  —  Boost.Align (C++ AVX)  vs  Rust AlignedBuffer (Rust AVX)\n");
    printf("  Buffer : %zu floats (%zu MB)     Iterations : %d\n",
           N, N * sizeof(float) >> 20, ITERS);
    sep('=', W);

    // ---- Implementation details -----------------------------
    printf("\n  IMPLEMENTATION\n");
    sep('-', W);
    printf("  %-16s  %-28s  %-28s\n", "",           A->name,      B->name);
    sep('-', W);
    printf("  %-16s  %-28s  %-28s\n", "Allocator",  A->allocator, B->allocator);
    printf("  %-16s  %-28s  %-28s\n", "AVX kernel", A->kernel,    B->kernel);

    // ---- Correctness ----------------------------------------
    printf("\n  CORRECTNESS\n");
    sep('-', W);
    printf("  %-24s  %-24s  %-24s\n", "Check", A->name, B->name);
    sep('-', W);
    printf("  %-24s  %-24s  %-24s\n", "Result",
           A->result_ok ? "PASS" : "FAIL",
           B->result_ok ? "PASS" : "FAIL");
    printf("  %-24s  %-24s  %-24s\n", "32-byte alignment",
           A->align_ok  ? "OK  (rem=0)" : "FAIL",
           B->align_ok  ? "OK  (rem=0)" : "FAIL");

    // ---- Timing ---------------------------------------------
    printf("\n  TIMING\n");
    sep('-', W);
    printf("  %-26s  %12s  %12s  %10s\n", "Metric", A->name, B->name, "Winner");
    sep('-', W);
    printf("  %-26s  %11.4fs  %11.4fs  %s\n",
           "Total time",
           A->total_sec, B->total_sec,
           timing_winner(A->total_sec, B->total_sec, 1));
    printf("  %-26s  %10.4fms  %10.4fms  %s\n",
           "Per-iteration",
           A->per_iter_ms, B->per_iter_ms,
           timing_winner(A->per_iter_ms, B->per_iter_ms, 1));
    printf("  %-26s  %9.2f GB/s  %9.2f GB/s  %s\n",
           "Throughput",
           A->throughput_gbs, B->throughput_gbs,
           timing_winner(A->throughput_gbs, B->throughput_gbs, 0));

    printf("\n  Speed ratio (Boost / Rust) : %.3fx  ", speed_ratio);
    if      (speed_ratio > 1.02) printf("=> Rust is %.1f%% faster\n",  (speed_ratio - 1.0) * 100.0);
    else if (speed_ratio < 0.98) printf("=> Boost is %.1f%% faster\n", (1.0 / speed_ratio - 1.0) * 100.0);
    else                         printf("=> Effectively equal\n");

    // ---- Memory — Alloc -------------------------------------
    printf("\n  MEMORY — ALLOCATION\n");
    sep('-', W);
    printf("  %-26s  %12s  %12s  %10s\n", "Metric", A->name, B->name, "Winner");
    sep('-', W);
    printf("  %-26s  %+11ldKB  %+11ldKB  %s\n",
           "RSS delta",
           a_rss_alloc, b_rss_alloc,
           timing_winner((double)a_rss_alloc, (double)b_rss_alloc, 1));
    printf("  %-26s  %+11ldKB  %+11ldKB  %s\n",
           "Virtual delta",
           a_virt_alloc, b_virt_alloc,
           timing_winner((double)a_virt_alloc, (double)b_virt_alloc, 1));

    // ---- Memory — Run ---------------------------------------
    printf("\n  MEMORY — DURING RUN\n");
    sep('-', W);
    printf("  %-26s  %+11ldKB  %+11ldKB\n", "RSS delta (run)", a_rss_run, b_rss_run);
    printf("  (near-zero expected — pages committed during warm-up)\n");

    // ---- Memory — Free --------------------------------------
    printf("\n  MEMORY — DEALLOCATION\n");
    sep('-', W);
    printf("  %-26s  %+11ldKB  %+11ldKB\n", "RSS delta (free)", a_rss_free, b_rss_free);
    printf("  %-26s  %-24s  %-24s\n",
           "Pages returned to OS",
           (a_rss_free < -32000) ? "YES  (munmap)"  : "NO   (pooled by glibc)",
           (b_rss_free < -32000) ? "YES  (munmap)"  : "NO   (pooled by glibc)");

    // ---- Summary --------------------------------------------
    printf("\n");
    sep('=', W);
    printf("  SUMMARY\n");
    sep('=', W);
    printf("  %-30s  %-20s  %-20s\n", "Metric", A->name, B->name);
    sep('-', W);
    printf("  %-30s  %-20s  %-20s\n", "Allocator",
           "Boost", "Rust");
    printf("  %-30s  %-20s  %-20s\n", "AVX kernel",
           "C++ (_mm256_mul_ps)", "Rust (FFI)");
    printf("  %-30s  %-20s  %-20s\n", "Correctness",
           A->result_ok ? "PASS" : "FAIL",
           B->result_ok ? "PASS" : "FAIL");
    printf("  %-30s  %-20s  %-20s\n", "32-byte alignment",
           A->align_ok  ? "OK" : "FAIL",
           B->align_ok  ? "OK" : "FAIL");
    printf("  %-30s  %.4fms             %.4fms\n",
           "Avg iteration time", A->per_iter_ms, B->per_iter_ms);
    printf("  %-30s  %.2f GB/s            %.2f GB/s\n",
           "Throughput", A->throughput_gbs, B->throughput_gbs);
    printf("  %-30s  %+ldKB                %+ldKB\n",
           "RSS on alloc", a_rss_alloc, b_rss_alloc);
    printf("  %-30s  %-20s  %-20s\n", "Memory returned on free",
           (a_rss_free < -32000) ? "Yes (munmap)" : "No  (pooled)",
           (b_rss_free < -32000) ? "Yes (munmap)" : "No  (pooled)");
    printf("\n  Speed ratio (Boost / Rust) : %.3fx\n", speed_ratio);
    sep('=', W);
}

// ============================================================
//  main
// ============================================================

int main() {
    const std::size_t N     = 1 << 24;   // 16M floats = 64 MB
    const int         ITERS = 50;

    BenchResult boost_r, rust_r;
    memset(&boost_r, 0, sizeof(boost_r));
    memset(&rust_r,  0, sizeof(rust_r));

    printf("Running Boost.Align benchmark...\n");
    run_boost(&boost_r, N, ITERS);

    printf("Running Rust AlignedBuffer benchmark...\n\n");
    run_rust(&rust_r, N, ITERS);

    print_report(&boost_r, &rust_r, N, ITERS);
    return 0;
}
