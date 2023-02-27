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
|        | `364.94 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2115526_bytes`            |
|:-------|:-------------------------- |
|        | `232.10 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2057684_bytes`            |
|:-------|:-------------------------- |
|        | `159.29 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2057689_bytes`            |
|:-------|:-------------------------- |
|        | `158.87 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2052877_bytes`            |
|:-------|:-------------------------- |
|        | `137.61 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2052882_bytes`            |
|:-------|:-------------------------- |
|        | `136.14 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2026347_bytes`            |
|:-------|:-------------------------- |
|        | `134.33 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2024570_bytes`            |
|:-------|:-------------------------- |
|        | `116.86 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2026854_bytes`            |
|:-------|:-------------------------- |
|        | `134.17 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2024738_bytes`            |
|:-------|:-------------------------- |
|        | `121.68 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2146613_bytes`            |
|:-------|:-------------------------- |
|        | `329.82 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2057654_bytes`            |
|:-------|:-------------------------- |
|        | `163.79 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2057659_bytes`            |
|:-------|:-------------------------- |
|        | `158.54 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2053801_bytes`            |
|:-------|:-------------------------- |
|        | `136.37 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2053806_bytes`            |
|:-------|:-------------------------- |
|        | `141.84 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2026248_bytes`            |
|:-------|:-------------------------- |
|        | `138.12 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2025398_bytes`            |
|:-------|:-------------------------- |
|        | `118.71 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2026749_bytes`            |
|:-------|:-------------------------- |
|        | `135.94 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2025572_bytes`            |
|:-------|:-------------------------- |
|        | `117.04 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2115159_bytes`            |
|:-------|:-------------------------- |
|        | `189.00 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2082916_bytes`            |
|:-------|:-------------------------- |
|        | `179.21 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2082922_bytes`            |
|:-------|:-------------------------- |
|        | `176.55 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2082920_bytes`            |
|:-------|:-------------------------- |
|        | `180.36 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2082921_bytes`            |
|:-------|:-------------------------- |
|        | `175.76 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2052264_bytes`            |
|:-------|:-------------------------- |
|        | `142.87 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2052264_bytes`            |
|:-------|:-------------------------- |
|        | `148.09 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2052467_bytes`            |
|:-------|:-------------------------- |
|        | `145.43 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2052467_bytes`            |
|:-------|:-------------------------- |
|        | `143.74 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2057226_bytes`            |
|:-------|:-------------------------- |
|        | `140.95 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2057231_bytes`            |
|:-------|:-------------------------- |
|        | `145.89 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2046406_bytes`            |
|:-------|:-------------------------- |
|        | `135.20 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2046407_bytes`            |
|:-------|:-------------------------- |
|        | `133.05 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2024113_bytes`            |
|:-------|:-------------------------- |
|        | `122.23 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2024116_bytes`            |
|:-------|:-------------------------- |
|        | `122.86 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2024113_bytes`            |
|:-------|:-------------------------- |
|        | `125.66 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2028300_bytes`            |
|:-------|:-------------------------- |
|        | `123.88 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2045524_bytes`            |
|:-------|:-------------------------- |
|        | `139.63 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2028426_bytes`            |
|:-------|:-------------------------- |
|        | `124.41 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2046587_bytes`            |
|:-------|:-------------------------- |
|        | `136.40 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2046592_bytes`            |
|:-------|:-------------------------- |
|        | `139.34 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2209532_bytes`           |
|:-------|:------------------------- |
|        | `69.34 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2115526_bytes`           |
|:-------|:------------------------- |
|        | `21.08 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2057684_bytes`            |
|:-------|:-------------------------- |
|        | `685.59 us` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2057689_bytes`           |
|:-------|:------------------------- |
|        | `19.24 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-10:default

|        | `2052877_bytes`           |
|:-------|:------------------------- |
|        | `1.37 ms` (✅ **1.00x**)   |

### exec:bls12-381-msm-g2-1000:default

|        | `2052882_bytes`           |
|:-------|:------------------------- |
|        | `47.50 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

