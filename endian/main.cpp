// main.cpp — Demonstration of the Rust Boost.Endian reimplementation (cxx bridge).
//
// Build:
//   cargo build --release
//   g++ -std=c++11 -O2 main.cpp              \
//       -I target/cxxbridge                   \
//       -L target/release                     \
//       -l endian -l endian_bridge            \
//       -lpthread -ldl -o endian_demo
//   ./endian_demo
//
// The cxx bridge generates two headers under target/cxxbridge/:
//   rust/cxx.h                       — cxx runtime (rust::Slice, …)
//   boost-endian/src/ffi.rs.h        — bridge declarations in namespace boost_endian

#include "rust/cxx.h"
#include "boost-endian/src/ffi.rs.h"

#include <cstdio>
#include <cassert>
#include <cstring>

using namespace boost_endian;

// ── Helpers ───────────────────────────────────────────────────────────────────

static void print_bytes(const char* label, const uint8_t* p, int n) {
    printf("  %-30s  [", label);
    for (int i = 0; i < n; ++i) printf("%s%02X", i ? " " : "", p[i]);
    printf("]\n");
}

static bool all_passed = true;

static void check(const char* desc, bool ok) {
    printf("  %-55s %s\n", desc, ok ? "OK" : "FAIL");
    if (!ok) all_passed = false;
}

// ── Approach 1: Conversion functions ─────────────────────────────────────────

static void demo_conversion() {
    printf("\n=== Approach 1: Conversion Functions ===\n");

    // endian_reverse
    uint32_t r = endian_reverse_u32(0x01020304u);
    check("endian_reverse_u32(0x01020304) == 0x04030201", r == 0x04030201u);

    // big_to_native / native_to_big (roundtrip)
    int32_t native_val = 0x12345678;
    int32_t back = big_to_native_i32(native_to_big_i32(native_val));
    check("native_to_big / big_to_native roundtrip i32", back == native_val);

    // little_to_native / native_to_little (roundtrip)
    uint64_t u = UINT64_C(0xDEADBEEFCAFEBABE);
    check("native_to_little / little_to_native roundtrip u64",
          little_to_native_u64(native_to_little_u64(u)) == u);

    // ── Store + Load via rust::Slice ──────────────────────────────────────────
    // rust::Slice<uint8_t>(ptr, len) wraps an existing array — no allocation.
    uint8_t buf[8] = {};
    store_big_u32(rust::Slice<uint8_t>(buf, 4), 0xCAFEBABEu);
    check("store_big_u32 byte[0] == 0xCA", buf[0] == 0xCA);
    check("load_big_u32 roundtrip",
          load_big_u32(rust::Slice<const uint8_t>(buf, 4)) == 0xCAFEBABEu);

    store_little_u32(rust::Slice<uint8_t>(buf, 4), 0x01020304u);
    check("store_little_u32 byte[0] == 0x04", buf[0] == 0x04);
    check("load_little_u32 roundtrip",
          load_little_u32(rust::Slice<const uint8_t>(buf, 4)) == 0x01020304u);

    // signed 64-bit roundtrip
    store_big_s64(rust::Slice<uint8_t>(buf, 8), INT64_C(-1));
    check("store/load_big_s64(-1) roundtrip",
          load_big_s64(rust::Slice<const uint8_t>(buf, 8)) == INT64_C(-1));

    // Print byte layout of big-endian 0x01020304 (should be 01 02 03 04)
    uint32_t be = native_to_big_u32(0x01020304u);
    print_bytes("BE 0x01020304", reinterpret_cast<const uint8_t*>(&be), 4);
}

// ── Approach 2: Buffer types ──────────────────────────────────────────────────

static void demo_buffer() {
    printf("\n=== Approach 2: Buffer Types (cxx shared structs) ===\n");

    // The cxx bridge defines BigInt32Buf { int32_t raw; } in the boost_endian
    // namespace.  `raw` holds the value in big-endian byte order.
    // big_int32_buf_new / big_int32_buf_get are the explicit conversion API.

    BigInt32Buf b = big_int32_buf_new(0x01020304);
    check("big_int32_buf_get() == 0x01020304", big_int32_buf_get(b) == 0x01020304);

    // On a little-endian host the raw field should contain the bytes [01 02 03 04].
    print_bytes("BigInt32Buf.raw (BE)", reinterpret_cast<const uint8_t*>(&b.raw), 4);
    check("BigInt32Buf.raw byte[0] == 0x01 on little-endian host",
          reinterpret_cast<const uint8_t*>(&b.raw)[0] == 0x01);

    LittleInt32Buf lb = little_int32_buf_new(0x01020304);
    print_bytes("LittleInt32Buf.raw (LE)",
                reinterpret_cast<const uint8_t*>(&lb.raw), 4);
    check("LittleInt32Buf.raw byte[0] == 0x04 on little-endian host",
          reinterpret_cast<const uint8_t*>(&lb.raw)[0] == 0x04);

    check("little_int32_buf_get() == 0x01020304",
          little_int32_buf_get(lb) == 0x01020304);

    // 64-bit signed roundtrip
    LittleInt64Buf lb64 = little_int64_buf_new(INT64_C(-9999999999));
    check("LittleInt64Buf roundtrip",
          little_int64_buf_get(lb64) == INT64_C(-9999999999));

    // GIS-style mixed-endian record using cxx shared structs.
    // sizeof fields: 4+4+4+4 = 16 — all fields are naturally aligned integers
    // (the raw field in each struct has the same alignment as the primitive).
    struct Header {
        BigInt32Buf  file_code;
        BigInt32Buf  file_length;
        LittleInt32Buf version;
        LittleInt32Buf shape_type;
    };
    static_assert(sizeof(Header) == 16, "Header must be 16 bytes");

    Header h;
    h.file_code   = big_int32_buf_new(0x01020304);
    h.file_length = big_int32_buf_new(static_cast<int32_t>(sizeof(Header)));
    h.version     = little_int32_buf_new(1);
    h.shape_type  = little_int32_buf_new(0x01020304);

    check("file_code.get() == 0x01020304",
          big_int32_buf_get(h.file_code) == 0x01020304);
    check("shape_type.get() == 0x01020304",
          little_int32_buf_get(h.shape_type) == 0x01020304);
    check("file_code and shape_type have opposite byte order",
          reinterpret_cast<const uint8_t*>(&h.file_code.raw)[0] == 0x01 &&
          reinterpret_cast<const uint8_t*>(&h.shape_type.raw)[0] == 0x04);
}

// ── Approach 3: Arithmetic types (C++ wrapper on top of cxx API) ──────────────
//
// cxx does not expose operator-overloaded arithmetic types directly.  Instead
// we build a thin C++11 template on top of the bridge functions — mirroring
// Boost.Endian's endian_arithmetic class.

namespace {

template<class BufT, class T,
         BufT (*MkBuf)(T),
         T    (*GetBuf)(BufT)>
class EndianArith {
    BufT buf_;
    T native() const { return GetBuf(buf_); }
    void store(T v) { buf_ = MkBuf(v); }

public:
    EndianArith() { store(T(0)); }
    EndianArith(T v) { store(v); }       // NOLINT: intentionally implicit
    operator T() const { return native(); }

    EndianArith& operator=(T v)   { store(v); return *this; }
    EndianArith& operator+=(T v)  { store(native() + v); return *this; }
    EndianArith& operator-=(T v)  { store(native() - v); return *this; }
    EndianArith& operator*=(T v)  { store(native() * v); return *this; }
    EndianArith& operator/=(T v)  { store(native() / v); return *this; }
    EndianArith& operator&=(T v)  { store(native() & v); return *this; }
    EndianArith& operator|=(T v)  { store(native() | v); return *this; }
    EndianArith& operator^=(T v)  { store(native() ^ v); return *this; }
    EndianArith& operator<<=(T v) { store(native() << v); return *this; }
    EndianArith& operator>>=(T v) { store(native() >> v); return *this; }
    EndianArith& operator++()     { store(native() + T(1)); return *this; }
    EndianArith& operator--()     { store(native() - T(1)); return *this; }
    EndianArith  operator++(int)  { EndianArith t(*this); ++(*this); return t; }
    EndianArith  operator--(int)  { EndianArith t(*this); --(*this); return t; }
};

using big_int32_t  = EndianArith<BigInt32Buf,  int32_t,  big_int32_buf_new,  big_int32_buf_get>;
using big_uint32_t = EndianArith<BigUInt32Buf, uint32_t, big_uint32_buf_new, big_uint32_buf_get>;
using big_int64_t  = EndianArith<BigInt64Buf,  int64_t,  big_int64_buf_new,  big_int64_buf_get>;
using little_uint32_t = EndianArith<LittleUInt32Buf, uint32_t,
                                    little_uint32_buf_new, little_uint32_buf_get>;
using little_uint64_t = EndianArith<LittleUInt64Buf, uint64_t,
                                    little_uint64_buf_new, little_uint64_buf_get>;

} // anonymous namespace

static void demo_arithmetic() {
    printf("\n=== Approach 3: Arithmetic Types (C++ template over cxx bridge) ===\n");

    big_int32_t x = 100;
    check("big_int32_t implicit from int (x == 100)", int32_t(x) == 100);

    x += 42;  check("x += 42  => 142", int32_t(x) == 142);
    x *= 2;   check("x *= 2   => 284", int32_t(x) == 284);
    x -= 84;  check("x -= 84  => 200", int32_t(x) == 200);
    x /= 4;   check("x /= 4   =>  50", int32_t(x) ==  50);

    big_uint32_t ctr = 0u;
    ++ctr; ++ctr;
    check("++ctr twice   => 2", uint32_t(ctr) == 2u);
    ctr++;
    check("ctr++ post    => 3", uint32_t(ctr) == 3u);

    little_uint32_t mask = 0xFF00u;
    mask |= 0x00FFu;  check("mask |= 0x00FF => 0xFFFF", uint32_t(mask) == 0xFFFFu);
    mask &= 0x0F0Fu;  check("mask &= 0x0F0F => 0x0F0F", uint32_t(mask) == 0x0F0Fu);
    mask ^= 0xFFFFu;  check("mask ^= 0xFFFF => 0xF0F0", uint32_t(mask) == 0xF0F0u);

    little_uint64_t s = 1ull;
    s <<= 16;
    check("1 <<= 16 => 0x10000", uint64_t(s) == 0x10000ull);

    // Comparison: semantically on native values
    big_int32_t a = 10, b = 20;
    check("a < b (int32_t comparison)", int32_t(a) < int32_t(b));

    // Gauss sum via loop (Boost.Endian "Example 2")
    big_int32_t accum = 0;
    for (int32_t i = 1; i <= 100; ++i) accum += i;
    check("sum 1..100 == 5050", int32_t(accum) == 5050);

    // Hoist pattern: load once, work in native, store once
    big_int32_t big_v = 0;
    int32_t native_v = int32_t(big_v);
    for (int32_t i = 0; i < 1000; ++i) native_v += i;
    big_v = native_v;
    check("hoist conversion out of loop (Gauss 0..999)", int32_t(big_v) == 499500);
}

// ── Mixed-endian I/O record ───────────────────────────────────────────────────

static void demo_io_record() {
    printf("\n=== Real-world: mixed-endian binary record ===\n");

    // Protocol packet layout — different endianness per field.
    // cxx shared structs have the same alignment as the raw primitive, so the
    // compiler may insert padding.  Static_assert guards against surprises.
    struct Record {
        BigUInt32Buf   magic;        // big-endian  (4 bytes)
        LittleUInt16Buf seq;         // little-endian (2 bytes)
        BigInt32Buf    payload_len;  // big-endian  (4 bytes)
    };
    // 4 + 2 + [2 padding] + 4 = 12 bytes (natural alignment of int32_t).
    // Use static_assert so we notice if the ABI changes.
    static_assert(sizeof(Record) == 12, "Record size must be 12 bytes");

    Record rec;
    rec.magic       = big_uint32_buf_new(0xDEADBEEFu);
    rec.seq         = little_uint16_buf_new(42u);
    rec.payload_len = big_int32_buf_new(1024);

    check("magic.get() == 0xDEADBEEF",
          big_uint32_buf_get(rec.magic) == 0xDEADBEEFu);
    check("seq.get() == 42",
          little_uint16_buf_get(rec.seq) == 42u);
    check("payload_len.get() == 1024",
          big_int32_buf_get(rec.payload_len) == 1024);

    printf("  Record hex dump (%zu bytes): ", sizeof(rec));
    const uint8_t* p = reinterpret_cast<const uint8_t*>(&rec);
    for (size_t i = 0; i < sizeof(rec); ++i) printf("%02X ", p[i]);
    printf("\n");
    // On little-endian: DE AD BE EF  2A 00  ?? ??  00 00 04 00
    check("magic[0] == 0xDE (big-endian MSB first)", p[0] == 0xDE);
    check("seq[0]   == 0x2A (little-endian LSB first)", p[4] == 0x2A);
}

// ── main ──────────────────────────────────────────────────────────────────────

int main() {
    printf("Boost.Endian (Rust, cxx bridge) — C++ integration demo\n");
    printf("========================================================\n");

    demo_conversion();
    demo_buffer();
    demo_arithmetic();
    demo_io_record();

    printf("\n========================================================\n");
    printf("Result: %s\n", all_passed ? "ALL PASSED" : "SOME FAILED");
    return all_passed ? 0 : 1;
}
