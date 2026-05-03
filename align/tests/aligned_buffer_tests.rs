use boost_align::*; // use your crate name here

aligned_type!(AlignedF32x32, f32, 32);

#[test]
fn test_alignment() {
    let buffer = AlignedBuffer::new(16, AlignedF32x32(1.0));
    let buffer_pointer = buffer.as_ptr() as usize;

    assert_eq!(buffer_pointer % 32, 0);
}

#[test]
fn test_simd() {
    let mut buffer = AlignedBuffer::new(16, AlignedF32x32(1.0));

    unsafe {
        let buffer_pointer = buffer.as_mut_ptr() as *mut f32;
        aligned_buffer::simd::double_f32_avx(buffer_pointer, buffer.len());
    }

    for v in buffer.as_slice() {
        assert_eq!(**v, 2.0);
    }
}