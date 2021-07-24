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
gcd/64bits/uint         time:   [89.468 ns 89.580 ns 89.748 ns]
gcd/64bits/branchless   time:   [78.486 ns 78.521 ns 78.566 ns]
gcd/64bits/num          time:   [782.86 ns 785.06 ns 787.47 ns]
gcd/64bits/num_svec     time:   [767.71 ns 769.20 ns 770.69 ns]
gcd/64bits/rug          time:   [92.243 ns 92.347 ns 92.465 ns]
gcd/64bits/ramp         time:   [640.62 ns 642.47 ns 644.51 ns]
gcd/64bits/ubig         time:   [760.43 ns 761.73 ns 763.50 ns]

gcd/128/uint            time:   [559.59 ns 559.96 ns 560.35 ns]
gcd/128/branchless      time:   [510.66 ns 510.90 ns 511.16 ns]
gcd/128/num             time:   [1.5971 us 1.6001 us 1.6034 us]
gcd/128/num_svec        time:   [1.7142 us 1.7171 us 1.7195 us]
gcd/128/rug             time:   [207.84 ns 208.03 ns 208.24 ns]
gcd/128/ramp            time:   [1.2787 us 1.2819 us 1.2853 us]
gcd/128/ubig            time:   [2.3143 us 2.3171 us 2.3200 us]

gcd/1024/num            time:   [17.860 us 17.870 us 17.880 us]
gcd/1024/num_svec       time:   [17.126 us 17.150 us 17.184 us]
gcd/1024/rug            time:   [4.4507 us 4.4556 us 4.4611 us]
gcd/1024/ramp           time:   [15.402 us 15.443 us 15.486 us]
gcd/1024/ubig           time:   [27.521 us 27.566 us 27.613 us]

gcd/4096/num            time:   [168.10 us 168.25 us 168.41 us]
gcd/4096/num_svec       time:   [167.95 us 168.11 us 168.28 us]
gcd/4096/rug            time:   [21.647 us 21.689 us 21.730 us]
gcd/4096/ramp           time:   [152.30 us 152.72 us 153.22 us]
gcd/4096/ubig           time:   [213.04 us 213.43 us 213.84 us]

gcd/32768/num           time:   [9.5425 ms 9.5463 ms 9.5507 ms]
gcd/32768/num_svec      time:   [9.6564 ms 9.6597 ms 9.6633 ms]
gcd/32768/rug           time:   [387.99 us 388.12 us 388.26 us]
gcd/32768/ramp          time:   [9.9475 ms 9.9570 ms 9.9673 ms]
gcd/32768/ubig          time:   [10.160 ms 10.168 ms 10.177 ms]

mul/64bits/uint         time:   [962.73 ps 973.52 ps 987.97 ps]
mul/64bits/num          time:   [38.372 ns 38.484 ns 38.598 ns]
mul/64bits/unum         time:   [29.377 ns 29.499 ns 29.620 ns]
mul/64bits/num_svec     time:   [6.5527 ns 6.6016 ns 6.6452 ns]
mul/64bits/unum_svec    time:   [4.1087 ns 4.1774 ns 4.2367 ns]
mul/64bits/rug          time:   [24.710 ns 24.873 ns 25.083 ns]
mul/64bits/ramp         time:   [50.239 ns 50.377 ns 50.520 ns]
mul/64bits/ibig         time:   [27.520 ns 27.621 ns 27.732 ns]
mul/64bits/ubig         time:   [26.706 ns 26.817 ns 26.932 ns]

mul/128/uint            time:   [2.0339 ns 2.0478 ns 2.0659 ns]
mul/128/num             time:   [43.128 ns 43.277 ns 43.419 ns]
mul/128/unum            time:   [34.186 ns 34.339 ns 34.484 ns]
mul/128/num_svec        time:   [37.085 ns 37.266 ns 37.436 ns]
mul/128/unum_svec       time:   [35.469 ns 35.647 ns 35.804 ns]
mul/128/rug             time:   [30.753 ns 30.935 ns 31.131 ns]
mul/128/ramp            time:   [36.485 ns 36.600 ns 36.716 ns]
mul/128/ibig            time:   [35.489 ns 35.712 ns 35.936 ns]
mul/128/ubig            time:   [34.283 ns 34.476 ns 34.653 ns]

mul/1024/uint           time:   [240.03 ns 240.61 ns 241.33 ns]
mul/1024/num            time:   [257.05 ns 258.26 ns 259.31 ns]
mul/1024/unum           time:   [245.08 ns 246.37 ns 247.73 ns]
mul/1024/num_svec       time:   [249.67 ns 250.37 ns 251.14 ns]
mul/1024/unum_svec      time:   [250.02 ns 250.70 ns 251.37 ns]
mul/1024/rug            time:   [175.14 ns 175.60 ns 176.15 ns]
mul/1024/ramp           time:   [444.13 ns 444.98 ns 445.88 ns]
mul/1024/ibig           time:   [248.03 ns 248.64 ns 249.38 ns]
mul/1024/ubig           time:   [254.59 ns 256.11 ns 257.58 ns]

mul/4096/uint           time:   [5.0928 us 5.0987 us 5.1055 us]
mul/4096/num            time:   [3.3689 us 3.3727 us 3.3767 us]
mul/4096/unum           time:   [3.3650 us 3.3749 us 3.3851 us]
mul/4096/num_svec       time:   [3.3891 us 3.3991 us 3.4106 us]
mul/4096/unum_svec      time:   [3.3620 us 3.3652 us 3.3684 us]
mul/4096/rug            time:   [1.3685 us 1.3770 us 1.3860 us]
mul/4096/ramp           time:   [4.2545 us 4.2706 us 4.2880 us]
mul/4096/ibig           time:   [2.9850 us 2.9967 us 3.0091 us]
mul/4096/ubig           time:   [2.9997 us 3.0119 us 3.0258 us]

mul/32768/num           time:   [112.58 us 112.65 us 112.73 us]
mul/32768/unum          time:   [111.68 us 111.73 us 111.79 us]
mul/32768/num_svec      time:   [106.45 us 106.50 us 106.55 us]
mul/32768/unum_svec     time:   [106.85 us 106.95 us 107.03 us]
mul/32768/rug           time:   [29.492 us 29.514 us 29.536 us]
mul/32768/ramp          time:   [119.70 us 119.74 us 119.79 us]
mul/32768/ibig          time:   [91.401 us 91.446 us 91.497 us]
mul/32768/ubig          time:   [91.455 us 91.540 us 91.656 us]

mula/64bits/uint        time:   [960.87 ps 962.47 ps 964.46 ps]
mula/64bits/num         time:   [66.894 ns 67.083 ns 67.253 ns]
mula/64bits/unum        time:   [57.017 ns 57.189 ns 57.366 ns]
mula/64bits/num_svec    time:   [10.020 ns 10.032 ns 10.044 ns]
mula/64bits/unum_svec   time:   [3.1152 ns 3.1190 ns 3.1233 ns]
mula/64bits/rug         time:   [4.0280 ns 4.0409 ns 4.0551 ns]
mula/64bits/ramp        time:   [53.766 ns 53.799 ns 53.835 ns]
mula/64bits/ibig        time:   [27.663 ns 27.689 ns 27.718 ns]
mula/64bits/ubig        time:   [26.475 ns 26.502 ns 26.531 ns]

mula/128/uint           time:   [2.1702 ns 2.1739 ns 2.1780 ns]
mula/128/num            time:   [69.972 ns 70.068 ns 70.172 ns]
mula/128/unum           time:   [61.611 ns 63.489 ns 66.135 ns]
mula/128/num_svec       time:   [41.228 ns 41.376 ns 41.548 ns]
mula/128/unum_svec      time:   [35.121 ns 35.227 ns 35.341 ns]
mula/128/rug            time:   [15.883 ns 15.935 ns 15.994 ns]
mula/128/ramp           time:   [40.542 ns 40.640 ns 40.763 ns]
mula/128/ibig           time:   [74.862 ns 75.036 ns 75.245 ns]
mula/128/ubig           time:   [66.774 ns 66.967 ns 67.149 ns]

mula/1024/uint          time:   [240.79 ns 241.24 ns 241.70 ns]
mula/1024/num           time:   [310.82 ns 312.85 ns 314.64 ns]
mula/1024/unum          time:   [303.00 ns 304.75 ns 306.29 ns]
mula/1024/num_svec      time:   [311.92 ns 313.83 ns 315.50 ns]
mula/1024/unum_svec     time:   [306.04 ns 307.80 ns 309.30 ns]
mula/1024/rug           time:   [217.23 ns 218.39 ns 219.59 ns]
mula/1024/ramp          time:   [490.90 ns 491.85 ns 492.90 ns]
mula/1024/ibig          time:   [297.14 ns 297.94 ns 298.78 ns]
mula/1024/ubig          time:   [289.88 ns 291.07 ns 292.11 ns]

mula/4096/uint          time:   [5.0669 us 5.0692 us 5.0718 us]
mula/4096/num           time:   [3.3826 us 3.3865 us 3.3901 us]
mula/4096/unum          time:   [3.3768 us 3.3784 us 3.3800 us]
mula/4096/num_svec      time:   [3.3970 us 3.3992 us 3.4012 us]
mula/4096/unum_svec     time:   [3.3909 us 3.3933 us 3.3956 us]
mula/4096/rug           time:   [1.3843 us 1.3859 us 1.3875 us]
mula/4096/ramp          time:   [4.2571 us 4.2594 us 4.2622 us]
mula/4096/ibig          time:   [2.9878 us 2.9900 us 2.9926 us]
mula/4096/ubig          time:   [2.9938 us 2.9963 us 2.9992 us]

mula/32768/num          time:   [112.81 us 113.01 us 113.23 us]
mula/32768/unum         time:   [112.44 us 112.49 us 112.55 us]
mula/32768/num_svec     time:   [106.42 us 106.46 us 106.51 us]
mula/32768/unum_svec    time:   [107.02 us 107.15 us 107.25 us]
mula/32768/rug          time:   [29.538 us 29.558 us 29.578 us]
mula/32768/ramp         time:   [119.83 us 119.90 us 119.98 us]
mula/32768/ibig         time:   [91.262 us 91.311 us 91.371 us]
mula/32768/ubig         time:   [91.278 us 91.309 us 91.343 us]

div/64bits/uint         time:   [961.61 ps 970.59 ps 983.12 ps]
div/64bits/num          time:   [73.499 ns 73.626 ns 73.766 ns]
div/64bits/unum         time:   [60.094 ns 60.243 ns 60.410 ns]
div/64bits/num_svec     time:   [25.351 ns 25.436 ns 25.512 ns]
div/64bits/unum_svec    time:   [17.582 ns 17.681 ns 17.773 ns]
div/64bits/rug          time:   [34.515 ns 34.555 ns 34.603 ns]
div/64bits/ramp         time:   [42.106 ns 42.187 ns 42.275 ns]
div/64bits/ibig         time:   [3.5925 ns 3.6347 ns 3.6819 ns]
div/64bits/ubig         time:   [2.4457 ns 2.4712 ns 2.5042 ns]

div/128/uint            time:   [6.0747 ns 6.1125 ns 6.1530 ns]
div/128/num             time:   [111.01 ns 111.14 ns 111.26 ns]
div/128/unum            time:   [96.672 ns 96.792 ns 96.921 ns]
div/128/num_svec        time:   [69.522 ns 69.729 ns 69.912 ns]
div/128/unum_svec       time:   [57.940 ns 58.105 ns 58.268 ns]
div/128/rug             time:   [52.401 ns 52.533 ns 52.683 ns]
div/128/ramp            time:   [101.07 ns 101.35 ns 101.59 ns]
div/128/ibig            time:   [121.50 ns 121.92 ns 122.36 ns]
div/128/ubig            time:   [120.31 ns 120.40 ns 120.50 ns]

div/1024/uint           time:   [76.572 ns 76.703 ns 76.852 ns]
div/1024/num            time:   [131.91 ns 132.12 ns 132.33 ns]
div/1024/unum           time:   [115.59 ns 115.74 ns 115.90 ns]
div/1024/num_svec       time:   [120.93 ns 121.15 ns 121.38 ns]
div/1024/unum_svec      time:   [110.87 ns 111.00 ns 111.14 ns]
div/1024/rug            time:   [68.661 ns 68.930 ns 69.239 ns]
div/1024/ramp           time:   [159.32 ns 159.91 ns 160.67 ns]
div/1024/ibig           time:   [151.78 ns 152.26 ns 152.73 ns]
div/1024/ubig           time:   [149.58 ns 149.89 ns 150.19 ns]

div/4096/uint           time:   [279.97 ns 280.64 ns 281.35 ns]
div/4096/num            time:   [221.54 ns 222.59 ns 223.89 ns]
div/4096/unum           time:   [205.95 ns 206.49 ns 207.14 ns]
div/4096/num_svec       time:   [214.01 ns 214.27 ns 214.56 ns]
div/4096/unum_svec      time:   [206.05 ns 206.97 ns 207.85 ns]
div/4096/rug            time:   [65.190 ns 65.501 ns 65.886 ns]
div/4096/ramp           time:   [247.33 ns 247.91 ns 248.59 ns]
div/4096/ibig           time:   [233.33 ns 233.61 ns 233.94 ns]
div/4096/ubig           time:   [233.76 ns 234.22 ns 234.74 ns]

div/32768/num           time:   [944.08 ns 945.11 ns 945.98 ns]
div/32768/unum          time:   [943.10 ns 943.71 ns 944.38 ns]
div/32768/num_svec      time:   [956.54 ns 957.75 ns 958.72 ns]
div/32768/unum_svec     time:   [965.00 ns 966.87 ns 968.56 ns]
div/32768/rug           time:   [69.360 ns 69.603 ns 69.932 ns]
div/32768/ramp          time:   [971.70 ns 973.79 ns 975.63 ns]
div/32768/ibig          time:   [961.94 ns 971.38 ns 982.03 ns]
div/32768/ubig          time:   [1.0201 us 1.0441 us 1.0709 us]

add/64bits/uint         time:   [963.73 ps 973.21 ps 985.85 ps]
add/64bits/num          time:   [46.937 ns 47.144 ns 47.419 ns]
add/64bits/unum         time:   [47.039 ns 47.133 ns 47.216 ns]
add/64bits/num_svec     time:   [19.624 ns 19.691 ns 19.746 ns]
add/64bits/unum_svec    time:   [10.097 ns 10.163 ns 10.215 ns]
add/64bits/rug          time:   [32.195 ns 32.237 ns 32.284 ns]
add/64bits/ramp         time:   [45.135 ns 45.293 ns 45.445 ns]
add/64bits/ibig         time:   [17.045 ns 17.087 ns 17.133 ns]
add/64bits/ubig         time:   [16.102 ns 16.137 ns 16.177 ns]

add/128/uint            time:   [1.1605 ns 1.1771 ns 1.1978 ns]
add/128/num             time:   [48.125 ns 48.245 ns 48.354 ns]
add/128/unum            time:   [48.546 ns 48.672 ns 48.785 ns]
add/128/num_svec        time:   [30.759 ns 30.903 ns 31.024 ns]
add/128/unum_svec       time:   [28.829 ns 29.043 ns 29.249 ns]
add/128/rug             time:   [34.208 ns 34.281 ns 34.368 ns]
add/128/ramp            time:   [50.681 ns 50.852 ns 51.018 ns]
add/128/ibig            time:   [42.462 ns 42.529 ns 42.603 ns]
add/128/ubig            time:   [35.460 ns 35.531 ns 35.603 ns]

add/1024/uint           time:   [58.042 ns 58.213 ns 58.408 ns]
add/1024/num            time:   [70.471 ns 70.719 ns 70.985 ns]
add/1024/unum           time:   [77.085 ns 77.540 ns 78.044 ns]
add/1024/num_svec       time:   [73.016 ns 73.203 ns 73.407 ns]
add/1024/unum_svec      time:   [81.669 ns 82.288 ns 82.956 ns]
add/1024/rug            time:   [41.792 ns 42.392 ns 43.134 ns]
add/1024/ramp           time:   [68.778 ns 69.248 ns 69.766 ns]
add/1024/ibig           time:   [59.608 ns 59.909 ns 60.258 ns]
add/1024/ubig           time:   [55.533 ns 56.081 ns 56.584 ns]

add/4096/uint           time:   [256.58 ns 257.20 ns 257.91 ns]
add/4096/num            time:   [156.75 ns 157.75 ns 158.74 ns]
add/4096/unum           time:   [169.24 ns 169.93 ns 170.62 ns]
add/4096/num_svec       time:   [158.79 ns 159.67 ns 160.53 ns]
add/4096/unum_svec      time:   [173.12 ns 173.89 ns 174.69 ns]
add/4096/rug            time:   [58.739 ns 60.427 ns 62.011 ns]
add/4096/ramp           time:   [117.37 ns 118.12 ns 118.88 ns]
add/4096/ibig           time:   [125.04 ns 127.07 ns 128.93 ns]
add/4096/ubig           time:   [126.09 ns 128.74 ns 131.19 ns]

add/32768/num           time:   [1.1174 us 1.1270 us 1.1344 us]
add/32768/unum          time:   [1.2101 us 1.2250 us 1.2376 us]
add/32768/num_svec      time:   [1.1349 us 1.1457 us 1.1546 us]
add/32768/unum_svec     time:   [1.2305 us 1.2446 us 1.2562 us]
add/32768/rug           time:   [240.58 ns 244.14 ns 247.79 ns]
add/32768/ramp          time:   [707.21 ns 712.11 ns 717.27 ns]
add/32768/ibig          time:   [849.37 ns 857.83 ns 867.28 ns]
add/32768/ubig          time:   [836.20 ns 846.05 ns 858.77 ns]

adda/64bits/uint        time:   [960.32 ps 962.20 ps 964.80 ps]
adda/64bits/num         time:   [36.617 ns 36.671 ns 36.724 ns]
adda/64bits/unum        time:   [37.954 ns 38.064 ns 38.174 ns]
adda/64bits/num_svec    time:   [16.995 ns 17.007 ns 17.019 ns]
adda/64bits/unum_svec   time:   [7.9059 ns 7.9101 ns 7.9145 ns]
adda/64bits/rug         time:   [12.496 ns 12.516 ns 12.541 ns]
adda/64bits/ramp        time:   [12.155 ns 12.165 ns 12.177 ns]
adda/64bits/ibig        time:   [19.565 ns 19.587 ns 19.612 ns]
adda/64bits/ubig        time:   [17.417 ns 17.438 ns 17.462 ns]

adda/128/uint           time:   [1.5321 ns 1.5358 ns 1.5399 ns]
adda/128/num            time:   [37.328 ns 37.369 ns 37.408 ns]
adda/128/unum           time:   [39.082 ns 39.141 ns 39.188 ns]
adda/128/num_svec       time:   [27.116 ns 27.141 ns 27.170 ns]
adda/128/unum_svec      time:   [22.045 ns 22.153 ns 22.250 ns]
adda/128/rug            time:   [13.055 ns 13.074 ns 13.094 ns]
adda/128/ramp           time:   [12.993 ns 13.010 ns 13.032 ns]
adda/128/ibig           time:   [22.160 ns 22.187 ns 22.225 ns]
adda/128/ubig           time:   [13.936 ns 13.961 ns 13.989 ns]

adda/1024/uint          time:   [58.575 ns 58.679 ns 58.807 ns]
adda/1024/num           time:   [53.927 ns 54.002 ns 54.082 ns]
adda/1024/unum          time:   [67.168 ns 69.062 ns 71.164 ns]
adda/1024/num_svec      time:   [52.257 ns 52.348 ns 52.434 ns]
adda/1024/unum_svec     time:   [69.348 ns 71.181 ns 73.215 ns]
adda/1024/rug           time:   [13.331 ns 13.723 ns 14.057 ns]
adda/1024/ramp          time:   [23.745 ns 23.983 ns 24.209 ns]
adda/1024/ibig          time:   [35.095 ns 35.439 ns 35.753 ns]
adda/1024/ubig          time:   [28.941 ns 29.023 ns 29.126 ns]

adda/4096/uint          time:   [257.97 ns 258.40 ns 258.83 ns]
adda/4096/num           time:   [113.94 ns 114.05 ns 114.18 ns]
adda/4096/unum          time:   [147.58 ns 149.73 ns 151.69 ns]
adda/4096/num_svec      time:   [112.22 ns 112.31 ns 112.39 ns]
adda/4096/unum_svec     time:   [151.25 ns 153.31 ns 155.16 ns]
adda/4096/rug           time:   [29.498 ns 29.614 ns 29.770 ns]
adda/4096/ramp          time:   [63.527 ns 63.702 ns 63.915 ns]
adda/4096/ibig          time:   [86.830 ns 87.035 ns 87.284 ns]
adda/4096/ubig          time:   [83.615 ns 83.804 ns 84.031 ns]

adda/32768/num          time:   [888.10 ns 891.06 ns 894.19 ns]
adda/32768/unum         time:   [982.02 ns 987.51 ns 993.27 ns]
adda/32768/num_svec     time:   [886.12 ns 889.02 ns 892.06 ns]
adda/32768/unum_svec    time:   [1.0232 us 1.0299 us 1.0376 us]
adda/32768/rug          time:   [188.57 ns 189.08 ns 189.72 ns]
adda/32768/ramp         time:   [548.31 ns 549.90 ns 551.76 ns]
adda/32768/ibig         time:   [700.24 ns 704.19 ns 708.85 ns]
adda/32768/ubig         time:   [671.26 ns 673.77 ns 676.67 ns]

shra/64bits/uint        time:   [957.84 ps 959.65 ps 962.17 ps]
shra/64bits/num         time:   [9.6050 ns 9.6128 ns 9.6202 ns]
shra/64bits/unum        time:   [5.1692 ns 5.1933 ns 5.2197 ns]
shra/64bits/num_svec    time:   [8.7274 ns 8.7376 ns 8.7485 ns]
shra/64bits/unum_svec   time:   [3.7043 ns 3.7093 ns 3.7148 ns]
shra/64bits/rug         time:   [7.3529 ns 7.3650 ns 7.3774 ns]
shra/64bits/ramp        time:   [3.5044 ns 3.5122 ns 3.5219 ns]
shra/64bits/ibig        time:   [10.505 ns 10.529 ns 10.554 ns]
shra/64bits/ubig        time:   [3.1989 ns 3.2089 ns 3.2185 ns]

shra/128/uint           time:   [963.16 ps 966.03 ps 969.92 ps]
shra/128/num            time:   [10.094 ns 10.106 ns 10.118 ns]
shra/128/unum           time:   [5.5404 ns 5.5809 ns 5.6233 ns]
shra/128/num_svec       time:   [8.9103 ns 8.9199 ns 8.9307 ns]
shra/128/unum_svec      time:   [4.2299 ns 4.2323 ns 4.2350 ns]
shra/128/rug            time:   [7.7016 ns 7.7152 ns 7.7287 ns]
shra/128/ramp           time:   [3.5081 ns 3.5178 ns 3.5302 ns]
shra/128/ibig           time:   [17.878 ns 17.904 ns 17.932 ns]
shra/128/ubig           time:   [7.6138 ns 7.6350 ns 7.6544 ns]

shra/1024/uint          time:   [10.693 ns 10.742 ns 10.786 ns]
shra/1024/num           time:   [14.583 ns 14.623 ns 14.658 ns]
shra/1024/unum          time:   [11.244 ns 11.310 ns 11.366 ns]
shra/1024/num_svec      time:   [13.374 ns 13.406 ns 13.438 ns]
shra/1024/unum_svec     time:   [8.4588 ns 8.4889 ns 8.5141 ns]
shra/1024/rug           time:   [11.866 ns 12.061 ns 12.227 ns]
shra/1024/ramp          time:   [9.3244 ns 9.5065 ns 9.7305 ns]
shra/1024/ibig          time:   [24.773 ns 25.018 ns 25.222 ns]
shra/1024/ubig          time:   [13.300 ns 13.396 ns 13.497 ns]

shra/4096/uint          time:   [50.908 ns 51.135 ns 51.428 ns]
shra/4096/num           time:   [31.525 ns 31.608 ns 31.693 ns]
shra/4096/unum          time:   [29.292 ns 30.274 ns 31.676 ns]
shra/4096/num_svec      time:   [29.791 ns 29.843 ns 29.899 ns]
shra/4096/unum_svec     time:   [26.700 ns 26.773 ns 26.840 ns]
shra/4096/rug           time:   [24.361 ns 24.437 ns 24.536 ns]
shra/4096/ramp          time:   [35.050 ns 35.166 ns 35.374 ns]
shra/4096/ibig          time:   [49.001 ns 49.342 ns 49.782 ns]
shra/4096/ubig          time:   [40.377 ns 40.604 ns 40.871 ns]

shra/32768/num          time:   [270.18 ns 283.70 ns 295.73 ns]
shra/32768/unum         time:   [263.75 ns 278.21 ns 290.96 ns]
shra/32768/num_svec     time:   [285.83 ns 299.14 ns 310.89 ns]
shra/32768/unum_svec    time:   [297.55 ns 307.42 ns 316.55 ns]
shra/32768/rug          time:   [167.46 ns 167.64 ns 167.85 ns]
shra/32768/ramp         time:   [345.77 ns 346.21 ns 346.69 ns]
shra/32768/ibig         time:   [235.26 ns 236.82 ns 238.73 ns]
shra/32768/ubig         time:   [228.60 ns 230.30 ns 232.36 ns]

cmp/64bits/uint         time:   [962.85 ps 967.96 ps 974.70 ps]
cmp/64bits/num          time:   [6.4066 ns 6.4184 ns 6.4337 ns]
cmp/64bits/unum         time:   [1.3325 ns 1.3362 ns 1.3400 ns]
cmp/64bits/num_svec     time:   [6.6038 ns 6.6121 ns 6.6209 ns]
cmp/64bits/unum_svec    time:   [2.1070 ns 2.1121 ns 2.1175 ns]
cmp/64bits/rug          time:   [4.4393 ns 4.4628 ns 4.4888 ns]
cmp/64bits/ramp         time:   [6.1713 ns 6.1885 ns 6.2091 ns]
cmp/64bits/ibig         time:   [6.7842 ns 6.7937 ns 6.8044 ns]
cmp/64bits/ubig         time:   [1.6326 ns 1.6397 ns 1.6477 ns]

cmp/128/uint            time:   [1.3751 ns 1.3801 ns 1.3867 ns]
cmp/128/num             time:   [6.3983 ns 6.4059 ns 6.4139 ns]
cmp/128/unum            time:   [1.3322 ns 1.3365 ns 1.3409 ns]
cmp/128/num_svec        time:   [6.6165 ns 6.6226 ns 6.6284 ns]
cmp/128/unum_svec       time:   [2.1012 ns 2.1046 ns 2.1081 ns]
cmp/128/rug             time:   [4.4348 ns 4.4682 ns 4.5071 ns]
cmp/128/ramp            time:   [6.1720 ns 6.1873 ns 6.2071 ns]
cmp/128/ibig            time:   [6.9671 ns 6.9791 ns 6.9936 ns]
cmp/128/ubig            time:   [1.8942 ns 1.9014 ns 1.9106 ns]

cmp/1024/uint           time:   [1.4666 ns 1.4925 ns 1.5235 ns]
cmp/1024/num            time:   [6.4669 ns 6.4787 ns 6.4889 ns]
cmp/1024/unum           time:   [2.3351 ns 2.4029 ns 2.4616 ns]
cmp/1024/num_svec       time:   [6.6397 ns 6.6558 ns 6.6705 ns]
cmp/1024/unum_svec      time:   [2.9078 ns 2.9912 ns 3.0665 ns]
cmp/1024/rug            time:   [2.5776 ns 2.7356 ns 2.9597 ns]
cmp/1024/ramp           time:   [5.4784 ns 5.5434 ns 5.5985 ns]
cmp/1024/ibig           time:   [6.6963 ns 6.7636 ns 6.8200 ns]
cmp/1024/ubig           time:   [2.8917 ns 2.9341 ns 2.9790 ns]

cmp/4096/uint           time:   [1.4914 ns 1.5262 ns 1.5650 ns]
cmp/4096/num            time:   [6.5201 ns 6.5712 ns 6.6197 ns]
cmp/4096/unum           time:   [2.9664 ns 3.1574 ns 3.3277 ns]
cmp/4096/num_svec       time:   [6.7473 ns 6.8063 ns 6.8637 ns]
cmp/4096/unum_svec      time:   [3.6277 ns 3.8119 ns 3.9740 ns]
cmp/4096/rug            time:   [3.2112 ns 3.2875 ns 3.3492 ns]
cmp/4096/ramp           time:   [5.7749 ns 5.8056 ns 5.8301 ns]
cmp/4096/ibig           time:   [6.0307 ns 6.1559 ns 6.2514 ns]
cmp/4096/ubig           time:   [2.8788 ns 2.9152 ns 2.9524 ns]

cmp/32768/num           time:   [7.2549 ns 7.6939 ns 8.0708 ns]
cmp/32768/unum          time:   [6.1124 ns 6.7816 ns 7.3693 ns]
cmp/32768/num_svec      time:   [7.9888 ns 8.4342 ns 8.8250 ns]
cmp/32768/unum_svec     time:   [7.2948 ns 7.9842 ns 8.5867 ns]
cmp/32768/rug           time:   [1.9654 ns 2.0750 ns 2.2071 ns]
cmp/32768/ramp          time:   [4.4546 ns 4.5189 ns 4.5713 ns]
cmp/32768/ibig          time:   [2.5566 ns 2.6056 ns 2.6720 ns]

clone/64bits/uint       time:   [957.43 ps 961.90 ps 968.23 ps]
clone/64bits/num        time:   [22.854 ns 22.925 ns 22.990 ns]
clone/64bits/unum       time:   [22.762 ns 22.839 ns 22.910 ns]
clone/64bits/num_svec   time:   [3.6911 ns 3.7379 ns 3.7774 ns]
clone/64bits/unum_svec  time:   [3.3832 ns 3.4384 ns 3.4839 ns]
clone/64bits/rug        time:   [21.969 ns 22.013 ns 22.059 ns]
clone/64bits/ramp       time:   [24.718 ns 24.749 ns 24.782 ns]
clone/64bits/ibig       time:   [2.6781 ns 2.7075 ns 2.7395 ns]
clone/64bits/ubig       time:   [2.1449 ns 2.1787 ns 2.2169 ns]

clone/128/uint          time:   [959.40 ps 964.79 ps 972.59 ps]
clone/128/num           time:   [23.034 ns 23.103 ns 23.165 ns]
clone/128/unum          time:   [23.018 ns 23.115 ns 23.211 ns]
clone/128/num_svec      time:   [4.7748 ns 4.8709 ns 4.9794 ns]
clone/128/unum_svec     time:   [4.3215 ns 4.3958 ns 4.4563 ns]
clone/128/rug           time:   [20.703 ns 20.756 ns 20.818 ns]
clone/128/ramp          time:   [25.206 ns 25.245 ns 25.289 ns]
clone/128/ibig          time:   [25.457 ns 25.510 ns 25.567 ns]
clone/128/ubig          time:   [25.899 ns 25.998 ns 26.092 ns]

clone/1024/uint         time:   [4.7154 ns 4.8031 ns 4.9150 ns]
clone/1024/num          time:   [31.591 ns 31.804 ns 32.033 ns]
clone/1024/unum         time:   [31.249 ns 31.422 ns 31.617 ns]
clone/1024/num_svec     time:   [32.549 ns 32.764 ns 32.974 ns]
clone/1024/unum_svec    time:   [31.997 ns 32.222 ns 32.452 ns]
clone/1024/rug          time:   [27.536 ns 27.760 ns 28.000 ns]
clone/1024/ramp         time:   [30.895 ns 31.045 ns 31.215 ns]
clone/1024/ibig         time:   [27.653 ns 27.763 ns 27.892 ns]
clone/1024/ubig         time:   [30.130 ns 30.533 ns 30.948 ns]

clone/4096/uint         time:   [18.885 ns 19.121 ns 19.417 ns]
clone/4096/num          time:   [56.768 ns 58.068 ns 59.214 ns]
clone/4096/unum         time:   [55.427 ns 56.531 ns 57.491 ns]
clone/4096/num_svec     time:   [55.611 ns 56.952 ns 58.080 ns]
clone/4096/unum_svec    time:   [56.329 ns 57.750 ns 58.999 ns]
clone/4096/rug          time:   [40.644 ns 42.415 ns 44.010 ns]
clone/4096/ramp         time:   [41.480 ns 42.293 ns 43.125 ns]
clone/4096/ibig         time:   [44.308 ns 46.364 ns 48.230 ns]
clone/4096/ubig         time:   [42.852 ns 44.971 ns 46.865 ns]

clone/32768/num         time:   [386.21 ns 400.22 ns 411.45 ns]
clone/32768/unum        time:   [393.95 ns 409.72 ns 422.49 ns]
clone/32768/num_svec    time:   [388.66 ns 403.67 ns 415.53 ns]
clone/32768/unum_svec   time:   [387.83 ns 403.03 ns 415.58 ns]
clone/32768/rug         time:   [155.27 ns 157.42 ns 159.85 ns]
clone/32768/ramp        time:   [179.66 ns 182.24 ns 185.04 ns]
clone/32768/ibig        time:   [162.94 ns 169.45 ns 176.75 ns]
clone/32768/ubig        time:   [180.32 ns 198.58 ns 217.94 ns]
```

## 3950X

|    gcd     |   uint   | branchless |   num    | num_svec |   rug    |   ramp   |   ubig   |
|------------| -------: | ---------: | -------: | -------: | -------: | -------: | -------: |
|    64 bits | 175.0 ns |    85.8 ns | 972.1 ns | 861.9 ns |  75.0 ns | 604.2 ns |   1.2 μs |
|   128 bits | 603.4 ns |   396.3 ns |   1.9 μs |   1.7 μs | 216.5 ns |   1.3 μs |   3.6 μs |
|  1024 bits |          |            |  17.4 μs |  17.0 μs |   5.4 μs |  16.0 μs |  37.8 μs |
|  4096 bits |          |            | 113.6 μs | 106.1 μs |  25.2 μs | 172.4 μs | 225.2 μs |
| 32768 bits |          |            |   4.5 ms |   4.4 ms | 414.0 μs |   9.7 ms |   7.4 ms |

|    mul     |   uint   |   num    |   unum   | num_svec | unum_svec |   rug    |   ramp   |   ibig   |   ubig   |
|------------| -------: | -------: | -------: | -------: | --------: | -------: | -------: | -------: | -------: |
|    64 bits |   1.6 ns |  34.0 ns |  25.2 ns |  13.9 ns |    7.6 ns |  20.4 ns |  57.2 ns |  34.0 ns |  28.9 ns |
|   128 bits |   2.7 ns |  37.6 ns |  27.2 ns |  36.3 ns |   35.4 ns |  23.0 ns |  33.9 ns |  49.0 ns |  34.3 ns |
|  1024 bits | 307.0 ns | 248.4 ns | 245.1 ns | 259.1 ns |  256.4 ns | 149.0 ns | 216.9 ns | 260.5 ns | 247.7 ns |
|  4096 bits |   6.1 μs |   3.0 μs |   3.0 μs |   3.0 μs |    3.0 μs |   1.4 μs |   2.2 μs |   2.7 μs |   2.6 μs |
| 32768 bits |          |  94.8 μs |  92.8 μs |  76.5 μs |   76.1 μs |  29.7 μs |  60.1 μs |  75.6 μs |  75.1 μs |

|    mula    |   uint   |   num    |   unum   | num_svec | unum_svec |   rug    |   ramp   |   ibig   |   ubig   |
|------------| -------: | -------: | -------: | -------: | --------: | -------: | -------: | -------: | -------: |
|    64 bits |   1.2 ns |  37.7 ns |  28.6 ns |  10.0 ns |    3.1 ns |   6.0 ns |  42.7 ns |  31.3 ns |  30.1 ns |
|   128 bits |   2.5 ns |  46.9 ns |  37.1 ns |  53.5 ns |   46.5 ns |   9.2 ns |  41.8 ns |  53.4 ns |  43.1 ns |
|  1024 bits | 295.8 ns | 274.5 ns | 256.6 ns | 277.1 ns |  274.9 ns | 203.7 ns | 249.9 ns | 285.3 ns | 272.4 ns |
|  4096 bits |   5.9 μs |   2.9 μs |   2.9 μs |   2.9 μs |    2.9 μs |   1.4 μs |   2.2 μs |   2.6 μs |   2.6 μs |
| 32768 bits |          |  92.5 μs |  92.8 μs |  79.0 μs |   79.6 μs |  31.0 μs |  62.0 μs |  74.4 μs |  75.0 μs |

|    div     |   uint   |   num    |   unum   | num_svec | unum_svec |   rug    |   ramp   |   ibig   |   ubig   |
|------------| -------: | -------: | -------: | -------: | --------: | -------: | -------: | -------: | -------: |
|    64 bits |   3.3 ns |  57.8 ns |  46.4 ns |  32.5 ns |   17.4 ns |  34.1 ns |  38.5 ns |  12.1 ns |   6.9 ns |
|   128 bits |  10.2 ns |  88.3 ns |  71.2 ns |  81.6 ns |   68.7 ns |  41.9 ns |  77.8 ns |  63.5 ns |  61.7 ns |
|  1024 bits |  88.5 ns | 117.1 ns | 102.6 ns | 110.5 ns |   99.3 ns |  58.5 ns | 137.6 ns |  86.2 ns |  82.6 ns |
|  4096 bits | 273.7 ns | 197.4 ns | 181.3 ns | 190.2 ns |  175.6 ns |  62.8 ns | 192.3 ns | 167.0 ns | 163.7 ns |
| 32768 bits |          | 925.5 ns | 926.7 ns | 972.6 ns |  940.6 ns |  70.3 ns | 732.7 ns |   1.0 μs |   1.0 μs |

|    add     |   uint   |   num    |   unum   | num_svec | unum_svec |   rug    |   ramp   |   ibig   |   ubig   |
|------------| -------: | -------: | -------: | -------: | --------: | -------: | -------: | -------: | -------: |
|    64 bits |   1.6 ns |  43.4 ns |  35.5 ns |  22.8 ns |   13.2 ns |  29.9 ns |  46.9 ns |  19.0 ns |  15.8 ns |
|   128 bits |   1.6 ns |  47.9 ns |  41.2 ns |  28.9 ns |   26.2 ns |  30.3 ns |  49.4 ns |  45.9 ns |  32.3 ns |
|  1024 bits |  60.3 ns |  47.3 ns |  41.6 ns |  49.8 ns |   45.4 ns |  34.6 ns |  60.0 ns |  50.2 ns |  37.4 ns |
|  4096 bits | 224.1 ns |  98.7 ns |  96.8 ns |  98.0 ns |  106.6 ns |  52.2 ns | 107.6 ns |  78.6 ns |  67.1 ns |
| 32768 bits |          | 576.9 ns | 568.0 ns | 580.6 ns |  575.9 ns | 227.3 ns | 611.8 ns | 399.0 ns | 375.4 ns |

|    adda    |   uint   |   num    |   unum   | num_svec | unum_svec |   rug    |   ramp   |   ibig   |   ubig   |
|------------| -------: | -------: | -------: | -------: | --------: | -------: | -------: | -------: | -------: |
|    64 bits |   1.1 ns |  33.0 ns |  23.1 ns |  23.5 ns |    6.7 ns |  11.2 ns |  12.2 ns |  20.3 ns |  16.2 ns |
|   128 bits |   1.4 ns |  33.9 ns |  24.3 ns |  30.2 ns |   17.9 ns |  11.7 ns |  12.9 ns |  27.1 ns |  18.3 ns |
|  1024 bits |  57.8 ns |  41.1 ns |  35.8 ns |  40.0 ns |   37.3 ns |  17.2 ns |  22.9 ns |  34.0 ns |  23.2 ns |
|  4096 bits | 236.7 ns |  75.2 ns |  79.7 ns |  76.1 ns |   79.1 ns |  34.6 ns |  65.0 ns |  58.1 ns |  46.2 ns |
| 32768 bits |          | 372.7 ns | 412.5 ns | 380.2 ns |  400.2 ns | 179.4 ns | 445.3 ns | 284.2 ns | 265.3 ns |

|    shra    |   uint   |   num    |   unum   | num_svec | unum_svec |   rug    |   ramp   |   ibig   |   ubig   |
|------------| -------: | -------: | -------: | -------: | --------: | -------: | -------: | -------: | -------: |
|    64 bits |   1.2 ns |  15.8 ns |  14.3 ns |  11.7 ns |    5.9 ns |   9.5 ns |   6.2 ns |  19.4 ns |   8.7 ns |
|   128 bits |   1.8 ns |  15.3 ns |  13.8 ns |  12.0 ns |    6.4 ns |   9.6 ns |   8.0 ns |  40.6 ns |  19.5 ns |
|  1024 bits |  11.2 ns |  23.6 ns |  24.1 ns |  17.8 ns |   13.0 ns |  13.5 ns |  14.7 ns |  44.3 ns |  23.0 ns |
|  4096 bits |  70.6 ns |  56.9 ns |  54.9 ns |  48.1 ns |   43.6 ns |  26.1 ns |  35.8 ns |  78.6 ns |  57.1 ns |
| 32768 bits |          | 273.1 ns | 259.5 ns | 259.1 ns |  252.4 ns | 179.8 ns | 222.9 ns | 375.0 ns | 344.6 ns |

|    cmp     |   uint   |   num    |   unum   | num_svec | unum_svec |   rug    |   ramp   |   ibig   |   ubig   |
|------------| -------: | -------: | -------: | -------: | --------: | -------: | -------: | -------: | -------: |
|    64 bits |   1.4 ns |   6.9 ns |   2.0 ns |   7.3 ns |    1.9 ns |   5.0 ns |   6.8 ns |   7.2 ns |   2.0 ns |
|   128 bits |   1.8 ns |   6.9 ns |   1.9 ns |   7.2 ns |    1.8 ns |   5.0 ns |   6.7 ns |   7.4 ns |   2.2 ns |
|  1024 bits |   1.9 ns |   7.6 ns |   3.6 ns |   8.0 ns |    3.9 ns |   5.1 ns |   7.0 ns |   8.0 ns |   2.9 ns |
|  4096 bits |   3.0 ns |   7.9 ns |   4.0 ns |   8.2 ns |    4.2 ns |   5.3 ns |   6.9 ns |   7.7 ns |   2.9 ns |
| 32768 bits |          |   8.7 ns |   7.9 ns |   8.8 ns |    8.1 ns |   5.9 ns |   6.7 ns |   8.2 ns |   3.8 ns |

|   clone    |   uint   |   num    |   unum   | num_svec | unum_svec |   rug    |   ramp   |   ibig   |   ubig   |
|------------| -------: | -------: | -------: | -------: | --------: | -------: | -------: | -------: | -------: |
|    64 bits |   1.4 ns |  14.7 ns |  14.1 ns |   3.2 ns |    2.5 ns |  16.8 ns |  21.5 ns |   7.5 ns |   6.2 ns |
|   128 bits |   1.3 ns |  16.8 ns |  17.1 ns |   3.3 ns |    2.3 ns |  16.8 ns |  25.4 ns |  24.5 ns |  22.9 ns |
|  1024 bits |   5.1 ns |  17.0 ns |  16.4 ns |  17.6 ns |   17.0 ns |  19.5 ns |  25.2 ns |  25.5 ns |  24.2 ns |
|  4096 bits |  16.7 ns |  56.5 ns |  57.0 ns |  60.4 ns |   57.1 ns |  29.0 ns |  38.6 ns |  35.1 ns |  33.7 ns |
| 32768 bits |          | 304.1 ns | 297.1 ns | 301.7 ns |  305.6 ns | 129.7 ns | 170.2 ns | 136.3 ns | 129.0 ns |

[num]: https://crates.io/crates/num
[rug]: https://crates.io/crates/rug
[ramp]: https://crates.io/crates/ramp
[gmp]: https://gmplib.org
