
Running benchmarks for 256 floats (0 MB)...
==========================================================================================
  FFI OVERHEAD ANALYSIS  --  Boost.Align (C++ AVX) vs Rust (FFI)
  Buffer : 256 floats (0 MB)     Iterations : 50
==========================================================================================

  IMPLEMENTATION
------------------------------------------------------------------------------------------
  Method                                    Details                                         
------------------------------------------------------------------------------------------
  C++ Native                                boost::aligned_alloc + _mm256_mul_ps            
  Rust FFI (AlignedBuffer)                  Rust Box wrapper + FFI call + assertions        
  Rust FFI (raw ptr)                        C++-alloc'd ptr + direct Rust kernel            

  CORRECTNESS
------------------------------------------------------------------------------------------
  Method                                    Result
------------------------------------------------------------------------------------------
  C++ Native                                PASS
  Rust FFI (AlignedBuffer)                  PASS
  Rust FFI (raw ptr)                        PASS

  TIMING (Per-iteration)
------------------------------------------------------------------------------------------
  Method                                         Time (ms)       Time (ns)  Throughput
------------------------------------------------------------------------------------------
  C++ Native                                       0.0001ms            148ns      13.86 GB/s
  Rust FFI (AlignedBuffer)                         0.0001ms            146ns      14.02 GB/s  ~ TIE
  Rust FFI (raw ptr)                               0.0001ms            137ns      14.99 GB/s  <-- FASTER

  FFI OVERHEAD ANALYSIS
------------------------------------------------------------------------------------------
  Method comparison vs C++ Native:

    Rust FFI (AlignedBuffer): -1.12% overhead (-2 ns)
    Rust FFI (raw ptr):       -7.55% overhead (-11 ns)

    Breakdown:
      AlignedBuffer wrapper overhead: +9 ns
      (Box dereferencing + assertions)

==========================================================================================
  SUMMARY
==========================================================================================
  Method                                     Per-iter (ns)      Throughput
------------------------------------------------------------------------------------------
  C++ Native                                          148          13.86 GB/s
  Rust FFI (AlignedBuffer)                            146          14.02 GB/s  (-1.1%)
  Rust FFI (raw ptr)                                  137          14.99 GB/s  (-7.6%)
==========================================================================================


Running benchmarks for 16384 floats (0 MB)...
==========================================================================================
  FFI OVERHEAD ANALYSIS  --  Boost.Align (C++ AVX) vs Rust (FFI)
  Buffer : 16384 floats (0 MB)     Iterations : 50
==========================================================================================

  IMPLEMENTATION
------------------------------------------------------------------------------------------
  Method                                    Details                                         
------------------------------------------------------------------------------------------
  C++ Native                                boost::aligned_alloc + _mm256_mul_ps            
  Rust FFI (AlignedBuffer)                  Rust Box wrapper + FFI call + assertions        
  Rust FFI (raw ptr)                        C++-alloc'd ptr + direct Rust kernel            

  CORRECTNESS
------------------------------------------------------------------------------------------
  Method                                    Result
------------------------------------------------------------------------------------------
  C++ Native                                PASS
  Rust FFI (AlignedBuffer)                  PASS
  Rust FFI (raw ptr)                        PASS

  TIMING (Per-iteration)
------------------------------------------------------------------------------------------
  Method                                         Time (ms)       Time (ns)  Throughput
------------------------------------------------------------------------------------------
  C++ Native                                       0.0087ms           8746ns      14.99 GB/s
  Rust FFI (AlignedBuffer)                         0.0083ms           8332ns      15.73 GB/s  <-- FASTER
  Rust FFI (raw ptr)                               0.0083ms           8285ns      15.82 GB/s  <-- FASTER

  FFI OVERHEAD ANALYSIS
------------------------------------------------------------------------------------------
  Method comparison vs C++ Native:

    Rust FFI (AlignedBuffer): -4.73% overhead (-414 ns)
    Rust FFI (raw ptr):       -5.27% overhead (-461 ns)

    Breakdown:
      AlignedBuffer wrapper overhead: +47 ns
      (Box dereferencing + assertions)

==========================================================================================
  SUMMARY
==========================================================================================
  Method                                     Per-iter (ns)      Throughput
------------------------------------------------------------------------------------------
  C++ Native                                         8746          14.99 GB/s
  Rust FFI (AlignedBuffer)                           8332          15.73 GB/s  (-4.7%)
  Rust FFI (raw ptr)                                 8285          15.82 GB/s  (-5.3%)
==========================================================================================


Running benchmarks for 1048576 floats (4 MB)...
==========================================================================================
  FFI OVERHEAD ANALYSIS  --  Boost.Align (C++ AVX) vs Rust (FFI)
  Buffer : 1048576 floats (4 MB)     Iterations : 50
==========================================================================================

  IMPLEMENTATION
------------------------------------------------------------------------------------------
  Method                                    Details                                         
------------------------------------------------------------------------------------------
  C++ Native                                boost::aligned_alloc + _mm256_mul_ps            
  Rust FFI (AlignedBuffer)                  Rust Box wrapper + FFI call + assertions        
  Rust FFI (raw ptr)                        C++-alloc'd ptr + direct Rust kernel            

  CORRECTNESS
------------------------------------------------------------------------------------------
  Method                                    Result
------------------------------------------------------------------------------------------
  C++ Native                                PASS
  Rust FFI (AlignedBuffer)                  PASS
  Rust FFI (raw ptr)                        PASS

  TIMING (Per-iteration)
------------------------------------------------------------------------------------------
  Method                                         Time (ms)       Time (ns)  Throughput
------------------------------------------------------------------------------------------
  C++ Native                                       0.2931ms         293074ns      28.62 GB/s
  Rust FFI (AlignedBuffer)                         0.2856ms         285596ns      29.37 GB/s  <-- FASTER
  Rust FFI (raw ptr)                               0.2716ms         271576ns      30.89 GB/s  <-- FASTER

  FFI OVERHEAD ANALYSIS
------------------------------------------------------------------------------------------
  Method comparison vs C++ Native:

    Rust FFI (AlignedBuffer): -2.55% overhead (-7477 ns)
    Rust FFI (raw ptr):       -7.34% overhead (-21498 ns)

    Breakdown:
      AlignedBuffer wrapper overhead: +14021 ns
      (Box dereferencing + assertions)

==========================================================================================
  SUMMARY
==========================================================================================
  Method                                     Per-iter (ns)      Throughput
------------------------------------------------------------------------------------------
  C++ Native                                       293074          28.62 GB/s
  Rust FFI (AlignedBuffer)                         285596          29.37 GB/s  (-2.6%)
  Rust FFI (raw ptr)                               271576          30.89 GB/s  (-7.3%)
==========================================================================================


Running benchmarks for 16777216 floats (64 MB)...
==========================================================================================
  FFI OVERHEAD ANALYSIS  --  Boost.Align (C++ AVX) vs Rust (FFI)
  Buffer : 16777216 floats (64 MB)     Iterations : 50
==========================================================================================

  IMPLEMENTATION
------------------------------------------------------------------------------------------
  Method                                    Details                                         
------------------------------------------------------------------------------------------
  C++ Native                                boost::aligned_alloc + _mm256_mul_ps            
  Rust FFI (AlignedBuffer)                  Rust Box wrapper + FFI call + assertions        
  Rust FFI (raw ptr)                        C++-alloc'd ptr + direct Rust kernel            

  CORRECTNESS
------------------------------------------------------------------------------------------
  Method                                    Result
------------------------------------------------------------------------------------------
  C++ Native                                PASS
  Rust FFI (AlignedBuffer)                  PASS
  Rust FFI (raw ptr)                        PASS

  TIMING (Per-iteration)
------------------------------------------------------------------------------------------
  Method                                         Time (ms)       Time (ns)  Throughput
------------------------------------------------------------------------------------------
  C++ Native                                       7.1101ms        7110144ns      18.88 GB/s
  Rust FFI (AlignedBuffer)                         7.1034ms        7103436ns      18.89 GB/s  ~ TIE
  Rust FFI (raw ptr)                               6.9813ms        6981330ns      19.23 GB/s  ~ TIE

  FFI OVERHEAD ANALYSIS
------------------------------------------------------------------------------------------
  Method comparison vs C++ Native:

    Rust FFI (AlignedBuffer): -0.09% overhead (-6708 ns)
    Rust FFI (raw ptr):       -1.81% overhead (-128814 ns)

    Breakdown:
      AlignedBuffer wrapper overhead: +122106 ns
      (Box dereferencing + assertions)

==========================================================================================
  SUMMARY
==========================================================================================
  Method                                     Per-iter (ns)      Throughput
------------------------------------------------------------------------------------------
  C++ Native                                      7110144          18.88 GB/s
  Rust FFI (AlignedBuffer)                        7103436          18.89 GB/s  (-0.1%)
  Rust FFI (raw ptr)                              6981330          19.23 GB/s  (-1.8%)
==========================================================================================


================================================================================
Interpretation Guide:
  * FFI overhead (positive %): FFI call cost on top of work
  * Overhead trend: decreases as buffer size increases (fixed cost amortized)
  * AlignedBuffer vs raw ptr: wrapper overhead (Box, assertions, ptr derefs)
================================================================================

