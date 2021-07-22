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
gcd/64bits/uint         time:   [167.26 ns 167.33 ns 167.41 ns]
gcd/64bits/branchless   time:   [84.554 ns 85.160 ns 85.782 ns]
gcd/64bits/num          time:   [1.1841 us 1.1843 us 1.1846 us]
gcd/64bits/num_svec     time:   [1.0337 us 1.0342 us 1.0348 us]
gcd/64bits/rug          time:   [76.743 ns 76.936 ns 77.136 ns]
gcd/64bits/ramp         time:   [616.75 ns 617.13 ns 617.51 ns]

gcd/128/uint            time:   [629.01 ns 629.28 ns 629.54 ns]
gcd/128/branchless      time:   [402.50 ns 402.65 ns 402.82 ns]
gcd/128/num             time:   [2.3435 us 2.3442 us 2.3450 us]
gcd/128/num_svec        time:   [2.1317 us 2.1325 us 2.1334 us]
gcd/128/rug             time:   [222.59 ns 223.11 ns 223.58 ns]
gcd/128/ramp            time:   [1.3134 us 1.3146 us 1.3161 us]

gcd/1024/num            time:   [21.107 us 21.130 us 21.156 us]
gcd/1024/num_svec       time:   [17.725 us 17.731 us 17.738 us]
gcd/1024/rug            time:   [5.3528 us 5.3551 us 5.3578 us]
gcd/1024/ramp           time:   [16.115 us 16.144 us 16.184 us]

gcd/4096/num            time:   [130.29 us 130.33 us 130.37 us]
gcd/4096/num_svec       time:   [109.74 us 109.80 us 109.88 us]
gcd/4096/rug            time:   [25.571 us 25.577 us 25.584 us]
gcd/4096/ramp           time:   [176.15 us 176.21 us 176.28 us]

gcd/32768/num           time:   [4.6917 ms 4.6933 ms 4.6952 ms]
gcd/32768/num_svec      time:   [4.5085 ms 4.5096 ms 4.5108 ms]
gcd/32768/rug           time:   [419.78 us 420.04 us 420.34 us]
gcd/32768/ramp          time:   [9.9151 ms 9.9178 ms 9.9213 ms]

mul/64bits/uint         time:   [3.3255 ns 3.3284 ns 3.3317 ns]
mul/64bits/num          time:   [37.978 ns 38.297 ns 38.572 ns]
mul/64bits/num_svec     time:   [13.632 ns 13.644 ns 13.657 ns]
mul/64bits/rug          time:   [23.400 ns 23.478 ns 23.559 ns]
mul/64bits/ramp         time:   [56.899 ns 56.932 ns 56.970 ns]

mul/128/uint            time:   [10.143 ns 10.162 ns 10.182 ns]
mul/128/num             time:   [39.266 ns 39.618 ns 39.947 ns]
mul/128/num_svec        time:   [46.277 ns 46.320 ns 46.370 ns]
mul/128/rug             time:   [24.413 ns 24.459 ns 24.505 ns]
mul/128/ramp            time:   [33.602 ns 33.848 ns 34.095 ns]

mul/1024/uint           time:   [90.948 ns 91.055 ns 91.173 ns]
mul/1024/num            time:   [258.95 ns 259.29 ns 259.65 ns]
mul/1024/num_svec       time:   [255.34 ns 255.43 ns 255.52 ns]
mul/1024/rug            time:   [156.35 ns 156.51 ns 156.69 ns]
mul/1024/ramp           time:   [220.17 ns 220.40 ns 220.62 ns]

mul/4096/uint           time:   [268.20 ns 268.51 ns 268.78 ns]
mul/4096/num            time:   [2.9871 us 2.9885 us 2.9900 us]
mul/4096/num_svec       time:   [2.9832 us 2.9845 us 2.9858 us]
mul/4096/rug            time:   [1.3917 us 1.3927 us 1.3938 us]
mul/4096/ramp           time:   [2.2199 us 2.2210 us 2.2222 us]

mul/32768/num           time:   [95.620 us 95.688 us 95.765 us]
mul/32768/num_svec      time:   [79.814 us 79.844 us 79.880 us]
mul/32768/rug           time:   [31.553 us 31.576 us 31.601 us]
mul/32768/ramp          time:   [62.363 us 62.385 us 62.410 us]

mut/64bits/num          time:   [39.178 ns 39.348 ns 39.530 ns]
mut/64bits/num_svec     time:   [14.697 ns 14.706 ns 14.716 ns]
mut/64bits/rug_mut      time:   [6.6020 ns 6.6073 ns 6.6125 ns]
mut/64bits/ramp         time:   [44.348 ns 44.455 ns 44.559 ns]

mut/128/num             time:   [46.580 ns 46.824 ns 47.054 ns]
mut/128/num_svec        time:   [54.009 ns 54.025 ns 54.042 ns]
mut/128/rug_mut         time:   [10.207 ns 10.245 ns 10.300 ns]
mut/128/ramp            time:   [41.280 ns 41.346 ns 41.417 ns]

mut/1024/num            time:   [291.54 ns 291.69 ns 291.87 ns]
mut/1024/num_svec       time:   [298.84 ns 299.00 ns 299.14 ns]
mut/1024/rug_mut        time:   [202.24 ns 202.40 ns 202.57 ns]
mut/1024/ramp           time:   [261.74 ns 262.86 ns 263.99 ns]

mut/4096/num            time:   [2.9775 us 2.9786 us 2.9799 us]
mut/4096/num_svec       time:   [2.9764 us 2.9777 us 2.9791 us]
mut/4096/rug_mut        time:   [1.4236 us 1.4260 us 1.4305 us]
mut/4096/ramp           time:   [2.2458 us 2.2471 us 2.2485 us]

mut/32768/num           time:   [94.800 us 94.860 us 94.928 us]
mut/32768/num_svec      time:   [79.705 us 79.734 us 79.769 us]
mut/32768/rug_mut       time:   [31.647 us 31.707 us 31.771 us]
mut/32768/ramp          time:   [62.335 us 62.362 us 62.392 us]

div/64bits/uint         time:   [3.3311 ns 3.3349 ns 3.3389 ns]
div/64bits/num          time:   [60.101 ns 60.218 ns 60.336 ns]
div/64bits/num_svec     time:   [43.590 ns 43.684 ns 43.773 ns]
div/64bits/rug          time:   [36.470 ns 36.505 ns 36.545 ns]
div/64bits/ramp         time:   [41.111 ns 41.145 ns 41.182 ns]

div/128/uint            time:   [10.382 ns 10.390 ns 10.399 ns]
div/128/num             time:   [93.616 ns 93.653 ns 93.693 ns]
div/128/num_svec        time:   [66.427 ns 66.456 ns 66.486 ns]
div/128/rug             time:   [43.157 ns 43.383 ns 43.729 ns]
div/128/ramp            time:   [83.928 ns 84.038 ns 84.156 ns]

div/1024/uint           time:   [93.123 ns 93.190 ns 93.262 ns]
div/1024/num            time:   [120.27 ns 120.88 ns 121.63 ns]
div/1024/num_svec       time:   [115.46 ns 115.55 ns 115.63 ns]
div/1024/rug            time:   [58.831 ns 58.911 ns 58.998 ns]
div/1024/ramp           time:   [151.52 ns 152.33 ns 153.15 ns]

div/4096/uint           time:   [255.86 ns 256.26 ns 256.68 ns]
div/4096/num            time:   [211.45 ns 212.72 ns 213.94 ns]
div/4096/num_svec       time:   [197.01 ns 199.07 ns 201.19 ns]
div/4096/rug            time:   [64.941 ns 65.050 ns 65.179 ns]
div/4096/ramp           time:   [221.87 ns 223.27 ns 224.67 ns]

div/32768/num           time:   [962.90 ns 974.80 ns 985.34 ns]
div/32768/num_svec      time:   [930.44 ns 944.33 ns 956.80 ns]
div/32768/rug           time:   [68.619 ns 68.770 ns 68.933 ns]
div/32768/ramp          time:   [702.18 ns 703.69 ns 705.00 ns]

add/64bits/uint         time:   [3.1345 ns 3.1379 ns 3.1417 ns]
add/64bits/num          time:   [49.607 ns 49.631 ns 49.653 ns]
add/64bits/num_svec     time:   [27.780 ns 27.862 ns 27.953 ns]
add/64bits/rug          time:   [33.313 ns 33.347 ns 33.384 ns]
add/64bits/ramp         time:   [48.566 ns 48.596 ns 48.629 ns]

add/128/uint            time:   [10.481 ns 10.490 ns 10.498 ns]
add/128/num             time:   [56.595 ns 56.630 ns 56.669 ns]
add/128/num_svec        time:   [35.341 ns 35.356 ns 35.371 ns]
add/128/rug             time:   [33.997 ns 34.034 ns 34.073 ns]
add/128/ramp            time:   [51.682 ns 51.817 ns 51.937 ns]

add/1024/uint           time:   [95.093 ns 95.200 ns 95.325 ns]
add/1024/num            time:   [60.204 ns 60.281 ns 60.350 ns]
add/1024/num_svec       time:   [61.536 ns 61.636 ns 61.730 ns]
add/1024/rug            time:   [37.344 ns 37.386 ns 37.430 ns]
add/1024/ramp           time:   [63.286 ns 63.331 ns 63.382 ns]

add/4096/uint           time:   [270.35 ns 270.74 ns 271.12 ns]
add/4096/num            time:   [104.19 ns 108.85 ns 112.88 ns]
add/4096/num_svec       time:   [106.84 ns 111.63 ns 115.75 ns]
add/4096/rug            time:   [54.905 ns 54.989 ns 55.070 ns]
add/4096/ramp           time:   [111.79 ns 111.99 ns 112.20 ns]

add/32768/num           time:   [550.76 ns 592.63 ns 629.03 ns]
add/32768/num_svec      time:   [551.18 ns 590.90 ns 625.08 ns]
add/32768/rug           time:   [236.60 ns 237.29 ns 238.03 ns]
add/32768/ramp          time:   [632.44 ns 633.25 ns 634.09 ns]

clone/64bits/uint       time:   [3.3028 ns 3.3053 ns 3.3079 ns]
clone/64bits/num        time:   [19.721 ns 19.740 ns 19.759 ns]
clone/64bits/num_svec   time:   [10.345 ns 10.352 ns 10.359 ns]
clone/64bits/rug        time:   [17.457 ns 17.470 ns 17.484 ns]
clone/64bits/ramp       time:   [25.881 ns 25.979 ns 26.076 ns]

clone/128/uint          time:   [10.379 ns 10.388 ns 10.397 ns]
clone/128/num           time:   [19.494 ns 19.505 ns 19.517 ns]
clone/128/num_svec      time:   [10.324 ns 10.331 ns 10.337 ns]
clone/128/rug           time:   [18.230 ns 18.262 ns 18.293 ns]
clone/128/ramp          time:   [30.110 ns 30.165 ns 30.220 ns]

clone/1024/uint         time:   [94.849 ns 94.935 ns 95.029 ns]
clone/1024/num          time:   [24.036 ns 24.236 ns 24.417 ns]
clone/1024/num_svec     time:   [24.899 ns 25.128 ns 25.335 ns]
clone/1024/rug          time:   [22.149 ns 22.187 ns 22.220 ns]
clone/1024/ramp         time:   [29.393 ns 29.475 ns 29.555 ns]

clone/4096/uint         time:   [269.13 ns 269.46 ns 269.78 ns]
clone/4096/num          time:   [56.909 ns 62.068 ns 66.558 ns]
clone/4096/num_svec     time:   [56.704 ns 61.871 ns 66.383 ns]
clone/4096/rug          time:   [32.764 ns 32.846 ns 32.914 ns]
clone/4096/ramp         time:   [41.951 ns 42.028 ns 42.109 ns]

clone/32768/num         time:   [278.39 ns 304.92 ns 327.48 ns]
clone/32768/num_svec    time:   [272.20 ns 298.19 ns 320.18 ns]
clone/32768/rug         time:   [140.52 ns 140.92 ns 141.29 ns]
clone/32768/ramp        time:   [183.62 ns 184.21 ns 184.82 ns]
```

[num]: https://crates.io/crates/num
[rug]: https://crates.io/crates/rug
[ramp]: https://crates.io/crates/ramp
[gmp]: https://gmplib.org
