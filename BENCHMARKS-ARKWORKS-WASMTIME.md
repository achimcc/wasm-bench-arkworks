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
|        | `286.95 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2115526_bytes`            |
|:-------|:-------------------------- |
|        | `179.63 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2057684_bytes`            |
|:-------|:-------------------------- |
|        | `126.39 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2057689_bytes`            |
|:-------|:-------------------------- |
|        | `126.12 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2052877_bytes`            |
|:-------|:-------------------------- |
|        | `108.43 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2052882_bytes`            |
|:-------|:-------------------------- |
|        | `108.39 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2026347_bytes`            |
|:-------|:-------------------------- |
|        | `109.51 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2024570_bytes`           |
|:-------|:------------------------- |
|        | `95.19 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2026854_bytes`            |
|:-------|:-------------------------- |
|        | `109.92 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2024738_bytes`           |
|:-------|:------------------------- |
|        | `95.43 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2146613_bytes`            |
|:-------|:-------------------------- |
|        | `261.40 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2057654_bytes`            |
|:-------|:-------------------------- |
|        | `125.55 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2057659_bytes`            |
|:-------|:-------------------------- |
|        | `125.54 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2053801_bytes`            |
|:-------|:-------------------------- |
|        | `110.41 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2053806_bytes`            |
|:-------|:-------------------------- |
|        | `112.19 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2026248_bytes`            |
|:-------|:-------------------------- |
|        | `109.83 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2025398_bytes`            |
|:-------|:-------------------------- |
|        | `100.43 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2026749_bytes`            |
|:-------|:-------------------------- |
|        | `111.20 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2025572_bytes`           |
|:-------|:------------------------- |
|        | `97.50 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2115159_bytes`            |
|:-------|:-------------------------- |
|        | `154.38 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2082916_bytes`            |
|:-------|:-------------------------- |
|        | `146.25 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2082922_bytes`            |
|:-------|:-------------------------- |
|        | `146.48 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2082920_bytes`            |
|:-------|:-------------------------- |
|        | `147.34 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2082921_bytes`            |
|:-------|:-------------------------- |
|        | `146.94 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2052264_bytes`            |
|:-------|:-------------------------- |
|        | `119.29 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2052264_bytes`            |
|:-------|:-------------------------- |
|        | `119.39 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2052467_bytes`            |
|:-------|:-------------------------- |
|        | `118.73 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2052467_bytes`            |
|:-------|:-------------------------- |
|        | `121.36 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2057226_bytes`            |
|:-------|:-------------------------- |
|        | `114.37 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2057231_bytes`            |
|:-------|:-------------------------- |
|        | `114.24 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2046406_bytes`            |
|:-------|:-------------------------- |
|        | `108.30 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2046407_bytes`            |
|:-------|:-------------------------- |
|        | `107.90 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2024113_bytes`           |
|:-------|:------------------------- |
|        | `98.78 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2024116_bytes`            |
|:-------|:-------------------------- |
|        | `100.28 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2024113_bytes`           |
|:-------|:------------------------- |
|        | `99.34 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2028300_bytes`           |
|:-------|:------------------------- |
|        | `97.74 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2045524_bytes`            |
|:-------|:-------------------------- |
|        | `105.84 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2028426_bytes`           |
|:-------|:------------------------- |
|        | `97.37 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2046587_bytes`            |
|:-------|:-------------------------- |
|        | `108.09 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2046592_bytes`            |
|:-------|:-------------------------- |
|        | `109.34 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2209532_bytes`           |
|:-------|:------------------------- |
|        | `56.67 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2115526_bytes`           |
|:-------|:------------------------- |
|        | `17.22 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2057684_bytes`            |
|:-------|:-------------------------- |
|        | `483.26 us` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2057689_bytes`           |
|:-------|:------------------------- |
|        | `15.12 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-10:default

|        | `2052877_bytes`           |
|:-------|:------------------------- |
|        | `1.07 ms` (✅ **1.00x**)   |

### exec:bls12-381-msm-g2-1000:default

|        | `2052882_bytes`           |
|:-------|:------------------------- |
|        | `39.43 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

