# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [compile:bls12-381-pairing:default](#compile:bls12-381-pairing:default)
    - [compile:bls12-381-pairing:with-fuel](#compile:bls12-381-pairing:with-fuel)
    - [compile:bls12-381-msm-g1-10:default](#compile:bls12-381-msm-g1-10:default)
    - [compile:bls12-381-msm-g1-10:with-fuel](#compile:bls12-381-msm-g1-10:with-fuel)
    - [compile:bls12-381-msm-g1-1000:default](#compile:bls12-381-msm-g1-1000:default)
    - [compile:bls12-381-msm-g1-1000:with-fuel](#compile:bls12-381-msm-g1-1000:with-fuel)
    - [compile:bls12-381-msm-g2-10:default](#compile:bls12-381-msm-g2-10:default)
    - [compile:bls12-381-msm-g2-10:with-fuel](#compile:bls12-381-msm-g2-10:with-fuel)
    - [compile:bls12-381-msm-g2-1000:default](#compile:bls12-381-msm-g2-1000:default)
    - [compile:bls12-381-msm-g2-1000:with-fuel](#compile:bls12-381-msm-g2-1000:with-fuel)
    - [compile:bls12-381-mul-affine-g1:default](#compile:bls12-381-mul-affine-g1:default)
    - [compile:bls12-381-mul-affine-g1:with-fuel](#compile:bls12-381-mul-affine-g1:with-fuel)
    - [compile:bls12-381-mul-affine-g2:default](#compile:bls12-381-mul-affine-g2:default)
    - [compile:bls12-381-mul-affine-g2:with-fuel](#compile:bls12-381-mul-affine-g2:with-fuel)
    - [compile:bls12-381-mul-projective-g1:default](#compile:bls12-381-mul-projective-g1:default)
    - [compile:bls12-381-mul-projective-g1:with-fuel](#compile:bls12-381-mul-projective-g1:with-fuel)
    - [compile:bls12-381-mul-projective-g2:default](#compile:bls12-381-mul-projective-g2:default)
    - [compile:bls12-381-mul-projective-g2:with-fuel](#compile:bls12-381-mul-projective-g2:with-fuel)
    - [compile:bls12-377-pairing:default](#compile:bls12-377-pairing:default)
    - [compile:bls12-377-pairing:with-fuel](#compile:bls12-377-pairing:with-fuel)
    - [compile:bls12-377-msm-g1-10:default](#compile:bls12-377-msm-g1-10:default)
    - [compile:bls12-377-msm-g1-10:with-fuel](#compile:bls12-377-msm-g1-10:with-fuel)
    - [compile:bls12-377-msm-g1-1000:default](#compile:bls12-377-msm-g1-1000:default)
    - [compile:bls12-377-msm-g1-1000:with-fuel](#compile:bls12-377-msm-g1-1000:with-fuel)
    - [compile:bls12-377-msm-g2-10:default](#compile:bls12-377-msm-g2-10:default)
    - [compile:bls12-377-msm-g2-10:with-fuel](#compile:bls12-377-msm-g2-10:with-fuel)
    - [compile:bls12-377-msm-g2-1000:default](#compile:bls12-377-msm-g2-1000:default)
    - [compile:bls12-377-msm-g2-1000:with-fuel](#compile:bls12-377-msm-g2-1000:with-fuel)
    - [compile:bls12-377-mul-affine-g1:default](#compile:bls12-377-mul-affine-g1:default)
    - [compile:bls12-377-mul-affine-g1:with-fuel](#compile:bls12-377-mul-affine-g1:with-fuel)
    - [compile:bls12-377-mul-affine-g2:default](#compile:bls12-377-mul-affine-g2:default)
    - [compile:bls12-377-mul-affine-g2:with-fuel](#compile:bls12-377-mul-affine-g2:with-fuel)
    - [compile:bls12-377-mul-projective-g1:default](#compile:bls12-377-mul-projective-g1:default)
    - [compile:bls12-377-mul-projective-g1:with-fuel](#compile:bls12-377-mul-projective-g1:with-fuel)
    - [compile:bls12-377-mul-projective-g2:default](#compile:bls12-377-mul-projective-g2:default)
    - [compile:bls12-377-mul-projective-g2:with-fuel](#compile:bls12-377-mul-projective-g2:with-fuel)
    - [compile:bw6-761-pairing:default](#compile:bw6-761-pairing:default)
    - [compile:bw6-761-pairing:with-fuel](#compile:bw6-761-pairing:with-fuel)
    - [compile:bw6-761-msm-g1-10:default](#compile:bw6-761-msm-g1-10:default)
    - [compile:bw6-761-msm-g1-10:with-fuel](#compile:bw6-761-msm-g1-10:with-fuel)
    - [compile:bw6-761-msm-g1-1000:default](#compile:bw6-761-msm-g1-1000:default)
    - [compile:bw6-761-msm-g1-1000:with-fuel](#compile:bw6-761-msm-g1-1000:with-fuel)
    - [compile:bw6-761-msm-g2-10:default](#compile:bw6-761-msm-g2-10:default)
    - [compile:bw6-761-msm-g2-10:with-fuel](#compile:bw6-761-msm-g2-10:with-fuel)
    - [compile:bw6-761-msm-g2-1000:default](#compile:bw6-761-msm-g2-1000:default)
    - [compile:bw6-761-msm-g2-1000:with-fuel](#compile:bw6-761-msm-g2-1000:with-fuel)
    - [compile:bw6-761-mul-affine-g1:default](#compile:bw6-761-mul-affine-g1:default)
    - [compile:bw6-761-mul-affine-g1:with-fuel](#compile:bw6-761-mul-affine-g1:with-fuel)
    - [compile:bw6-761-mul-affine-g2:default](#compile:bw6-761-mul-affine-g2:default)
    - [compile:bw6-761-mul-affine-g2:with-fuel](#compile:bw6-761-mul-affine-g2:with-fuel)
    - [compile:bw6-761-mul-projective-g1:default](#compile:bw6-761-mul-projective-g1:default)
    - [compile:bw6-761-mul-projective-g1:with-fuel](#compile:bw6-761-mul-projective-g1:with-fuel)
    - [compile:bw6-761-mul-projective-g2:default](#compile:bw6-761-mul-projective-g2:default)
    - [compile:bw6-761-mul-projective-g2:with-fuel](#compile:bw6-761-mul-projective-g2:with-fuel)
    - [compile:ed-on-bls12-381-msm-sw-10:default](#compile:ed-on-bls12-381-msm-sw-10:default)
    - [compile:ed-on-bls12-381-msm-sw-10:with-fuel](#compile:ed-on-bls12-381-msm-sw-10:with-fuel)
    - [compile:ed-on-bls12-381-msm-sw-1000:default](#compile:ed-on-bls12-381-msm-sw-1000:default)
    - [compile:ed-on-bls12-381-msm-sw-1000:with-fuel](#compile:ed-on-bls12-381-msm-sw-1000:with-fuel)
    - [compile:ed-on-bls12-381-msm-te-10:default](#compile:ed-on-bls12-381-msm-te-10:default)
    - [compile:ed-on-bls12-381-msm-te-10:with-fuel](#compile:ed-on-bls12-381-msm-te-10:with-fuel)
    - [compile:ed-on-bls12-381-msm-te-1000:default](#compile:ed-on-bls12-381-msm-te-1000:default)
    - [compile:ed-on-bls12-381-msm-te-1000:with-fuel](#compile:ed-on-bls12-381-msm-te-1000:with-fuel)
    - [compile:ed-on-bls12-381-mul-affine-sw:default](#compile:ed-on-bls12-381-mul-affine-sw:default)
    - [compile:ed-on-bls12-381-mul-affine-sw:with-fuel](#compile:ed-on-bls12-381-mul-affine-sw:with-fuel)
    - [compile:ed-on-bls12-381-mul-affine-te:default](#compile:ed-on-bls12-381-mul-affine-te:default)
    - [compile:ed-on-bls12-381-mul-affine-te:with-fuel](#compile:ed-on-bls12-381-mul-affine-te:with-fuel)
    - [compile:ed-on-bls12-381-mul-projective-sw:default](#compile:ed-on-bls12-381-mul-projective-sw:default)
    - [compile:ed-on-bls12-381-mul-projective-sw:with-fuel](#compile:ed-on-bls12-381-mul-projective-sw:with-fuel)
    - [compile:ed-on-bls12-381-mul-projective-te:default](#compile:ed-on-bls12-381-mul-projective-te:default)
    - [compile:ed-on-bls12-381-mul-projective-te:with-fuel](#compile:ed-on-bls12-381-mul-projective-te:with-fuel)
    - [compile:ed-on-bls12-377-mul-affine:default](#compile:ed-on-bls12-377-mul-affine:default)
    - [compile:ed-on-bls12-377-mul-affine:with-fuel](#compile:ed-on-bls12-377-mul-affine:with-fuel)
    - [compile:ed-on-bls12-377-mul-projective:default](#compile:ed-on-bls12-377-mul-projective:default)
    - [compile:ed-on-bls12-377-mul-projective:with-fuel](#compile:ed-on-bls12-377-mul-projective:with-fuel)
    - [compile:ed-on-bls12-377-msm-10:default](#compile:ed-on-bls12-377-msm-10:default)
    - [compile:ed-on-bls12-377-msm-10:with-fuel](#compile:ed-on-bls12-377-msm-10:with-fuel)
    - [compile:ed-on-bls12-377-msm-1000:default](#compile:ed-on-bls12-377-msm-1000:default)
    - [compile:ed-on-bls12-377-msm-1000:with-fuel](#compile:ed-on-bls12-377-msm-1000:with-fuel)
    - [exec:bls12-381-pairing:default](#exec:bls12-381-pairing:default)
    - [exec:bls12-381-pairing:with-fuel](#exec:bls12-381-pairing:with-fuel)

## Benchmark Results

### compile:bls12-381-pairing:default

|        | `2115573_bytes`           |
|:-------|:------------------------- |
|        | `89.83 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:with-fuel

|        | `2115573_bytes`            |
|:-------|:-------------------------- |
|        | `117.91 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2046278_bytes`           |
|:-------|:------------------------- |
|        | `68.92 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:with-fuel

|        | `2046278_bytes`           |
|:-------|:------------------------- |
|        | `91.88 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2046285_bytes`           |
|:-------|:------------------------- |
|        | `68.93 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:with-fuel

|        | `2046285_bytes`           |
|:-------|:------------------------- |
|        | `91.85 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2041468_bytes`           |
|:-------|:------------------------- |
|        | `64.21 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:with-fuel

|        | `2041468_bytes`           |
|:-------|:------------------------- |
|        | `87.17 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2041470_bytes`           |
|:-------|:------------------------- |
|        | `63.93 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:with-fuel

|        | `2041470_bytes`           |
|:-------|:------------------------- |
|        | `86.67 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2026350_bytes`           |
|:-------|:------------------------- |
|        | `61.00 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:with-fuel

|        | `2026350_bytes`           |
|:-------|:------------------------- |
|        | `84.34 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2024576_bytes`           |
|:-------|:------------------------- |
|        | `55.27 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:with-fuel

|        | `2024576_bytes`           |
|:-------|:------------------------- |
|        | `77.63 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2026851_bytes`           |
|:-------|:------------------------- |
|        | `61.34 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:with-fuel

|        | `2026851_bytes`           |
|:-------|:------------------------- |
|        | `84.57 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2024741_bytes`           |
|:-------|:------------------------- |
|        | `56.53 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:with-fuel

|        | `2024741_bytes`           |
|:-------|:------------------------- |
|        | `77.83 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2146625_bytes`            |
|:-------|:-------------------------- |
|        | `109.67 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:with-fuel

|        | `2146625_bytes`            |
|:-------|:-------------------------- |
|        | `130.68 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2046250_bytes`           |
|:-------|:------------------------- |
|        | `68.67 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:with-fuel

|        | `2046250_bytes`           |
|:-------|:------------------------- |
|        | `90.83 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2046255_bytes`           |
|:-------|:------------------------- |
|        | `68.68 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:with-fuel

|        | `2046255_bytes`           |
|:-------|:------------------------- |
|        | `90.89 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2042394_bytes`           |
|:-------|:------------------------- |
|        | `64.55 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:with-fuel

|        | `2042394_bytes`           |
|:-------|:------------------------- |
|        | `87.51 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2042395_bytes`           |
|:-------|:------------------------- |
|        | `64.78 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:with-fuel

|        | `2042395_bytes`           |
|:-------|:------------------------- |
|        | `87.78 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2026248_bytes`           |
|:-------|:------------------------- |
|        | `60.97 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:with-fuel

|        | `2026248_bytes`           |
|:-------|:------------------------- |
|        | `84.68 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2025398_bytes`           |
|:-------|:------------------------- |
|        | `56.00 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:with-fuel

|        | `2025398_bytes`           |
|:-------|:------------------------- |
|        | `78.31 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2026749_bytes`           |
|:-------|:------------------------- |
|        | `61.30 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:with-fuel

|        | `2026749_bytes`           |
|:-------|:------------------------- |
|        | `84.52 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2025569_bytes`           |
|:-------|:------------------------- |
|        | `56.00 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:with-fuel

|        | `2025569_bytes`           |
|:-------|:------------------------- |
|        | `78.44 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2115281_bytes`           |
|:-------|:------------------------- |
|        | `93.44 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:with-fuel

|        | `2115281_bytes`            |
|:-------|:-------------------------- |
|        | `123.53 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2071584_bytes`           |
|:-------|:------------------------- |
|        | `78.71 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:with-fuel

|        | `2071584_bytes`            |
|:-------|:-------------------------- |
|        | `105.98 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2071591_bytes`           |
|:-------|:------------------------- |
|        | `79.25 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:with-fuel

|        | `2071591_bytes`            |
|:-------|:-------------------------- |
|        | `105.53 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2071582_bytes`           |
|:-------|:------------------------- |
|        | `80.25 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:with-fuel

|        | `2071582_bytes`            |
|:-------|:-------------------------- |
|        | `106.09 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2071586_bytes`           |
|:-------|:------------------------- |
|        | `78.81 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:with-fuel

|        | `2071586_bytes`            |
|:-------|:-------------------------- |
|        | `105.37 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2052267_bytes`           |
|:-------|:------------------------- |
|        | `69.22 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:with-fuel

|        | `2052267_bytes`           |
|:-------|:------------------------- |
|        | `95.30 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2052267_bytes`           |
|:-------|:------------------------- |
|        | `68.89 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:with-fuel

|        | `2052267_bytes`           |
|:-------|:------------------------- |
|        | `94.19 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2052464_bytes`           |
|:-------|:------------------------- |
|        | `69.72 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:with-fuel

|        | `2052464_bytes`           |
|:-------|:------------------------- |
|        | `95.96 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2052464_bytes`           |
|:-------|:------------------------- |
|        | `69.22 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:with-fuel

|        | `2052464_bytes`           |
|:-------|:------------------------- |
|        | `95.43 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2045821_bytes`           |
|:-------|:------------------------- |
|        | `65.21 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:with-fuel

|        | `2045821_bytes`           |
|:-------|:------------------------- |
|        | `88.34 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2045823_bytes`           |
|:-------|:------------------------- |
|        | `65.19 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:with-fuel

|        | `2045823_bytes`           |
|:-------|:------------------------- |
|        | `88.21 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2035019_bytes`           |
|:-------|:------------------------- |
|        | `58.04 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:with-fuel

|        | `2035019_bytes`           |
|:-------|:------------------------- |
|        | `80.58 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2035023_bytes`           |
|:-------|:------------------------- |
|        | `58.13 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:with-fuel

|        | `2035023_bytes`           |
|:-------|:------------------------- |
|        | `81.10 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `57.06 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:with-fuel

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `80.06 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `56.89 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:with-fuel

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `80.15 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2024107_bytes`           |
|:-------|:------------------------- |
|        | `56.74 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:with-fuel

|        | `2024107_bytes`           |
|:-------|:------------------------- |
|        | `80.16 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2016898_bytes`           |
|:-------|:------------------------- |
|        | `52.00 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:with-fuel

|        | `2016898_bytes`           |
|:-------|:------------------------- |
|        | `74.08 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2034122_bytes`           |
|:-------|:------------------------- |
|        | `56.43 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:with-fuel

|        | `2034122_bytes`           |
|:-------|:------------------------- |
|        | `80.80 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2017021_bytes`           |
|:-------|:------------------------- |
|        | `51.92 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:with-fuel

|        | `2017021_bytes`           |
|:-------|:------------------------- |
|        | `74.05 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2035199_bytes`           |
|:-------|:------------------------- |
|        | `57.74 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:with-fuel

|        | `2035199_bytes`           |
|:-------|:------------------------- |
|        | `81.46 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2035200_bytes`           |
|:-------|:------------------------- |
|        | `57.80 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:with-fuel

|        | `2035200_bytes`           |
|:-------|:------------------------- |
|        | `81.31 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2115573_bytes`           |
|:-------|:------------------------- |
|        | `12.68 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:with-fuel

|        | `2115573_bytes`           |
|:-------|:------------------------- |
|        | `15.07 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

