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
|        | `227.83 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2279395_bytes`            |
|:-------|:-------------------------- |
|        | `218.59 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2193456_bytes`            |
|:-------|:-------------------------- |
|        | `129.37 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2192640_bytes`            |
|:-------|:-------------------------- |
|        | `128.93 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2184436_bytes`            |
|:-------|:-------------------------- |
|        | `117.98 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2183027_bytes`            |
|:-------|:-------------------------- |
|        | `116.54 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2164128_bytes`            |
|:-------|:-------------------------- |
|        | `105.04 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2174293_bytes`            |
|:-------|:-------------------------- |
|        | `110.15 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2162920_bytes`            |
|:-------|:-------------------------- |
|        | `101.53 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2173340_bytes`            |
|:-------|:-------------------------- |
|        | `109.65 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2338263_bytes`            |
|:-------|:-------------------------- |
|        | `313.10 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2193106_bytes`            |
|:-------|:-------------------------- |
|        | `129.42 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2192284_bytes`            |
|:-------|:-------------------------- |
|        | `128.77 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2185765_bytes`            |
|:-------|:-------------------------- |
|        | `118.01 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2184366_bytes`            |
|:-------|:-------------------------- |
|        | `117.39 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2163988_bytes`            |
|:-------|:-------------------------- |
|        | `105.12 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2175581_bytes`            |
|:-------|:-------------------------- |
|        | `115.30 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2162787_bytes`            |
|:-------|:-------------------------- |
|        | `102.03 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2174450_bytes`            |
|:-------|:-------------------------- |
|        | `110.45 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2236037_bytes`            |
|:-------|:-------------------------- |
|        | `182.04 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2209260_bytes`            |
|:-------|:-------------------------- |
|        | `174.76 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`            |
|:-------|:-------------------------- |
|        | `171.36 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2209261_bytes`            |
|:-------|:-------------------------- |
|        | `174.27 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2207591_bytes`            |
|:-------|:-------------------------- |
|        | `171.03 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2195269_bytes`            |
|:-------|:-------------------------- |
|        | `159.93 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2195269_bytes`            |
|:-------|:-------------------------- |
|        | `160.37 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2190885_bytes`            |
|:-------|:-------------------------- |
|        | `149.74 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2190884_bytes`            |
|:-------|:-------------------------- |
|        | `150.22 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2176876_bytes`            |
|:-------|:-------------------------- |
|        | `109.95 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2175469_bytes`            |
|:-------|:-------------------------- |
|        | `109.74 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2165841_bytes`            |
|:-------|:-------------------------- |
|        | `106.40 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2163915_bytes`            |
|:-------|:-------------------------- |
|        | `105.55 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2157674_bytes`            |
|:-------|:-------------------------- |
|        | `107.44 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2149670_bytes`            |
|:-------|:-------------------------- |
|        | `100.78 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2158557_bytes`           |
|:-------|:------------------------- |
|        | `95.03 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2150657_bytes`            |
|:-------|:-------------------------- |
|        | `104.22 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2149855_bytes`            |
|:-------|:-------------------------- |
|        | `100.75 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2150847_bytes`            |
|:-------|:-------------------------- |
|        | `104.11 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`            |
|:-------|:-------------------------- |
|        | `107.74 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2164134_bytes`            |
|:-------|:-------------------------- |
|        | `107.02 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2307737_bytes`           |
|:-------|:------------------------- |
|        | `52.01 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2279395_bytes`           |
|:-------|:------------------------- |
|        | `37.33 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2193456_bytes`           |
|:-------|:------------------------- |
|        | `33.24 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2192640_bytes`           |
|:-------|:------------------------- |
|        | `2.41 s` (✅ **1.00x**)    |

### exec:bls12-381-msm-g2-10:default

|        | `2184436_bytes`            |
|:-------|:-------------------------- |
|        | `210.51 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2183027_bytes`           |
|:-------|:------------------------- |
|        | `18.51 s` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g1:default

|        | `2164128_bytes`           |
|:-------|:------------------------- |
|        | `2.25 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g2:default

|        | `2174293_bytes`           |
|:-------|:------------------------- |
|        | `21.57 ms` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g1:default

|        | `2162920_bytes`           |
|:-------|:------------------------- |
|        | `2.34 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-projective-g2:default

|        | `2173340_bytes`           |
|:-------|:------------------------- |
|        | `21.89 ms` (✅ **1.00x**)  |

### exec:bls12-377-pairing:default

|        | `2338263_bytes`           |
|:-------|:------------------------- |
|        | `35.41 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-10:default

|        | `2193106_bytes`           |
|:-------|:------------------------- |
|        | `32.09 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-1000:default

|        | `2192284_bytes`           |
|:-------|:------------------------- |
|        | `2.36 s` (✅ **1.00x**)    |

### exec:bls12-377-msm-g2-10:default

|        | `2185765_bytes`            |
|:-------|:-------------------------- |
|        | `211.86 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g2-1000:default

|        | `2184366_bytes`           |
|:-------|:------------------------- |
|        | `18.08 s` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g1:default

|        | `2163988_bytes`           |
|:-------|:------------------------- |
|        | `2.36 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g2:default

|        | `2175581_bytes`           |
|:-------|:------------------------- |
|        | `18.72 ms` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g1:default

|        | `2162787_bytes`           |
|:-------|:------------------------- |
|        | `2.44 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-projective-g2:default

|        | `2174450_bytes`           |
|:-------|:------------------------- |
|        | `19.09 ms` (✅ **1.00x**)  |

### exec:bw6-761-pairing:default

|        | `2236037_bytes`            |
|:-------|:-------------------------- |
|        | `120.93 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-10:default

|        | `2209260_bytes`            |
|:-------|:-------------------------- |
|        | `275.47 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`           |
|:-------|:------------------------- |
|        | `21.98 s` (✅ **1.00x**)   |

### exec:bw6-761-msm-g2-10:default

|        | `2209261_bytes`            |
|:-------|:-------------------------- |
|        | `250.68 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g2-1000:default

|        | `2207591_bytes`           |
|:-------|:------------------------- |
|        | `21.95 s` (✅ **1.00x**)   |

### exec:bw6-761-mul-affine-g1:default

|        | `2195269_bytes`           |
|:-------|:------------------------- |
|        | `25.41 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g2:default

|        | `2195269_bytes`           |
|:-------|:------------------------- |
|        | `22.32 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g1:default

|        | `2190885_bytes`           |
|:-------|:------------------------- |
|        | `25.68 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g2:default

|        | `2190884_bytes`           |
|:-------|:------------------------- |
|        | `22.59 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-10:default

|        | `2176876_bytes`           |
|:-------|:------------------------- |
|        | `11.94 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-1000:default

|        | `2175469_bytes`            |
|:-------|:-------------------------- |
|        | `539.12 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-10:default

|        | `2165841_bytes`           |
|:-------|:------------------------- |
|        | `14.30 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-1000:default

|        | `2163915_bytes`            |
|:-------|:-------------------------- |
|        | `616.37 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-sw:default

|        | `2157674_bytes`            |
|:-------|:-------------------------- |
|        | `620.88 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-te:default

|        | `2149670_bytes`            |
|:-------|:-------------------------- |
|        | `626.22 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-sw:default

|        | `2158557_bytes`            |
|:-------|:-------------------------- |
|        | `661.09 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-te:default

|        | `2150657_bytes`            |
|:-------|:-------------------------- |
|        | `611.47 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-affine:default

|        | `2149855_bytes`            |
|:-------|:-------------------------- |
|        | `797.90 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-projective:default

|        | `2150847_bytes`            |
|:-------|:-------------------------- |
|        | `782.71 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`           |
|:-------|:------------------------- |
|        | `14.16 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-1000:default

|        | `2164134_bytes`            |
|:-------|:-------------------------- |
|        | `659.94 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

