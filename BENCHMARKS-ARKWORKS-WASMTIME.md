# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [compile:groth16:default](#compile:groth16:default)
    - [compile:bls12-381-pairing:default](#compile:bls12-381-pairing:default)
    - [compile:bls12-381-msm-g1-10:default](#compile:bls12-381-msm-g1-10:default)
    - [compile:bls12-381-msm-g1-1000:default](#compile:bls12-381-msm-g1-1000:default)
    - [compile:bls12-381-msm-g2-10:default](#compile:bls12-381-msm-g2-10:default)
    - [compile:bls12-381-msm-g2-1000:default](#compile:bls12-381-msm-g2-1000:default)
    - [compile:bls12-381-mul-affine-g1:default](#compile:bls12-381-mul-affine-g1:default)
    - [compile:bls12-381-mul-affine-g2:default](#compile:bls12-381-mul-affine-g2:default)
    - [compile:bls12-381-mul-projective-g1:default](#compile:bls12-381-mul-projective-g1:default)
    - [compile:bls12-381-mul-projective-g2:default](#compile:bls12-381-mul-projective-g2:default)
    - [compile:bls12-377-pairing:default](#compile:bls12-377-pairing:default)
    - [compile:bls12-377-msm-g1-10:default](#compile:bls12-377-msm-g1-10:default)
    - [compile:bls12-377-msm-g1-1000:default](#compile:bls12-377-msm-g1-1000:default)
    - [compile:bls12-377-msm-g2-10:default](#compile:bls12-377-msm-g2-10:default)
    - [compile:bls12-377-msm-g2-1000:default](#compile:bls12-377-msm-g2-1000:default)
    - [compile:bls12-377-mul-affine-g1:default](#compile:bls12-377-mul-affine-g1:default)
    - [compile:bls12-377-mul-affine-g2:default](#compile:bls12-377-mul-affine-g2:default)
    - [compile:bls12-377-mul-projective-g1:default](#compile:bls12-377-mul-projective-g1:default)
    - [compile:bls12-377-mul-projective-g2:default](#compile:bls12-377-mul-projective-g2:default)
    - [compile:bw6-761-pairing:default](#compile:bw6-761-pairing:default)
    - [compile:bw6-761-msm-g1-10:default](#compile:bw6-761-msm-g1-10:default)
    - [compile:bw6-761-msm-g1-1000:default](#compile:bw6-761-msm-g1-1000:default)
    - [compile:bw6-761-msm-g2-10:default](#compile:bw6-761-msm-g2-10:default)
    - [compile:bw6-761-msm-g2-1000:default](#compile:bw6-761-msm-g2-1000:default)
    - [compile:bw6-761-mul-affine-g1:default](#compile:bw6-761-mul-affine-g1:default)
    - [compile:bw6-761-mul-affine-g2:default](#compile:bw6-761-mul-affine-g2:default)
    - [compile:bw6-761-mul-projective-g1:default](#compile:bw6-761-mul-projective-g1:default)
    - [compile:bw6-761-mul-projective-g2:default](#compile:bw6-761-mul-projective-g2:default)
    - [compile:ed-on-bls12-381-msm-sw-10:default](#compile:ed-on-bls12-381-msm-sw-10:default)
    - [compile:ed-on-bls12-381-msm-sw-1000:default](#compile:ed-on-bls12-381-msm-sw-1000:default)
    - [compile:ed-on-bls12-381-msm-te-10:default](#compile:ed-on-bls12-381-msm-te-10:default)
    - [compile:ed-on-bls12-381-msm-te-1000:default](#compile:ed-on-bls12-381-msm-te-1000:default)
    - [compile:ed-on-bls12-381-mul-affine-sw:default](#compile:ed-on-bls12-381-mul-affine-sw:default)
    - [compile:ed-on-bls12-381-mul-affine-te:default](#compile:ed-on-bls12-381-mul-affine-te:default)
    - [compile:ed-on-bls12-381-mul-projective-sw:default](#compile:ed-on-bls12-381-mul-projective-sw:default)
    - [compile:ed-on-bls12-381-mul-projective-te:default](#compile:ed-on-bls12-381-mul-projective-te:default)
    - [compile:ed-on-bls12-377-mul-affine:default](#compile:ed-on-bls12-377-mul-affine:default)
    - [compile:ed-on-bls12-377-mul-projective:default](#compile:ed-on-bls12-377-mul-projective:default)
    - [compile:ed-on-bls12-377-msm-10:default](#compile:ed-on-bls12-377-msm-10:default)
    - [compile:ed-on-bls12-377-msm-1000:default](#compile:ed-on-bls12-377-msm-1000:default)
    - [exec:groth16:default](#exec:groth16:default)
    - [exec:bls12-381-pairing:default](#exec:bls12-381-pairing:default)
    - [exec:bls12-381-msm-g1-10:default](#exec:bls12-381-msm-g1-10:default)
    - [exec:bls12-381-msm-g1-1000:default](#exec:bls12-381-msm-g1-1000:default)
    - [exec:bls12-381-msm-g2-10:default](#exec:bls12-381-msm-g2-10:default)
    - [exec:bls12-381-msm-g2-1000:default](#exec:bls12-381-msm-g2-1000:default)
    - [exec:bls12-381-mul-affine-g1:default](#exec:bls12-381-mul-affine-g1:default)
    - [exec:bls12-381-mul-affine-g2:default](#exec:bls12-381-mul-affine-g2:default)
    - [exec:bls12-381-mul-projective-g1:default](#exec:bls12-381-mul-projective-g1:default)
    - [exec:bls12-381-mul-projective-g2:default](#exec:bls12-381-mul-projective-g2:default)
    - [exec:bls12-377-pairing:default](#exec:bls12-377-pairing:default)
    - [exec:bls12-377-msm-g1-10:default](#exec:bls12-377-msm-g1-10:default)
    - [exec:bls12-377-msm-g1-1000:default](#exec:bls12-377-msm-g1-1000:default)
    - [exec:bls12-377-msm-g2-10:default](#exec:bls12-377-msm-g2-10:default)
    - [exec:bls12-377-msm-g2-1000:default](#exec:bls12-377-msm-g2-1000:default)
    - [exec:bls12-377-mul-affine-g1:default](#exec:bls12-377-mul-affine-g1:default)
    - [exec:bls12-377-mul-affine-g2:default](#exec:bls12-377-mul-affine-g2:default)
    - [exec:bls12-377-mul-projective-g1:default](#exec:bls12-377-mul-projective-g1:default)
    - [exec:bls12-377-mul-projective-g2:default](#exec:bls12-377-mul-projective-g2:default)
    - [exec:bw6-761-pairing:default](#exec:bw6-761-pairing:default)
    - [exec:bw6-761-msm-g1-10:default](#exec:bw6-761-msm-g1-10:default)
    - [exec:bw6-761-msm-g1-1000:default](#exec:bw6-761-msm-g1-1000:default)
    - [exec:bw6-761-msm-g2-10:default](#exec:bw6-761-msm-g2-10:default)
    - [exec:bw6-761-msm-g2-1000:default](#exec:bw6-761-msm-g2-1000:default)
    - [exec:bw6-761-mul-affine-g1:default](#exec:bw6-761-mul-affine-g1:default)
    - [exec:bw6-761-mul-affine-g2:default](#exec:bw6-761-mul-affine-g2:default)
    - [exec:bw6-761-mul-projective-g1:default](#exec:bw6-761-mul-projective-g1:default)
    - [exec:bw6-761-mul-projective-g2:default](#exec:bw6-761-mul-projective-g2:default)
    - [exec:ed-on-bls12-381-msm-sw-10:default](#exec:ed-on-bls12-381-msm-sw-10:default)
    - [exec:ed-on-bls12-381-msm-sw-1000:default](#exec:ed-on-bls12-381-msm-sw-1000:default)
    - [exec:ed-on-bls12-381-msm-te-10:default](#exec:ed-on-bls12-381-msm-te-10:default)
    - [exec:ed-on-bls12-381-msm-te-1000:default](#exec:ed-on-bls12-381-msm-te-1000:default)
    - [exec:ed-on-bls12-381-mul-affine-sw:default](#exec:ed-on-bls12-381-mul-affine-sw:default)
    - [exec:ed-on-bls12-381-mul-affine-te:default](#exec:ed-on-bls12-381-mul-affine-te:default)
    - [exec:ed-on-bls12-381-mul-projective-sw:default](#exec:ed-on-bls12-381-mul-projective-sw:default)
    - [exec:ed-on-bls12-381-mul-projective-te:default](#exec:ed-on-bls12-381-mul-projective-te:default)
    - [exec:ed-on-bls12-377-mul-affine:default](#exec:ed-on-bls12-377-mul-affine:default)
    - [exec:ed-on-bls12-377-mul-projective:default](#exec:ed-on-bls12-377-mul-projective:default)
    - [exec:ed-on-bls12-377-msm-10:default](#exec:ed-on-bls12-377-msm-10:default)
    - [exec:ed-on-bls12-377-msm-1000:default](#exec:ed-on-bls12-377-msm-1000:default)

## Benchmark Results

### compile:groth16:default

|        | `2307737_bytes`            |
|:-------|:-------------------------- |
|        | `278.14 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2279395_bytes`            |
|:-------|:-------------------------- |
|        | `267.00 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2193456_bytes`            |
|:-------|:-------------------------- |
|        | `157.02 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2192640_bytes`            |
|:-------|:-------------------------- |
|        | `160.02 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2184436_bytes`            |
|:-------|:-------------------------- |
|        | `140.00 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2183027_bytes`            |
|:-------|:-------------------------- |
|        | `139.56 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2164128_bytes`            |
|:-------|:-------------------------- |
|        | `126.79 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2174293_bytes`            |
|:-------|:-------------------------- |
|        | `134.19 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2162920_bytes`            |
|:-------|:-------------------------- |
|        | `125.41 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2173340_bytes`            |
|:-------|:-------------------------- |
|        | `132.05 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2338263_bytes`            |
|:-------|:-------------------------- |
|        | `382.37 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2193106_bytes`            |
|:-------|:-------------------------- |
|        | `158.50 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2192284_bytes`            |
|:-------|:-------------------------- |
|        | `154.75 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2185765_bytes`            |
|:-------|:-------------------------- |
|        | `147.04 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2184366_bytes`            |
|:-------|:-------------------------- |
|        | `141.04 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2163988_bytes`            |
|:-------|:-------------------------- |
|        | `127.01 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2175581_bytes`            |
|:-------|:-------------------------- |
|        | `137.64 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2162787_bytes`            |
|:-------|:-------------------------- |
|        | `123.92 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2174450_bytes`            |
|:-------|:-------------------------- |
|        | `133.12 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2236037_bytes`            |
|:-------|:-------------------------- |
|        | `223.05 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2209260_bytes`            |
|:-------|:-------------------------- |
|        | `212.29 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`            |
|:-------|:-------------------------- |
|        | `208.69 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2209261_bytes`            |
|:-------|:-------------------------- |
|        | `211.99 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2207591_bytes`            |
|:-------|:-------------------------- |
|        | `210.44 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2195269_bytes`            |
|:-------|:-------------------------- |
|        | `193.32 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2195269_bytes`            |
|:-------|:-------------------------- |
|        | `194.70 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2190885_bytes`            |
|:-------|:-------------------------- |
|        | `180.89 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2190884_bytes`            |
|:-------|:-------------------------- |
|        | `184.27 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2176876_bytes`            |
|:-------|:-------------------------- |
|        | `131.48 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2175469_bytes`            |
|:-------|:-------------------------- |
|        | `131.17 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2165841_bytes`            |
|:-------|:-------------------------- |
|        | `128.10 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2163915_bytes`            |
|:-------|:-------------------------- |
|        | `126.27 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2157674_bytes`            |
|:-------|:-------------------------- |
|        | `127.74 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2149670_bytes`            |
|:-------|:-------------------------- |
|        | `120.37 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2158557_bytes`            |
|:-------|:-------------------------- |
|        | `129.76 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2150657_bytes`            |
|:-------|:-------------------------- |
|        | `124.52 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2149855_bytes`            |
|:-------|:-------------------------- |
|        | `122.75 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2150847_bytes`            |
|:-------|:-------------------------- |
|        | `125.31 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`            |
|:-------|:-------------------------- |
|        | `130.04 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2164134_bytes`            |
|:-------|:-------------------------- |
|        | `127.59 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2307737_bytes`           |
|:-------|:------------------------- |
|        | `62.50 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2279395_bytes`           |
|:-------|:------------------------- |
|        | `44.82 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2193456_bytes`           |
|:-------|:------------------------- |
|        | `40.68 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2192640_bytes`           |
|:-------|:------------------------- |
|        | `2.90 s` (✅ **1.00x**)    |

### exec:bls12-381-msm-g2-10:default

|        | `2184436_bytes`            |
|:-------|:-------------------------- |
|        | `255.42 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2183027_bytes`           |
|:-------|:------------------------- |
|        | `22.17 s` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g1:default

|        | `2164128_bytes`           |
|:-------|:------------------------- |
|        | `2.70 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g2:default

|        | `2174293_bytes`           |
|:-------|:------------------------- |
|        | `25.82 ms` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g1:default

|        | `2162920_bytes`           |
|:-------|:------------------------- |
|        | `2.91 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-projective-g2:default

|        | `2173340_bytes`           |
|:-------|:------------------------- |
|        | `26.23 ms` (✅ **1.00x**)  |

### exec:bls12-377-pairing:default

|        | `2338263_bytes`           |
|:-------|:------------------------- |
|        | `42.54 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-10:default

|        | `2193106_bytes`           |
|:-------|:------------------------- |
|        | `38.63 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-1000:default

|        | `2192284_bytes`           |
|:-------|:------------------------- |
|        | `2.84 s` (✅ **1.00x**)    |

### exec:bls12-377-msm-g2-10:default

|        | `2185765_bytes`            |
|:-------|:-------------------------- |
|        | `254.06 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g2-1000:default

|        | `2184366_bytes`           |
|:-------|:------------------------- |
|        | `21.61 s` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g1:default

|        | `2163988_bytes`           |
|:-------|:------------------------- |
|        | `2.84 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g2:default

|        | `2175581_bytes`           |
|:-------|:------------------------- |
|        | `23.07 ms` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g1:default

|        | `2162787_bytes`           |
|:-------|:------------------------- |
|        | `2.96 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-projective-g2:default

|        | `2174450_bytes`           |
|:-------|:------------------------- |
|        | `22.84 ms` (✅ **1.00x**)  |

### exec:bw6-761-pairing:default

|        | `2236037_bytes`            |
|:-------|:-------------------------- |
|        | `145.12 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-10:default

|        | `2209260_bytes`            |
|:-------|:-------------------------- |
|        | `331.10 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`           |
|:-------|:------------------------- |
|        | `26.39 s` (✅ **1.00x**)   |

### exec:bw6-761-msm-g2-10:default

|        | `2209261_bytes`            |
|:-------|:-------------------------- |
|        | `302.30 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g2-1000:default

|        | `2207591_bytes`           |
|:-------|:------------------------- |
|        | `26.34 s` (✅ **1.00x**)   |

### exec:bw6-761-mul-affine-g1:default

|        | `2195269_bytes`           |
|:-------|:------------------------- |
|        | `30.50 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g2:default

|        | `2195269_bytes`           |
|:-------|:------------------------- |
|        | `26.82 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g1:default

|        | `2190885_bytes`           |
|:-------|:------------------------- |
|        | `31.58 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g2:default

|        | `2190884_bytes`           |
|:-------|:------------------------- |
|        | `27.18 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-10:default

|        | `2176876_bytes`           |
|:-------|:------------------------- |
|        | `14.33 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-1000:default

|        | `2175469_bytes`            |
|:-------|:-------------------------- |
|        | `646.94 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-10:default

|        | `2165841_bytes`           |
|:-------|:------------------------- |
|        | `17.09 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-1000:default

|        | `2163915_bytes`            |
|:-------|:-------------------------- |
|        | `743.34 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-sw:default

|        | `2157674_bytes`            |
|:-------|:-------------------------- |
|        | `772.07 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-te:default

|        | `2149670_bytes`            |
|:-------|:-------------------------- |
|        | `755.03 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-sw:default

|        | `2158557_bytes`            |
|:-------|:-------------------------- |
|        | `802.17 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-te:default

|        | `2150657_bytes`            |
|:-------|:-------------------------- |
|        | `735.14 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-affine:default

|        | `2149855_bytes`            |
|:-------|:-------------------------- |
|        | `959.43 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-projective:default

|        | `2150847_bytes`            |
|:-------|:-------------------------- |
|        | `937.71 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`           |
|:-------|:------------------------- |
|        | `17.03 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-1000:default

|        | `2164134_bytes`            |
|:-------|:-------------------------- |
|        | `792.58 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

