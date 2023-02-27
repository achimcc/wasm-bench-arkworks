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

## Benchmark Results

### compile:groth16:default

|        | `2209532_bytes`            |
|:-------|:-------------------------- |
|        | `323.63 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2115526_bytes`            |
|:-------|:-------------------------- |
|        | `220.86 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2057684_bytes`            |
|:-------|:-------------------------- |
|        | `154.48 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2057689_bytes`            |
|:-------|:-------------------------- |
|        | `146.82 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2052877_bytes`            |
|:-------|:-------------------------- |
|        | `115.31 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2052882_bytes`            |
|:-------|:-------------------------- |
|        | `126.40 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2026347_bytes`            |
|:-------|:-------------------------- |
|        | `133.71 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2024570_bytes`            |
|:-------|:-------------------------- |
|        | `117.18 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2026854_bytes`            |
|:-------|:-------------------------- |
|        | `122.09 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2024738_bytes`            |
|:-------|:-------------------------- |
|        | `102.51 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2146613_bytes`            |
|:-------|:-------------------------- |
|        | `287.63 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2057654_bytes`            |
|:-------|:-------------------------- |
|        | `138.38 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2057659_bytes`            |
|:-------|:-------------------------- |
|        | `132.68 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2053801_bytes`            |
|:-------|:-------------------------- |
|        | `125.80 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2053806_bytes`            |
|:-------|:-------------------------- |
|        | `120.49 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2026248_bytes`            |
|:-------|:-------------------------- |
|        | `119.88 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2025398_bytes`            |
|:-------|:-------------------------- |
|        | `107.51 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2026749_bytes`            |
|:-------|:-------------------------- |
|        | `129.80 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2025572_bytes`            |
|:-------|:-------------------------- |
|        | `116.47 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2115159_bytes`            |
|:-------|:-------------------------- |
|        | `180.98 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2082916_bytes`            |
|:-------|:-------------------------- |
|        | `174.16 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2082922_bytes`            |
|:-------|:-------------------------- |
|        | `154.12 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2082920_bytes`            |
|:-------|:-------------------------- |
|        | `154.33 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2082921_bytes`            |
|:-------|:-------------------------- |
|        | `171.68 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2052264_bytes`            |
|:-------|:-------------------------- |
|        | `139.70 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2052264_bytes`            |
|:-------|:-------------------------- |
|        | `143.97 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2052467_bytes`            |
|:-------|:-------------------------- |
|        | `144.96 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2052467_bytes`            |
|:-------|:-------------------------- |
|        | `138.93 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2057226_bytes`            |
|:-------|:-------------------------- |
|        | `124.77 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2057231_bytes`            |
|:-------|:-------------------------- |
|        | `119.16 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2046406_bytes`            |
|:-------|:-------------------------- |
|        | `115.74 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2046407_bytes`            |
|:-------|:-------------------------- |
|        | `129.85 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2024113_bytes`            |
|:-------|:-------------------------- |
|        | `116.34 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2024116_bytes`            |
|:-------|:-------------------------- |
|        | `118.63 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2024113_bytes`            |
|:-------|:-------------------------- |
|        | `120.24 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2028300_bytes`            |
|:-------|:-------------------------- |
|        | `108.61 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2045524_bytes`            |
|:-------|:-------------------------- |
|        | `120.86 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2028426_bytes`            |
|:-------|:-------------------------- |
|        | `111.66 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2046587_bytes`            |
|:-------|:-------------------------- |
|        | `129.44 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2046592_bytes`            |
|:-------|:-------------------------- |
|        | `131.36 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2209532_bytes`           |
|:-------|:------------------------- |
|        | `66.49 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2115526_bytes`           |
|:-------|:------------------------- |
|        | `21.11 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2057684_bytes`            |
|:-------|:-------------------------- |
|        | `572.30 us` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2057689_bytes`           |
|:-------|:------------------------- |
|        | `17.11 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-10:default

|        | `2052877_bytes`           |
|:-------|:------------------------- |
|        | `1.31 ms` (✅ **1.00x**)   |

### exec:bls12-381-msm-g2-1000:default

|        | `2052882_bytes`           |
|:-------|:------------------------- |
|        | `48.83 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

