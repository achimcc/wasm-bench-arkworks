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

|        | `2327246_bytes`           |
|:-------|:------------------------- |
|        | `94.97 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2207671_bytes`           |
|:-------|:------------------------- |
|        | `73.15 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2193459_bytes`           |
|:-------|:------------------------- |
|        | `73.26 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2192640_bytes`           |
|:-------|:------------------------- |
|        | `72.85 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2184436_bytes`           |
|:-------|:------------------------- |
|        | `66.89 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2183034_bytes`           |
|:-------|:------------------------- |
|        | `66.30 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2164124_bytes`           |
|:-------|:------------------------- |
|        | `69.09 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2137607_bytes`           |
|:-------|:------------------------- |
|        | `44.27 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2162920_bytes`           |
|:-------|:------------------------- |
|        | `64.52 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2173340_bytes`           |
|:-------|:------------------------- |
|        | `61.83 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2337925_bytes`            |
|:-------|:-------------------------- |
|        | `143.42 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2193106_bytes`           |
|:-------|:------------------------- |
|        | `72.97 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2192284_bytes`           |
|:-------|:------------------------- |
|        | `72.63 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2185769_bytes`           |
|:-------|:------------------------- |
|        | `67.79 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2184366_bytes`           |
|:-------|:------------------------- |
|        | `66.97 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2163991_bytes`           |
|:-------|:------------------------- |
|        | `65.68 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2175581_bytes`           |
|:-------|:------------------------- |
|        | `63.96 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2139821_bytes`           |
|:-------|:------------------------- |
|        | `55.00 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2138541_bytes`           |
|:-------|:------------------------- |
|        | `43.00 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2204841_bytes`           |
|:-------|:------------------------- |
|        | `99.74 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2209257_bytes`           |
|:-------|:------------------------- |
|        | `93.26 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`           |
|:-------|:------------------------- |
|        | `92.26 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2209261_bytes`           |
|:-------|:------------------------- |
|        | `92.63 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`           |
|:-------|:------------------------- |
|        | `95.94 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2165342_bytes`           |
|:-------|:------------------------- |
|        | `95.04 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2165339_bytes`           |
|:-------|:------------------------- |
|        | `94.09 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2190885_bytes`            |
|:-------|:-------------------------- |
|        | `109.23 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2190884_bytes`            |
|:-------|:-------------------------- |
|        | `109.06 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2176876_bytes`           |
|:-------|:------------------------- |
|        | `84.49 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2175792_bytes`           |
|:-------|:------------------------- |
|        | `84.38 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2165840_bytes`           |
|:-------|:------------------------- |
|        | `80.38 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2163915_bytes`           |
|:-------|:------------------------- |
|        | `58.33 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2157674_bytes`           |
|:-------|:------------------------- |
|        | `59.27 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2149670_bytes`           |
|:-------|:------------------------- |
|        | `72.09 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2137124_bytes`           |
|:-------|:------------------------- |
|        | `59.32 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2148055_bytes`           |
|:-------|:------------------------- |
|        | `62.68 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2129469_bytes`           |
|:-------|:------------------------- |
|        | `53.07 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2150847_bytes`           |
|:-------|:------------------------- |
|        | `74.10 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`           |
|:-------|:------------------------- |
|        | `82.27 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2164134_bytes`           |
|:-------|:------------------------- |
|        | `80.18 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2327246_bytes`           |
|:-------|:------------------------- |
|        | `43.36 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2207671_bytes`           |
|:-------|:------------------------- |
|        | `11.37 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2193459_bytes`           |
|:-------|:------------------------- |
|        | `24.20 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2192640_bytes`           |
|:-------|:------------------------- |
|        | `2.00 s` (✅ **1.00x**)    |

### exec:bls12-381-msm-g2-10:default

|        | `2184436_bytes`            |
|:-------|:-------------------------- |
|        | `172.76 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2183034_bytes`           |
|:-------|:------------------------- |
|        | `15.13 s` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g1:default

|        | `2164124_bytes`           |
|:-------|:------------------------- |
|        | `1.99 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g2:default

|        | `2137607_bytes`           |
|:-------|:------------------------- |
|        | `33.69 us` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g1:default

|        | `2162920_bytes`           |
|:-------|:------------------------- |
|        | `2.08 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-projective-g2:default

|        | `2173340_bytes`           |
|:-------|:------------------------- |
|        | `20.26 ms` (✅ **1.00x**)  |

### exec:bls12-377-pairing:default

|        | `2337925_bytes`           |
|:-------|:------------------------- |
|        | `27.44 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-10:default

|        | `2193106_bytes`           |
|:-------|:------------------------- |
|        | `23.55 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-1000:default

|        | `2192284_bytes`           |
|:-------|:------------------------- |
|        | `1.96 s` (✅ **1.00x**)    |

### exec:bls12-377-msm-g2-10:default

|        | `2185769_bytes`            |
|:-------|:-------------------------- |
|        | `156.99 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g2-1000:default

|        | `2184366_bytes`           |
|:-------|:------------------------- |
|        | `14.81 s` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g1:default

|        | `2163991_bytes`           |
|:-------|:------------------------- |
|        | `2.09 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g2:default

|        | `2175581_bytes`           |
|:-------|:------------------------- |
|        | `16.57 ms` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g1:default

|        | `2139821_bytes`           |
|:-------|:------------------------- |
|        | `25.15 us` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g2:default

|        | `2138541_bytes`           |
|:-------|:------------------------- |
|        | `33.83 us` (✅ **1.00x**)  |

### exec:bw6-761-pairing:default

|        | `2204841_bytes`           |
|:-------|:------------------------- |
|        | `76.39 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-10:default

|        | `2209257_bytes`            |
|:-------|:-------------------------- |
|        | `234.12 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`           |
|:-------|:------------------------- |
|        | `19.31 s` (✅ **1.00x**)   |

### exec:bw6-761-msm-g2-10:default

|        | `2209261_bytes`            |
|:-------|:-------------------------- |
|        | `225.56 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`           |
|:-------|:------------------------- |
|        | `19.44 s` (✅ **1.00x**)   |

### exec:bw6-761-mul-affine-g1:default

|        | `2165342_bytes`           |
|:-------|:------------------------- |
|        | `42.09 us` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g2:default

|        | `2165339_bytes`           |
|:-------|:------------------------- |
|        | `42.94 us` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g1:default

|        | `2190885_bytes`           |
|:-------|:------------------------- |
|        | `22.89 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g2:default

|        | `2190884_bytes`           |
|:-------|:------------------------- |
|        | `20.87 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-10:default

|        | `2176876_bytes`           |
|:-------|:------------------------- |
|        | `10.47 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-1000:default

|        | `2175792_bytes`            |
|:-------|:-------------------------- |
|        | `474.03 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-10:default

|        | `2165840_bytes`           |
|:-------|:------------------------- |
|        | `12.42 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-1000:default

|        | `2163915_bytes`            |
|:-------|:-------------------------- |
|        | `539.65 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-sw:default

|        | `2157674_bytes`            |
|:-------|:-------------------------- |
|        | `555.81 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-te:default

|        | `2149670_bytes`            |
|:-------|:-------------------------- |
|        | `559.46 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-sw:default

|        | `2137124_bytes`           |
|:-------|:------------------------- |
|        | `21.25 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-te:default

|        | `2148055_bytes`           |
|:-------|:------------------------- |
|        | `26.08 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-affine:default

|        | `2129469_bytes`           |
|:-------|:------------------------- |
|        | `28.17 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-projective:default

|        | `2150847_bytes`            |
|:-------|:-------------------------- |
|        | `699.44 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`           |
|:-------|:------------------------- |
|        | `12.45 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-1000:default

|        | `2164134_bytes`            |
|:-------|:-------------------------- |
|        | `578.34 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

