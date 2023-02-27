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
|        | `279.06 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2115526_bytes`            |
|:-------|:-------------------------- |
|        | `173.87 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2057684_bytes`            |
|:-------|:-------------------------- |
|        | `120.96 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2057689_bytes`            |
|:-------|:-------------------------- |
|        | `120.96 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2052877_bytes`            |
|:-------|:-------------------------- |
|        | `103.74 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2052882_bytes`            |
|:-------|:-------------------------- |
|        | `103.86 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2026347_bytes`            |
|:-------|:-------------------------- |
|        | `105.65 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2024570_bytes`           |
|:-------|:------------------------- |
|        | `90.94 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2026854_bytes`            |
|:-------|:-------------------------- |
|        | `107.27 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2024738_bytes`           |
|:-------|:------------------------- |
|        | `91.86 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2146613_bytes`            |
|:-------|:-------------------------- |
|        | `260.97 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2057654_bytes`            |
|:-------|:-------------------------- |
|        | `110.56 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2057659_bytes`            |
|:-------|:-------------------------- |
|        | `106.97 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2053801_bytes`            |
|:-------|:-------------------------- |
|        | `106.35 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2053806_bytes`            |
|:-------|:-------------------------- |
|        | `105.84 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2026248_bytes`            |
|:-------|:-------------------------- |
|        | `105.04 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2025398_bytes`           |
|:-------|:------------------------- |
|        | `94.08 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2026749_bytes`            |
|:-------|:-------------------------- |
|        | `105.92 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2025572_bytes`           |
|:-------|:------------------------- |
|        | `92.98 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2115159_bytes`            |
|:-------|:-------------------------- |
|        | `149.37 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2082916_bytes`            |
|:-------|:-------------------------- |
|        | `138.89 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2082922_bytes`            |
|:-------|:-------------------------- |
|        | `139.39 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2082920_bytes`            |
|:-------|:-------------------------- |
|        | `138.99 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2082921_bytes`            |
|:-------|:-------------------------- |
|        | `138.95 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2052264_bytes`            |
|:-------|:-------------------------- |
|        | `114.00 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2052264_bytes`            |
|:-------|:-------------------------- |
|        | `113.54 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2052467_bytes`            |
|:-------|:-------------------------- |
|        | `113.76 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2052467_bytes`            |
|:-------|:-------------------------- |
|        | `113.50 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2057226_bytes`            |
|:-------|:-------------------------- |
|        | `110.29 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2057231_bytes`            |
|:-------|:-------------------------- |
|        | `111.32 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2046406_bytes`            |
|:-------|:-------------------------- |
|        | `104.18 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2046407_bytes`            |
|:-------|:-------------------------- |
|        | `104.29 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2024113_bytes`           |
|:-------|:------------------------- |
|        | `95.13 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `95.36 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2024113_bytes`           |
|:-------|:------------------------- |
|        | `95.64 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2028300_bytes`           |
|:-------|:------------------------- |
|        | `93.66 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2045524_bytes`            |
|:-------|:-------------------------- |
|        | `101.99 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2028426_bytes`           |
|:-------|:------------------------- |
|        | `93.82 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2046587_bytes`            |
|:-------|:-------------------------- |
|        | `103.51 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2046592_bytes`            |
|:-------|:-------------------------- |
|        | `103.87 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2209532_bytes`           |
|:-------|:------------------------- |
|        | `52.28 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2115526_bytes`           |
|:-------|:------------------------- |
|        | `16.19 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2057684_bytes`            |
|:-------|:-------------------------- |
|        | `581.70 us` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2057689_bytes`           |
|:-------|:------------------------- |
|        | `14.95 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-10:default

|        | `2052877_bytes`           |
|:-------|:------------------------- |
|        | `1.10 ms` (✅ **1.00x**)   |

### exec:bls12-381-msm-g2-1000:default

|        | `2052882_bytes`           |
|:-------|:------------------------- |
|        | `37.76 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

