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

```
gcd/64bits/native       time:   [75.055 ns 75.313 ns 75.674 ns]
gcd/64bits/num          time:   [615.11 ns 617.90 ns 620.57 ns]
gcd/64bits/rug          time:   [66.573 ns 66.846 ns 67.120 ns]
gcd/64bits/ramp         time:   [369.34 ns 370.75 ns 372.25 ns]

gcd/128bits/num         time:   [1.4324 us 1.4417 us 1.4512 us]
gcd/128bits/rug         time:   [178.23 ns 178.56 ns 178.96 ns]
gcd/128bits/ramp        time:   [787.39 ns 788.61 ns 789.96 ns]

gcd/1024bits/num        time:   [16.436 us 16.456 us 16.478 us]
gcd/1024bits/rug        time:   [3.1053 us 3.1090 us 3.1140 us]
gcd/1024bits/ramp       time:   [11.347 us 11.380 us 11.415 us]

gcd/4096bits/num        time:   [163.52 us 165.30 us 167.45 us]
gcd/4096bits/rug        time:   [16.006 us 16.147 us 16.321 us]
gcd/4096bits/ramp       time:   [145.15 us 145.73 us 146.36 us]

gcd/32768bits/num       time:   [9.5474 ms 9.5659 ms 9.5858 ms]
gcd/32768bits/rug       time:   [367.52 us 368.61 us 369.66 us]
gcd/32768bits/ramp      time:   [9.9427 ms 9.9481 ms 9.9548 ms]

mul/64bits/native       time:   [3.9357 ns 3.9378 ns 3.9402 ns]
mul/64bits/num          time:   [33.312 ns 33.356 ns 33.406 ns]
mul/64bits/rug          time:   [29.716 ns 29.783 ns 29.858 ns]
mul/64bits/ramp         time:   [54.423 ns 54.655 ns 54.993 ns]

mul/128bits/num         time:   [36.687 ns 36.803 ns 36.934 ns]
mul/128bits/rug         time:   [34.843 ns 34.931 ns 35.016 ns]
mul/128bits/ramp        time:   [36.389 ns 36.483 ns 36.590 ns]

mul/1024bits/num        time:   [259.43 ns 262.17 ns 265.05 ns]
mul/1024bits/rug        time:   [181.46 ns 182.78 ns 184.20 ns]
mul/1024bits/ramp       time:   [469.58 ns 470.52 ns 471.53 ns]

mul/4096bits/num        time:   [3.2926 us 3.2982 us 3.3042 us]
mul/4096bits/rug        time:   [1.3051 us 1.3083 us 1.3124 us]
mul/4096bits/ramp       time:   [4.2432 us 4.2610 us 4.2889 us]

mul/32768bits/num       time:   [111.99 us 112.12 us 112.30 us]
mul/32768bits/rug       time:   [29.283 us 29.316 us 29.349 us]
mul/32768bits/ramp      time:   [119.66 us 119.79 us 119.92 us]

div/64bits/native       time:   [4.0349 ns 4.0393 ns 4.0448 ns]
div/64bits/num          time:   [85.769 ns 85.979 ns 86.347 ns]
div/64bits/rug          time:   [34.345 ns 34.397 ns 34.452 ns]
div/64bits/ramp         time:   [34.374 ns 34.408 ns 34.448 ns]

div/128bits/num         time:   [104.49 ns 105.16 ns 105.74 ns]
div/128bits/rug         time:   [36.743 ns 36.782 ns 36.825 ns]
div/128bits/ramp        time:   [152.98 ns 155.36 ns 157.73 ns]

div/1024bits/num        time:   [124.37 ns 125.81 ns 127.21 ns]
div/1024bits/rug        time:   [47.130 ns 47.176 ns 47.230 ns]
div/1024bits/ramp       time:   [159.06 ns 159.81 ns 160.70 ns]

div/4096bits/num        time:   [434.24 ns 437.11 ns 439.96 ns]
div/4096bits/rug        time:   [62.682 ns 63.150 ns 63.721 ns]
div/4096bits/ramp       time:   [421.90 ns 422.37 ns 422.82 ns]

div/32768bits/num       time:   [837.39 ns 837.72 ns 838.07 ns]
div/32768bits/rug       time:   [47.400 ns 47.420 ns 47.441 ns]
div/32768bits/ramp      time:   [1.0238 us 1.0245 us 1.0252 us]

add/64bits/native       time:   [4.0199 ns 4.0237 ns 4.0283 ns]
add/64bits/num          time:   [88.535 ns 89.647 ns 90.899 ns]
add/64bits/rug          time:   [28.447 ns 28.497 ns 28.547 ns]
add/64bits/ramp         time:   [55.690 ns 55.802 ns 55.963 ns]

add/128bits/num         time:   [91.990 ns 93.102 ns 94.176 ns]
add/128bits/rug         time:   [28.374 ns 28.405 ns 28.442 ns]
add/128bits/ramp        time:   [99.565 ns 100.45 ns 101.33 ns]

add/1024bits/num        time:   [104.11 ns 105.64 ns 107.08 ns]
add/1024bits/rug        time:   [30.435 ns 30.486 ns 30.540 ns]
add/1024bits/ramp       time:   [119.01 ns 119.94 ns 120.96 ns]

add/4096bits/num        time:   [126.86 ns 127.44 ns 128.18 ns]
add/4096bits/rug        time:   [72.101 ns 72.243 ns 72.359 ns]
add/4096bits/ramp       time:   [109.30 ns 109.46 ns 109.65 ns]

add/32768bits/num       time:   [729.39 ns 730.52 ns 731.73 ns]
add/32768bits/rug       time:   [187.72 ns 188.19 ns 188.68 ns]
add/32768bits/ramp      time:   [614.46 ns 614.94 ns 615.49 ns]
```

[num]: https://crates.io/crates/num
[rug]: https://crates.io/crates/rug
[ramp]: https://crates.io/crates/ramp
[gmp]: https://gmplib.org
