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

|        | `2327243_bytes`            |
|:-------|:-------------------------- |
|        | `296.66 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2207671_bytes`            |
|:-------|:-------------------------- |
|        | `148.39 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2193459_bytes`            |
|:-------|:-------------------------- |
|        | `155.92 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2192638_bytes`            |
|:-------|:-------------------------- |
|        | `155.49 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2184437_bytes`            |
|:-------|:-------------------------- |
|        | `139.91 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2183034_bytes`            |
|:-------|:-------------------------- |
|        | `141.06 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2139407_bytes`            |
|:-------|:-------------------------- |
|        | `101.21 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2137607_bytes`           |
|:-------|:------------------------- |
|        | `92.90 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2139911_bytes`            |
|:-------|:-------------------------- |
|        | `101.13 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2137786_bytes`           |
|:-------|:------------------------- |
|        | `89.78 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2231604_bytes`            |
|:-------|:-------------------------- |
|        | `219.75 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2180758_bytes`            |
|:-------|:-------------------------- |
|        | `143.07 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2179357_bytes`            |
|:-------|:-------------------------- |
|        | `141.97 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2185760_bytes`            |
|:-------|:-------------------------- |
|        | `140.93 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2184358_bytes`            |
|:-------|:-------------------------- |
|        | `141.33 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2139314_bytes`            |
|:-------|:-------------------------- |
|        | `101.43 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2138368_bytes`           |
|:-------|:------------------------- |
|        | `92.28 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2139821_bytes`            |
|:-------|:-------------------------- |
|        | `102.19 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2138541_bytes`           |
|:-------|:------------------------- |
|        | `92.64 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2204841_bytes`            |
|:-------|:-------------------------- |
|        | `158.49 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2209257_bytes`            |
|:-------|:-------------------------- |
|        | `208.35 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2207594_bytes`            |
|:-------|:-------------------------- |
|        | `206.42 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `209.52 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`            |
|:-------|:-------------------------- |
|        | `204.93 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2165342_bytes`            |
|:-------|:-------------------------- |
|        | `135.12 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2165339_bytes`            |
|:-------|:-------------------------- |
|        | `134.42 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2165539_bytes`            |
|:-------|:-------------------------- |
|        | `135.72 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2165542_bytes`            |
|:-------|:-------------------------- |
|        | `135.10 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2176588_bytes`            |
|:-------|:-------------------------- |
|        | `132.09 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2175792_bytes`            |
|:-------|:-------------------------- |
|        | `131.73 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2165840_bytes`            |
|:-------|:-------------------------- |
|        | `129.70 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2164247_bytes`            |
|:-------|:-------------------------- |
|        | `127.32 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2137124_bytes`           |
|:-------|:------------------------- |
|        | `87.91 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2137127_bytes`           |
|:-------|:------------------------- |
|        | `89.24 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2137124_bytes`           |
|:-------|:------------------------- |
|        | `91.88 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2148055_bytes`            |
|:-------|:-------------------------- |
|        | `108.38 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2129469_bytes`           |
|:-------|:------------------------- |
|        | `88.31 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2148175_bytes`            |
|:-------|:-------------------------- |
|        | `109.01 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`            |
|:-------|:-------------------------- |
|        | `130.36 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2164136_bytes`            |
|:-------|:-------------------------- |
|        | `125.78 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2327243_bytes`           |
|:-------|:------------------------- |
|        | `62.31 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2207671_bytes`           |
|:-------|:------------------------- |
|        | `19.37 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2193459_bytes`           |
|:-------|:------------------------- |
|        | `39.89 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2192638_bytes`           |
|:-------|:------------------------- |
|        | `2.89 s` (✅ **1.00x**)    |

### exec:bls12-381-msm-g2-10:default

|        | `2184437_bytes`            |
|:-------|:-------------------------- |
|        | `253.73 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2183034_bytes`           |
|:-------|:------------------------- |
|        | `22.35 s` (✅ **1.00x**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

