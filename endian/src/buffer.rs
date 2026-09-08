//! Endian buffer types (Boost.Endian "Approach 2").
//!
//! [`EndianBuf<E, T>`] stores a value of type `T` in the byte order determined
//! by the encoding marker `E`. Conversion to/from native endian is always
//! **explicit** via `.get()` / `.set()`.
//!
//! This mirrors `boost/endian/buffers.hpp`.
//!
//! # Example
//! ```
//! use endian::buffer::{BigEndianBuf, LittleEndianBuf};
//!
//! let mut buf: BigEndianBuf<u32> = BigEndianBuf::new(0x0102_0304);
//! assert_eq!(buf.get(), 0x0102_0304);
//! buf.set(0xDEAD_BEEF);
//! assert_eq!(buf.get(), 0xDEAD_BEEF);
//! ```

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::conversion::{EndianReversible, Order, NATIVE_ORDER};

// ── Encoding markers ──────────────────────────────────────────────────────────

/// Marker type: store in big-endian byte order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BigEndian;

/// Marker type: store in little-endian byte order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LittleEndian;

/// Marker type: store in the platform's native byte order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeEndian;

// ── Encoding trait ────────────────────────────────────────────────────────────

/// Defines how a value is converted to/from its stored representation.
pub trait Encoding: Copy {
    /// Convert a native-endian value to the stored byte order.
    fn to_stored<T: EndianReversible>(native: T) -> T;
    /// Convert a stored value back to native byte order.
    fn to_native<T: EndianReversible>(stored: T) -> T;
}

impl Encoding for BigEndian {
    #[inline(always)]
    fn to_stored<T: EndianReversible>(v: T) -> T {
        if NATIVE_ORDER == Order::Big { v } else { v.endian_reverse() }
    }
    #[inline(always)]
    fn to_native<T: EndianReversible>(v: T) -> T {
        Self::to_stored(v) // symmetric: same operation both ways
    }
}

impl Encoding for LittleEndian {
    #[inline(always)]
    fn to_stored<T: EndianReversible>(v: T) -> T {
        if NATIVE_ORDER == Order::Little { v } else { v.endian_reverse() }
    }
    #[inline(always)]
    fn to_native<T: EndianReversible>(v: T) -> T {
        Self::to_stored(v)
    }
}

impl Encoding for NativeEndian {
    #[inline(always)]
    fn to_stored<T: EndianReversible>(v: T) -> T { v }
    #[inline(always)]
    fn to_native<T: EndianReversible>(v: T) -> T { v }
}

// ── EndianBuf ─────────────────────────────────────────────────────────────────

/// A byte buffer holding `T` in the byte order dictated by `E`.
///
/// Internally the value is held already converted to the target byte order, so
/// the in-memory byte sequence is directly suitable for binary I/O.
///
/// # Layout
/// `#[repr(transparent)]` over `T`. The size, alignment, and ABI of
/// `EndianBuf<E, T>` are identical to `T`.
#[repr(transparent)]
pub struct EndianBuf<E: Encoding, T: EndianReversible> {
    stored: T,
    _enc: PhantomData<E>,
}

impl<E: Encoding, T: EndianReversible> EndianBuf<E, T> {
    /// Construct from a **native-endian** value.
    #[inline]
    pub fn new(native: T) -> Self {
        Self {
            stored: E::to_stored(native),
            _enc: PhantomData,
        }
    }

    /// Return the value in **native** byte order.
    #[inline]
    pub fn get(self) -> T {
        E::to_native(self.stored)
    }

    /// Overwrite with a **native-endian** value.
    #[inline]
    pub fn set(&mut self, native: T) {
        self.stored = E::to_stored(native);
    }

    /// Return a reference to the raw stored bytes in target byte order.
    ///
    /// The byte slice is suitable for binary I/O without further conversion.
    #[inline]
    pub fn data(&self) -> &T {
        &self.stored
    }

    /// Return a mutable reference to the raw stored bytes.
    ///
    /// **Safety:** writing arbitrary bytes may produce a value that does not
    /// correspond to any well-defined integer; caller is responsible.
    #[inline]
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.stored
    }
}

// ── Standard trait impls ──────────────────────────────────────────────────────

impl<E: Encoding, T: EndianReversible + Clone> Clone for EndianBuf<E, T> {
    fn clone(&self) -> Self {
        Self {
            stored: self.stored.clone(),
            _enc: PhantomData,
        }
    }
}

impl<E: Encoding, T: EndianReversible + Copy> Copy for EndianBuf<E, T> {}

impl<E: Encoding, T: EndianReversible + Default> Default for EndianBuf<E, T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<E: Encoding, T: EndianReversible + fmt::Debug> fmt::Debug for EndianBuf<E, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EndianBuf({:?})", self.get())
    }
}

impl<E: Encoding, T: EndianReversible + fmt::Display> fmt::Display for EndianBuf<E, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Equality is based on the stored bytes (same endian encoding must be compared).
impl<E: Encoding, T: EndianReversible + PartialEq> PartialEq for EndianBuf<E, T> {
    fn eq(&self, other: &Self) -> bool {
        self.stored == other.stored
    }
}
impl<E: Encoding, T: EndianReversible + Eq> Eq for EndianBuf<E, T> {}

/// Ordering is performed on the native values (semantically correct).
impl<E: Encoding, T: EndianReversible + PartialOrd> PartialOrd for EndianBuf<E, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}
impl<E: Encoding, T: EndianReversible + Ord> Ord for EndianBuf<E, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

/// Hashing is on the stored representation so that hash(a) == hash(b) iff a == b.
impl<E: Encoding, T: EndianReversible + Hash> Hash for EndianBuf<E, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.stored.hash(state)
    }
}

// ── Type aliases ──────────────────────────────────────────────────────────────

/// Value stored in big-endian byte order; explicit `.get()` / `.set()`.
pub type BigEndianBuf<T>    = EndianBuf<BigEndian,    T>;
/// Value stored in little-endian byte order; explicit `.get()` / `.set()`.
pub type LittleEndianBuf<T> = EndianBuf<LittleEndian, T>;
/// Value stored in native byte order; explicit `.get()` / `.set()`.
pub type NativeEndianBuf<T> = EndianBuf<NativeEndian, T>;

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_buf_roundtrip() {
        let buf: BigEndianBuf<u32> = BigEndianBuf::new(0xDEAD_BEEF);
        assert_eq!(buf.get(), 0xDEAD_BEEF);
    }

    #[test]
    fn little_buf_roundtrip() {
        let buf: LittleEndianBuf<i64> = LittleEndianBuf::new(-1_000_000);
        assert_eq!(buf.get(), -1_000_000);
    }

    #[test]
    fn big_buf_bytes_on_little_endian() {
        // On a little-endian host, 0x01020304u32 stored as big-endian
        // should occupy bytes [0x01, 0x02, 0x03, 0x04].
        #[cfg(target_endian = "little")]
        {
            let buf: BigEndianBuf<u32> = BigEndianBuf::new(0x01020304u32);
            let bytes: [u8; 4] = buf.get().to_be_bytes();
            assert_eq!(bytes, [0x01, 0x02, 0x03, 0x04]);
        }
    }

    #[test]
    fn buf_eq_and_ord() {
        let a: BigEndianBuf<i32> = BigEndianBuf::new(10);
        let b: BigEndianBuf<i32> = BigEndianBuf::new(20);
        assert!(a < b);
        assert_eq!(a, BigEndianBuf::new(10));
    }

    #[test]
    fn buf_set() {
        let mut buf: LittleEndianBuf<u16> = LittleEndianBuf::new(1);
        buf.set(0xABCD);
        assert_eq!(buf.get(), 0xABCD);
    }

    #[test]
    fn buf_size_matches_inner() {
        use std::mem::size_of;
        assert_eq!(size_of::<BigEndianBuf<u32>>(), size_of::<u32>());
        assert_eq!(size_of::<LittleEndianBuf<i64>>(), size_of::<i64>());
    }
}
