//! Type-safe cxx bridge (C++11 compatible).
//!
//! # How it works
//!
//! The `#[cxx::bridge]` macro declares the shared interface.  cxx-build
//! compiles the generated C++ glue into **`libendian_bridge.a`** and exports
//! two headers under `target/cxxbridge/`:
//!
//! | File | Purpose |
//! |------|---------|
//! | `rust/cxx.h` | cxx runtime (`rust::Slice`, etc.) |
//! | `endian/src/ffi.rs.h` | generated bridge declarations |
//!
//! # Compiling main.cpp manually
//!
//! ```bash
//! cargo build --release
//! g++ -std=c++11 -O2 main.cpp           \
//!     -I target/cxxbridge               \
//!     -L target/release                 \
//!     -l endian -l endian_bridge        \
//!     -lpthread -ldl -o endian_demo
//! ```

use crate::conversion as conv;

// ── cxx bridge ────────────────────────────────────────────────────────────────

#[cxx::bridge(namespace = "boost_endian")]
pub mod ffi {
    // ── Shared buffer structs ─────────────────────────────────────────────────
    //
    // `raw` stores the value in the *target* byte order — the bytes are already
    // suitable for binary I/O without further conversion.  The field is
    // intentionally public so C++ code can memcpy to/from network packets.

    // big-endian signed
    struct BigInt16Buf  { raw: i16 }
    struct BigInt32Buf  { raw: i32 }
    struct BigInt64Buf  { raw: i64 }
    // big-endian unsigned
    struct BigUInt16Buf { raw: u16 }
    struct BigUInt32Buf { raw: u32 }
    struct BigUInt64Buf { raw: u64 }
    // little-endian signed
    struct LittleInt16Buf  { raw: i16 }
    struct LittleInt32Buf  { raw: i32 }
    struct LittleInt64Buf  { raw: i64 }
    // little-endian unsigned
    struct LittleUInt16Buf { raw: u16 }
    struct LittleUInt32Buf { raw: u32 }
    struct LittleUInt64Buf { raw: u64 }

    // ── Rust functions exposed to C++ ─────────────────────────────────────────
    extern "Rust" {
        // ── Endian reversal ───────────────────────────────────────────────────
        fn endian_reverse_u16(v: u16) -> u16;
        fn endian_reverse_i16(v: i16) -> i16;
        fn endian_reverse_u32(v: u32) -> u32;
        fn endian_reverse_i32(v: i32) -> i32;
        fn endian_reverse_u64(v: u64) -> u64;
        fn endian_reverse_i64(v: i64) -> i64;

        // ── big ↔ native ──────────────────────────────────────────────────────
        fn big_to_native_u16(v: u16) -> u16;
        fn big_to_native_i16(v: i16) -> i16;
        fn big_to_native_u32(v: u32) -> u32;
        fn big_to_native_i32(v: i32) -> i32;
        fn big_to_native_u64(v: u64) -> u64;
        fn big_to_native_i64(v: i64) -> i64;

        fn native_to_big_u16(v: u16) -> u16;
        fn native_to_big_i16(v: i16) -> i16;
        fn native_to_big_u32(v: u32) -> u32;
        fn native_to_big_i32(v: i32) -> i32;
        fn native_to_big_u64(v: u64) -> u64;
        fn native_to_big_i64(v: i64) -> i64;

        // ── little ↔ native ───────────────────────────────────────────────────
        fn little_to_native_u16(v: u16) -> u16;
        fn little_to_native_i16(v: i16) -> i16;
        fn little_to_native_u32(v: u32) -> u32;
        fn little_to_native_i32(v: i32) -> i32;
        fn little_to_native_u64(v: u64) -> u64;
        fn little_to_native_i64(v: i64) -> i64;

        fn native_to_little_u16(v: u16) -> u16;
        fn native_to_little_i16(v: i16) -> i16;
        fn native_to_little_u32(v: u32) -> u32;
        fn native_to_little_i32(v: i32) -> i32;
        fn native_to_little_u64(v: u64) -> u64;
        fn native_to_little_i64(v: i64) -> i64;

        // ── Load  (byte slice → native value) ─────────────────────────────────
        // p must be a slice of at least N bytes; panics otherwise.
        // In C++: rust::Slice<const uint8_t>(ptr, len)
        fn load_big_u16   (p: &[u8]) -> u16;
        fn load_big_s16   (p: &[u8]) -> i16;
        fn load_little_u16(p: &[u8]) -> u16;
        fn load_little_s16(p: &[u8]) -> i16;

        fn load_big_u32   (p: &[u8]) -> u32;
        fn load_big_s32   (p: &[u8]) -> i32;
        fn load_little_u32(p: &[u8]) -> u32;
        fn load_little_s32(p: &[u8]) -> i32;

        fn load_big_u64   (p: &[u8]) -> u64;
        fn load_big_s64   (p: &[u8]) -> i64;
        fn load_little_u64(p: &[u8]) -> u64;
        fn load_little_s64(p: &[u8]) -> i64;

        // ── Store  (native value → mutable byte slice) ────────────────────────
        // In C++: rust::Slice<uint8_t>(ptr, len)
        fn store_big_u16   (p: &mut [u8], v: u16);
        fn store_big_s16   (p: &mut [u8], v: i16);
        fn store_little_u16(p: &mut [u8], v: u16);
        fn store_little_s16(p: &mut [u8], v: i16);

        fn store_big_u32   (p: &mut [u8], v: u32);
        fn store_big_s32   (p: &mut [u8], v: i32);
        fn store_little_u32(p: &mut [u8], v: u32);
        fn store_little_s32(p: &mut [u8], v: i32);

        fn store_big_u64   (p: &mut [u8], v: u64);
        fn store_big_s64   (p: &mut [u8], v: i64);
        fn store_little_u64(p: &mut [u8], v: u64);
        fn store_little_s64(p: &mut [u8], v: i64);

        // ── Buffer constructors ───────────────────────────────────────────────
        // Accept a native value, return a struct whose `raw` holds it in the
        // target byte order.  Accessors convert back to native on demand.
        fn big_int16_buf_new  (v: i16) -> BigInt16Buf;
        fn big_int32_buf_new  (v: i32) -> BigInt32Buf;
        fn big_int64_buf_new  (v: i64) -> BigInt64Buf;
        fn big_uint16_buf_new (v: u16) -> BigUInt16Buf;
        fn big_uint32_buf_new (v: u32) -> BigUInt32Buf;
        fn big_uint64_buf_new (v: u64) -> BigUInt64Buf;

        fn little_int16_buf_new  (v: i16) -> LittleInt16Buf;
        fn little_int32_buf_new  (v: i32) -> LittleInt32Buf;
        fn little_int64_buf_new  (v: i64) -> LittleInt64Buf;
        fn little_uint16_buf_new (v: u16) -> LittleUInt16Buf;
        fn little_uint32_buf_new (v: u32) -> LittleUInt32Buf;
        fn little_uint64_buf_new (v: u64) -> LittleUInt64Buf;

        // ── Buffer accessors ──────────────────────────────────────────────────
        fn big_int16_buf_get  (b: BigInt16Buf)  -> i16;
        fn big_int32_buf_get  (b: BigInt32Buf)  -> i32;
        fn big_int64_buf_get  (b: BigInt64Buf)  -> i64;
        fn big_uint16_buf_get (b: BigUInt16Buf) -> u16;
        fn big_uint32_buf_get (b: BigUInt32Buf) -> u32;
        fn big_uint64_buf_get (b: BigUInt64Buf) -> u64;

        fn little_int16_buf_get  (b: LittleInt16Buf)  -> i16;
        fn little_int32_buf_get  (b: LittleInt32Buf)  -> i32;
        fn little_int64_buf_get  (b: LittleInt64Buf)  -> i64;
        fn little_uint16_buf_get (b: LittleUInt16Buf) -> u16;
        fn little_uint32_buf_get (b: LittleUInt32Buf) -> u32;
        fn little_uint64_buf_get (b: LittleUInt64Buf) -> u64;
    }
}

// ── Implementations ───────────────────────────────────────────────────────────
// Must be in the same module scope as the bridge declaration (this file is the
// `ffi` module from lib.rs, so cxx resolves `super::fn_name` here).

// Endian reversal
fn endian_reverse_u16(v: u16) -> u16 { conv::endian_reverse(v) }
fn endian_reverse_i16(v: i16) -> i16 { conv::endian_reverse(v) }
fn endian_reverse_u32(v: u32) -> u32 { conv::endian_reverse(v) }
fn endian_reverse_i32(v: i32) -> i32 { conv::endian_reverse(v) }
fn endian_reverse_u64(v: u64) -> u64 { conv::endian_reverse(v) }
fn endian_reverse_i64(v: i64) -> i64 { conv::endian_reverse(v) }

// big ↔ native
fn big_to_native_u16(v: u16) -> u16 { conv::big_to_native(v) }
fn big_to_native_i16(v: i16) -> i16 { conv::big_to_native(v) }
fn big_to_native_u32(v: u32) -> u32 { conv::big_to_native(v) }
fn big_to_native_i32(v: i32) -> i32 { conv::big_to_native(v) }
fn big_to_native_u64(v: u64) -> u64 { conv::big_to_native(v) }
fn big_to_native_i64(v: i64) -> i64 { conv::big_to_native(v) }

fn native_to_big_u16(v: u16) -> u16 { conv::native_to_big(v) }
fn native_to_big_i16(v: i16) -> i16 { conv::native_to_big(v) }
fn native_to_big_u32(v: u32) -> u32 { conv::native_to_big(v) }
fn native_to_big_i32(v: i32) -> i32 { conv::native_to_big(v) }
fn native_to_big_u64(v: u64) -> u64 { conv::native_to_big(v) }
fn native_to_big_i64(v: i64) -> i64 { conv::native_to_big(v) }

// little ↔ native
fn little_to_native_u16(v: u16) -> u16 { conv::little_to_native(v) }
fn little_to_native_i16(v: i16) -> i16 { conv::little_to_native(v) }
fn little_to_native_u32(v: u32) -> u32 { conv::little_to_native(v) }
fn little_to_native_i32(v: i32) -> i32 { conv::little_to_native(v) }
fn little_to_native_u64(v: u64) -> u64 { conv::little_to_native(v) }
fn little_to_native_i64(v: i64) -> i64 { conv::little_to_native(v) }

fn native_to_little_u16(v: u16) -> u16 { conv::native_to_little(v) }
fn native_to_little_i16(v: i16) -> i16 { conv::native_to_little(v) }
fn native_to_little_u32(v: u32) -> u32 { conv::native_to_little(v) }
fn native_to_little_i32(v: i32) -> i32 { conv::native_to_little(v) }
fn native_to_little_u64(v: u64) -> u64 { conv::native_to_little(v) }
fn native_to_little_i64(v: i64) -> i64 { conv::native_to_little(v) }

// Load — slice must be at least N bytes; panics otherwise (converted to
// std::terminate / noexcept abort on the C++ side by cxx's default policy).
fn load_big_u16   (p: &[u8]) -> u16 { u16::from_be_bytes([p[0], p[1]]) }
fn load_big_s16   (p: &[u8]) -> i16 { i16::from_be_bytes([p[0], p[1]]) }
fn load_little_u16(p: &[u8]) -> u16 { u16::from_le_bytes([p[0], p[1]]) }
fn load_little_s16(p: &[u8]) -> i16 { i16::from_le_bytes([p[0], p[1]]) }

fn load_big_u32   (p: &[u8]) -> u32 { u32::from_be_bytes([p[0],p[1],p[2],p[3]]) }
fn load_big_s32   (p: &[u8]) -> i32 { i32::from_be_bytes([p[0],p[1],p[2],p[3]]) }
fn load_little_u32(p: &[u8]) -> u32 { u32::from_le_bytes([p[0],p[1],p[2],p[3]]) }
fn load_little_s32(p: &[u8]) -> i32 { i32::from_le_bytes([p[0],p[1],p[2],p[3]]) }

fn load_big_u64(p: &[u8]) -> u64 {
    u64::from_be_bytes([p[0],p[1],p[2],p[3],p[4],p[5],p[6],p[7]])
}
fn load_big_s64(p: &[u8]) -> i64 {
    i64::from_be_bytes([p[0],p[1],p[2],p[3],p[4],p[5],p[6],p[7]])
}
fn load_little_u64(p: &[u8]) -> u64 {
    u64::from_le_bytes([p[0],p[1],p[2],p[3],p[4],p[5],p[6],p[7]])
}
fn load_little_s64(p: &[u8]) -> i64 {
    i64::from_le_bytes([p[0],p[1],p[2],p[3],p[4],p[5],p[6],p[7]])
}

// Store — writes N bytes into the slice; panics if slice is too short.
fn store_big_u16   (p: &mut [u8], v: u16) { p[..2].copy_from_slice(&v.to_be_bytes()); }
fn store_big_s16   (p: &mut [u8], v: i16) { p[..2].copy_from_slice(&v.to_be_bytes()); }
fn store_little_u16(p: &mut [u8], v: u16) { p[..2].copy_from_slice(&v.to_le_bytes()); }
fn store_little_s16(p: &mut [u8], v: i16) { p[..2].copy_from_slice(&v.to_le_bytes()); }

fn store_big_u32   (p: &mut [u8], v: u32) { p[..4].copy_from_slice(&v.to_be_bytes()); }
fn store_big_s32   (p: &mut [u8], v: i32) { p[..4].copy_from_slice(&v.to_be_bytes()); }
fn store_little_u32(p: &mut [u8], v: u32) { p[..4].copy_from_slice(&v.to_le_bytes()); }
fn store_little_s32(p: &mut [u8], v: i32) { p[..4].copy_from_slice(&v.to_le_bytes()); }

fn store_big_u64   (p: &mut [u8], v: u64) { p[..8].copy_from_slice(&v.to_be_bytes()); }
fn store_big_s64   (p: &mut [u8], v: i64) { p[..8].copy_from_slice(&v.to_be_bytes()); }
fn store_little_u64(p: &mut [u8], v: u64) { p[..8].copy_from_slice(&v.to_le_bytes()); }
fn store_little_s64(p: &mut [u8], v: i64) { p[..8].copy_from_slice(&v.to_le_bytes()); }

// Buffer constructors — convert native → target byte order, store in `raw`.
fn big_int16_buf_new  (v: i16) -> ffi::BigInt16Buf  { ffi::BigInt16Buf  { raw: conv::native_to_big(v) } }
fn big_int32_buf_new  (v: i32) -> ffi::BigInt32Buf  { ffi::BigInt32Buf  { raw: conv::native_to_big(v) } }
fn big_int64_buf_new  (v: i64) -> ffi::BigInt64Buf  { ffi::BigInt64Buf  { raw: conv::native_to_big(v) } }
fn big_uint16_buf_new (v: u16) -> ffi::BigUInt16Buf { ffi::BigUInt16Buf { raw: conv::native_to_big(v) } }
fn big_uint32_buf_new (v: u32) -> ffi::BigUInt32Buf { ffi::BigUInt32Buf { raw: conv::native_to_big(v) } }
fn big_uint64_buf_new (v: u64) -> ffi::BigUInt64Buf { ffi::BigUInt64Buf { raw: conv::native_to_big(v) } }

fn little_int16_buf_new  (v: i16) -> ffi::LittleInt16Buf  { ffi::LittleInt16Buf  { raw: conv::native_to_little(v) } }
fn little_int32_buf_new  (v: i32) -> ffi::LittleInt32Buf  { ffi::LittleInt32Buf  { raw: conv::native_to_little(v) } }
fn little_int64_buf_new  (v: i64) -> ffi::LittleInt64Buf  { ffi::LittleInt64Buf  { raw: conv::native_to_little(v) } }
fn little_uint16_buf_new (v: u16) -> ffi::LittleUInt16Buf { ffi::LittleUInt16Buf { raw: conv::native_to_little(v) } }
fn little_uint32_buf_new (v: u32) -> ffi::LittleUInt32Buf { ffi::LittleUInt32Buf { raw: conv::native_to_little(v) } }
fn little_uint64_buf_new (v: u64) -> ffi::LittleUInt64Buf { ffi::LittleUInt64Buf { raw: conv::native_to_little(v) } }

// Buffer accessors — convert stored bytes back to native.
fn big_int16_buf_get  (b: ffi::BigInt16Buf)  -> i16 { conv::big_to_native(b.raw) }
fn big_int32_buf_get  (b: ffi::BigInt32Buf)  -> i32 { conv::big_to_native(b.raw) }
fn big_int64_buf_get  (b: ffi::BigInt64Buf)  -> i64 { conv::big_to_native(b.raw) }
fn big_uint16_buf_get (b: ffi::BigUInt16Buf) -> u16 { conv::big_to_native(b.raw) }
fn big_uint32_buf_get (b: ffi::BigUInt32Buf) -> u32 { conv::big_to_native(b.raw) }
fn big_uint64_buf_get (b: ffi::BigUInt64Buf) -> u64 { conv::big_to_native(b.raw) }

fn little_int16_buf_get  (b: ffi::LittleInt16Buf)  -> i16 { conv::little_to_native(b.raw) }
fn little_int32_buf_get  (b: ffi::LittleInt32Buf)  -> i32 { conv::little_to_native(b.raw) }
fn little_int64_buf_get  (b: ffi::LittleInt64Buf)  -> i64 { conv::little_to_native(b.raw) }
fn little_uint16_buf_get (b: ffi::LittleUInt16Buf) -> u16 { conv::little_to_native(b.raw) }
fn little_uint32_buf_get (b: ffi::LittleUInt32Buf) -> u32 { conv::little_to_native(b.raw) }
fn little_uint64_buf_get (b: ffi::LittleUInt64Buf) -> u64 { conv::little_to_native(b.raw) }

