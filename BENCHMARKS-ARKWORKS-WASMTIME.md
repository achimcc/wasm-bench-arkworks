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
    - [compile:ed-on-bls12-381-bandersnatch-msm-sw-10:default](#compile:ed-on-bls12-381-bandersnatch-msm-sw-10:default)
    - [compile:ed-on-bls12-381-bandersnatch-msm-sw-1000:default](#compile:ed-on-bls12-381-bandersnatch-msm-sw-1000:default)
    - [compile:ed-on-bls12-381-bandersnatch-msm-te-10:default](#compile:ed-on-bls12-381-bandersnatch-msm-te-10:default)
    - [compile:ed-on-bls12-381-bandersnatch-msm-te-1000:default](#compile:ed-on-bls12-381-bandersnatch-msm-te-1000:default)
    - [compile:ed-on-bls12-381-bandersnatch-mul-affine-sw:default](#compile:ed-on-bls12-381-bandersnatch-mul-affine-sw:default)
    - [compile:ed-on-bls12-381-bandersnatch-mul-affine-te:default](#compile:ed-on-bls12-381-bandersnatch-mul-affine-te:default)
    - [compile:ed-on-bls12-381-bandersnatch-mul-projective-sw:default](#compile:ed-on-bls12-381-bandersnatch-mul-projective-sw:default)
    - [compile:ed-on-bls12-381-bandersnatch-mul-projective-te:default](#compile:ed-on-bls12-381-bandersnatch-mul-projective-te:default)
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
    - [exec:ed-on-bls12-381-bandersnatch-msm-sw-10:default](#exec:ed-on-bls12-381-bandersnatch-msm-sw-10:default)
    - [exec:ed-on-bls12-381-bandersnatch-msm-sw-1000:default](#exec:ed-on-bls12-381-bandersnatch-msm-sw-1000:default)
    - [exec:ed-on-bls12-381-bandersnatch-msm-te-10:default](#exec:ed-on-bls12-381-bandersnatch-msm-te-10:default)
    - [exec:ed-on-bls12-381-bandersnatch-msm-te-1000:default](#exec:ed-on-bls12-381-bandersnatch-msm-te-1000:default)
    - [exec:ed-on-bls12-381-bandersnatch-mul-affine-sw:default](#exec:ed-on-bls12-381-bandersnatch-mul-affine-sw:default)
    - [exec:ed-on-bls12-381-bandersnatch-mul-affine-te:default](#exec:ed-on-bls12-381-bandersnatch-mul-affine-te:default)
    - [exec:ed-on-bls12-381-bandersnatch-mul-projective-sw:default](#exec:ed-on-bls12-381-bandersnatch-mul-projective-sw:default)
    - [exec:ed-on-bls12-381-bandersnatch-mul-projective-te:default](#exec:ed-on-bls12-381-bandersnatch-mul-projective-te:default)
    - [exec:ed-on-bls12-377-mul-affine:default](#exec:ed-on-bls12-377-mul-affine:default)
    - [exec:ed-on-bls12-377-mul-projective:default](#exec:ed-on-bls12-377-mul-projective:default)
    - [exec:ed-on-bls12-377-msm-10:default](#exec:ed-on-bls12-377-msm-10:default)
    - [exec:ed-on-bls12-377-msm-1000:default](#exec:ed-on-bls12-377-msm-1000:default)

## Benchmark Results

### compile:groth16:default

|        | `2307742_bytes`            |
|:-------|:-------------------------- |
|        | `289.46 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2289912_bytes`            |
|:-------|:-------------------------- |
|        | `299.22 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2180522_bytes`            |
|:-------|:-------------------------- |
|        | `150.42 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2179710_bytes`            |
|:-------|:-------------------------- |
|        | `149.34 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2184433_bytes`            |
|:-------|:-------------------------- |
|        | `146.88 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2183029_bytes`            |
|:-------|:-------------------------- |
|        | `147.15 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2164125_bytes`            |
|:-------|:-------------------------- |
|        | `131.46 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2174305_bytes`            |
|:-------|:-------------------------- |
|        | `138.82 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2162833_bytes`            |
|:-------|:-------------------------- |
|        | `130.81 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2173405_bytes`            |
|:-------|:-------------------------- |
|        | `139.10 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2362038_bytes`            |
|:-------|:-------------------------- |
|        | `448.65 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2180410_bytes`            |
|:-------|:-------------------------- |
|        | `148.85 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2179593_bytes`            |
|:-------|:-------------------------- |
|        | `150.18 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2185770_bytes`            |
|:-------|:-------------------------- |
|        | `149.44 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2184363_bytes`            |
|:-------|:-------------------------- |
|        | `149.22 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2163991_bytes`            |
|:-------|:-------------------------- |
|        | `130.78 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2175594_bytes`            |
|:-------|:-------------------------- |
|        | `140.71 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2162705_bytes`            |
|:-------|:-------------------------- |
|        | `128.59 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2174518_bytes`            |
|:-------|:-------------------------- |
|        | `137.92 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2236113_bytes`            |
|:-------|:-------------------------- |
|        | `227.35 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2209266_bytes`            |
|:-------|:-------------------------- |
|        | `215.84 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2207602_bytes`            |
|:-------|:-------------------------- |
|        | `213.29 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2209249_bytes`            |
|:-------|:-------------------------- |
|        | `216.72 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`            |
|:-------|:-------------------------- |
|        | `215.09 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2195272_bytes`            |
|:-------|:-------------------------- |
|        | `200.94 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2195269_bytes`            |
|:-------|:-------------------------- |
|        | `199.56 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2190807_bytes`            |
|:-------|:-------------------------- |
|        | `187.52 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2190806_bytes`            |
|:-------|:-------------------------- |
|        | `187.63 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-bandersnatch-msm-sw-10:default

|        | `2176868_bytes`            |
|:-------|:-------------------------- |
|        | `137.52 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-bandersnatch-msm-sw-1000:default

|        | `2175464_bytes`            |
|:-------|:-------------------------- |
|        | `137.17 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-bandersnatch-msm-te-10:default

|        | `2169415_bytes`            |
|:-------|:-------------------------- |
|        | `139.26 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-bandersnatch-msm-te-1000:default

|        | `2167209_bytes`            |
|:-------|:-------------------------- |
|        | `133.82 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-bandersnatch-mul-affine-sw:default

|        | `2157670_bytes`            |
|:-------|:-------------------------- |
|        | `130.46 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-bandersnatch-mul-affine-te:default

|        | `2151228_bytes`            |
|:-------|:-------------------------- |
|        | `128.45 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-bandersnatch-mul-projective-sw:default

|        | `2158557_bytes`            |
|:-------|:-------------------------- |
|        | `128.71 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-bandersnatch-mul-projective-te:default

|        | `2153123_bytes`            |
|:-------|:-------------------------- |
|        | `125.08 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2149769_bytes`            |
|:-------|:-------------------------- |
|        | `128.89 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2150824_bytes`            |
|:-------|:-------------------------- |
|        | `129.61 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2166339_bytes`            |
|:-------|:-------------------------- |
|        | `136.03 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2164131_bytes`            |
|:-------|:-------------------------- |
|        | `132.31 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2307742_bytes`           |
|:-------|:------------------------- |
|        | `69.25 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2289912_bytes`           |
|:-------|:------------------------- |
|        | `48.82 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2180522_bytes`           |
|:-------|:------------------------- |
|        | `45.23 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2179710_bytes`           |
|:-------|:------------------------- |
|        | `3.28 s` (✅ **1.00x**)    |

### exec:bls12-381-msm-g2-10:default

|        | `2184433_bytes`            |
|:-------|:-------------------------- |
|        | `275.01 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2183029_bytes`           |
|:-------|:------------------------- |
|        | `24.01 s` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g1:default

|        | `2164125_bytes`           |
|:-------|:------------------------- |
|        | `3.06 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g2:default

|        | `2174305_bytes`           |
|:-------|:------------------------- |
|        | `28.30 ms` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g1:default

|        | `2162833_bytes`           |
|:-------|:------------------------- |
|        | `3.17 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-projective-g2:default

|        | `2173405_bytes`           |
|:-------|:------------------------- |
|        | `28.87 ms` (✅ **1.00x**)  |

### exec:bls12-377-pairing:default

|        | `2362038_bytes`           |
|:-------|:------------------------- |
|        | `46.49 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-10:default

|        | `2180410_bytes`           |
|:-------|:------------------------- |
|        | `43.22 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-1000:default

|        | `2179593_bytes`           |
|:-------|:------------------------- |
|        | `3.16 s` (✅ **1.00x**)    |

### exec:bls12-377-msm-g2-10:default

|        | `2185770_bytes`            |
|:-------|:-------------------------- |
|        | `277.80 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g2-1000:default

|        | `2184363_bytes`           |
|:-------|:------------------------- |
|        | `23.64 s` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g1:default

|        | `2163991_bytes`           |
|:-------|:------------------------- |
|        | `3.24 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g2:default

|        | `2175594_bytes`           |
|:-------|:------------------------- |
|        | `24.72 ms` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g1:default

|        | `2162705_bytes`           |
|:-------|:------------------------- |
|        | `3.30 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-projective-g2:default

|        | `2174518_bytes`           |
|:-------|:------------------------- |
|        | `24.92 ms` (✅ **1.00x**)  |

### exec:bw6-761-pairing:default

|        | `2236113_bytes`            |
|:-------|:-------------------------- |
|        | `164.17 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-10:default

|        | `2209266_bytes`            |
|:-------|:-------------------------- |
|        | `379.41 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-1000:default

|        | `2207602_bytes`           |
|:-------|:------------------------- |
|        | `29.98 s` (✅ **1.00x**)   |

### exec:bw6-761-msm-g2-10:default

|        | `2209249_bytes`            |
|:-------|:-------------------------- |
|        | `346.63 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`           |
|:-------|:------------------------- |
|        | `30.05 s` (✅ **1.00x**)   |

### exec:bw6-761-mul-affine-g1:default

|        | `2195272_bytes`           |
|:-------|:------------------------- |
|        | `34.72 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g2:default

|        | `2195269_bytes`           |
|:-------|:------------------------- |
|        | `30.16 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g1:default

|        | `2190807_bytes`           |
|:-------|:------------------------- |
|        | `35.35 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g2:default

|        | `2190806_bytes`           |
|:-------|:------------------------- |
|        | `31.03 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-bandersnatch-msm-sw-10:default

|        | `2176868_bytes`           |
|:-------|:------------------------- |
|        | `13.32 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-bandersnatch-msm-sw-1000:default

|        | `2175464_bytes`            |
|:-------|:-------------------------- |
|        | `720.32 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-bandersnatch-msm-te-10:default

|        | `2169415_bytes`           |
|:-------|:------------------------- |
|        | `19.01 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-bandersnatch-msm-te-1000:default

|        | `2167209_bytes`            |
|:-------|:-------------------------- |
|        | `798.63 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-bandersnatch-mul-affine-sw:default

|        | `2157670_bytes`           |
|:-------|:------------------------- |
|        | `1.22 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-381-bandersnatch-mul-affine-te:default

|        | `2151228_bytes`            |
|:-------|:-------------------------- |
|        | `779.96 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-bandersnatch-mul-projective-sw:default

|        | `2158557_bytes`           |
|:-------|:------------------------- |
|        | `1.26 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-381-bandersnatch-mul-projective-te:default

|        | `2153123_bytes`            |
|:-------|:-------------------------- |
|        | `762.62 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-affine:default

|        | `2149769_bytes`           |
|:-------|:------------------------- |
|        | `1.07 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-377-mul-projective:default

|        | `2150824_bytes`           |
|:-------|:------------------------- |
|        | `1.04 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-377-msm-10:default

|        | `2166339_bytes`           |
|:-------|:------------------------- |
|        | `18.68 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-1000:default

|        | `2164131_bytes`            |
|:-------|:-------------------------- |
|        | `859.74 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

