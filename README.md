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

[num]: https://crates.io/crates/num
[rug]: https://crates.io/crates/rug
[ramp]: https://crates.io/crates/ramp
[gmp]: https://gmplib.org
