// #[cxx::bridge]
// mod ffi {
//     // Opaque Rust type — C++ holds it via Box<AlignedBufferF32>
//     extern "Rust" {
//         type AlignedBufferF32;

//         // Constructor: creates a buffer of `size` f32s initialized to `value`
//         fn aligned_buffer_new(size: usize, value: f32) -> Box<AlignedBufferF32>;

//         fn len(self: &AlignedBufferF32) -> usize;

//         // Returns a raw pointer for C++ to read
//         fn as_ptr(self: &AlignedBufferF32) -> *const f32;

//         // Apply AVX double-in-place (unsafe internally, safe at bridge boundary)
//         fn double_f32_avx_safe(buf: &mut AlignedBufferF32);
//     }
// }

// // Newtype so cxx can name it
// pub struct AlignedBufferF32(crate::AlignedBuffer<f32>);

// pub fn aligned_buffer_new(size: usize, value: f32) -> Box<AlignedBufferF32> {
//     Box::new(AlignedBufferF32(crate::AlignedBuffer::new(size, value)))
// }

// impl AlignedBufferF32 {
//     pub fn len(&self) -> usize {
//         self.0.len()
//     }

//     pub fn as_ptr(&self) -> *const f32 {
//         self.0.as_ptr()
//     }
// }

// pub fn double_f32_avx_safe(buf: &mut AlignedBufferF32) {
//     let slice = buf.0.as_mut_slice();
//     assert!(slice.len() % 8 == 0, "Length must be multiple of 8 for AVX");
//     #[cfg(target_arch = "x86_64")]
//     unsafe {
//         crate::simd::double_f32_avx(slice.as_mut_ptr(), slice.len());
//     }
// }

use crate::AlignedBuffer;

/// Opaque handle — C++03 gets a raw void pointer
#[no_mangle]
pub extern "C" fn aligned_buffer_create(size: usize, value: f32) -> *mut AlignedBuffer<f32> {
    let buf = Box::new(AlignedBuffer::new(size, value));
    Box::into_raw(buf)
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_len(buf: *const AlignedBuffer<f32>) -> usize {
    assert!(!buf.is_null());
    (*buf).len()
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_as_ptr(buf: *const AlignedBuffer<f32>) -> *const f32 {
    assert!(!buf.is_null());
    (*buf).as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_as_mut_ptr(buf: *mut AlignedBuffer<f32>) -> *mut f32 {
    assert!(!buf.is_null());
    (*buf).as_mut_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_destroy(buf: *mut AlignedBuffer<f32>) {
    if !buf.is_null() {
        drop(Box::from_raw(buf));
    }
}

/// Safe wrapper: asserts len % 8 == 0 before calling AVX intrinsics
#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_double_avx(buf: *mut AlignedBuffer<f32>) {
    assert!(!buf.is_null());
    let slice = (*buf).as_mut_slice();
    assert!(slice.len() % 8 == 0, "AVX requires length multiple of 8");
    #[cfg(target_arch = "x86_64")]
    crate::simd::double_f32_avx(slice.as_mut_ptr(), slice.len());
}

/// Standalone: operate on a caller-provided aligned pointer (C++03 friendly)
#[no_mangle]
pub unsafe extern "C" fn double_f32_avx_raw(ptr: *mut f32, len: usize) {
    #[cfg(target_arch = "x86_64")]
    crate::simd::double_f32_avx(ptr, len);
}