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

|        | `2209532 bytes`            |
|:-------|:-------------------------- |
|        | `159.68 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2115526 bytes`            |
|:-------|:-------------------------- |
|        | `111.25 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2057684 bytes`           |
|:-------|:------------------------- |
|        | `99.57 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2057689 bytes`           |
|:-------|:------------------------- |
|        | `99.57 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2052877 bytes`           |
|:-------|:------------------------- |
|        | `83.85 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2052882 bytes`           |
|:-------|:------------------------- |
|        | `90.63 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2026347 bytes`           |
|:-------|:------------------------- |
|        | `69.22 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2024570 bytes`           |
|:-------|:------------------------- |
|        | `71.22 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2026854 bytes`           |
|:-------|:------------------------- |
|        | `95.75 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2024738 bytes`           |
|:-------|:------------------------- |
|        | `52.42 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2146613 bytes`            |
|:-------|:-------------------------- |
|        | `180.36 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2057654 bytes`           |
|:-------|:------------------------- |
|        | `99.58 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2057659 bytes`           |
|:-------|:------------------------- |
|        | `92.79 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2053801 bytes`           |
|:-------|:------------------------- |
|        | `96.20 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2053806 bytes`           |
|:-------|:------------------------- |
|        | `73.35 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2026248 bytes`           |
|:-------|:------------------------- |
|        | `98.05 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2025398 bytes`           |
|:-------|:------------------------- |
|        | `54.51 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2026749 bytes`           |
|:-------|:------------------------- |
|        | `74.75 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2025572 bytes`           |
|:-------|:------------------------- |
|        | `63.43 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2115159 bytes`            |
|:-------|:-------------------------- |
|        | `110.57 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2082916 bytes`            |
|:-------|:-------------------------- |
|        | `119.90 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2082922 bytes`            |
|:-------|:-------------------------- |
|        | `123.93 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2082920 bytes`            |
|:-------|:-------------------------- |
|        | `113.83 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2082921 bytes`            |
|:-------|:-------------------------- |
|        | `123.48 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2052264 bytes`            |
|:-------|:-------------------------- |
|        | `146.33 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2052264 bytes`            |
|:-------|:-------------------------- |
|        | `130.98 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2052467 bytes`            |
|:-------|:-------------------------- |
|        | `114.55 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2052467 bytes`            |
|:-------|:-------------------------- |
|        | `132.90 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2057226 bytes`           |
|:-------|:------------------------- |
|        | `71.59 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2057231 bytes`           |
|:-------|:------------------------- |
|        | `66.03 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2046406 bytes`           |
|:-------|:------------------------- |
|        | `60.49 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2046407 bytes`           |
|:-------|:------------------------- |
|        | `59.55 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2024113 bytes`           |
|:-------|:------------------------- |
|        | `49.54 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2024116 bytes`           |
|:-------|:------------------------- |
|        | `49.44 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2024113 bytes`           |
|:-------|:------------------------- |
|        | `49.17 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2028300 bytes`           |
|:-------|:------------------------- |
|        | `49.91 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2045524 bytes`           |
|:-------|:------------------------- |
|        | `94.62 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2028426 bytes`           |
|:-------|:------------------------- |
|        | `67.80 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2046587 bytes`           |
|:-------|:------------------------- |
|        | `81.58 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2046592 bytes`           |
|:-------|:------------------------- |
|        | `65.27 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2209532 bytes`           |
|:-------|:------------------------- |
|        | `50.91 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2115526 bytes`           |
|:-------|:------------------------- |
|        | `15.99 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2057684 bytes`            |
|:-------|:-------------------------- |
|        | `445.30 us` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2057689 bytes`           |
|:-------|:------------------------- |
|        | `13.58 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-10:default

|        | `2052877 bytes`            |
|:-------|:-------------------------- |
|        | `985.99 us` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2052882 bytes`           |
|:-------|:------------------------- |
|        | `31.97 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

