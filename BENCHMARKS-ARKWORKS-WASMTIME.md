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

|        | `2327248_bytes`            |
|:-------|:-------------------------- |
|        | `249.12 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2290079_bytes`            |
|:-------|:-------------------------- |
|        | `240.99 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2180526_bytes`            |
|:-------|:-------------------------- |
|        | `120.13 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2179710_bytes`            |
|:-------|:-------------------------- |
|        | `118.53 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2184433_bytes`            |
|:-------|:-------------------------- |
|        | `118.79 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2183033_bytes`            |
|:-------|:-------------------------- |
|        | `116.93 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2164125_bytes`            |
|:-------|:-------------------------- |
|        | `105.61 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2174305_bytes`            |
|:-------|:-------------------------- |
|        | `110.60 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2162832_bytes`            |
|:-------|:-------------------------- |
|        | `104.95 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2173411_bytes`            |
|:-------|:-------------------------- |
|        | `109.60 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2361861_bytes`            |
|:-------|:-------------------------- |
|        | `360.25 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2180414_bytes`            |
|:-------|:-------------------------- |
|        | `118.94 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2179596_bytes`            |
|:-------|:-------------------------- |
|        | `117.60 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2185765_bytes`            |
|:-------|:-------------------------- |
|        | `118.66 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2184361_bytes`            |
|:-------|:-------------------------- |
|        | `117.96 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2163991_bytes`            |
|:-------|:-------------------------- |
|        | `104.66 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2175595_bytes`            |
|:-------|:-------------------------- |
|        | `113.57 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2162703_bytes`            |
|:-------|:-------------------------- |
|        | `104.37 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2174518_bytes`            |
|:-------|:-------------------------- |
|        | `110.54 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2236239_bytes`            |
|:-------|:-------------------------- |
|        | `184.28 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `176.72 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`            |
|:-------|:-------------------------- |
|        | `174.05 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `177.49 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`            |
|:-------|:-------------------------- |
|        | `173.29 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2195270_bytes`            |
|:-------|:-------------------------- |
|        | `163.49 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2195273_bytes`            |
|:-------|:-------------------------- |
|        | `162.79 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2190797_bytes`            |
|:-------|:-------------------------- |
|        | `152.38 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2190804_bytes`            |
|:-------|:-------------------------- |
|        | `151.81 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2176873_bytes`            |
|:-------|:-------------------------- |
|        | `110.36 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2175473_bytes`            |
|:-------|:-------------------------- |
|        | `110.14 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2166118_bytes`            |
|:-------|:-------------------------- |
|        | `108.48 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2163912_bytes`            |
|:-------|:-------------------------- |
|        | `105.97 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2157753_bytes`            |
|:-------|:-------------------------- |
|        | `107.23 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2149587_bytes`            |
|:-------|:-------------------------- |
|        | `104.30 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2158554_bytes`            |
|:-------|:-------------------------- |
|        | `103.01 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2150637_bytes`            |
|:-------|:-------------------------- |
|        | `107.04 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2149766_bytes`            |
|:-------|:-------------------------- |
|        | `103.39 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2150821_bytes`            |
|:-------|:-------------------------- |
|        | `107.09 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2166337_bytes`            |
|:-------|:-------------------------- |
|        | `108.85 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2164132_bytes`            |
|:-------|:-------------------------- |
|        | `106.98 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2327248_bytes`           |
|:-------|:------------------------- |
|        | `52.08 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2290079_bytes`           |
|:-------|:------------------------- |
|        | `37.48 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2180526_bytes`           |
|:-------|:------------------------- |
|        | `33.37 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2179710_bytes`           |
|:-------|:------------------------- |
|        | `2.42 s` (✅ **1.00x**)    |

### exec:bls12-381-msm-g2-10:default

|        | `2184433_bytes`            |
|:-------|:-------------------------- |
|        | `210.30 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2183033_bytes`           |
|:-------|:------------------------- |
|        | `18.48 s` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g1:default

|        | `2164125_bytes`           |
|:-------|:------------------------- |
|        | `2.24 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g2:default

|        | `2174305_bytes`           |
|:-------|:------------------------- |
|        | `21.54 ms` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g1:default

|        | `2162832_bytes`           |
|:-------|:------------------------- |
|        | `2.33 ms` (✅ **1.00x**)   |

### exec:bls12-381-mul-projective-g2:default

|        | `2173411_bytes`           |
|:-------|:------------------------- |
|        | `21.91 ms` (✅ **1.00x**)  |

### exec:bls12-377-pairing:default

|        | `2361861_bytes`           |
|:-------|:------------------------- |
|        | `35.50 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-10:default

|        | `2180414_bytes`           |
|:-------|:------------------------- |
|        | `32.25 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-1000:default

|        | `2179596_bytes`           |
|:-------|:------------------------- |
|        | `2.37 s` (✅ **1.00x**)    |

### exec:bls12-377-msm-g2-10:default

|        | `2185765_bytes`            |
|:-------|:-------------------------- |
|        | `211.57 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g2-1000:default

|        | `2184361_bytes`           |
|:-------|:------------------------- |
|        | `18.00 s` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g1:default

|        | `2163991_bytes`           |
|:-------|:------------------------- |
|        | `2.36 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-affine-g2:default

|        | `2175595_bytes`           |
|:-------|:------------------------- |
|        | `18.69 ms` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g1:default

|        | `2162703_bytes`           |
|:-------|:------------------------- |
|        | `2.44 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-projective-g2:default

|        | `2174518_bytes`           |
|:-------|:------------------------- |
|        | `19.11 ms` (✅ **1.00x**)  |

### exec:bw6-761-pairing:default

|        | `2236239_bytes`            |
|:-------|:-------------------------- |
|        | `121.38 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `275.35 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`           |
|:-------|:------------------------- |
|        | `21.97 s` (✅ **1.00x**)   |

### exec:bw6-761-msm-g2-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `250.66 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`           |
|:-------|:------------------------- |
|        | `22.02 s` (✅ **1.00x**)   |

### exec:bw6-761-mul-affine-g1:default

|        | `2195270_bytes`           |
|:-------|:------------------------- |
|        | `25.43 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g2:default

|        | `2195273_bytes`           |
|:-------|:------------------------- |
|        | `22.32 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g1:default

|        | `2190797_bytes`           |
|:-------|:------------------------- |
|        | `25.69 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g2:default

|        | `2190804_bytes`           |
|:-------|:------------------------- |
|        | `22.60 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-10:default

|        | `2176873_bytes`           |
|:-------|:------------------------- |
|        | `11.99 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-1000:default

|        | `2175473_bytes`            |
|:-------|:-------------------------- |
|        | `540.46 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-10:default

|        | `2166118_bytes`           |
|:-------|:------------------------- |
|        | `14.28 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-1000:default

|        | `2163912_bytes`            |
|:-------|:-------------------------- |
|        | `619.37 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-sw:default

|        | `2157753_bytes`            |
|:-------|:-------------------------- |
|        | `622.47 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-te:default

|        | `2149587_bytes`            |
|:-------|:-------------------------- |
|        | `630.02 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-sw:default

|        | `2158554_bytes`            |
|:-------|:-------------------------- |
|        | `664.97 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-te:default

|        | `2150637_bytes`            |
|:-------|:-------------------------- |
|        | `613.99 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-affine:default

|        | `2149766_bytes`            |
|:-------|:-------------------------- |
|        | `802.13 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-projective:default

|        | `2150821_bytes`            |
|:-------|:-------------------------- |
|        | `783.43 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-10:default

|        | `2166337_bytes`           |
|:-------|:------------------------- |
|        | `14.23 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-1000:default

|        | `2164132_bytes`            |
|:-------|:-------------------------- |
|        | `663.00 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

