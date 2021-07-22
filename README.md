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
gcd/64bits/uint         time:   [88.907 ns 88.969 ns 89.047 ns]
gcd/64bits/branchless   time:   [78.564 ns 78.627 ns 78.716 ns]
gcd/64bits/num          time:   [774.57 ns 775.21 ns 775.94 ns]
gcd/64bits/num_svec     time:   [698.10 ns 698.59 ns 699.14 ns]
gcd/64bits/rug          time:   [92.897 ns 92.996 ns 93.101 ns]
gcd/64bits/ramp         time:   [640.92 ns 642.57 ns 644.43 ns]

gcd/128/uint            time:   [551.23 ns 551.45 ns 551.72 ns]
gcd/128/branchless      time:   [510.70 ns 511.04 ns 511.38 ns]
gcd/128/num             time:   [1.5911 us 1.5936 us 1.5965 us]
gcd/128/num_svec        time:   [1.4480 us 1.4489 us 1.4498 us]
gcd/128/rug             time:   [207.57 ns 207.72 ns 207.92 ns]
gcd/128/ramp            time:   [1.2712 us 1.2738 us 1.2769 us]

gcd/1024/num            time:   [17.892 us 17.904 us 17.918 us]
gcd/1024/num_svec       time:   [17.200 us 17.210 us 17.225 us]
gcd/1024/rug            time:   [4.4385 us 4.4407 us 4.4430 us]
gcd/1024/ramp           time:   [15.288 us 15.310 us 15.335 us]

gcd/4096/num            time:   [167.51 us 167.70 us 167.93 us]
gcd/4096/num_svec       time:   [167.23 us 167.34 us 167.48 us]
gcd/4096/rug            time:   [21.443 us 21.451 us 21.460 us]
gcd/4096/ramp           time:   [151.59 us 151.93 us 152.34 us]

gcd/32768/num           time:   [9.5455 ms 9.5556 ms 9.5663 ms]
gcd/32768/num_svec      time:   [9.5075 ms 9.5120 ms 9.5182 ms]
gcd/32768/rug           time:   [386.76 us 386.94 us 387.18 us]
gcd/32768/ramp          time:   [9.9280 ms 9.9327 ms 9.9389 ms]

mul/64bits/uint         time:   [649.21 ps 659.56 ps 673.04 ps]
mul/64bits/num          time:   [38.096 ns 38.228 ns 38.364 ns]
mul/64bits/num_svec     time:   [6.4929 ns 6.5538 ns 6.6133 ns]
mul/64bits/rug          time:   [25.332 ns 25.376 ns 25.421 ns]
mul/64bits/ramp         time:   [49.789 ns 49.886 ns 49.982 ns]

mul/128/uint            time:   [5.5114 ns 5.5383 ns 5.5694 ns]
mul/128/num             time:   [42.503 ns 42.739 ns 42.970 ns]
mul/128/num_svec        time:   [38.038 ns 38.245 ns 38.417 ns]
mul/128/rug             time:   [31.783 ns 31.831 ns 31.883 ns]
mul/128/ramp            time:   [36.293 ns 36.364 ns 36.447 ns]

mul/1024/uint           time:   [75.712 ns 75.850 ns 76.017 ns]
mul/1024/num            time:   [257.73 ns 258.62 ns 259.52 ns]
mul/1024/num_svec       time:   [250.05 ns 250.64 ns 251.20 ns]
mul/1024/rug            time:   [176.67 ns 177.19 ns 177.77 ns]
mul/1024/ramp           time:   [444.25 ns 444.89 ns 445.58 ns]

mul/4096/uint           time:   [262.47 ns 262.92 ns 263.40 ns]
mul/4096/num            time:   [3.3541 us 3.3564 us 3.3588 us]
mul/4096/num_svec       time:   [3.3532 us 3.3571 us 3.3617 us]
mul/4096/rug            time:   [1.3542 us 1.3553 us 1.3565 us]
mul/4096/ramp           time:   [4.2274 us 4.2302 us 4.2335 us]

mul/32768/num           time:   [112.67 us 112.73 us 112.80 us]
mul/32768/num_svec      time:   [106.58 us 106.65 us 106.74 us]
mul/32768/rug           time:   [29.461 us 29.486 us 29.514 us]
mul/32768/ramp          time:   [119.69 us 119.75 us 119.82 us]

mut/64bits/num          time:   [66.517 ns 66.631 ns 66.732 ns]
mut/64bits/num_svec     time:   [10.373 ns 10.395 ns 10.417 ns]
mut/64bits/rug_mut      time:   [4.2292 ns 4.2452 ns 4.2663 ns]
mut/64bits/ramp         time:   [54.248 ns 54.276 ns 54.309 ns]

mut/128/num             time:   [70.122 ns 70.292 ns 70.490 ns]
mut/128/num_svec        time:   [41.598 ns 41.706 ns 41.812 ns]
mut/128/rug_mut         time:   [15.802 ns 15.823 ns 15.849 ns]
mut/128/ramp            time:   [40.757 ns 40.803 ns 40.859 ns]

mut/1024/num            time:   [311.46 ns 313.41 ns 315.18 ns]
mut/1024/num_svec       time:   [307.80 ns 309.45 ns 310.89 ns]
mut/1024/rug_mut        time:   [216.02 ns 216.57 ns 217.19 ns]
mut/1024/ramp           time:   [488.86 ns 489.72 ns 490.69 ns]

mut/4096/num            time:   [3.3882 us 3.3912 us 3.3942 us]
mut/4096/num_svec       time:   [3.3827 us 3.3877 us 3.3952 us]
mut/4096/rug_mut        time:   [1.3837 us 1.3853 us 1.3868 us]
mut/4096/ramp           time:   [4.2548 us 4.2764 us 4.3240 us]

mut/32768/num           time:   [112.54 us 112.63 us 112.74 us]
mut/32768/num_svec      time:   [107.46 us 107.51 us 107.56 us]
mut/32768/rug_mut       time:   [29.523 us 29.543 us 29.564 us]
mut/32768/ramp          time:   [119.80 us 119.86 us 119.93 us]

div/64bits/uint         time:   [646.40 ps 656.83 ps 671.24 ps]
div/64bits/num          time:   [72.993 ns 73.205 ns 73.383 ns]
div/64bits/num_svec     time:   [25.243 ns 25.357 ns 25.497 ns]
div/64bits/rug          time:   [35.466 ns 35.521 ns 35.582 ns]
div/64bits/ramp         time:   [42.159 ns 42.224 ns 42.296 ns]

div/128/uint            time:   [5.4894 ns 5.5172 ns 5.5500 ns]
div/128/num             time:   [111.01 ns 111.11 ns 111.21 ns]
div/128/num_svec        time:   [56.089 ns 56.211 ns 56.312 ns]
div/128/rug             time:   [53.020 ns 53.087 ns 53.162 ns]
div/128/ramp            time:   [100.64 ns 100.74 ns 100.85 ns]

div/1024/uint           time:   [75.644 ns 75.781 ns 75.947 ns]
div/1024/num            time:   [131.72 ns 131.90 ns 132.10 ns]
div/1024/num_svec       time:   [123.06 ns 123.25 ns 123.44 ns]
div/1024/rug            time:   [69.630 ns 69.849 ns 70.114 ns]
div/1024/ramp           time:   [161.07 ns 161.39 ns 161.75 ns]

div/4096/uint           time:   [262.75 ns 263.21 ns 263.75 ns]
div/4096/num            time:   [220.91 ns 221.21 ns 221.54 ns]
div/4096/num_svec       time:   [213.32 ns 213.54 ns 213.80 ns]
div/4096/rug            time:   [65.170 ns 65.470 ns 65.815 ns]
div/4096/ramp           time:   [245.73 ns 246.23 ns 246.82 ns]

div/32768/num           time:   [943.26 ns 944.32 ns 945.31 ns]
div/32768/num_svec      time:   [943.19 ns 944.28 ns 945.19 ns]
div/32768/rug           time:   [69.491 ns 69.958 ns 70.768 ns]
div/32768/ramp          time:   [971.72 ns 974.19 ns 976.84 ns]

add/64bits/uint         time:   [650.77 ps 666.87 ps 688.76 ps]
add/64bits/num          time:   [48.326 ns 48.581 ns 48.785 ns]
add/64bits/num_svec     time:   [21.474 ns 21.524 ns 21.569 ns]
add/64bits/rug          time:   [32.636 ns 32.671 ns 32.708 ns]
add/64bits/ramp         time:   [44.907 ns 45.059 ns 45.206 ns]

add/128/uint            time:   [5.4765 ns 5.5104 ns 5.5513 ns]
add/128/num             time:   [49.417 ns 49.721 ns 49.968 ns]
add/128/num_svec        time:   [29.095 ns 29.183 ns 29.259 ns]
add/128/rug             time:   [34.819 ns 34.864 ns 34.916 ns]
add/128/ramp            time:   [50.404 ns 50.586 ns 50.762 ns]

add/1024/uint           time:   [75.976 ns 76.107 ns 76.266 ns]
add/1024/num            time:   [70.476 ns 70.633 ns 70.796 ns]
add/1024/num_svec       time:   [71.654 ns 71.811 ns 71.978 ns]
add/1024/rug            time:   [41.184 ns 41.249 ns 41.315 ns]
add/1024/ramp           time:   [68.910 ns 69.345 ns 69.837 ns]

add/4096/uint           time:   [262.12 ns 262.70 ns 263.38 ns]
add/4096/num            time:   [155.46 ns 156.47 ns 157.49 ns]
add/4096/num_svec       time:   [154.71 ns 155.43 ns 156.08 ns]
add/4096/rug            time:   [59.411 ns 61.338 ns 63.115 ns]
add/4096/ramp           time:   [116.76 ns 117.91 ns 119.13 ns]

add/32768/num           time:   [1.1135 us 1.1257 us 1.1357 us]
add/32768/num_svec      time:   [1.1237 us 1.1358 us 1.1456 us]
add/32768/rug           time:   [238.10 ns 241.42 ns 244.72 ns]
add/32768/ramp          time:   [703.61 ns 707.50 ns 711.29 ns]

clone/64bits/uint       time:   [644.87 ps 650.27 ps 657.00 ps]
clone/64bits/num        time:   [23.173 ns 23.249 ns 23.321 ns]
clone/64bits/num_svec   time:   [4.9729 ns 5.0136 ns 5.0483 ns]
clone/64bits/rug        time:   [22.557 ns 22.598 ns 22.645 ns]
clone/64bits/ramp       time:   [24.831 ns 24.876 ns 24.929 ns]

clone/128/uint          time:   [5.4782 ns 5.5012 ns 5.5294 ns]
clone/128/num           time:   [23.158 ns 23.261 ns 23.372 ns]
clone/128/num_svec      time:   [5.5211 ns 5.5689 ns 5.6102 ns]
clone/128/rug           time:   [20.969 ns 21.015 ns 21.071 ns]
clone/128/ramp          time:   [25.080 ns 25.115 ns 25.154 ns]

clone/1024/uint         time:   [75.928 ns 76.054 ns 76.186 ns]
clone/1024/num          time:   [31.896 ns 32.075 ns 32.252 ns]
clone/1024/num_svec     time:   [32.911 ns 33.354 ns 33.866 ns]
clone/1024/rug          time:   [28.226 ns 28.414 ns 28.641 ns]
clone/1024/ramp         time:   [30.130 ns 30.238 ns 30.347 ns]

clone/4096/uint         time:   [262.76 ns 263.02 ns 263.31 ns]
clone/4096/num          time:   [57.901 ns 59.488 ns 60.894 ns]
clone/4096/num_svec     time:   [57.263 ns 58.916 ns 60.398 ns]
clone/4096/rug          time:   [41.460 ns 43.525 ns 45.424 ns]
clone/4096/ramp         time:   [41.066 ns 41.900 ns 42.788 ns]

clone/32768/num         time:   [384.10 ns 400.37 ns 413.67 ns]
clone/32768/num_svec    time:   [378.69 ns 395.23 ns 408.60 ns]
clone/32768/rug         time:   [154.50 ns 156.85 ns 159.45 ns]
clone/32768/ramp        time:   [184.59 ns 187.15 ns 189.98 ns]
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
