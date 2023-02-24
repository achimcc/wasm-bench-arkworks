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

|        | `2115573_bytes`            |
|:-------|:-------------------------- |
|        | `111.23 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:with-fuel

|        | `2115573_bytes`            |
|:-------|:-------------------------- |
|        | `144.16 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2046278_bytes`           |
|:-------|:------------------------- |
|        | `86.55 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:with-fuel

|        | `2046278_bytes`            |
|:-------|:-------------------------- |
|        | `115.69 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2046285_bytes`           |
|:-------|:------------------------- |
|        | `84.83 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:with-fuel

|        | `2046285_bytes`            |
|:-------|:-------------------------- |
|        | `110.09 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2041462_bytes`           |
|:-------|:------------------------- |
|        | `79.87 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:with-fuel

|        | `2041462_bytes`            |
|:-------|:-------------------------- |
|        | `105.29 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2041473_bytes`           |
|:-------|:------------------------- |
|        | `77.66 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:with-fuel

|        | `2041473_bytes`            |
|:-------|:-------------------------- |
|        | `105.77 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2026350_bytes`           |
|:-------|:------------------------- |
|        | `74.65 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:with-fuel

|        | `2026350_bytes`            |
|:-------|:-------------------------- |
|        | `103.65 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2024576_bytes`           |
|:-------|:------------------------- |
|        | `70.68 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:with-fuel

|        | `2024576_bytes`           |
|:-------|:------------------------- |
|        | `94.43 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2026851_bytes`           |
|:-------|:------------------------- |
|        | `75.63 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:with-fuel

|        | `2026851_bytes`            |
|:-------|:-------------------------- |
|        | `106.89 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2024741_bytes`           |
|:-------|:------------------------- |
|        | `71.30 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:with-fuel

|        | `2024741_bytes`           |
|:-------|:------------------------- |
|        | `93.91 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2146625_bytes`            |
|:-------|:-------------------------- |
|        | `135.98 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:with-fuel

|        | `2146625_bytes`            |
|:-------|:-------------------------- |
|        | `163.90 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2046250_bytes`           |
|:-------|:------------------------- |
|        | `84.55 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:with-fuel

|        | `2046250_bytes`            |
|:-------|:-------------------------- |
|        | `109.65 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2046255_bytes`           |
|:-------|:------------------------- |
|        | `84.66 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:with-fuel

|        | `2046255_bytes`            |
|:-------|:-------------------------- |
|        | `112.09 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2042394_bytes`           |
|:-------|:------------------------- |
|        | `82.13 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:with-fuel

|        | `2042394_bytes`            |
|:-------|:-------------------------- |
|        | `106.61 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2042395_bytes`           |
|:-------|:------------------------- |
|        | `80.12 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:with-fuel

|        | `2042395_bytes`            |
|:-------|:-------------------------- |
|        | `106.61 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2026248_bytes`           |
|:-------|:------------------------- |
|        | `74.66 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:with-fuel

|        | `2026248_bytes`            |
|:-------|:-------------------------- |
|        | `106.55 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2025398_bytes`           |
|:-------|:------------------------- |
|        | `72.43 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:with-fuel

|        | `2025398_bytes`           |
|:-------|:------------------------- |
|        | `94.81 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2026749_bytes`           |
|:-------|:------------------------- |
|        | `75.75 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:with-fuel

|        | `2026749_bytes`            |
|:-------|:-------------------------- |
|        | `102.37 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2025569_bytes`           |
|:-------|:------------------------- |
|        | `69.49 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:with-fuel

|        | `2025569_bytes`           |
|:-------|:------------------------- |
|        | `94.74 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2115281_bytes`            |
|:-------|:-------------------------- |
|        | `117.09 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:with-fuel

|        | `2115281_bytes`            |
|:-------|:-------------------------- |
|        | `148.39 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2071584_bytes`           |
|:-------|:------------------------- |
|        | `97.03 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:with-fuel

|        | `2071584_bytes`            |
|:-------|:-------------------------- |
|        | `134.65 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2071591_bytes`           |
|:-------|:------------------------- |
|        | `96.85 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:with-fuel

|        | `2071591_bytes`            |
|:-------|:-------------------------- |
|        | `135.43 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2071582_bytes`            |
|:-------|:-------------------------- |
|        | `103.20 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:with-fuel

|        | `2071582_bytes`            |
|:-------|:-------------------------- |
|        | `129.79 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2071586_bytes`           |
|:-------|:------------------------- |
|        | `99.13 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:with-fuel

|        | `2071586_bytes`            |
|:-------|:-------------------------- |
|        | `130.83 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2052267_bytes`           |
|:-------|:------------------------- |
|        | `86.72 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:with-fuel

|        | `2052267_bytes`            |
|:-------|:-------------------------- |
|        | `118.32 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2052267_bytes`           |
|:-------|:------------------------- |
|        | `87.26 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:with-fuel

|        | `2052267_bytes`            |
|:-------|:-------------------------- |
|        | `116.42 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2052464_bytes`           |
|:-------|:------------------------- |
|        | `87.23 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:with-fuel

|        | `2052464_bytes`            |
|:-------|:-------------------------- |
|        | `118.65 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2052464_bytes`           |
|:-------|:------------------------- |
|        | `88.88 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:with-fuel

|        | `2052464_bytes`            |
|:-------|:-------------------------- |
|        | `118.79 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2045821_bytes`           |
|:-------|:------------------------- |
|        | `80.28 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:with-fuel

|        | `2045821_bytes`            |
|:-------|:-------------------------- |
|        | `106.25 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2045823_bytes`           |
|:-------|:------------------------- |
|        | `82.76 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:with-fuel

|        | `2045823_bytes`            |
|:-------|:-------------------------- |
|        | `110.82 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2035016_bytes`           |
|:-------|:------------------------- |
|        | `74.13 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:with-fuel

|        | `2035016_bytes`            |
|:-------|:-------------------------- |
|        | `102.20 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2035020_bytes`           |
|:-------|:------------------------- |
|        | `71.95 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:with-fuel

|        | `2035020_bytes`           |
|:-------|:------------------------- |
|        | `99.04 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `69.58 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:with-fuel

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `97.60 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `71.73 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:with-fuel

|        | `2024116_bytes`           |
|:-------|:------------------------- |
|        | `97.39 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2024107_bytes`           |
|:-------|:------------------------- |
|        | `70.63 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:with-fuel

|        | `2024107_bytes`           |
|:-------|:------------------------- |
|        | `95.93 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2016898_bytes`           |
|:-------|:------------------------- |
|        | `66.33 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:with-fuel

|        | `2016898_bytes`           |
|:-------|:------------------------- |
|        | `92.84 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2034122_bytes`           |
|:-------|:------------------------- |
|        | `72.39 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:with-fuel

|        | `2034122_bytes`            |
|:-------|:-------------------------- |
|        | `100.33 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2017021_bytes`           |
|:-------|:------------------------- |
|        | `63.51 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:with-fuel

|        | `2017021_bytes`           |
|:-------|:------------------------- |
|        | `90.56 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2035199_bytes`           |
|:-------|:------------------------- |
|        | `74.67 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:with-fuel

|        | `2035199_bytes`           |
|:-------|:------------------------- |
|        | `98.79 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2035201_bytes`           |
|:-------|:------------------------- |
|        | `71.95 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:with-fuel

|        | `2035201_bytes`           |
|:-------|:------------------------- |
|        | `99.05 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2115573_bytes`           |
|:-------|:------------------------- |
|        | `14.12 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:with-fuel

|        | `2115573_bytes`           |
|:-------|:------------------------- |
|        | `17.60 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

