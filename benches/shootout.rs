use criterion::{
    criterion_group,
    criterion_main,
    measurement::WallTime,
    BatchSize,
    // Throughput,
    // Bencher,
    BenchmarkGroup,
    Criterion,
};

use num_bigint::BigInt;
#[cfg(feature = "num-bigint-small")]
use num_bigint_small::BigInt as BigIntSmall;
#[cfg(feature = "num-bigint-small")]
#[allow(unused_imports)]
use num_bigint_small::BigUint as BigUintSmall;

use num_integer::Integer;
use std::ops::*;

#[cfg(feature = "uint")]
use uint::construct_uint;

#[cfg(feature = "rug")]
use rug::Integer as RugInteger;

#[cfg(feature = "ramp")]
use ramp::int::Int as RampInt;

mod rng;
// use rand::RngCore;
use rng::get_rng;

#[cfg(feature = "uint")]
construct_uint! {
    pub struct U1024(16);
}

// uint doesn't go higher than this.
#[cfg(feature = "uint")]
construct_uint! {
    pub struct U4096(64);
}

macro_rules! uint {
    ( $group: ident, $bits: ident, $ty:ty, $run: expr ) => {
        use num_bigint::RandBigInt;
        $group.bench_function("uint", |b| {
            let mut rng = get_rng();
            b.iter_batched_ref(
                || {
                    let x = <$ty>::from_str_radix(&rng.gen_biguint($bits).to_str_radix(16), 16)
                        .unwrap();
                    let y = <$ty>::from_str_radix(&rng.gen_biguint($bits).to_str_radix(16), 16)
                        .unwrap();
                    (x, y)
                },
                $run,
                BatchSize::SmallInput,
            )
        });
    };
}

macro_rules! uint_all {
    ( $group: ident, $bits: ident, $run: expr) => {
        if $bits == 64 {
            uint!($group, $bits, u64, |(x, y)| x.checked_div(*y));
        }
        if $bits == 128 {
            uint!($group, $bits, u128, |(x, y)| x.checked_div(*y));
        }
        #[cfg(feature = "uint")]
        if $bits == 1024 {
            uint!($group, $bits, U1024, |(x, y)| x.checked_div(*y));
        }
        #[cfg(feature = "uint")]
        if $bits == 4096 {
            uint!($group, $bits, U4096, |(x, y)| x.checked_div(*y));
        }
    };
}

type BenchGroup = fn(_: &mut BenchmarkGroup<'_, WallTime>, bits: u64);

fn mk_benchmark(c: &mut Criterion, name: &str, run_group: BenchGroup) {
    for &bits in [64, 128, 1024, 4096, 32768].iter() {
        let mut group = if bits == 64 {
            c.benchmark_group(format!("{}/{}bits", name, bits))
        } else {
            c.benchmark_group(format!("{}/{}", name, bits))
        };
        run_group(&mut group, bits);
        group.finish();
    }
}

type GCD = u64;
fn fast_gcd(mut m: GCD, mut n: GCD) -> GCD {
    // min_diff(a,b) = (min(a,b), |a-b|)
    pub fn min_diff(a: GCD, b: GCD) -> (GCD, GCD) {
        let (t, o) = a.overflowing_sub(b);
        if o {
            // a<b
            (a, !t + 1)
        } else {
            // a>b
            (b, t)
        }
    }

    // Use Stein's algorithm
    if m == 0 || n == 0 {
        return m | n;
    }

    // find common factors of 2
    let shift = (m | n).trailing_zeros();

    // divide n and m by 2 until odd
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        // New code:
        let (n_, m_) = min_diff(m, n);
        n = n_;
        m = m_ >> m_.trailing_zeros();

        // Old code:
        // if m > n {
        //     m -= n;
        //     m >>= m.trailing_zeros();
        // } else {
        //     n -= m;
        //     n >>= n.trailing_zeros();
        // }
    }
    m << shift
}

fn fast_gcd_u128(mut m: u128, mut n: u128) -> u128 {
    pub fn min_diff(a: u128, b: u128) -> (u128, u128) {
        let (t, o) = a.overflowing_sub(b);
        if o {
            // a<b
            (a, !t + 1)
        } else {
            // a>b
            (b, t)
        }
    }

    // Use Stein's algorithm
    if m == 0 || n == 0 {
        return m | n;
    }

    // find common factors of 2
    let shift = (m | n).trailing_zeros();

    // divide n and m by 2 until odd
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        // New code:
        let (n_, m_) = min_diff(m, n);
        n = n_;
        m = m_ >> m_.trailing_zeros();

        // Old code:
        // if m > n {
        //     m -= n;
        //     m >>= m.trailing_zeros();
        // } else {
        //     n -= m;
        //     n >>= n.trailing_zeros();
        // }
    }
    m << shift
}

#[cfg(feature = "rug")]
fn bigint_to_rug(big: BigInt) -> RugInteger {
    RugInteger::from_str_radix(&big.to_str_radix(16), 16).unwrap()
}

#[cfg(feature = "ramp")]
fn bigint_to_ramp(big: BigInt) -> RampInt {
    RampInt::from_str_radix(&big.to_str_radix(16), 16).unwrap()
}

fn bigint(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: fn(_: &BigInt, _: &BigInt) -> BigInt,
) {
    use num_bigint::RandBigInt;
    group.bench_function("num", |b| {
        let mut rng = get_rng();

        b.iter_batched_ref(
            || (rng.gen_bigint(bits), rng.gen_bigint(bits)),
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

fn bigint_mut(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: fn(_: &mut BigInt, _: &BigInt),
) {
    use num_bigint::RandBigInt;
    group.bench_function("num", |b| {
        let mut rng = get_rng();

        b.iter_batched_ref(
            || (rng.gen_bigint(bits), rng.gen_bigint(bits)),
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "num-bigint-small")]
fn smallint(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: fn(_: &BigIntSmall, _: &BigIntSmall) -> BigIntSmall,
    // run: fn(_: &BigUintSmall, _: &BigUintSmall) -> BigUintSmall,
) {
    use num_bigint_small::RandBigInt;
    group.bench_function("num_svec", |b| {
        let mut rng = get_rng();

        b.iter_batched_ref(
            || (rng.gen_bigint(bits), rng.gen_bigint(bits)),
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "num-bigint-small")]
fn smallint_mut(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: fn(_: &mut BigIntSmall, _: &BigIntSmall),
    // run: fn(_: &BigUintSmall, _: &BigUintSmall) -> BigUintSmall,
) {
    use num_bigint_small::RandBigInt;
    group.bench_function("num_svec", |b| {
        let mut rng = get_rng();

        b.iter_batched_ref(
            || (rng.gen_bigint(bits), rng.gen_bigint(bits)),
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "rug")]
fn rug(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: fn(_: &RugInteger, _: &RugInteger) -> RugInteger,
) {
    use num_bigint::RandBigInt;
    group.bench_function("rug", |b| {
        let mut rng = get_rng();
        b.iter_batched_ref(
            || {
                (
                    bigint_to_rug(rng.gen_bigint(bits)),
                    bigint_to_rug(rng.gen_bigint(bits)),
                )
            },
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "rug")]
fn rug_mut(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: fn(_: &mut RugInteger, _: &RugInteger),
) {
    use num_bigint::RandBigInt;
    group.bench_function("rug_mut", |b| {
        let mut rng = get_rng();
        b.iter_batched_ref(
            || {
                (
                    bigint_to_rug(rng.gen_bigint(bits)),
                    bigint_to_rug(rng.gen_bigint(bits)),
                )
            },
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "ramp")]
fn ramp(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: fn(_: &RampInt, _: &RampInt) -> RampInt,
) {
    use num_bigint::RandBigInt;
    group.bench_function("ramp", |b| {
        let mut rng = get_rng();
        b.iter_batched_ref(
            || {
                (
                    bigint_to_ramp(rng.gen_bigint(bits)),
                    bigint_to_ramp(rng.gen_bigint(bits)),
                )
            },
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "ramp")]
fn ramp_mut(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: fn(_: &mut RampInt, _: &RampInt),
) {
    use num_bigint::RandBigInt;
    group.bench_function("ramp", |b| {
        let mut rng = get_rng();
        b.iter_batched_ref(
            || {
                (
                    bigint_to_ramp(rng.gen_bigint(bits)),
                    bigint_to_ramp(rng.gen_bigint(bits)),
                )
            },
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

///////////////////////////////////////////////////////////////////////////////
// Groups

fn gcd_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    if bits == 64 {
        uint!(group, bits, u64, |(x, y)| x.gcd(y));
    }
    if bits == 64 {
        use num_bigint::RandBigInt;
        group.bench_function("branchless", |b| {
            let mut rng = get_rng();
            b.iter_batched_ref(
                || {
                    let x =
                        u64::from_str_radix(&rng.gen_biguint(bits).to_str_radix(16), 16).unwrap();
                    let y =
                        u64::from_str_radix(&rng.gen_biguint(bits).to_str_radix(16), 16).unwrap();
                    (x, y)
                },
                |(x, y)| fast_gcd(*x, *y),
                BatchSize::SmallInput,
            )
        });
    }
    if bits == 128 {
        uint!(group, bits, u128, |(x, y)| x.gcd(y));
    }
    if bits == 128 {
        use num_bigint::RandBigInt;
        group.bench_function("branchless", |b| {
            let mut rng = get_rng();
            b.iter_batched_ref(
                || {
                    let x =
                        u128::from_str_radix(&rng.gen_biguint(bits).to_str_radix(16), 16).unwrap();
                    let y =
                        u128::from_str_radix(&rng.gen_biguint(bits).to_str_radix(16), 16).unwrap();
                    (x, y)
                },
                |(x, y)| fast_gcd_u128(*x, *y),
                BatchSize::SmallInput,
            )
        });
    }
    bigint(group, bits, |x, y| x.gcd(y));
    #[cfg(feature = "num-bigint-small")]
    smallint(group, bits, |x, y| x.gcd(y));
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x.gcd_ref(y)));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x.gcd(y));
}

fn mul_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    uint_all!(group, bits, |(x, y)| x.checked_mul(*y));
    bigint(group, bits, |x, y| x * y);
    #[cfg(feature = "num-bigint-small")]
    smallint(group, bits, |x, y| x * y);
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x * y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x * y);
}

fn mul_mut_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    // uint_all!(group, bits, |(x, y)| x.overflowing_mul(*y));
    bigint_mut(group, bits, |x, y| x.mul_assign(y));
    #[cfg(feature = "num-bigint-small")]
    smallint_mut(group, bits, |x, y| x.mul_assign(y));
    #[cfg(feature = "rug")]
    rug_mut(group, bits, |x, y| x.mul_assign(y));
    #[cfg(feature = "ramp")]
    ramp_mut(group, bits, |x, y| x.mul_assign(y));
}

fn add_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    uint_all!(group, bits, |(x, y)| x.checked_add(*y));
    bigint(group, bits, |x, y| x + y);
    #[cfg(feature = "num-bigint-small")]
    smallint(group, bits, |x, y| x + y);
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x + y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x + y);
}

fn div_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    uint_all!(group, bits, |(x, y)| x.checked_div(*y));

    bigint(group, bits, |x, y| x / y);

    #[cfg(feature = "num-bigint-small")]
    smallint(group, bits, |x, y| x / y);

    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x / y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x / y);
}

fn clone_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    uint_all!(group, bits, |(x, _y)| x);

    bigint(group, bits, |x, _y| x.clone());

    #[cfg(feature = "num-bigint-small")]
    smallint(group, bits, |x, _y| x.clone());

    #[cfg(feature = "num-bigint-small")]
    denseint(group, bits, |x, _y| x.clone());

    #[cfg(feature = "rug")]
    rug(group, bits, |x, _y| RugInteger::from(x));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, _y| x.clone());
}

///////////////////////////////////////////////////////////////////////////////
// Benchmarks

fn benchmarks(c: &mut Criterion) {
    mk_benchmark(c, "gcd", gcd_group);
    mk_benchmark(c, "mul", mul_group);
    mk_benchmark(c, "mut", mul_mut_group);
    mk_benchmark(c, "div", div_group);
    mk_benchmark(c, "add", add_group);
    mk_benchmark(c, "clone", clone_group);
}

criterion_group!(benches, benchmarks,);
criterion_main!(benches);
