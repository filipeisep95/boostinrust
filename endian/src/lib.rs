//! Boost.Endian reimplemented in idiomatic Rust.
//!
//! Three approaches mirror the original Boost.Endian library:
//!
//! 1. **Conversion functions** ([`conversion`]) — operate on standard integer
//!    and float types; the caller decides when conversion occurs.
//! 2. **Buffer types** ([`buffer`]) — store values as raw bytes in a fixed byte
//!    order; conversion to/from native is **explicit** via `.get()` / `.set()`.
//! 3. **Arithmetic types** ([`arithmetic`]) — like buffer types but with all
//!    arithmetic operators; conversion to/from native is **implicit**.
//!
//! The `ffi` module exposes all three via a **`cxx` bridge** (type-safe, C++11-compatible).

#![allow(dead_code)]

pub mod arithmetic;
pub mod buffer;
pub mod conversion;
pub mod ffi;

// ── Re-export conversion functions at crate root ──────────────────────────────

pub use conversion::{
    big_to_native, big_to_native_inplace, conditional_reverse, endian_reverse,
    endian_reverse_inplace, little_to_native, little_to_native_inplace, native_to_big,
    native_to_big_inplace, native_to_little, native_to_little_inplace, EndianReversible,
    Order, NATIVE_ORDER,
    // load
    load_big_s16, load_big_s24, load_big_s32, load_big_s64,
    load_big_u16, load_big_u24, load_big_u32, load_big_u64,
    load_little_s16, load_little_s24, load_little_s32, load_little_s64,
    load_little_u16, load_little_u24, load_little_u32, load_little_u64,
    // store
    store_big_s16, store_big_s24, store_big_s32, store_big_s64,
    store_big_u16, store_big_u24, store_big_u32, store_big_u64,
    store_little_s16, store_little_s24, store_little_s32, store_little_s64,
    store_little_u16, store_little_u24, store_little_u32, store_little_u64,
};

// ── Re-export buffer & arithmetic types ──────────────────────────────────────

pub use buffer::{BigEndian, BigEndianBuf, Encoding, EndianBuf, LittleEndian, LittleEndianBuf,
                 NativeEndian, NativeEndianBuf};
pub use arithmetic::{BigEndianInt, EndianInt, LittleEndianInt, NativeEndianInt};

// ── Boost.Endian-style type aliases ──────────────────────────────────────────

// Buffer types (explicit conversion)
pub type BigInt8BufT     = BigEndianBuf<i8>;
pub type BigUInt8BufT    = BigEndianBuf<u8>;
pub type BigInt16BufT    = BigEndianBuf<i16>;
pub type BigUInt16BufT   = BigEndianBuf<u16>;
pub type BigInt32BufT    = BigEndianBuf<i32>;
pub type BigUInt32BufT   = BigEndianBuf<u32>;
pub type BigInt64BufT    = BigEndianBuf<i64>;
pub type BigUInt64BufT   = BigEndianBuf<u64>;
pub type BigFloat32BufT  = BigEndianBuf<f32>;
pub type BigFloat64BufT  = BigEndianBuf<f64>;

pub type LittleInt8BufT     = LittleEndianBuf<i8>;
pub type LittleUInt8BufT    = LittleEndianBuf<u8>;
pub type LittleInt16BufT    = LittleEndianBuf<i16>;
pub type LittleUInt16BufT   = LittleEndianBuf<u16>;
pub type LittleInt32BufT    = LittleEndianBuf<i32>;
pub type LittleUInt32BufT   = LittleEndianBuf<u32>;
pub type LittleInt64BufT    = LittleEndianBuf<i64>;
pub type LittleUInt64BufT   = LittleEndianBuf<u64>;
pub type LittleFloat32BufT  = LittleEndianBuf<f32>;
pub type LittleFloat64BufT  = LittleEndianBuf<f64>;

// Arithmetic types (implicit conversion + full arithmetic ops)
pub type BigInt8T     = BigEndianInt<i8>;
pub type BigUInt8T    = BigEndianInt<u8>;
pub type BigInt16T    = BigEndianInt<i16>;
pub type BigUInt16T   = BigEndianInt<u16>;
pub type BigInt32T    = BigEndianInt<i32>;
pub type BigUInt32T   = BigEndianInt<u32>;
pub type BigInt64T    = BigEndianInt<i64>;
pub type BigUInt64T   = BigEndianInt<u64>;
pub type BigFloat32T  = BigEndianInt<f32>;
pub type BigFloat64T  = BigEndianInt<f64>;

pub type LittleInt8T     = LittleEndianInt<i8>;
pub type LittleUInt8T    = LittleEndianInt<u8>;
pub type LittleInt16T    = LittleEndianInt<i16>;
pub type LittleUInt16T   = LittleEndianInt<u16>;
pub type LittleInt32T    = LittleEndianInt<i32>;
pub type LittleUInt32T   = LittleEndianInt<u32>;
pub type LittleInt64T    = LittleEndianInt<i64>;
pub type LittleUInt64T   = LittleEndianInt<u64>;
pub type LittleFloat32T  = LittleEndianInt<f32>;
pub type LittleFloat64T  = LittleEndianInt<f64>;
