//! Endian arithmetic types (Boost.Endian "Approach 3").
//!
//! [`EndianInt<E, T>`] behaves like [`crate::buffer::EndianBuf`] in layout but
//! additionally provides the full set of arithmetic and bitwise operators.
//! Conversion to/from native endian is **implicit** via `From<T>` / `Into<T>`.
//!
//! This mirrors `boost/endian/arithmetic.hpp`.
//!
//! # Example
//! ```
//! use endian::arithmetic::BigEndianInt;
//!
//! let mut x: BigEndianInt<i32> = 100.into();
//! x += 42;
//! assert_eq!(x.get(), 142);
//! let y: i32 = x.into();
//! assert_eq!(y, 142);
//! ```

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::*;

use crate::buffer::{BigEndian, Encoding, LittleEndian, NativeEndian};
use crate::conversion::EndianReversible;

// ── EndianInt ─────────────────────────────────────────────────────────────────

/// An integer stored in the byte order specified by `E` with full arithmetic.
///
/// # Layout
/// `#[repr(transparent)]` over `T` — identical ABI to the raw integer type.
///
/// # Conversions
/// - `EndianInt::from(native: T)` — store a native value (implicit via `From`).
/// - `.get() -> T` — retrieve as native (explicit).
/// - `Into::<T>::into(x)` — retrieve as native (implicit).
#[repr(transparent)]
pub struct EndianInt<E: Encoding, T: EndianReversible> {
    stored: T,
    _enc: PhantomData<E>,
}

impl<E: Encoding, T: EndianReversible> EndianInt<E, T> {
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

    /// Raw stored bytes (target byte order, suitable for binary I/O).
    #[inline]
    pub fn data(&self) -> &T {
        &self.stored
    }
}

// ── Implicit conversions ──────────────────────────────────────────────────────

impl<E: Encoding, T: EndianReversible> From<T> for EndianInt<E, T> {
    #[inline]
    fn from(native: T) -> Self {
        Self::new(native)
    }
}

// Implement Into<T> for each concrete primitive type.
macro_rules! impl_into_prim {
    ($($t:ty),+) => {
        $(impl<E: Encoding> From<EndianInt<E, $t>> for $t {
            #[inline] fn from(v: EndianInt<E, $t>) -> $t { v.get() }
        })+
    };
}
impl_into_prim!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

// ── Standard derives ──────────────────────────────────────────────────────────

impl<E: Encoding, T: EndianReversible + Clone> Clone for EndianInt<E, T> {
    fn clone(&self) -> Self {
        Self {
            stored: self.stored.clone(),
            _enc: PhantomData,
        }
    }
}
impl<E: Encoding, T: EndianReversible + Copy> Copy for EndianInt<E, T> {}

impl<E: Encoding, T: EndianReversible + Default> Default for EndianInt<E, T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<E: Encoding, T: EndianReversible + fmt::Debug> fmt::Debug for EndianInt<E, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EndianInt({:?})", self.get())
    }
}

impl<E: Encoding, T: EndianReversible + fmt::Display> fmt::Display for EndianInt<E, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl<E: Encoding, T: EndianReversible + PartialEq> PartialEq for EndianInt<E, T> {
    fn eq(&self, other: &Self) -> bool { self.stored == other.stored }
}
impl<E: Encoding, T: EndianReversible + Eq> Eq for EndianInt<E, T> {}

impl<E: Encoding, T: EndianReversible + PartialOrd> PartialOrd for EndianInt<E, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}
impl<E: Encoding, T: EndianReversible + Ord> Ord for EndianInt<E, T> {
    fn cmp(&self, other: &Self) -> Ordering { self.get().cmp(&other.get()) }
}

impl<E: Encoding, T: EndianReversible + Hash> Hash for EndianInt<E, T> {
    fn hash<H: Hasher>(&self, state: &mut H) { self.stored.hash(state) }
}

// ── Binary arithmetic operators (EndianInt op EndianInt) ──────────────────────

macro_rules! impl_binop {
    ($trait:ident, $method:ident) => {
        // EndianInt op EndianInt → EndianInt
        impl<E: Encoding, T: EndianReversible + $trait<Output = T>> $trait for EndianInt<E, T> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: Self) -> Self {
                Self::new(self.get().$method(rhs.get()))
            }
        }
        // EndianInt op T → EndianInt  (convenient scalar form)
        impl<E: Encoding, T: EndianReversible + $trait<Output = T>> $trait<T> for EndianInt<E, T> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: T) -> Self {
                Self::new(self.get().$method(rhs))
            }
        }
    };
}

impl_binop!(Add, add);
impl_binop!(Sub, sub);
impl_binop!(Mul, mul);
impl_binop!(Div, div);
impl_binop!(Rem, rem);
impl_binop!(BitAnd, bitand);
impl_binop!(BitOr, bitor);
impl_binop!(BitXor, bitxor);
impl_binop!(Shl, shl);
impl_binop!(Shr, shr);

// ── Assignment operators ──────────────────────────────────────────────────────

macro_rules! impl_assign {
    ($trait:ident, $method:ident, $base:ident, $base_method:ident) => {
        impl<E: Encoding, T: EndianReversible + $base<Output = T>> $trait for EndianInt<E, T> {
            #[inline]
            fn $method(&mut self, rhs: Self) {
                self.set(self.get().$base_method(rhs.get()));
            }
        }
        impl<E: Encoding, T: EndianReversible + $base<Output = T>> $trait<T> for EndianInt<E, T> {
            #[inline]
            fn $method(&mut self, rhs: T) {
                self.set(self.get().$base_method(rhs));
            }
        }
    };
}

impl_assign!(AddAssign, add_assign, Add, add);
impl_assign!(SubAssign, sub_assign, Sub, sub);
impl_assign!(MulAssign, mul_assign, Mul, mul);
impl_assign!(DivAssign, div_assign, Div, div);
impl_assign!(RemAssign, rem_assign, Rem, rem);
impl_assign!(BitAndAssign, bitand_assign, BitAnd, bitand);
impl_assign!(BitOrAssign, bitor_assign, BitOr, bitor);
impl_assign!(BitXorAssign, bitxor_assign, BitXor, bitxor);
impl_assign!(ShlAssign, shl_assign, Shl, shl);
impl_assign!(ShrAssign, shr_assign, Shr, shr);

// ── Unary operators ───────────────────────────────────────────────────────────

impl<E: Encoding, T: EndianReversible + Neg<Output = T>> Neg for EndianInt<E, T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self { Self::new(-self.get()) }
}

impl<E: Encoding, T: EndianReversible + Not<Output = T>> Not for EndianInt<E, T> {
    type Output = Self;
    #[inline]
    fn not(self) -> Self { Self::new(!self.get()) }
}

// ── Prefix / postfix increment / decrement ────────────────────────────────────

// Rust has no ++ / -- but we can implement them via the increment traits pattern.
// Provide a `.inc()` / `.dec()` convenience for callers who would write `++x`.
impl<E: Encoding, T: EndianReversible + Add<Output = T> + From<u8>> EndianInt<E, T> {
    /// Increment by one (equivalent to `++x` in C++).
    #[inline]
    pub fn inc(&mut self) {
        self.set(self.get() + T::from(1u8));
    }

    /// Decrement by one (equivalent to `--x` in C++).
    #[inline]
    pub fn dec(&mut self) where T: Sub<Output = T> {
        self.set(self.get() - T::from(1u8));
    }
}

// ── Type aliases ──────────────────────────────────────────────────────────────

/// Implicit-conversion big-endian arithmetic type.
pub type BigEndianInt<T>    = EndianInt<BigEndian,    T>;
/// Implicit-conversion little-endian arithmetic type.
pub type LittleEndianInt<T> = EndianInt<LittleEndian, T>;
/// Implicit-conversion native-endian arithmetic type.
pub type NativeEndianInt<T> = EndianInt<NativeEndian,  T>;

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    type BEi32 = BigEndianInt<i32>;
    type LEu32 = LittleEndianInt<u32>;

    #[test]
    fn from_and_into() {
        let x: BEi32 = 42.into();
        let v: i32 = x.into();
        assert_eq!(v, 42);
    }

    #[test]
    fn arithmetic_ops() {
        let mut x: BEi32 = 10.into();
        x += 5;
        assert_eq!(x.get(), 15);
        x *= 3;
        assert_eq!(x.get(), 45);
        x -= 5;
        assert_eq!(x.get(), 40);
    }

    #[test]
    fn bitwise_ops() {
        let a: LEu32 = 0xFF00u32.into();
        let b: LEu32 = 0x00FFu32.into();
        assert_eq!((a | b).get(), 0xFFFF);
        assert_eq!((a & b).get(), 0x0000);
        assert_eq!((a ^ b).get(), 0xFFFF);
    }

    #[test]
    fn neg_and_not() {
        let x: BEi32 = 7.into();
        assert_eq!((-x).get(), -7);
        let mask: BigEndianInt<u32> = 0u32.into();
        assert_eq!((!mask).get(), u32::MAX);
    }

    #[test]
    fn inc_dec() {
        let mut x: BEi32 = 0.into();
        x.inc();
        x.inc();
        assert_eq!(x.get(), 2);
        x.dec();
        assert_eq!(x.get(), 1);
    }

    #[test]
    fn ordering() {
        let a: BEi32 = 1.into();
        let b: BEi32 = 2.into();
        assert!(a < b);
        assert_eq!(a.min(b).get(), 1);
    }

    #[test]
    fn size_matches_inner() {
        use std::mem::size_of;
        assert_eq!(size_of::<BEi32>(), size_of::<i32>());
        assert_eq!(size_of::<LittleEndianInt<u64>>(), size_of::<u64>());
    }

    #[test]
    fn loop_accumulation() {
        // Mirror the Boost.Endian "Example 1": add 1..=100 to a big-endian i32.
        let mut x: BEi32 = 0.into();
        for i in 1..=100_i32 {
            x += i;
        }
        assert_eq!(x.get(), 5050);
    }
}
