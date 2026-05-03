#![allow(dead_code)] // crate level:  silencing warnings about functions, structs, fields, or other items that are defined but never used.

use std::ops::{Deref, DerefMut};
#[repr(align(32))] // default wrapper enforces alignment at type level
pub struct Aligned<T>(pub T);

impl<T> Deref for Aligned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Aligned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[macro_export]
macro_rules! aligned_type {
    ($name:struct_name, $inner_type:type_input, $align:alignment) => {
        #[repr(align($align))]
        pub struct $name(pub $inner_type);

        impl std::ops::Deref for $name {
            type Target = $inner_type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub struct AlignedBuffer<T> {
    data: Vec<T>,
}

impl<T: Clone> AlignedBuffer<T> {
    /// Create a new buffer with a given size and default value
    pub fn new(size: usize, value: T) -> Self {
        Self {
            data: vec![value; size],
        }
    }
}

impl<T> AlignedBuffer<T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
}

/// SIMD utilities (x86_64 AVX example)
#[cfg(target_arch = "x86_64")]
pub mod simd {
    use std::arch::x86_64::*;

    /// Apply f(x) = x + x using AVX on aligned data
    pub unsafe fn double_f32_avx(ptr: *mut f32, len: usize) {
        for i in (0..len).step_by(8) {
            let v = _mm256_load_ps(ptr.add(i));
            let v = _mm256_add_ps(v, v);
            _mm256_store_ps(ptr.add(i), v);
        }
    }
}

