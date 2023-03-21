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

|        | `2327243_bytes`            |
|:-------|:-------------------------- |
|        | `128.26 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2207671_bytes`           |
|:-------|:------------------------- |
|        | `74.52 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2193459_bytes`           |
|:-------|:------------------------- |
|        | `73.83 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2192638_bytes`           |
|:-------|:------------------------- |
|        | `73.43 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2184437_bytes`           |
|:-------|:------------------------- |
|        | `67.32 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2183034_bytes`           |
|:-------|:------------------------- |
|        | `66.62 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2139407_bytes`           |
|:-------|:------------------------- |
|        | `46.61 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2137607_bytes`           |
|:-------|:------------------------- |
|        | `44.00 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2139911_bytes`           |
|:-------|:------------------------- |
|        | `55.62 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2137786_bytes`           |
|:-------|:------------------------- |
|        | `42.66 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2231604_bytes`           |
|:-------|:------------------------- |
|        | `93.18 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2180758_bytes`           |
|:-------|:------------------------- |
|        | `71.35 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2179357_bytes`           |
|:-------|:------------------------- |
|        | `76.95 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2185760_bytes`           |
|:-------|:------------------------- |
|        | `68.51 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2184358_bytes`           |
|:-------|:------------------------- |
|        | `91.55 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2139314_bytes`           |
|:-------|:------------------------- |
|        | `62.44 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2138368_bytes`           |
|:-------|:------------------------- |
|        | `48.49 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2139821_bytes`           |
|:-------|:------------------------- |
|        | `54.23 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2138541_bytes`           |
|:-------|:------------------------- |
|        | `42.54 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2204841_bytes`           |
|:-------|:------------------------- |
|        | `98.79 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2209257_bytes`           |
|:-------|:------------------------- |
|        | `92.07 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2207594_bytes`            |
|:-------|:-------------------------- |
|        | `121.00 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `119.68 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`            |
|:-------|:-------------------------- |
|        | `119.18 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2165342_bytes`           |
|:-------|:------------------------- |
|        | `96.29 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2165339_bytes`           |
|:-------|:------------------------- |
|        | `78.41 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2165539_bytes`           |
|:-------|:------------------------- |
|        | `97.96 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2165542_bytes`           |
|:-------|:------------------------- |
|        | `98.47 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2176588_bytes`           |
|:-------|:------------------------- |
|        | `62.11 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2175792_bytes`           |
|:-------|:------------------------- |
|        | `77.90 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2165840_bytes`           |
|:-------|:------------------------- |
|        | `87.04 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2164247_bytes`           |
|:-------|:------------------------- |
|        | `81.22 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2137124_bytes`           |
|:-------|:------------------------- |
|        | `59.67 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2137127_bytes`           |
|:-------|:------------------------- |
|        | `56.46 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2137124_bytes`           |
|:-------|:------------------------- |
|        | `45.35 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2148055_bytes`           |
|:-------|:------------------------- |
|        | `46.58 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2129469_bytes`           |
|:-------|:------------------------- |
|        | `39.70 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2148175_bytes`           |
|:-------|:------------------------- |
|        | `63.86 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`           |
|:-------|:------------------------- |
|        | `83.34 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2164136_bytes`           |
|:-------|:------------------------- |
|        | `81.61 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2327243_bytes`           |
|:-------|:------------------------- |
|        | `45.07 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2207671_bytes`           |
|:-------|:------------------------- |
|        | `14.14 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2193459_bytes`           |
|:-------|:------------------------- |
|        | `24.65 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2192638_bytes`           |
|:-------|:------------------------- |
|        | `1.91 s` (✅ **1.00x**)    |

### exec:bls12-381-msm-g2-10:default

|        | `2184437_bytes`            |
|:-------|:-------------------------- |
|        | `185.24 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2183034_bytes`           |
|:-------|:------------------------- |
|        | `14.85 s` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g1:default

|        | `2139407_bytes`           |
|:-------|:------------------------- |
|        | `20.28 us` (✅ **1.00x**)  |

### exec:bls12-381-mul-affine-g2:default

|        | `2137607_bytes`           |
|:-------|:------------------------- |
|        | `27.85 us` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g1:default

|        | `2139911_bytes`           |
|:-------|:------------------------- |
|        | `19.85 us` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g2:default

|        | `2137786_bytes`           |
|:-------|:------------------------- |
|        | `34.40 us` (✅ **1.00x**)  |

### exec:bls12-377-pairing:default

|        | `2231604_bytes`           |
|:-------|:------------------------- |
|        | `15.16 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-10:default

|        | `2180758_bytes`           |
|:-------|:------------------------- |
|        | `28.62 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-1000:default

|        | `2179357_bytes`           |
|:-------|:------------------------- |
|        | `1.92 s` (✅ **1.00x**)    |

### exec:bls12-377-msm-g2-10:default

|        | `2185760_bytes`            |
|:-------|:-------------------------- |
|        | `162.87 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g2-1000:default

|        | `2184358_bytes`           |
|:-------|:------------------------- |
|        | `14.57 s` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g1:default

|        | `2139314_bytes`           |
|:-------|:------------------------- |
|        | `24.49 us` (✅ **1.00x**)  |

### exec:bls12-377-mul-affine-g2:default

|        | `2138368_bytes`           |
|:-------|:------------------------- |
|        | `38.94 us` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g1:default

|        | `2139821_bytes`           |
|:-------|:------------------------- |
|        | `19.38 us` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g2:default

|        | `2138541_bytes`           |
|:-------|:------------------------- |
|        | `28.26 us` (✅ **1.00x**)  |

### exec:bw6-761-pairing:default

|        | `2204841_bytes`           |
|:-------|:------------------------- |
|        | `55.44 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-10:default

|        | `2209257_bytes`            |
|:-------|:-------------------------- |
|        | `206.61 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-1000:default

|        | `2207594_bytes`           |
|:-------|:------------------------- |
|        | `18.01 s` (✅ **1.00x**)   |

### exec:bw6-761-msm-g2-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `212.28 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`           |
|:-------|:------------------------- |
|        | `18.02 s` (✅ **1.00x**)   |

### exec:bw6-761-mul-affine-g1:default

|        | `2165342_bytes`           |
|:-------|:------------------------- |
|        | `35.64 us` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g2:default

|        | `2165339_bytes`           |
|:-------|:------------------------- |
|        | `34.68 us` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g1:default

|        | `2165539_bytes`           |
|:-------|:------------------------- |
|        | `34.82 us` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g2:default

|        | `2165542_bytes`           |
|:-------|:------------------------- |
|        | `35.42 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-10:default

|        | `2176588_bytes`           |
|:-------|:------------------------- |
|        | `8.61 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-381-msm-sw-1000:default

|        | `2175792_bytes`            |
|:-------|:-------------------------- |
|        | `430.70 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-10:default

|        | `2165840_bytes`           |
|:-------|:------------------------- |
|        | `12.47 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-1000:default

|        | `2164247_bytes`            |
|:-------|:-------------------------- |
|        | `533.49 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-sw:default

|        | `2137124_bytes`           |
|:-------|:------------------------- |
|        | `17.77 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-te:default

|        | `2137127_bytes`           |
|:-------|:------------------------- |
|        | `17.25 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-sw:default

|        | `2137124_bytes`           |
|:-------|:------------------------- |
|        | `17.11 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-te:default

|        | `2148055_bytes`           |
|:-------|:------------------------- |
|        | `22.37 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-affine:default

|        | `2129469_bytes`           |
|:-------|:------------------------- |
|        | `22.34 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-projective:default

|        | `2148175_bytes`           |
|:-------|:------------------------- |
|        | `22.48 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-10:default

|        | `2166342_bytes`           |
|:-------|:------------------------- |
|        | `10.06 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-1000:default

|        | `2164136_bytes`            |
|:-------|:-------------------------- |
|        | `537.81 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

