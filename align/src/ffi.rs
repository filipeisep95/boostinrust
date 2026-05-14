use crate::AlignedBuffer;

#[no_mangle]
pub extern "C" fn aligned_buffer_create(size: usize, value: f32) -> *mut AlignedBuffer {
    let buffer = Box::new(AlignedBuffer::new(size, value));
    Box::into_raw(buffer)
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_len(buffer: *const AlignedBuffer) -> usize {
    assert!(!buffer.is_null());
    (*buffer).len()
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_as_ptr(buffer: *const AlignedBuffer) -> *const f32 {
    assert!(!buffer.is_null());
    (*buffer).as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_as_mut_ptr(buffer: *mut AlignedBuffer) -> *mut f32 {
    assert!(!buffer.is_null());
    (*buffer).as_mut_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_destroy(buffer: *mut AlignedBuffer) {
    if !buffer.is_null() {
        drop(Box::from_raw(buffer));
    }
}

#[no_mangle]
pub unsafe extern "C" fn aligned_buffer_double_avx(buffer: *mut AlignedBuffer) {
    assert!(!buffer.is_null());
    let slice = (*buffer).as_mut_slice();
    assert!(slice.len() % 8 == 0, "AVX requires length multiple of 8");

    // sanity-check alignment at runtime (fires in debug and release)
    assert_eq!(
        slice.as_ptr() as usize % 32,
        0,
        "buffer pointer is not 32-byte aligned: {:p}",
        slice.as_ptr()
    );

    #[cfg(target_arch = "x86_64")]
    crate::simd::double_f32_avx(slice.as_mut_ptr(), slice.len());
}

#[no_mangle]
pub unsafe extern "C" fn double_f32_avx_raw(ptr: *mut f32, len: usize) {
    #[cfg(target_arch = "x86_64")]
    crate::simd::double_f32_avx(ptr, len);
}
