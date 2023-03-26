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
|        | `245.66 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2290079_bytes`            |
|:-------|:-------------------------- |
|        | `237.42 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2180526_bytes`            |
|:-------|:-------------------------- |
|        | `116.44 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2179710_bytes`            |
|:-------|:-------------------------- |
|        | `116.58 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2184433_bytes`            |
|:-------|:-------------------------- |
|        | `114.31 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2183033_bytes`            |
|:-------|:-------------------------- |
|        | `114.34 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2164125_bytes`            |
|:-------|:-------------------------- |
|        | `105.09 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2174305_bytes`            |
|:-------|:-------------------------- |
|        | `110.37 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2162832_bytes`            |
|:-------|:-------------------------- |
|        | `101.00 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2173411_bytes`            |
|:-------|:-------------------------- |
|        | `108.83 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2361861_bytes`            |
|:-------|:-------------------------- |
|        | `352.06 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2180414_bytes`            |
|:-------|:-------------------------- |
|        | `117.57 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2179596_bytes`            |
|:-------|:-------------------------- |
|        | `116.69 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2185765_bytes`            |
|:-------|:-------------------------- |
|        | `117.42 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2184361_bytes`            |
|:-------|:-------------------------- |
|        | `117.62 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2163991_bytes`            |
|:-------|:-------------------------- |
|        | `103.82 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2175595_bytes`            |
|:-------|:-------------------------- |
|        | `111.36 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2162703_bytes`            |
|:-------|:-------------------------- |
|        | `101.73 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2174518_bytes`            |
|:-------|:-------------------------- |
|        | `109.95 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2236239_bytes`            |
|:-------|:-------------------------- |
|        | `181.03 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `172.78 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2207601_bytes`            |
|:-------|:-------------------------- |
|        | `169.93 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `172.64 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2207598_bytes`            |
|:-------|:-------------------------- |
|        | `170.24 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2195270_bytes`            |
|:-------|:-------------------------- |
|        | `159.62 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2195273_bytes`            |
|:-------|:-------------------------- |
|        | `158.64 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2190797_bytes`            |
|:-------|:-------------------------- |
|        | `148.62 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2190804_bytes`            |
|:-------|:-------------------------- |
|        | `149.39 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2176873_bytes`            |
|:-------|:-------------------------- |
|        | `110.69 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2175473_bytes`            |
|:-------|:-------------------------- |
|        | `109.65 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2166118_bytes`            |
|:-------|:-------------------------- |
|        | `105.52 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2163912_bytes`            |
|:-------|:-------------------------- |
|        | `106.25 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2157753_bytes`            |
|:-------|:-------------------------- |
|        | `105.55 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2149587_bytes`            |
|:-------|:-------------------------- |
|        | `103.76 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2158554_bytes`            |
|:-------|:-------------------------- |
|        | `102.22 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2150637_bytes`            |
|:-------|:-------------------------- |
|        | `107.03 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2149766_bytes`            |
|:-------|:-------------------------- |
|        | `103.94 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2150821_bytes`            |
|:-------|:-------------------------- |
|        | `106.55 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2166337_bytes`            |
|:-------|:-------------------------- |
|        | `106.56 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2164132_bytes`            |
|:-------|:-------------------------- |
|        | `105.56 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2327248_bytes`           |
|:-------|:------------------------- |
|        | `52.00 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2290079_bytes`           |
|:-------|:------------------------- |
|        | `37.39 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2180526_bytes`           |
|:-------|:------------------------- |
|        | `33.32 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2179710_bytes`           |
|:-------|:------------------------- |
|        | `2.42 s` (✅ **1.00x**)    |

### exec:bls12-381-msm-g2-10:default

|        | `2184433_bytes`            |
|:-------|:-------------------------- |
|        | `210.86 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-1000:default

|        | `2183033_bytes`           |
|:-------|:------------------------- |
|        | `18.50 s` (✅ **1.00x**)   |

### exec:bls12-381-mul-affine-g1:default

|        | `2164125_bytes`           |
|:-------|:------------------------- |
|        | `2.25 ms` (✅ **1.00x**)   |

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
|        | `21.89 ms` (✅ **1.00x**)  |

### exec:bls12-377-pairing:default

|        | `2361861_bytes`           |
|:-------|:------------------------- |
|        | `35.40 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-10:default

|        | `2180414_bytes`           |
|:-------|:------------------------- |
|        | `32.18 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-1000:default

|        | `2179596_bytes`           |
|:-------|:------------------------- |
|        | `2.36 s` (✅ **1.00x**)    |

### exec:bls12-377-msm-g2-10:default

|        | `2185765_bytes`            |
|:-------|:-------------------------- |
|        | `212.00 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g2-1000:default

|        | `2184361_bytes`           |
|:-------|:------------------------- |
|        | `18.02 s` (✅ **1.00x**)   |

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
|        | `2.45 ms` (✅ **1.00x**)   |

### exec:bls12-377-mul-projective-g2:default

|        | `2174518_bytes`           |
|:-------|:------------------------- |
|        | `19.19 ms` (✅ **1.00x**)  |

### exec:bw6-761-pairing:default

|        | `2236239_bytes`            |
|:-------|:-------------------------- |
|        | `121.34 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-10:default

|        | `2209265_bytes`            |
|:-------|:-------------------------- |
|        | `275.26 ms` (✅ **1.00x**)  |

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
|        | `25.47 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g2:default

|        | `2195273_bytes`           |
|:-------|:------------------------- |
|        | `22.32 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g1:default

|        | `2190797_bytes`           |
|:-------|:------------------------- |
|        | `25.71 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g2:default

|        | `2190804_bytes`           |
|:-------|:------------------------- |
|        | `22.61 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-10:default

|        | `2176873_bytes`           |
|:-------|:------------------------- |
|        | `11.94 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-1000:default

|        | `2175473_bytes`            |
|:-------|:-------------------------- |
|        | `538.92 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-10:default

|        | `2166118_bytes`           |
|:-------|:------------------------- |
|        | `14.18 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-te-1000:default

|        | `2163912_bytes`            |
|:-------|:-------------------------- |
|        | `616.06 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-sw:default

|        | `2157753_bytes`            |
|:-------|:-------------------------- |
|        | `620.31 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-te:default

|        | `2149587_bytes`            |
|:-------|:-------------------------- |
|        | `630.39 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-sw:default

|        | `2158554_bytes`            |
|:-------|:-------------------------- |
|        | `661.61 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-te:default

|        | `2150637_bytes`            |
|:-------|:-------------------------- |
|        | `612.92 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-affine:default

|        | `2149766_bytes`            |
|:-------|:-------------------------- |
|        | `802.11 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-projective:default

|        | `2150821_bytes`            |
|:-------|:-------------------------- |
|        | `782.43 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-10:default

|        | `2166337_bytes`           |
|:-------|:------------------------- |
|        | `14.15 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-1000:default

|        | `2164132_bytes`            |
|:-------|:-------------------------- |
|        | `660.02 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

