// boost_endian.h — C / C++ header for the Rust Boost.Endian reimplementation.
//
// Link against `libendian.a` (produced by `cargo build --release`).
//
// Three layers:
//   1. extern "C" declarations matching `src/ffi.rs`.
//   2. (C++ only) Inline helper overloads for type dispatch.
//   3. (C++ only) Template classes `BigEndianBuf<T>`, `LittleEndianBuf<T>`,
//      `BigEndianInt<T>`, `LittleEndianInt<T>` mirroring Boost.Endian's API.

#pragma once

#include <cstdint>
#include <cstring>   // memcpy

#ifdef __cplusplus
extern "C" {
#endif

// ── Endian reversal ───────────────────────────────────────────────────────────

uint8_t  endian_reverse_u8 (uint8_t  v);
int8_t   endian_reverse_i8 (int8_t   v);
uint16_t endian_reverse_u16(uint16_t v);
int16_t  endian_reverse_i16(int16_t  v);
uint32_t endian_reverse_u32(uint32_t v);
int32_t  endian_reverse_i32(int32_t  v);
uint64_t endian_reverse_u64(uint64_t v);
int64_t  endian_reverse_i64(int64_t  v);

// ── big ↔ native ──────────────────────────────────────────────────────────────

uint16_t big_to_native_u16(uint16_t v);
int16_t  big_to_native_i16(int16_t  v);
uint32_t big_to_native_u32(uint32_t v);
int32_t  big_to_native_i32(int32_t  v);
uint64_t big_to_native_u64(uint64_t v);
int64_t  big_to_native_i64(int64_t  v);

uint16_t native_to_big_u16(uint16_t v);
int16_t  native_to_big_i16(int16_t  v);
uint32_t native_to_big_u32(uint32_t v);
int32_t  native_to_big_i32(int32_t  v);
uint64_t native_to_big_u64(uint64_t v);
int64_t  native_to_big_i64(int64_t  v);

// ── little ↔ native ───────────────────────────────────────────────────────────

uint16_t little_to_native_u16(uint16_t v);
int16_t  little_to_native_i16(int16_t  v);
uint32_t little_to_native_u32(uint32_t v);
int32_t  little_to_native_i32(int32_t  v);
uint64_t little_to_native_u64(uint64_t v);
int64_t  little_to_native_i64(int64_t  v);

uint16_t native_to_little_u16(uint16_t v);
int16_t  native_to_little_i16(int16_t  v);
uint32_t native_to_little_u32(uint32_t v);
int32_t  native_to_little_i32(int32_t  v);
uint64_t native_to_little_u64(uint64_t v);
int64_t  native_to_little_i64(int64_t  v);

// ── Load: raw bytes → native value ───────────────────────────────────────────

uint16_t load_big_u16   (const uint8_t* p);
int16_t  load_big_s16   (const uint8_t* p);
uint16_t load_little_u16(const uint8_t* p);
int16_t  load_little_s16(const uint8_t* p);

uint32_t load_big_u24   (const uint8_t* p);
int32_t  load_big_s24   (const uint8_t* p);
uint32_t load_little_u24(const uint8_t* p);
int32_t  load_little_s24(const uint8_t* p);

uint32_t load_big_u32   (const uint8_t* p);
int32_t  load_big_s32   (const uint8_t* p);
uint32_t load_little_u32(const uint8_t* p);
int32_t  load_little_s32(const uint8_t* p);

uint64_t load_big_u64   (const uint8_t* p);
int64_t  load_big_s64   (const uint8_t* p);
uint64_t load_little_u64(const uint8_t* p);
int64_t  load_little_s64(const uint8_t* p);

// ── Store: native value → raw bytes ──────────────────────────────────────────

void store_big_u16   (uint8_t* p, uint16_t v);
void store_big_s16   (uint8_t* p, int16_t  v);
void store_little_u16(uint8_t* p, uint16_t v);
void store_little_s16(uint8_t* p, int16_t  v);

void store_big_u24   (uint8_t* p, uint32_t v);
void store_big_s24   (uint8_t* p, int32_t  v);
void store_little_u24(uint8_t* p, uint32_t v);
void store_little_s24(uint8_t* p, int32_t  v);

void store_big_u32   (uint8_t* p, uint32_t v);
void store_big_s32   (uint8_t* p, int32_t  v);
void store_little_u32(uint8_t* p, uint32_t v);
void store_little_s32(uint8_t* p, int32_t  v);

void store_big_u64   (uint8_t* p, uint64_t v);
void store_big_s64   (uint8_t* p, int64_t  v);
void store_little_u64(uint8_t* p, uint64_t v);
void store_little_s64(uint8_t* p, int64_t  v);

// ── Opaque buffer structs (layout matches Rust ffi.rs) ───────────────────────

typedef struct { int8_t   raw; } BigInt8Buf;
typedef struct { int16_t  raw; } BigInt16Buf;
typedef struct { int32_t  raw; } BigInt32Buf;
typedef struct { int64_t  raw; } BigInt64Buf;
typedef struct { uint8_t  raw; } BigUInt8Buf;
typedef struct { uint16_t raw; } BigUInt16Buf;
typedef struct { uint32_t raw; } BigUInt32Buf;
typedef struct { uint64_t raw; } BigUInt64Buf;

typedef struct { int8_t   raw; } LittleInt8Buf;
typedef struct { int16_t  raw; } LittleInt16Buf;
typedef struct { int32_t  raw; } LittleInt32Buf;
typedef struct { int64_t  raw; } LittleInt64Buf;
typedef struct { uint8_t  raw; } LittleUInt8Buf;
typedef struct { uint16_t raw; } LittleUInt16Buf;
typedef struct { uint32_t raw; } LittleUInt32Buf;
typedef struct { uint64_t raw; } LittleUInt64Buf;

typedef struct { uint8_t raw[3]; } BigInt24Buf;
typedef struct { uint8_t raw[3]; } BigUInt24Buf;
typedef struct { uint8_t raw[3]; } LittleInt24Buf;
typedef struct { uint8_t raw[3]; } LittleUInt24Buf;

// ── Buffer constructors / accessors ───────────────────────────────────────────

BigInt8Buf    big_int8_buf_new  (int8_t   v);  int8_t   big_int8_buf_get  (BigInt8Buf   b);
BigInt16Buf   big_int16_buf_new (int16_t  v);  int16_t  big_int16_buf_get (BigInt16Buf  b);
BigInt32Buf   big_int32_buf_new (int32_t  v);  int32_t  big_int32_buf_get (BigInt32Buf  b);
BigInt64Buf   big_int64_buf_new (int64_t  v);  int64_t  big_int64_buf_get (BigInt64Buf  b);
BigUInt8Buf   big_uint8_buf_new (uint8_t  v);  uint8_t  big_uint8_buf_get (BigUInt8Buf  b);
BigUInt16Buf  big_uint16_buf_new(uint16_t v);  uint16_t big_uint16_buf_get(BigUInt16Buf b);
BigUInt32Buf  big_uint32_buf_new(uint32_t v);  uint32_t big_uint32_buf_get(BigUInt32Buf b);
BigUInt64Buf  big_uint64_buf_new(uint64_t v);  uint64_t big_uint64_buf_get(BigUInt64Buf b);

LittleInt8Buf    little_int8_buf_new  (int8_t   v);  int8_t   little_int8_buf_get  (LittleInt8Buf   b);
LittleInt16Buf   little_int16_buf_new (int16_t  v);  int16_t  little_int16_buf_get (LittleInt16Buf  b);
LittleInt32Buf   little_int32_buf_new (int32_t  v);  int32_t  little_int32_buf_get (LittleInt32Buf  b);
LittleInt64Buf   little_int64_buf_new (int64_t  v);  int64_t  little_int64_buf_get (LittleInt64Buf  b);
LittleUInt8Buf   little_uint8_buf_new (uint8_t  v);  uint8_t  little_uint8_buf_get (LittleUInt8Buf  b);
LittleUInt16Buf  little_uint16_buf_new(uint16_t v);  uint16_t little_uint16_buf_get(LittleUInt16Buf b);
LittleUInt32Buf  little_uint32_buf_new(uint32_t v);  uint32_t little_uint32_buf_get(LittleUInt32Buf b);
LittleUInt64Buf  little_uint64_buf_new(uint64_t v);  uint64_t little_uint64_buf_get(LittleUInt64Buf b);

BigInt24Buf    big_int24_buf_new   (int32_t  v);  int32_t  big_int24_buf_get   (BigInt24Buf    b);
BigUInt24Buf   big_uint24_buf_new  (uint32_t v);  uint32_t big_uint24_buf_get  (BigUInt24Buf   b);
LittleInt24Buf  little_int24_buf_new (int32_t  v);  int32_t  little_int24_buf_get (LittleInt24Buf  b);
LittleUInt24Buf little_uint24_buf_new(uint32_t v);  uint32_t little_uint24_buf_get(LittleUInt24Buf b);

void big_int8_buf_set   (BigInt8Buf*    b, int8_t   v);
void big_int16_buf_set  (BigInt16Buf*   b, int16_t  v);
void big_int32_buf_set  (BigInt32Buf*   b, int32_t  v);
void big_int64_buf_set  (BigInt64Buf*   b, int64_t  v);
void big_uint8_buf_set  (BigUInt8Buf*   b, uint8_t  v);
void big_uint16_buf_set (BigUInt16Buf*  b, uint16_t v);
void big_uint32_buf_set (BigUInt32Buf*  b, uint32_t v);
void big_uint64_buf_set (BigUInt64Buf*  b, uint64_t v);
void little_int8_buf_set   (LittleInt8Buf*   b, int8_t   v);
void little_int16_buf_set  (LittleInt16Buf*  b, int16_t  v);
void little_int32_buf_set  (LittleInt32Buf*  b, int32_t  v);
void little_int64_buf_set  (LittleInt64Buf*  b, int64_t  v);
void little_uint8_buf_set  (LittleUInt8Buf*  b, uint8_t  v);
void little_uint16_buf_set (LittleUInt16Buf* b, uint16_t v);
void little_uint32_buf_set (LittleUInt32Buf* b, uint32_t v);
void little_uint64_buf_set (LittleUInt64Buf* b, uint64_t v);

#ifdef __cplusplus
} // extern "C"

// ─────────────────────────────────────────────────────────────────────────────
// C++ wrapper layer — mirrors Boost.Endian's buffer and arithmetic type APIs.
// ─────────────────────────────────────────────────────────────────────────────

namespace boost_endian {

// ── Dispatch helpers: native_to_be / be_to_native ────────────────────────────
// Overloaded inline functions let templates dispatch to the right C function.

inline uint8_t  native_to_be(uint8_t  v) { return v; }
inline int8_t   native_to_be(int8_t   v) { return v; }
inline uint16_t native_to_be(uint16_t v) { return native_to_big_u16(v); }
inline int16_t  native_to_be(int16_t  v) { return native_to_big_i16(v); }
inline uint32_t native_to_be(uint32_t v) { return native_to_big_u32(v); }
inline int32_t  native_to_be(int32_t  v) { return native_to_big_i32(v); }
inline uint64_t native_to_be(uint64_t v) { return native_to_big_u64(v); }
inline int64_t  native_to_be(int64_t  v) { return native_to_big_i64(v); }

inline uint8_t  be_to_native(uint8_t  v) { return v; }
inline int8_t   be_to_native(int8_t   v) { return v; }
inline uint16_t be_to_native(uint16_t v) { return big_to_native_u16(v); }
inline int16_t  be_to_native(int16_t  v) { return big_to_native_i16(v); }
inline uint32_t be_to_native(uint32_t v) { return big_to_native_u32(v); }
inline int32_t  be_to_native(int32_t  v) { return big_to_native_i32(v); }
inline uint64_t be_to_native(uint64_t v) { return big_to_native_u64(v); }
inline int64_t  be_to_native(int64_t  v) { return big_to_native_i64(v); }

inline uint8_t  native_to_le(uint8_t  v) { return v; }
inline int8_t   native_to_le(int8_t   v) { return v; }
inline uint16_t native_to_le(uint16_t v) { return native_to_little_u16(v); }
inline int16_t  native_to_le(int16_t  v) { return native_to_little_i16(v); }
inline uint32_t native_to_le(uint32_t v) { return native_to_little_u32(v); }
inline int32_t  native_to_le(int32_t  v) { return native_to_little_i32(v); }
inline uint64_t native_to_le(uint64_t v) { return native_to_little_u64(v); }
inline int64_t  native_to_le(int64_t  v) { return native_to_little_i64(v); }

inline uint8_t  le_to_native(uint8_t  v) { return v; }
inline int8_t   le_to_native(int8_t   v) { return v; }
inline uint16_t le_to_native(uint16_t v) { return little_to_native_u16(v); }
inline int16_t  le_to_native(int16_t  v) { return little_to_native_i16(v); }
inline uint32_t le_to_native(uint32_t v) { return little_to_native_u32(v); }
inline int32_t  le_to_native(int32_t  v) { return little_to_native_i32(v); }
inline uint64_t le_to_native(uint64_t v) { return little_to_native_u64(v); }
inline int64_t  le_to_native(int64_t  v) { return little_to_native_i64(v); }

// ── Encoding policy tags ─────────────────────────────────────────────────────

struct BigEndian {
    template<class T> static T to_stored(T v) { return native_to_be(v); }
    template<class T> static T to_native(T v) { return be_to_native(v); }
};

struct LittleEndian {
    template<class T> static T to_stored(T v) { return native_to_le(v); }
    template<class T> static T to_native(T v) { return le_to_native(v); }
};

// ── EndianBuf<E, T>  —  buffer type (explicit conversion) ────────────────────

/// Stores a value of type T in the byte order specified by E.
/// Conversion to/from native is always explicit via value() / operator=().
///
/// The internal storage is a byte array of alignment 1, matching
/// Boost.Endian's unaligned types: no compiler padding is inserted between
/// consecutive EndianBuf fields in a struct.
template<class E, class T>
class EndianBuf {
    // Byte-array storage — alignment 1, size sizeof(T), no padding from host.
    uint8_t raw_[sizeof(T)];

    void write(T v) noexcept {
        T stored = E::template to_stored<T>(v);
        std::memcpy(raw_, &stored, sizeof(T));
    }
    T read() const noexcept {
        T stored;
        std::memcpy(&stored, raw_, sizeof(T));
        return E::template to_native<T>(stored);
    }

public:
    EndianBuf() noexcept = default;
    explicit EndianBuf(T native) noexcept { write(native); }

    /// Return the value in native byte order.
    T value() const noexcept { return read(); }

    /// Assign from a native-endian value.
    EndianBuf& operator=(T native) noexcept { write(native); return *this; }

    /// Pointer to the stored raw bytes (in target byte order — I/O ready).
    const uint8_t* data() const noexcept { return raw_; }
    uint8_t*       data()       noexcept { return raw_; }
};

// ── EndianInt<E, T>  —  arithmetic type (implicit conversion + all ops) ───────

/// Like EndianBuf but with full arithmetic operator support.
/// Conversion to/from T is implicit.
template<class E, class T>
class EndianInt {
    T raw_; // stored in E's byte order

    T native() const noexcept { return E::template to_native<T>(raw_); }
    void store(T v) noexcept  { raw_ = E::template to_stored<T>(v); }

public:
    EndianInt() noexcept = default;
    EndianInt(T native) noexcept : raw_(E::template to_stored<T>(native)) {} // NOLINT

    // Implicit conversion to T
    operator T() const noexcept { return native(); }

    T value() const noexcept { return native(); }

    const uint8_t* data() const noexcept {
        return reinterpret_cast<const uint8_t*>(&raw_);
    }

    // Assignment
    EndianInt& operator=(T v) noexcept  { store(v); return *this; }

    // Compound assignment
    EndianInt& operator+=(T v) noexcept { store(native() + v); return *this; }
    EndianInt& operator-=(T v) noexcept { store(native() - v); return *this; }
    EndianInt& operator*=(T v) noexcept { store(native() * v); return *this; }
    EndianInt& operator/=(T v) noexcept { store(native() / v); return *this; }
    EndianInt& operator%=(T v) noexcept { store(native() % v); return *this; }
    EndianInt& operator&=(T v) noexcept { store(native() & v); return *this; }
    EndianInt& operator|=(T v) noexcept { store(native() | v); return *this; }
    EndianInt& operator^=(T v) noexcept { store(native() ^ v); return *this; }
    EndianInt& operator<<=(T v) noexcept { store(native() << v); return *this; }
    EndianInt& operator>>=(T v) noexcept { store(native() >> v); return *this; }

    // Prefix ++/--
    EndianInt& operator++() noexcept { store(native() + T(1)); return *this; }
    EndianInt& operator--() noexcept { store(native() - T(1)); return *this; }

    // Postfix ++/--
    EndianInt operator++(int) noexcept { EndianInt tmp(*this); ++(*this); return tmp; }
    EndianInt operator--(int) noexcept { EndianInt tmp(*this); --(*this); return tmp; }

    // Unary
    T operator+() const noexcept { return  native(); }
    T operator-() const noexcept { return -native(); }
    T operator~() const noexcept { return ~native(); }
    bool operator!() const noexcept { return !native(); }
};

// ── Convenient type aliases ───────────────────────────────────────────────────

// Buffer types
template<class T> using big_endian_buf_t    = EndianBuf<BigEndian,    T>;
template<class T> using little_endian_buf_t = EndianBuf<LittleEndian, T>;

using big_int8_buf_t     = big_endian_buf_t<int8_t>;
using big_uint8_buf_t    = big_endian_buf_t<uint8_t>;
using big_int16_buf_t    = big_endian_buf_t<int16_t>;
using big_uint16_buf_t   = big_endian_buf_t<uint16_t>;
using big_int32_buf_t    = big_endian_buf_t<int32_t>;
using big_uint32_buf_t   = big_endian_buf_t<uint32_t>;
using big_int64_buf_t    = big_endian_buf_t<int64_t>;
using big_uint64_buf_t   = big_endian_buf_t<uint64_t>;

using little_int8_buf_t     = little_endian_buf_t<int8_t>;
using little_uint8_buf_t    = little_endian_buf_t<uint8_t>;
using little_int16_buf_t    = little_endian_buf_t<int16_t>;
using little_uint16_buf_t   = little_endian_buf_t<uint16_t>;
using little_int32_buf_t    = little_endian_buf_t<int32_t>;
using little_uint32_buf_t   = little_endian_buf_t<uint32_t>;
using little_int64_buf_t    = little_endian_buf_t<int64_t>;
using little_uint64_buf_t   = little_endian_buf_t<uint64_t>;

// Arithmetic types  (mirroring Boost.Endian: big_int32_t etc.)
template<class T> using big_endian_t    = EndianInt<BigEndian,    T>;
template<class T> using little_endian_t = EndianInt<LittleEndian, T>;

using big_int8_t     = big_endian_t<int8_t>;
using big_uint8_t    = big_endian_t<uint8_t>;
using big_int16_t    = big_endian_t<int16_t>;
using big_uint16_t   = big_endian_t<uint16_t>;
using big_int32_t    = big_endian_t<int32_t>;
using big_uint32_t   = big_endian_t<uint32_t>;
using big_int64_t    = big_endian_t<int64_t>;
using big_uint64_t   = big_endian_t<uint64_t>;

using little_int8_t     = little_endian_t<int8_t>;
using little_uint8_t    = little_endian_t<uint8_t>;
using little_int16_t    = little_endian_t<int16_t>;
using little_uint16_t   = little_endian_t<uint16_t>;
using little_int32_t    = little_endian_t<int32_t>;
using little_uint32_t   = little_endian_t<uint32_t>;
using little_int64_t    = little_endian_t<int64_t>;
using little_uint64_t   = little_endian_t<uint64_t>;

} // namespace boost_endian

#endif // __cplusplus
