#![allow(dead_code)]

pub mod ffi;

use std::ops::{Deref, DerefMut};

// ─── Aligned<T> wrapper ──────────────────────────────────────────────────────

#[repr(align(32))]
pub struct Aligned<T>(pub T);

impl<T> Deref for Aligned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for Aligned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// ─── aligned_type! macro ─────────────────────────────────────────────────────

#[macro_export]
macro_rules! aligned_type {
    ($name:ident, $inner_type:ty, $align:expr) => {
        #[repr(align($align))]
        pub struct $name(pub $inner_type);

        impl std::ops::Deref for $name {
            type Target = $inner_type;
            fn deref(&self) -> &Self::Target { &self.0 }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    };
}

// ─── AlignedBuffer ───────────────────────────────────────────────────────────

/// One AVX register's worth of f32 values, 32-byte aligned.
/// Vec<F32x8> guarantees the backing allocation is 32-byte aligned
/// because Vec uses align_of::<T>() when calling the allocator.
#[repr(align(32))]
struct F32x8([f32; 8]);

pub struct AlignedBuffer {
    data: Vec<F32x8>,
    len: usize, // logical length in f32 elements
}

impl AlignedBuffer {
    /// Create a new buffer of `size` f32 elements, all initialised to `value`.
    /// `size` must be a multiple of 8 (one AVX register = 8 × f32).
    pub fn new(size: usize, value: f32) -> Self {
        assert!(size % 8 == 0, "AlignedBuffer size must be a multiple of 8");
        let chunks = size / 8;
        Self {
            data: (0..chunks).map(|_| F32x8([value; 8])).collect(),
            len: size,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }

    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.data.as_mut_ptr() as *mut f32
    }

    pub fn as_slice(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}

// ─── SIMD (x86_64 AVX) ───────────────────────────────────────────────────────

#[cfg(target_arch = "x86_64")]
pub mod simd {
    use std::arch::x86_64::*;

    /// Apply f(x) = x + x using AVX on a 32-byte-aligned pointer.
    /// Caller must guarantee:
    ///   - `ptr` is 32-byte aligned
    ///   - `len` is a multiple of 8
    ///   - the memory range [ptr, ptr+len) is valid and exclusively accessible
    #[target_feature(enable = "avx")]
    pub unsafe fn double_f32_avx(ptr: *mut f32, len: usize) {
        for i in (0..len).step_by(8) {
            let v = _mm256_load_ps(ptr.add(i));   // requires 32-byte alignment
            let result = _mm256_add_ps(v, v);
            _mm256_store_ps(ptr.add(i), result);
        }
    }
}
