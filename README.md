# num-criterion
Benchmarking arbitrary precision number crates for Rust.

## Benchmarked crates

| Library                                               | Version | Notes                                                  |
| --------------                                        | ------- | ------                                                 |
| [rug](https://crates.io/crates/rug)                   | 1.17.0  | Links to libc and [GMP](https://gmplib.org/)           |
| [ibig](https://crates.io/crates/ibig)                 | 0.3.5   | Pure Rust, no_std                                      |
| [dashu](https://crates.io/crates/dashu)               | 0.1.0   | Pure Rust, no_std                                      |
| [malachite-nz](https://crates.io/crates/malachite-nz) | 0.2.6   | Pure Rust, LGPL, derived from GMP and FLINT            |
| [num-bigint](https://crates.io/crates/num-bigint)     | 0.4.3   | Pure Rust, no_std                                      |
| [ramp](https://crates.io/crates/ramp)                 | 0.7.0   | Requires nightly Rust, uses x86_64 assembly, not maintained            |
| [uint](https://crates.io/crates/uint)                 | 0.9.3   | Fixed precision, not arbitrary precision               |


## Install

```cargo install cargo-criterion```

## Minimal run

```cargo criterion```

## Run with `rug`

```cargo criterion --features rug```

## Run with `ramp` (requires nightly)

```cargo +nightly criterion --features ramp```

# Benchmark results:

## M1

|    gcd     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |    87.8 ns |   547.1 ns |            |            |            |
| branchless |    77.3 ns |   501.3 ns |            |            |            |
|        num |   769.8 ns |     1.6 μs |    17.9 μs |   166.9 μs |     9.4 ms |
|   num_svec |   751.7 ns |     1.7 μs |    16.8 μs |   167.3 μs |     9.5 ms |
|        rug |    92.3 ns |   204.1 ns |     4.4 μs |    21.1 μs |   388.0 μs |
|       ramp |   643.2 ns |     1.3 μs |    15.4 μs |   152.0 μs |     9.9 ms |
|       ubig |   746.1 ns |     2.3 μs |    27.5 μs |   212.0 μs |    10.0 ms |

|    mul     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.0 ns |     2.1 ns |   238.6 ns |     5.1 μs |            |
|        num |    39.0 ns |    43.1 ns |   251.9 ns |     3.4 μs |   112.8 μs |
|       unum |    30.9 ns |    34.3 ns |   244.8 ns |     3.3 μs |   112.7 μs |
|   num_svec |     6.6 ns |    37.6 ns |   249.5 ns |     3.4 μs |   110.6 μs |
|  unum_svec |     4.0 ns |    36.3 ns |   246.1 ns |     3.4 μs |   105.1 μs |
|        rug |    24.6 ns |    30.6 ns |   177.0 ns |     1.4 μs |    29.0 μs |
|       ramp |    50.2 ns |    36.4 ns |   445.3 ns |     4.2 μs |   118.5 μs |
|       ibig |    27.8 ns |    35.2 ns |   248.8 ns |     2.9 μs |    89.7 μs |
|       ubig |    26.5 ns |    34.4 ns |   255.0 ns |     3.0 μs |    89.8 μs |

|    mula    |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     0.9 ns |     2.1 ns |   235.2 ns |     5.0 μs |            |
|        num |    65.6 ns |    68.8 ns |   309.6 ns |     3.4 μs |   113.2 μs |
|       unum |    56.3 ns |    64.7 ns |   308.2 ns |     3.4 μs |   113.4 μs |
|   num_svec |     9.9 ns |    40.6 ns |   311.1 ns |     3.4 μs |   107.8 μs |
|  unum_svec |     3.1 ns |    34.8 ns |   304.5 ns |     3.4 μs |   107.7 μs |
|        rug |     4.0 ns |    15.5 ns |   210.8 ns |     1.4 μs |    29.7 μs |
|       ramp |    52.9 ns |    39.7 ns |   480.9 ns |     4.2 μs |   117.8 μs |
|       ibig |    27.2 ns |    74.1 ns |   291.2 ns |     3.0 μs |    89.6 μs |
|       ubig |    26.7 ns |    67.0 ns |   284.6 ns |     3.0 μs |    91.1 μs |

|    div     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.0 ns |     6.0 ns |    77.2 ns |   275.3 ns |            |
|        num |    73.6 ns |   110.8 ns |   132.4 ns |   221.4 ns |   924.7 ns |
|       unum |    60.1 ns |    96.7 ns |   116.3 ns |   205.8 ns |   926.0 ns |
|   num_svec |    25.4 ns |    70.0 ns |   122.1 ns |   213.4 ns |   938.5 ns |
|  unum_svec |    17.6 ns |    58.0 ns |   111.6 ns |   240.3 ns |   946.8 ns |
|        rug |    34.6 ns |    52.3 ns |    69.1 ns |    63.6 ns |    67.6 ns |
|       ramp |    41.9 ns |   100.0 ns |   160.1 ns |   242.4 ns |   962.5 ns |
|       ibig |     3.7 ns |   121.3 ns |   154.1 ns |   229.3 ns |   948.8 ns |
|       ubig |     2.5 ns |   120.1 ns |   145.4 ns |   229.0 ns |   955.6 ns |

|    add     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.0 ns |     1.2 ns |    56.7 ns |   252.1 ns |            |
|        num |    46.1 ns |    47.3 ns |    69.4 ns |   151.0 ns |     1.1 μs |
|       unum |    46.6 ns |    48.3 ns |    77.7 ns |   164.3 ns |     1.2 μs |
|   num_svec |    19.4 ns |    30.1 ns |    71.8 ns |   155.4 ns |     1.1 μs |
|  unum_svec |    10.1 ns |    28.5 ns |    80.1 ns |   169.4 ns |     1.2 μs |
|        rug |    31.6 ns |    33.6 ns |    39.5 ns |    60.5 ns |   238.5 ns |
|       ramp |    44.9 ns |    50.1 ns |    69.3 ns |   116.0 ns |   697.6 ns |
|       ibig |    16.7 ns |    41.7 ns |    59.0 ns |   125.7 ns |   833.9 ns |
|       ubig |    16.0 ns |    35.0 ns |    55.2 ns |   126.2 ns |   820.2 ns |

|    adda    |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     0.9 ns |     1.5 ns |    58.4 ns |   253.0 ns |            |
|        num |    36.7 ns |    37.5 ns |    54.1 ns |   114.0 ns |   898.1 ns |
|       unum |    37.8 ns |    39.0 ns |    68.7 ns |   151.1 ns |   988.7 ns |
|   num_svec |    17.0 ns |    27.1 ns |    52.4 ns |   112.8 ns |   895.4 ns |
|  unum_svec |     7.9 ns |    22.3 ns |    70.6 ns |   152.6 ns |     1.0 μs |
|        rug |    12.5 ns |    13.1 ns |    12.7 ns |    29.3 ns |   186.2 ns |
|       ramp |    12.2 ns |    13.0 ns |    23.4 ns |    62.9 ns |   542.1 ns |
|       ibig |    19.4 ns |    22.2 ns |    34.7 ns |    87.0 ns |   700.6 ns |
|       ubig |    17.1 ns |    14.0 ns |    28.4 ns |    82.3 ns |   673.0 ns |

|    shra    |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.0 ns |     1.0 ns |    11.0 ns |    50.9 ns |            |
|        num |     9.6 ns |    10.1 ns |    14.6 ns |    31.5 ns |   353.5 ns |
|       unum |     5.2 ns |     5.6 ns |    11.3 ns |    28.0 ns |   306.0 ns |
|   num_svec |     8.8 ns |     8.9 ns |    13.3 ns |    29.7 ns |   380.2 ns |
|  unum_svec |     3.7 ns |     4.3 ns |     8.5 ns |    27.5 ns |   435.6 ns |
|        rug |     7.4 ns |     7.7 ns |    13.0 ns |    24.4 ns |   165.8 ns |
|       ramp |     3.5 ns |     3.5 ns |     9.4 ns |    35.2 ns |   340.3 ns |
|       ibig |    10.5 ns |    18.0 ns |    25.0 ns |    48.5 ns |   250.4 ns |
|       ubig |     3.2 ns |     7.5 ns |    12.9 ns |    40.7 ns |   227.1 ns |

|    cmp     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.0 ns |     1.4 ns |     1.5 ns |     1.6 ns |            |
|        num |     6.4 ns |     6.5 ns |     6.5 ns |     6.8 ns |     8.4 ns |
|       unum |     1.3 ns |     1.4 ns |     2.4 ns |     3.6 ns |     7.9 ns |
|   num_svec |     6.6 ns |     6.6 ns |     6.7 ns |     7.9 ns |     8.8 ns |
|  unum_svec |     2.1 ns |     2.1 ns |     3.0 ns |     4.8 ns |     9.2 ns |
|        rug |     4.5 ns |     4.4 ns |     2.3 ns |     4.3 ns |     2.5 ns |
|       ramp |     6.2 ns |     6.2 ns |     5.3 ns |     6.0 ns |     5.0 ns |
|       ibig |     6.8 ns |     7.0 ns |     5.8 ns |     6.3 ns |     3.4 ns |
|       ubig |     1.6 ns |     1.9 ns |     3.1 ns |     3.3 ns |     2.9 ns |

|   clone    |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.0 ns |     1.0 ns |     4.8 ns |    19.7 ns |            |
|        num |    22.9 ns |    23.0 ns |    32.1 ns |    58.1 ns |   408.5 ns |
|       unum |    22.9 ns |    23.0 ns |    31.7 ns |    56.5 ns |   422.4 ns |
|   num_svec |     3.7 ns |     4.7 ns |    32.5 ns |    58.3 ns |   432.3 ns |
|  unum_svec |     3.4 ns |     4.3 ns |    32.1 ns |    58.6 ns |   416.8 ns |
|        rug |    22.5 ns |    20.7 ns |    28.4 ns |    42.3 ns |   169.3 ns |
|       ramp |    24.8 ns |    25.3 ns |    30.7 ns |    41.2 ns |   177.8 ns |
|       ibig |     2.8 ns |    25.9 ns |    27.8 ns |    46.3 ns |   162.9 ns |
|       ubig |     2.2 ns |    25.6 ns |    29.6 ns |    45.3 ns |   208.9 ns |

## 3950X

|    gcd     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |   175.0 ns |   603.4 ns |            |            |            |
| branchless |    85.8 ns |   396.3 ns |            |            |            |
|        num |   972.1 ns |     1.9 μs |    17.4 μs |   113.6 μs |     4.5 ms |
|   num_svec |   861.9 ns |     1.7 μs |    17.0 μs |   106.1 μs |     4.4 ms |
|        rug |    75.0 ns |   216.5 ns |     5.4 μs |    25.2 μs |   414.0 μs |
|       ramp |   604.2 ns |     1.3 μs |    16.0 μs |   172.4 μs |     9.7 ms |
|       ubig |     1.2 μs |     3.6 μs |    37.8 μs |   225.2 μs |     7.4 ms |

|    mul     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.6 ns |     2.7 ns |   307.0 ns |     6.1 μs |            |
|        num |    34.0 ns |    37.6 ns |   248.4 ns |     3.0 μs |    94.8 μs |
|       unum |    25.2 ns |    27.2 ns |   245.1 ns |     3.0 μs |    92.8 μs |
|   num_svec |    13.9 ns |    36.3 ns |   259.1 ns |     3.0 μs |    76.5 μs |
|  unum_svec |     7.6 ns |    35.4 ns |   256.4 ns |     3.0 μs |    76.1 μs |
|        rug |    20.4 ns |    23.0 ns |   149.0 ns |     1.4 μs |    29.7 μs |
|       ramp |    57.2 ns |    33.9 ns |   216.9 ns |     2.2 μs |    60.1 μs |
|       ibig |    34.0 ns |    49.0 ns |   260.5 ns |     2.7 μs |    75.6 μs |
|       ubig |    28.9 ns |    34.3 ns |   247.7 ns |     2.6 μs |    75.1 μs |

|    mula    |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.2 ns |     2.5 ns |   295.8 ns |     5.9 μs |            |
|        num |    37.7 ns |    46.9 ns |   274.5 ns |     2.9 μs |    92.5 μs |
|       unum |    28.6 ns |    37.1 ns |   256.6 ns |     2.9 μs |    92.8 μs |
|   num_svec |    10.0 ns |    53.5 ns |   277.1 ns |     2.9 μs |    79.0 μs |
|  unum_svec |     3.1 ns |    46.5 ns |   274.9 ns |     2.9 μs |    79.6 μs |
|        rug |     6.0 ns |     9.2 ns |   203.7 ns |     1.4 μs |    31.0 μs |
|       ramp |    42.7 ns |    41.8 ns |   249.9 ns |     2.2 μs |    62.0 μs |
|       ibig |    31.3 ns |    53.4 ns |   285.3 ns |     2.6 μs |    74.4 μs |
|       ubig |    30.1 ns |    43.1 ns |   272.4 ns |     2.6 μs |    75.0 μs |

|    div     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     3.3 ns |    10.2 ns |    88.5 ns |   273.7 ns |            |
|        num |    57.8 ns |    88.3 ns |   117.1 ns |   197.4 ns |   925.5 ns |
|       unum |    46.4 ns |    71.2 ns |   102.6 ns |   181.3 ns |   926.7 ns |
|   num_svec |    32.5 ns |    81.6 ns |   110.5 ns |   190.2 ns |   972.6 ns |
|  unum_svec |    17.4 ns |    68.7 ns |    99.3 ns |   175.6 ns |   940.6 ns |
|        rug |    34.1 ns |    41.9 ns |    58.5 ns |    62.8 ns |    70.3 ns |
|       ramp |    38.5 ns |    77.8 ns |   137.6 ns |   192.3 ns |   732.7 ns |
|       ibig |    12.1 ns |    63.5 ns |    86.2 ns |   167.0 ns |     1.0 μs |
|       ubig |     6.9 ns |    61.7 ns |    82.6 ns |   163.7 ns |     1.0 μs |

|    add     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.6 ns |     1.6 ns |    60.3 ns |   224.1 ns |            |
|        num |    43.4 ns |    47.9 ns |    47.3 ns |    98.7 ns |   576.9 ns |
|       unum |    35.5 ns |    41.2 ns |    41.6 ns |    96.8 ns |   568.0 ns |
|   num_svec |    22.8 ns |    28.9 ns |    49.8 ns |    98.0 ns |   580.6 ns |
|  unum_svec |    13.2 ns |    26.2 ns |    45.4 ns |   106.6 ns |   575.9 ns |
|        rug |    29.9 ns |    30.3 ns |    34.6 ns |    52.2 ns |   227.3 ns |
|       ramp |    46.9 ns |    49.4 ns |    60.0 ns |   107.6 ns |   611.8 ns |
|       ibig |    19.0 ns |    45.9 ns |    50.2 ns |    78.6 ns |   399.0 ns |
|       ubig |    15.8 ns |    32.3 ns |    37.4 ns |    67.1 ns |   375.4 ns |

|    adda    |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.1 ns |     1.4 ns |    57.8 ns |   236.7 ns |            |
|        num |    33.0 ns |    33.9 ns |    41.1 ns |    75.2 ns |   372.7 ns |
|       unum |    23.1 ns |    24.3 ns |    35.8 ns |    79.7 ns |   412.5 ns |
|   num_svec |    23.5 ns |    30.2 ns |    40.0 ns |    76.1 ns |   380.2 ns |
|  unum_svec |     6.7 ns |    17.9 ns |    37.3 ns |    79.1 ns |   400.2 ns |
|        rug |    11.2 ns |    11.7 ns |    17.2 ns |    34.6 ns |   179.4 ns |
|       ramp |    12.2 ns |    12.9 ns |    22.9 ns |    65.0 ns |   445.3 ns |
|       ibig |    20.3 ns |    27.1 ns |    34.0 ns |    58.1 ns |   284.2 ns |
|       ubig |    16.2 ns |    18.3 ns |    23.2 ns |    46.2 ns |   265.3 ns |

|    shra    |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.2 ns |     1.8 ns |    11.2 ns |    70.6 ns |            |
|        num |    15.8 ns |    15.3 ns |    23.6 ns |    56.9 ns |   273.1 ns |
|       unum |    14.3 ns |    13.8 ns |    24.1 ns |    54.9 ns |   259.5 ns |
|   num_svec |    11.7 ns |    12.0 ns |    17.8 ns |    48.1 ns |   259.1 ns |
|  unum_svec |     5.9 ns |     6.4 ns |    13.0 ns |    43.6 ns |   252.4 ns |
|        rug |     9.5 ns |     9.6 ns |    13.5 ns |    26.1 ns |   179.8 ns |
|       ramp |     6.2 ns |     8.0 ns |    14.7 ns |    35.8 ns |   222.9 ns |
|       ibig |    19.4 ns |    40.6 ns |    44.3 ns |    78.6 ns |   375.0 ns |
|       ubig |     8.7 ns |    19.5 ns |    23.0 ns |    57.1 ns |   344.6 ns |

|    cmp     |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.4 ns |     1.8 ns |     1.9 ns |     3.0 ns |            |
|        num |     6.9 ns |     6.9 ns |     7.6 ns |     7.9 ns |     8.7 ns |
|       unum |     2.0 ns |     1.9 ns |     3.6 ns |     4.0 ns |     7.9 ns |
|   num_svec |     7.3 ns |     7.2 ns |     8.0 ns |     8.2 ns |     8.8 ns |
|  unum_svec |     1.9 ns |     1.8 ns |     3.9 ns |     4.2 ns |     8.1 ns |
|        rug |     5.0 ns |     5.0 ns |     5.1 ns |     5.3 ns |     5.9 ns |
|       ramp |     6.8 ns |     6.7 ns |     7.0 ns |     6.9 ns |     6.7 ns |
|       ibig |     7.2 ns |     7.4 ns |     8.0 ns |     7.7 ns |     8.2 ns |
|       ubig |     2.0 ns |     2.2 ns |     2.9 ns |     2.9 ns |     3.8 ns |

|   clone    |    64 bits |   128 bits |  1024 bits |  4096 bits | 32768 bits |
|------------| ---------: | ---------: | ---------: | ---------: | ---------: |
|       uint |     1.4 ns |     1.3 ns |     5.1 ns |    16.7 ns |            |
|        num |    14.7 ns |    16.8 ns |    17.0 ns |    56.5 ns |   304.1 ns |
|       unum |    14.1 ns |    17.1 ns |    16.4 ns |    57.0 ns |   297.1 ns |
|   num_svec |     3.2 ns |     3.3 ns |    17.6 ns |    60.4 ns |   301.7 ns |
|  unum_svec |     2.5 ns |     2.3 ns |    17.0 ns |    57.1 ns |   305.6 ns |
|        rug |    16.8 ns |    16.8 ns |    19.5 ns |    29.0 ns |   129.7 ns |
|       ramp |    21.5 ns |    25.4 ns |    25.2 ns |    38.6 ns |   170.2 ns |
|       ibig |     7.5 ns |    24.5 ns |    25.5 ns |    35.1 ns |   136.3 ns |
|       ubig |     6.2 ns |    22.9 ns |    24.2 ns |    33.7 ns |   129.0 ns |


[num]: https://crates.io/crates/num
[rug]: https://crates.io/crates/rug
[ramp]: https://crates.io/crates/ramp
[gmp]: https://gmplib.org
