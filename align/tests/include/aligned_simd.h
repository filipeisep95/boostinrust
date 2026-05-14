/* tests/include/aligned_simd.h */
#ifdef __cplusplus
extern "C" {
#endif

typedef void AlignedBufferF32;

AlignedBufferF32* aligned_buffer_create(size_t size, float value);
size_t            aligned_buffer_len(const AlignedBufferF32* buf);
const float*      aligned_buffer_as_ptr(const AlignedBufferF32* buf);
float*            aligned_buffer_as_mut_ptr(AlignedBufferF32* buf);
void              aligned_buffer_destroy(AlignedBufferF32* buf);
void              aligned_buffer_double_avx(AlignedBufferF32* buf);
void              double_f32_avx_raw(float* ptr, size_t len);

#ifdef __cplusplus
}
#endif