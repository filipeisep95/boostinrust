#ifndef ALIGNED_SIMD_H
#define ALIGNED_SIMD_H

/* C++03: no <cstddef> guaranteed to have size_t outside std::,
   include <stddef.h> for the C version which is always global. */
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Opaque handle — never dereference this in C++ */
typedef struct AlignedBufferF32 AlignedBufferF32;

AlignedBufferF32* aligned_buffer_create(size_t size, float value);
size_t            aligned_buffer_len(const AlignedBufferF32* buf);
const float*      aligned_buffer_as_ptr(const AlignedBufferF32* buf);
float*            aligned_buffer_as_mut_ptr(AlignedBufferF32* buf);
void              aligned_buffer_destroy(AlignedBufferF32* buf);
void              aligned_buffer_double_avx(AlignedBufferF32* buf);

/* Operate on a raw caller-aligned pointer */
void              double_f32_avx_raw(float* ptr, size_t len);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* ALIGNED_SIMD_H */