# num-criterion
Benchmarking arbitrary precision number crates for Rust.

## TL;DR

There are three major arbitrary-precision number crates: [num], [rug], and [ramp].

`Rug`, which uses the [GMP], is about 10x faster than `num` and `ramp`.

## Install

```cargo install cargo-criterion```

## Minimal run

```cargo criterion```

## Run with `rug`

```cargo criterion --features rug```

## Run with `ramp` (requires nightly)

```cargo +nightly criterion --features ramp```

# Benchmark results for `num` vs `rug` vs `ramp`:

## M1

```
gcd/64bits/uint         time:   [87.103 ns 87.327 ns 87.572 ns]
gcd/64bits/num          time:   [779.08 ns 779.46 ns 779.90 ns]
gcd/64bits/rug          time:   [92.888 ns 93.023 ns 93.173 ns]
gcd/64bits/ramp         time:   [636.93 ns 639.42 ns 642.03 ns]

gcd/128bits/uint        time:   [541.34 ns 542.66 ns 544.23 ns]
gcd/128bits/num         time:   [1.6009 us 1.6029 us 1.6047 us]
gcd/128bits/rug         time:   [207.93 ns 208.07 ns 208.24 ns]
gcd/128bits/ramp        time:   [1.2673 us 1.2699 us 1.2727 us]

gcd/1024bits/num        time:   [17.887 us 17.912 us 17.943 us]
gcd/1024bits/rug        time:   [4.3600 us 4.3622 us 4.3644 us]
gcd/1024bits/ramp       time:   [15.381 us 15.435 us 15.493 us]

gcd/4096bits/num        time:   [167.22 us 167.41 us 167.61 us]
gcd/4096bits/rug        time:   [21.054 us 21.096 us 21.152 us]
gcd/4096bits/ramp       time:   [151.66 us 151.97 us 152.32 us]

gcd/32768bits/num       time:   [9.4993 ms 9.5175 ms 9.5365 ms]
gcd/32768bits/rug       time:   [385.99 us 387.15 us 388.42 us]
gcd/32768bits/ramp      time:   [9.9327 ms 9.9359 ms 9.9396 ms]

mul/64bits/uint         time:   [648.40 ps 659.57 ps 673.72 ps]
mul/64bits/num          time:   [38.356 ns 38.502 ns 38.659 ns]
mul/64bits/rug          time:   [25.319 ns 25.371 ns 25.419 ns]
mul/64bits/ramp         time:   [49.704 ns 49.812 ns 49.909 ns]

mul/128bits/uint        time:   [5.5576 ns 5.5840 ns 5.6146 ns]
mul/128bits/num         time:   [42.454 ns 42.584 ns 42.705 ns]
mul/128bits/rug         time:   [31.822 ns 31.870 ns 31.921 ns]
mul/128bits/ramp        time:   [36.343 ns 36.408 ns 36.477 ns]

mul/1024bits/uint       time:   [76.045 ns 76.332 ns 76.658 ns]
mul/1024bits/num        time:   [252.08 ns 253.03 ns 254.08 ns]
mul/1024bits/rug        time:   [175.78 ns 176.18 ns 176.61 ns]
mul/1024bits/ramp       time:   [444.65 ns 445.41 ns 446.24 ns]

mul/4096bits/uint       time:   [263.77 ns 264.38 ns 265.06 ns]
mul/4096bits/num        time:   [3.3437 us 3.3468 us 3.3498 us]
mul/4096bits/rug        time:   [1.3624 us 1.3641 us 1.3662 us]
mul/4096bits/ramp       time:   [4.2345 us 4.2366 us 4.2391 us]

mul/32768bits/num       time:   [112.71 us 112.81 us 112.93 us]
mul/32768bits/rug       time:   [29.488 us 29.524 us 29.570 us]
mul/32768bits/ramp      time:   [119.78 us 119.84 us 119.91 us]

div/64bits/uint         time:   [648.41 ps 659.92 ps 675.06 ps]
div/64bits/num          time:   [73.165 ns 73.382 ns 73.567 ns]
div/64bits/rug          time:   [35.365 ns 35.417 ns 35.475 ns]
div/64bits/ramp         time:   [42.216 ns 42.268 ns 42.321 ns]

div/128bits/uint        time:   [5.6075 ns 5.6469 ns 5.6930 ns]
div/128bits/num         time:   [111.39 ns 111.56 ns 111.73 ns]
div/128bits/rug         time:   [52.955 ns 53.029 ns 53.112 ns]
div/128bits/ramp        time:   [100.60 ns 100.78 ns 101.00 ns]

div/1024bits/uint       time:   [75.546 ns 75.795 ns 76.154 ns]
div/1024bits/num        time:   [132.17 ns 132.36 ns 132.55 ns]
div/1024bits/rug        time:   [69.480 ns 69.610 ns 69.791 ns]
div/1024bits/ramp       time:   [160.14 ns 160.47 ns 160.88 ns]

div/4096bits/uint       time:   [261.70 ns 262.31 ns 262.98 ns]
div/4096bits/num        time:   [222.20 ns 222.83 ns 223.53 ns]
div/4096bits/rug        time:   [65.228 ns 65.663 ns 66.177 ns]
div/4096bits/ramp       time:   [246.88 ns 247.50 ns 248.24 ns]

div/32768bits/num       time:   [941.70 ns 943.22 ns 944.76 ns]
div/32768bits/rug       time:   [70.001 ns 70.991 ns 72.138 ns]
div/32768bits/ramp      time:   [977.58 ns 982.53 ns 988.11 ns]

add/64bits/uint         time:   [651.73 ps 662.24 ps 674.86 ps]
add/64bits/num          time:   [46.907 ns 47.023 ns 47.135 ns]
add/64bits/rug          time:   [32.673 ns 32.715 ns 32.757 ns]
add/64bits/ramp         time:   [44.952 ns 45.141 ns 45.325 ns]

add/128bits/uint        time:   [5.5462 ns 5.5747 ns 5.6095 ns]
add/128bits/num         time:   [49.510 ns 49.786 ns 50.009 ns]
add/128bits/rug         time:   [34.953 ns 35.039 ns 35.134 ns]
add/128bits/ramp        time:   [50.584 ns 50.731 ns 50.883 ns]

add/1024bits/uint       time:   [75.368 ns 75.507 ns 75.667 ns]
add/1024bits/num        time:   [70.296 ns 70.520 ns 70.751 ns]
add/1024bits/rug        time:   [41.120 ns 41.275 ns 41.473 ns]
add/1024bits/ramp       time:   [68.360 ns 68.604 ns 68.883 ns]

add/4096bits/uint       time:   [262.87 ns 263.45 ns 264.10 ns]
add/4096bits/num        time:   [162.20 ns 163.04 ns 163.73 ns]
add/4096bits/rug        time:   [59.540 ns 61.295 ns 62.940 ns]
add/4096bits/ramp       time:   [117.42 ns 118.35 ns 119.28 ns]

add/32768bits/num       time:   [1.1466 us 1.1597 us 1.1708 us]
add/32768bits/rug       time:   [240.19 ns 244.32 ns 248.68 ns]
add/32768bits/ramp      time:   [713.60 ns 719.17 ns 724.96 ns]
```

## 3950X

```
gcd/64bits/uint         time:   [176.25 ns 176.95 ns 177.72 ns]
gcd/64bits/num          time:   [1.1859 us 1.1868 us 1.1878 us]
gcd/64bits/rug          time:   [78.477 ns 78.738 ns 79.007 ns]
gcd/64bits/ramp         time:   [601.64 ns 604.93 ns 609.11 ns]

gcd/128bits/uint        time:   [605.41 ns 605.74 ns 606.14 ns]
gcd/128bits/num         time:   [2.2815 us 2.2843 us 2.2878 us]
gcd/128bits/rug         time:   [220.20 ns 220.40 ns 220.61 ns]
gcd/128bits/ramp        time:   [1.3226 us 1.3257 us 1.3289 us]

gcd/1024bits/num        time:   [21.236 us 21.299 us 21.378 us]
gcd/1024bits/rug        time:   [5.4747 us 5.4940 us 5.5133 us]
gcd/1024bits/ramp       time:   [16.012 us 16.028 us 16.045 us]

gcd/4096bits/num        time:   [132.22 us 132.32 us 132.42 us]
gcd/4096bits/rug        time:   [25.717 us 25.740 us 25.765 us]
gcd/4096bits/ramp       time:   [177.57 us 177.67 us 177.78 us]

gcd/32768bits/num       time:   [4.7872 ms 4.7891 ms 4.7909 ms]
gcd/32768bits/rug       time:   [431.35 us 431.73 us 432.13 us]
gcd/32768bits/ramp      time:   [10.165 ms 10.170 ms 10.176 ms]

mul/64bits/uint         time:   [3.3096 ns 3.3146 ns 3.3200 ns]
mul/64bits/num          time:   [41.279 ns 41.685 ns 42.199 ns]
mul/64bits/rug          time:   [23.331 ns 23.468 ns 23.617 ns]
mul/64bits/ramp         time:   [52.602 ns 52.704 ns 52.819 ns]

mul/128bits/uint        time:   [10.168 ns 10.223 ns 10.283 ns]
mul/128bits/num         time:   [47.152 ns 47.216 ns 47.290 ns]
mul/128bits/rug         time:   [24.622 ns 24.692 ns 24.773 ns]
mul/128bits/ramp        time:   [31.128 ns 31.202 ns 31.288 ns]

mul/1024bits/uint       time:   [91.256 ns 91.386 ns 91.512 ns]
mul/1024bits/num        time:   [258.35 ns 258.47 ns 258.63 ns]
mul/1024bits/rug        time:   [159.96 ns 160.38 ns 160.88 ns]
mul/1024bits/ramp       time:   [217.34 ns 217.51 ns 217.69 ns]

mul/4096bits/uint       time:   [278.86 ns 279.20 ns 279.56 ns]
mul/4096bits/num        time:   [2.9504 us 2.9524 us 2.9543 us]
mul/4096bits/rug        time:   [1.4290 us 1.4304 us 1.4318 us]
mul/4096bits/ramp       time:   [2.2358 us 2.2379 us 2.2399 us]

mul/32768bits/num       time:   [95.862 us 95.921 us 95.984 us]
mul/32768bits/rug       time:   [31.922 us 31.945 us 31.969 us]
mul/32768bits/ramp      time:   [62.969 us 63.018 us 63.070 us]

div/64bits/uint         time:   [3.3780 ns 3.3833 ns 3.3890 ns]
div/64bits/num          time:   [59.857 ns 60.118 ns 60.372 ns]
div/64bits/rug          time:   [36.950 ns 36.990 ns 37.028 ns]
div/64bits/ramp         time:   [39.169 ns 39.211 ns 39.257 ns]

div/128bits/uint        time:   [10.585 ns 10.601 ns 10.618 ns]
div/128bits/num         time:   [87.017 ns 87.588 ns 88.250 ns]
div/128bits/rug         time:   [42.672 ns 42.738 ns 42.809 ns]
div/128bits/ramp        time:   [88.556 ns 88.645 ns 88.738 ns]

div/1024bits/uint       time:   [94.492 ns 94.902 ns 95.305 ns]
div/1024bits/num        time:   [126.19 ns 126.27 ns 126.34 ns]
div/1024bits/rug        time:   [63.402 ns 63.487 ns 63.576 ns]
div/1024bits/ramp       time:   [151.48 ns 152.54 ns 153.83 ns]

div/4096bits/uint       time:   [268.89 ns 269.27 ns 269.66 ns]
div/4096bits/num        time:   [210.20 ns 211.46 ns 212.66 ns]
div/4096bits/rug        time:   [65.337 ns 65.492 ns 65.672 ns]
div/4096bits/ramp       time:   [203.29 ns 203.72 ns 204.20 ns]

div/32768bits/num       time:   [991.28 ns 998.69 ns 1.0053 us]
div/32768bits/rug       time:   [76.506 ns 76.801 ns 77.079 ns]
div/32768bits/ramp      time:   [746.42 ns 748.48 ns 750.29 ns]

add/64bits/uint         time:   [3.3767 ns 3.3803 ns 3.3842 ns]
add/64bits/num          time:   [56.805 ns 56.852 ns 56.894 ns]
add/64bits/rug          time:   [34.507 ns 34.578 ns 34.651 ns]
add/64bits/ramp         time:   [46.877 ns 46.919 ns 46.964 ns]

add/128bits/uint        time:   [10.605 ns 10.622 ns 10.639 ns]
add/128bits/num         time:   [58.941 ns 59.242 ns 59.588 ns]
add/128bits/rug         time:   [33.405 ns 33.478 ns 33.559 ns]
add/128bits/ramp        time:   [48.750 ns 48.810 ns 48.879 ns]

add/1024bits/uint       time:   [92.186 ns 92.291 ns 92.408 ns]
add/1024bits/num        time:   [57.005 ns 57.567 ns 58.175 ns]
add/1024bits/rug        time:   [37.518 ns 37.587 ns 37.662 ns]
add/1024bits/ramp       time:   [56.666 ns 56.765 ns 56.874 ns]

add/4096bits/uint       time:   [268.31 ns 268.63 ns 268.98 ns]
add/4096bits/num        time:   [103.10 ns 107.94 ns 112.14 ns]
add/4096bits/rug        time:   [53.443 ns 53.593 ns 53.750 ns]
add/4096bits/ramp       time:   [106.17 ns 106.32 ns 106.48 ns]

add/32768bits/num       time:   [568.74 ns 612.43 ns 650.12 ns]
add/32768bits/rug       time:   [236.31 ns 238.18 ns 240.53 ns]
add/32768bits/ramp      time:   [633.63 ns 634.85 ns 636.09 ns]
```

[num]: https://crates.io/crates/num
[rug]: https://crates.io/crates/rug
[ramp]: https://crates.io/crates/ramp
[gmp]: https://gmplib.org
