//! Endian conversion functions (Boost.Endian "Approach 1").
//!
//! The caller uses ordinary integer / float types and explicitly converts byte
//! order when needed. This mirrors `boost/endian/conversion.hpp`.
//!
//! Key functions:
//! - [`endian_reverse`] / [`endian_reverse_inplace`]
//! - [`big_to_native`] / [`native_to_big`]
//! - [`little_to_native`] / [`native_to_little`]
//! - `*_inplace` variants
//! - [`load_big_u32`] etc., [`store_big_u32`] etc.

// ── Order enum ────────────────────────────────────────────────────────────────

/// Byte ordering of a stored integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Big,
    Little,
    /// Resolves to `Big` or `Little` at compile time depending on the target.
    Native,
}

/// The native byte order of this compilation target.
pub const NATIVE_ORDER: Order = {
    #[cfg(target_endian = "big")]
    {
        Order::Big
    }
    #[cfg(target_endian = "little")]
    {
        Order::Little
    }
};

// ── EndianReversible trait ────────────────────────────────────────────────────

/// A type whose bytes can be reversed to change endianness.
///
/// Implemented for all primitive integer types and for `f32` / `f64`
/// (as byte-pattern reversal, not arithmetic negation).
pub trait EndianReversible: Sized + Copy {
    fn endian_reverse(self) -> Self;
}

macro_rules! impl_rev_int {
    ($($t:ty),+) => {
        $(impl EndianReversible for $t {
            #[inline(always)]
            fn endian_reverse(self) -> Self { self.swap_bytes() }
        })+
    };
}
impl_rev_int!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl EndianReversible for f32 {
    /// Reverses the byte pattern of the float (not a numeric operation).
    #[inline(always)]
    fn endian_reverse(self) -> Self {
        Self::from_bits(self.to_bits().swap_bytes())
    }
}

impl EndianReversible for f64 {
    #[inline(always)]
    fn endian_reverse(self) -> Self {
        Self::from_bits(self.to_bits().swap_bytes())
    }
}

// ── Value-returning conversion functions ─────────────────────────────────────

/// Returns `x` with its bytes reversed.
#[inline(always)]
pub fn endian_reverse<T: EndianReversible>(x: T) -> T {
    x.endian_reverse()
}

/// Converts `x` from big-endian to native byte order.
#[inline(always)]
pub fn big_to_native<T: EndianReversible>(x: T) -> T {
    if NATIVE_ORDER == Order::Big {
        x
    } else {
        x.endian_reverse()
    }
}

/// Converts `x` from native to big-endian byte order.
#[inline(always)]
pub fn native_to_big<T: EndianReversible>(x: T) -> T {
    big_to_native(x) // symmetric: both mean "swap iff native != big"
}

/// Converts `x` from little-endian to native byte order.
#[inline(always)]
pub fn little_to_native<T: EndianReversible>(x: T) -> T {
    if NATIVE_ORDER == Order::Little {
        x
    } else {
        x.endian_reverse()
    }
}

/// Converts `x` from native to little-endian byte order.
#[inline(always)]
pub fn native_to_little<T: EndianReversible>(x: T) -> T {
    little_to_native(x)
}

/// Returns `x` unchanged if `from == to`, otherwise returns `endian_reverse(x)`.
#[inline]
pub fn conditional_reverse<T: EndianReversible>(x: T, from: Order, to: Order) -> T {
    let from = resolve(from);
    let to = resolve(to);
    if from == to {
        x
    } else {
        x.endian_reverse()
    }
}

fn resolve(o: Order) -> Order {
    if o == Order::Native { NATIVE_ORDER } else { o }
}

// ── In-place conversion functions ─────────────────────────────────────────────

/// Reverses the bytes of `x` in place.
#[inline(always)]
pub fn endian_reverse_inplace<T: EndianReversible>(x: &mut T) {
    *x = x.endian_reverse();
}

/// Converts `x` from big-endian to native in place.
#[inline(always)]
pub fn big_to_native_inplace<T: EndianReversible>(x: &mut T) {
    *x = big_to_native(*x);
}

/// Converts `x` from native to big-endian in place.
#[inline(always)]
pub fn native_to_big_inplace<T: EndianReversible>(x: &mut T) {
    *x = native_to_big(*x);
}

/// Converts `x` from little-endian to native in place.
#[inline(always)]
pub fn little_to_native_inplace<T: EndianReversible>(x: &mut T) {
    *x = little_to_native(*x);
}

/// Converts `x` from native to little-endian in place.
#[inline(always)]
pub fn native_to_little_inplace<T: EndianReversible>(x: &mut T) {
    *x = native_to_little(*x);
}

// ── Load functions (byte slice → native value) ────────────────────────────────

// 16-bit
#[inline] pub fn load_big_u16   (p: &[u8; 2]) -> u16 { u16::from_be_bytes(*p) }
#[inline] pub fn load_big_s16   (p: &[u8; 2]) -> i16 { i16::from_be_bytes(*p) }
#[inline] pub fn load_little_u16(p: &[u8; 2]) -> u16 { u16::from_le_bytes(*p) }
#[inline] pub fn load_little_s16(p: &[u8; 2]) -> i16 { i16::from_le_bytes(*p) }

// 24-bit (sign-extend into 32-bit)
#[inline]
pub fn load_big_u24(p: &[u8; 3]) -> u32 {
    u32::from_be_bytes([0, p[0], p[1], p[2]])
}
#[inline]
pub fn load_big_s24(p: &[u8; 3]) -> i32 {
    let raw = load_big_u24(p);
    if raw & 0x0080_0000 != 0 { (raw | 0xFF00_0000) as i32 } else { raw as i32 }
}
#[inline]
pub fn load_little_u24(p: &[u8; 3]) -> u32 {
    u32::from_le_bytes([p[0], p[1], p[2], 0])
}
#[inline]
pub fn load_little_s24(p: &[u8; 3]) -> i32 {
    let raw = load_little_u24(p);
    if raw & 0x0080_0000 != 0 { (raw | 0xFF00_0000) as i32 } else { raw as i32 }
}

// 32-bit
#[inline] pub fn load_big_u32   (p: &[u8; 4]) -> u32 { u32::from_be_bytes(*p) }
#[inline] pub fn load_big_s32   (p: &[u8; 4]) -> i32 { i32::from_be_bytes(*p) }
#[inline] pub fn load_little_u32(p: &[u8; 4]) -> u32 { u32::from_le_bytes(*p) }
#[inline] pub fn load_little_s32(p: &[u8; 4]) -> i32 { i32::from_le_bytes(*p) }

// 64-bit
#[inline] pub fn load_big_u64   (p: &[u8; 8]) -> u64 { u64::from_be_bytes(*p) }
#[inline] pub fn load_big_s64   (p: &[u8; 8]) -> i64 { i64::from_be_bytes(*p) }
#[inline] pub fn load_little_u64(p: &[u8; 8]) -> u64 { u64::from_le_bytes(*p) }
#[inline] pub fn load_little_s64(p: &[u8; 8]) -> i64 { i64::from_le_bytes(*p) }

// ── Store functions (native value → byte slice) ───────────────────────────────

// 16-bit
#[inline] pub fn store_big_u16   (p: &mut [u8; 2], v: u16) { *p = v.to_be_bytes(); }
#[inline] pub fn store_big_s16   (p: &mut [u8; 2], v: i16) { *p = v.to_be_bytes(); }
#[inline] pub fn store_little_u16(p: &mut [u8; 2], v: u16) { *p = v.to_le_bytes(); }
#[inline] pub fn store_little_s16(p: &mut [u8; 2], v: i16) { *p = v.to_le_bytes(); }

// 24-bit (write 3 least-significant bytes)
#[inline]
pub fn store_big_u24(p: &mut [u8; 3], v: u32) {
    let b = v.to_be_bytes(); // [MSB … LSB]
    p[0] = b[1]; p[1] = b[2]; p[2] = b[3];
}
#[inline]
pub fn store_big_s24(p: &mut [u8; 3], v: i32)  { store_big_u24(p, v as u32); }
#[inline]
pub fn store_little_u24(p: &mut [u8; 3], v: u32) {
    let b = v.to_le_bytes(); // [LSB … MSB]
    p[0] = b[0]; p[1] = b[1]; p[2] = b[2];
}
#[inline]
pub fn store_little_s24(p: &mut [u8; 3], v: i32) { store_little_u24(p, v as u32); }

// 32-bit
#[inline] pub fn store_big_u32   (p: &mut [u8; 4], v: u32) { *p = v.to_be_bytes(); }
#[inline] pub fn store_big_s32   (p: &mut [u8; 4], v: i32) { *p = v.to_be_bytes(); }
#[inline] pub fn store_little_u32(p: &mut [u8; 4], v: u32) { *p = v.to_le_bytes(); }
#[inline] pub fn store_little_s32(p: &mut [u8; 4], v: i32) { *p = v.to_le_bytes(); }

// 64-bit
#[inline] pub fn store_big_u64   (p: &mut [u8; 8], v: u64) { *p = v.to_be_bytes(); }
#[inline] pub fn store_big_s64   (p: &mut [u8; 8], v: i64) { *p = v.to_be_bytes(); }
#[inline] pub fn store_little_u64(p: &mut [u8; 8], v: u64) { *p = v.to_le_bytes(); }
#[inline] pub fn store_little_s64(p: &mut [u8; 8], v: i64) { *p = v.to_le_bytes(); }

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_u32() {
        assert_eq!(endian_reverse(0x01020304u32), 0x04030201u32);
    }

    #[test]
    fn native_big_roundtrip_u32() {
        let v: u32 = 0xDEAD_BEEF;
        assert_eq!(big_to_native(native_to_big(v)), v);
    }

    #[test]
    fn native_little_roundtrip_u64() {
        let v: u64 = 0x0102_0304_0506_0708;
        assert_eq!(little_to_native(native_to_little(v)), v);
    }

    #[test]
    fn load_store_roundtrip_big_u32() {
        let mut buf = [0u8; 4];
        store_big_u32(&mut buf, 0xCAFE_BABE);
        assert_eq!(load_big_u32(&buf), 0xCAFE_BABE);
    }

    #[test]
    fn load_store_24bit_sign_extend() {
        let mut buf = [0u8; 3];
        store_big_s24(&mut buf, -1);
        assert_eq!(load_big_s24(&buf), -1);

        store_little_s24(&mut buf, -42);
        assert_eq!(load_little_s24(&buf), -42);
    }

    #[test]
    fn f32_reverse_roundtrip() {
        let v = std::f32::consts::PI;
        assert_eq!(endian_reverse(endian_reverse(v)), v);
    }

    #[test]
    fn inplace_matches_value() {
        let mut x: u32 = 0xAABBCCDD;
        let expected = big_to_native(x);
        big_to_native_inplace(&mut x);
        assert_eq!(x, expected);
    }
}
