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
gcd/64bits/num          time:   [830.77 ns 831.84 ns 833.22 ns]
gcd/64bits/rug          time:   [186.72 ns 186.97 ns 187.33 ns]
gcd/64bits/ramp         time:   [742.62 ns 743.27 ns 744.04 ns]

gcd/128bits/num         time:   [1.6514 us 1.6533 us 1.6553 us]
gcd/128bits/rug         time:   [302.38 ns 302.62 ns 302.93 ns]
gcd/128bits/ramp        time:   [1.4238 us 1.4245 us 1.4254 us]

gcd/1024bits/num        time:   [17.883 us 17.897 us 17.913 us]
gcd/1024bits/rug        time:   [4.5439 us 4.5506 us 4.5585 us]
gcd/1024bits/ramp       time:   [15.536 us 15.555 us 15.581 us]

gcd/4096bits/num        time:   [167.31 us 167.54 us 167.80 us]
gcd/4096bits/rug        time:   [21.506 us 21.512 us 21.519 us]
gcd/4096bits/ramp       time:   [151.23 us 151.40 us 151.61 us]

gcd/32768bits/num       time:   [9.5157 ms 9.5230 ms 9.5319 ms]
gcd/32768bits/rug       time:   [387.13 us 387.46 us 387.88 us]
gcd/32768bits/ramp      time:   [9.9340 ms 9.9405 ms 9.9491 ms]

mul/64bits/uint         time:   [640.38 ps 654.17 ps 670.54 ps]
mul/64bits/num          time:   [101.29 ns 103.15 ns 104.80 ns]
mul/64bits/rug          time:   [115.55 ns 115.93 ns 116.67 ns]
mul/64bits/ramp         time:   [88.979 ns 89.143 ns 89.367 ns]

mul/128bits/uint        time:   [1.7664 ns 1.7882 ns 1.8146 ns]
mul/128bits/num         time:   [109.22 ns 109.44 ns 109.66 ns]
mul/128bits/rug         time:   [78.464 ns 78.839 ns 79.322 ns]
mul/128bits/ramp        time:   [80.607 ns 80.991 ns 81.485 ns]

mul/1024bits/uint       time:   [240.25 ns 240.48 ns 240.75 ns]
mul/1024bits/num        time:   [340.57 ns 341.39 ns 342.23 ns]
mul/1024bits/rug        time:   [270.09 ns 270.48 ns 270.88 ns]
mul/1024bits/ramp       time:   [542.66 ns 543.12 ns 543.62 ns]

mul/4096bits/uint       time:   [5.1123 us 5.1201 us 5.1293 us]
mul/4096bits/num        time:   [3.4466 us 3.4500 us 3.4530 us]
mul/4096bits/rug        time:   [1.4324 us 1.4340 us 1.4360 us]
mul/4096bits/ramp       time:   [4.2776 us 4.2806 us 4.2842 us]

mul/32768bits/num       time:   [112.85 us 112.89 us 112.93 us]
mul/32768bits/rug       time:   [29.596 us 29.620 us 29.647 us]
mul/32768bits/ramp      time:   [119.85 us 119.96 us 120.09 us]

div/64bits/uint         time:   [647.51 ps 654.18 ps 662.69 ps]
div/64bits/num          time:   [132.28 ns 132.82 ns 133.69 ns]
div/64bits/rug          time:   [127.69 ns 127.82 ns 127.98 ns]
div/64bits/ramp         time:   [81.677 ns 81.822 ns 82.000 ns]

div/128bits/uint        time:   [5.5019 ns 5.5427 ns 5.5886 ns]
div/128bits/num         time:   [182.20 ns 182.61 ns 183.07 ns]
div/128bits/rug         time:   [147.44 ns 147.55 ns 147.70 ns]
div/128bits/ramp        time:   [240.54 ns 240.81 ns 241.13 ns]

div/1024bits/uint       time:   [78.993 ns 79.472 ns 80.304 ns]
div/1024bits/num        time:   [234.10 ns 234.90 ns 235.67 ns]
div/1024bits/rug        time:   [164.58 ns 164.82 ns 165.11 ns]
div/1024bits/ramp       time:   [285.48 ns 286.14 ns 286.88 ns]

div/4096bits/uint       time:   [275.68 ns 277.25 ns 280.11 ns]
div/4096bits/num        time:   [337.34 ns 338.52 ns 339.87 ns]
div/4096bits/rug        time:   [124.01 ns 124.40 ns 124.88 ns]
div/4096bits/ramp       time:   [309.24 ns 310.00 ns 310.91 ns]

div/32768bits/num       time:   [1.1426 us 1.1712 us 1.1992 us]
div/32768bits/rug       time:   [164.14 ns 165.93 ns 168.43 ns]
div/32768bits/ramp      time:   [1.1132 us 1.1163 us 1.1193 us]

add/64bits/uint         time:   [473.43 ps 491.49 ps 513.51 ps]
add/64bits/num          time:   [81.399 ns 81.571 ns 81.757 ns]
add/64bits/rug          time:   [124.95 ns 125.11 ns 125.32 ns]
add/64bits/ramp         time:   [88.173 ns 88.292 ns 88.454 ns]

add/128bits/uint        time:   [1.0256 ns 1.0808 ns 1.1610 ns]
add/128bits/num         time:   [82.482 ns 82.667 ns 82.853 ns]
add/128bits/rug         time:   [81.397 ns 81.527 ns 81.714 ns]
add/128bits/ramp        time:   [141.69 ns 141.84 ns 142.03 ns]

add/1024bits/uint       time:   [55.191 ns 55.411 ns 55.706 ns]
add/1024bits/num        time:   [128.12 ns 131.31 ns 134.30 ns]
add/1024bits/rug        time:   [83.319 ns 83.398 ns 83.477 ns]
add/1024bits/ramp       time:   [174.09 ns 174.54 ns 175.17 ns]

add/4096bits/uint       time:   [255.50 ns 257.60 ns 260.80 ns]
add/4096bits/num        time:   [234.95 ns 236.81 ns 238.45 ns]
add/4096bits/rug        time:   [115.61 ns 115.80 ns 116.05 ns]
add/4096bits/ramp       time:   [180.47 ns 180.69 ns 180.94 ns]

add/32768bits/num       time:   [825.97 ns 829.72 ns 833.99 ns]
add/32768bits/rug       time:   [244.27 ns 247.69 ns 252.24 ns]
add/32768bits/ramp      time:   [778.82 ns 780.26 ns 781.92 ns]
```

## 3950X

```
gcd/64bits/num          time:   [980.73 ns 981.14 ns 981.54 ns]
gcd/64bits/rug          time:   [91.093 ns 91.224 ns 91.350 ns]
gcd/64bits/ramp         time:   [625.48 ns 625.90 ns 626.34 ns]

gcd/128bits/num         time:   [1.9583 us 1.9592 us 1.9601 us]
gcd/128bits/rug         time:   [263.77 ns 263.99 ns 264.19 ns]
gcd/128bits/ramp        time:   [1.3596 us 1.3604 us 1.3614 us]

gcd/1024bits/num        time:   [18.101 us 18.109 us 18.117 us]
gcd/1024bits/rug        time:   [5.6577 us 5.6608 us 5.6640 us]
gcd/1024bits/ramp       time:   [16.691 us 16.699 us 16.709 us]

gcd/4096bits/num        time:   [116.43 us 116.48 us 116.53 us]
gcd/4096bits/rug        time:   [26.592 us 26.611 us 26.633 us]
gcd/4096bits/ramp       time:   [181.92 us 182.03 us 182.16 us]

gcd/32768bits/num       time:   [4.7134 ms 4.7164 ms 4.7195 ms]
gcd/32768bits/rug       time:   [439.33 us 439.63 us 439.98 us]
gcd/32768bits/ramp      time:   [10.347 ms 10.353 ms 10.358 ms]

mul/64bits/uint         time:   [702.21 ps 703.71 ps 705.45 ps]
mul/64bits/num          time:   [58.256 ns 58.287 ns 58.318 ns]
mul/64bits/rug          time:   [31.128 ns 31.152 ns 31.176 ns]
mul/64bits/ramp         time:   [56.471 ns 56.633 ns 56.795 ns]

mul/128bits/uint        time:   [1.9680 ns 1.9725 ns 1.9773 ns]
mul/128bits/num         time:   [80.803 ns 80.937 ns 81.049 ns]
mul/128bits/rug         time:   [32.206 ns 32.329 ns 32.522 ns]
mul/128bits/ramp        time:   [69.232 ns 69.298 ns 69.361 ns]

mul/1024bits/uint       time:   [307.28 ns 307.61 ns 307.96 ns]
mul/1024bits/num        time:   [288.07 ns 288.31 ns 288.54 ns]
mul/1024bits/rug        time:   [217.02 ns 217.42 ns 217.81 ns]
mul/1024bits/ramp       time:   [261.72 ns 262.39 ns 263.13 ns]

mul/4096bits/uint       time:   [5.8614 us 5.8678 us 5.8747 us]
mul/4096bits/num        time:   [2.8362 us 2.8396 us 2.8434 us]
mul/4096bits/rug        time:   [1.3950 us 1.3961 us 1.3972 us]
mul/4096bits/ramp       time:   [2.1982 us 2.2008 us 2.2034 us]

mul/32768bits/num       time:   [90.095 us 90.163 us 90.237 us]
mul/32768bits/rug       time:   [31.905 us 31.941 us 31.980 us]
mul/32768bits/ramp      time:   [63.691 us 63.735 us 63.778 us]

div/64bits/uint         time:   [3.3467 ns 3.3495 ns 3.3525 ns]
div/64bits/num          time:   [65.419 ns 65.446 ns 65.474 ns]
div/64bits/rug          time:   [45.649 ns 45.745 ns 45.850 ns]
div/64bits/ramp         time:   [50.399 ns 50.438 ns 50.478 ns]

div/128bits/uint        time:   [10.246 ns 10.293 ns 10.354 ns]
div/128bits/num         time:   [107.51 ns 107.64 ns 107.77 ns]
div/128bits/rug         time:   [77.877 ns 78.477 ns 78.966 ns]
div/128bits/ramp        time:   [90.324 ns 90.399 ns 90.473 ns]

div/1024bits/uint       time:   [99.092 ns 99.324 ns 99.595 ns]
div/1024bits/num        time:   [145.66 ns 146.01 ns 146.39 ns]
div/1024bits/rug        time:   [104.79 ns 104.90 ns 105.03 ns]
div/1024bits/ramp       time:   [191.57 ns 192.28 ns 193.04 ns]

div/4096bits/uint       time:   [290.62 ns 291.05 ns 291.48 ns]
div/4096bits/num        time:   [248.76 ns 249.55 ns 250.35 ns]
div/4096bits/rug        time:   [139.03 ns 139.43 ns 139.77 ns]
div/4096bits/ramp       time:   [268.43 ns 269.08 ns 269.70 ns]

div/32768bits/num       time:   [1.0093 us 1.0211 us 1.0322 us]
div/32768bits/rug       time:   [147.47 ns 149.00 ns 150.26 ns]
div/32768bits/ramp      time:   [831.19 ns 834.27 ns 836.91 ns]

add/64bits/uint         time:   [628.12 ps 630.45 ps 633.28 ps]
add/64bits/num          time:   [66.988 ns 67.052 ns 67.113 ns]
add/64bits/rug          time:   [40.561 ns 40.588 ns 40.615 ns]
add/64bits/ramp         time:   [58.539 ns 58.576 ns 58.615 ns]

add/128bits/uint        time:   [795.87 ps 800.16 ps 805.45 ps]
add/128bits/num         time:   [66.322 ns 66.397 ns 66.470 ns]
add/128bits/rug         time:   [59.298 ns 59.384 ns 59.463 ns]
add/128bits/ramp        time:   [53.181 ns 53.229 ns 53.281 ns]

add/1024bits/uint       time:   [55.241 ns 55.363 ns 55.479 ns]
add/1024bits/num        time:   [75.225 ns 75.618 ns 76.007 ns]
add/1024bits/rug        time:   [80.558 ns 80.768 ns 81.015 ns]
add/1024bits/ramp       time:   [65.506 ns 65.645 ns 65.792 ns]

add/4096bits/uint       time:   [220.45 ns 221.01 ns 221.50 ns]
add/4096bits/num        time:   [108.10 ns 110.97 ns 113.49 ns]
add/4096bits/rug        time:   [123.56 ns 123.89 ns 124.22 ns]
add/4096bits/ramp       time:   [114.38 ns 114.89 ns 115.34 ns]

add/32768bits/num       time:   [458.08 ns 477.35 ns 494.64 ns]
add/32768bits/rug       time:   [289.68 ns 292.16 ns 294.24 ns]
add/32768bits/ramp      time:   [690.49 ns 692.34 ns 694.10 ns]
```

[num]: https://crates.io/crates/num
[rug]: https://crates.io/crates/rug
[ramp]: https://crates.io/crates/ramp
[gmp]: https://gmplib.org
