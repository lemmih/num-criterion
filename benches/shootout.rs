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

use num_bigint::{BigInt, BigUint};
#[cfg(feature = "num-bigint-small")]
use num_bigint_small::{BigInt as BigIntSmall, BigUint as BigUintSmall};

use num_integer::Integer;
use std::ops::*;

#[cfg(feature = "uint")]
use uint::construct_uint;

#[cfg(feature = "rug")]
use rug::Integer as RugInteger;

#[cfg(feature = "ramp")]
use ramp::int::Int as RampInt;

#[cfg(feature = "ibig")]
use ibig::{IBig, UBig};

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
            let cb: fn(_: &mut $ty, _: &$ty) -> _ = $run;
            b.iter_batched_ref(
                || {
                    let x = <$ty>::from_str_radix(&rng.gen_biguint($bits).to_str_radix(16), 16)
                        .unwrap();
                    let y = <$ty>::from_str_radix(&rng.gen_biguint($bits).to_str_radix(16), 16)
                        .unwrap();
                    (x, y)
                },
                |(x, y)| cb(x, y),
                BatchSize::SmallInput,
            )
        });
    };
}

macro_rules! uint_all {
    ( $group: ident, $bits: ident, $run: expr) => {
        if $bits == 64 {
            uint!($group, $bits, u64, $run);
        }
        if $bits == 128 {
            uint!($group, $bits, u128, $run);
        }
        #[cfg(feature = "uint")]
        if $bits == 1024 {
            uint!($group, $bits, U1024, $run);
        }
        #[cfg(feature = "uint")]
        if $bits == 4096 {
            uint!($group, $bits, U4096, $run);
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

#[cfg(feature = "ibig")]
fn bigint_to_ibig(big: BigInt) -> IBig {
    IBig::from_str_radix(&big.to_str_radix(16), 16).unwrap()
}

#[cfg(feature = "ibig")]
fn biguint_to_ubig(big: BigUint) -> UBig {
    UBig::from_str_radix(&big.to_str_radix(16), 16).unwrap()
}

// Copied from num-integer.
#[cfg(feature = "ibig")]
fn gcd_ubig(m: &UBig, n: &UBig) -> UBig {
    use num_traits::identities::Zero;
    // Use Stein's algorithm
    let mut m = m.clone();
    let mut n = n.clone();
    if m.is_zero() || n.is_zero() {
        return m | n;
    }

    // find common factors of 2
    let m_zeros = m.trailing_zeros().unwrap_or(0);
    let n_zeros = n.trailing_zeros().unwrap_or(0);
    let shift = std::cmp::min(m_zeros, n_zeros);

    // divide n and m by 2 until odd
    m >>= m_zeros;
    n >>= n_zeros;

    while &m != &n {
        if m > n {
            m -= &n;
            m >>= m.trailing_zeros().unwrap_or(0);
        } else {
            n -= &m;
            n >>= n.trailing_zeros().unwrap_or(0);
        }
    }
    m << shift
}

fn bigint<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&BigInt, &BigInt) -> X,
) {
    bigint_mut(group, bits, move |x, y| run(&*x, y))
}

fn bigint_mut<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&mut BigInt, &BigInt) -> X,
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
fn biguint<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&BigUint, &BigUint) -> X,
) {
    biguint_mut(group, bits, |x, y| run(&*x, y))
}

fn biguint_mut<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&mut BigUint, &BigUint) -> X,
) {
    use num_bigint::RandBigInt;
    group.bench_function("unum", |b| {
        let mut rng = get_rng();

        b.iter_batched_ref(
            || (rng.gen_biguint(bits), rng.gen_biguint(bits)),
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "num-bigint-small")]
fn smallint<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&BigIntSmall, &BigIntSmall) -> X,
) {
    smallint_mut(group, bits, |x, y| run(&*x, y))
}

#[cfg(feature = "num-bigint-small")]
fn smallint_mut<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&mut BigIntSmall, &BigIntSmall) -> X,
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
fn smalluint<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&BigUintSmall, &BigUintSmall) -> X,
) {
    smalluint_mut(group, bits, |x, y| run(&*x, y))
}

#[cfg(feature = "num-bigint-small")]
fn smalluint_mut<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&mut BigUintSmall, &BigUintSmall) -> X,
) {
    use num_bigint_small::RandBigInt;
    group.bench_function("unum_svec", |b| {
        let mut rng = get_rng();

        b.iter_batched_ref(
            || (rng.gen_biguint(bits), rng.gen_biguint(bits)),
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "rug")]
fn rug<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&RugInteger, &RugInteger) -> X,
) {
    rug_mut(group, bits, |x, y| run(&*x, y))
}

#[cfg(feature = "rug")]
fn rug_mut<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&mut RugInteger, &RugInteger) -> X,
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

#[cfg(feature = "ramp")]
fn ramp<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&RampInt, &RampInt) -> X,
) {
    ramp_mut(group, bits, |x, y| run(&*x, y))
}

#[cfg(feature = "ramp")]
fn ramp_mut<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&mut RampInt, &RampInt) -> X,
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

#[cfg(feature = "ibig")]
fn ibig<X>(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64, run: impl Fn(&IBig, &IBig) -> X) {
    ibig_mut(group, bits, |x, y| run(&*x, y))
}

#[cfg(feature = "ibig")]
fn ibig_mut<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&mut IBig, &IBig) -> X,
) {
    use num_bigint::RandBigInt;
    group.bench_function("ibig", |b| {
        let mut rng = get_rng();
        b.iter_batched_ref(
            || {
                (
                    bigint_to_ibig(rng.gen_bigint(bits)),
                    bigint_to_ibig(rng.gen_bigint(bits)),
                )
            },
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "ibig")]
fn ubig<X>(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64, run: impl Fn(&UBig, &UBig) -> X) {
    ubig_mut(group, bits, |x, y| run(&*x, y))
}

#[cfg(feature = "ibig")]
fn ubig_mut<X>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    bits: u64,
    run: impl Fn(&mut UBig, &UBig) -> X,
) {
    use num_bigint::RandBigInt;
    group.bench_function("ubig", |b| {
        let mut rng = get_rng();
        b.iter_batched_ref(
            || {
                (
                    biguint_to_ubig(rng.gen_biguint(bits)),
                    biguint_to_ubig(rng.gen_biguint(bits)),
                )
            },
            |(x, y)| run(x, y),
            BatchSize::SmallInput,
        )
    });
}

macro_rules! bench_single {
    ( $group:ident, $bits:ident, uint => $run: expr ) => {
        uint_all!($group, $bits, $run)
    };
    ( $group:ident, $bits:ident, bigint => $run: expr ) => {
        bigint($group, $bits, $run)
    };
    ( $group:ident, $bits:ident, biguint => $run: expr ) => {
        biguint($group, $bits, $run)
    };
    ( $group:ident, $bits:ident, bigint_svec => $run: expr ) => {
        #[cfg(feature = "num-bigint-small")]
        smallint($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, biguint_svec => $run: expr ) => {
        #[cfg(feature = "num-bigint-small")]
        smalluint($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, rug => $run: expr ) => {
        #[cfg(feature = "rug")]
        rug($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, ramp => $run: expr ) => {
        #[cfg(feature = "ramp")]
        ramp($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, ibig => $run: expr ) => {
        #[cfg(feature = "ibig")]
        ibig($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, ubig => $run: expr ) => {
        #[cfg(feature = "ibig")]
        ubig($group, $bits, $run);
    };
}

macro_rules! bench {
    ( $group:ident, $bits:ident, $( $key:ident => $run:expr )* ) => {
        $( bench_single!( $group, $bits, $key => $run ); )*
    };
}

macro_rules! bench_mut_single {
    ( $group:ident, $bits:ident, uint => $run: expr ) => {
        uint_all!($group, $bits, $run)
    };
    ( $group:ident, $bits:ident, bigint => $run: expr ) => {
        bigint_mut($group, $bits, $run)
    };
    ( $group:ident, $bits:ident, biguint => $run: expr ) => {
        biguint_mut($group, $bits, $run)
    };
    ( $group:ident, $bits:ident, bigint_svec => $run: expr ) => {
        #[cfg(feature = "num-bigint-small")]
        smallint_mut($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, biguint_svec => $run: expr ) => {
        #[cfg(feature = "num-bigint-small")]
        smalluint_mut($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, rug => $run: expr ) => {
        #[cfg(feature = "rug")]
        rug_mut($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, ramp => $run: expr ) => {
        #[cfg(feature = "ramp")]
        ramp_mut($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, ibig => $run: expr ) => {
        #[cfg(feature = "ibig")]
        ibig_mut($group, $bits, $run);
    };
    ( $group:ident, $bits:ident, ubig => $run: expr ) => {
        #[cfg(feature = "ibig")]
        ubig_mut($group, $bits, $run);
    };
}

macro_rules! bench_mut {
    ( $group:ident, $bits:ident, $( $key:ident => $run:expr )* ) => {
        $( bench_mut_single!( $group, $bits, $key => $run ); )*
    };
}

///////////////////////////////////////////////////////////////////////////////
// Groups

fn gcd_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    if bits == 64 {
        uint!(group, bits, u64, |x, y| x.gcd(y));
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
        uint!(group, bits, u128, |x, y| x.gcd(y));
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

    bench!( group, bits,
        bigint      => | x, y | x.gcd(y)
        bigint_svec => | x, y | x.gcd(y)
        rug         => | x, y | RugInteger::from(x.gcd_ref(y))
        ramp        => | x, y | x.gcd(y)
        ubig        => | x, y | gcd_ubig(x,y)
    );
}

fn decimal_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench!( group, bits,
        uint         => | x, _y | format!("{}", x)
        bigint       => | x, _y | format!("{}", x)
        biguint      => | x, _y | format!("{}", x)
        bigint_svec  => | x, _y | format!("{}", x)
        biguint_svec => | x, _y | format!("{}", x)
        rug          => | x, _y | format!("{}", x)
        ramp         => | x, _y | format!("{}", x)
        ibig         => | x, _y | format!("{}", x)
        ubig         => | x, _y | format!("{}", x)
    );
}

fn hex_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench!( group, bits,
        uint         => | x, _y | format!("{:x}", x)
        bigint       => | x, _y | format!("{:x}", x)
        biguint      => | x, _y | format!("{:x}", x)
        bigint_svec  => | x, _y | format!("{:x}", x)
        biguint_svec => | x, _y | format!("{:x}", x)
        rug          => | x, _y | format!("{:x}", x)
        ramp         => | x, _y | format!("{:x}", x)
        ibig         => | x, _y | format!("{:x}", x)
        ubig         => | x, _y | format!("{:x}", x)
    );
}

fn mul_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench!( group, bits,
        uint         => | x, y | x.checked_mul(*y)
        bigint       => | x, y | x * y
        biguint      => | x, y | x * y
        bigint_svec  => | x, y | x * y
        biguint_svec => | x, y | x * y
        rug          => | x, y | RugInteger::from(x * y)
        ramp         => | x, y | x * y
        ibig         => | x, y | x * y
        ubig         => | x, y | x * y
    );
}

fn mula_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench_mut!( group, bits,
        uint         => | x, y | *x = x.saturating_mul(*y)
        bigint       => | x, y | x.mul_assign(y)
        biguint      => | x, y | x.mul_assign(y)
        bigint_svec  => | x, y | x.mul_assign(y)
        biguint_svec => | x, y | x.mul_assign(y)
        rug          => | x, y | x.mul_assign(y)
        ramp         => | x, y | x.mul_assign(y)
        ibig         => | x, y | x.mul_assign(y)
        ubig         => | x, y | x.mul_assign(y)
    );
}

fn add_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench!( group, bits,
        uint         => | x, y | x.checked_add(*y)
        bigint       => | x, y | x + y
        biguint      => | x, y | x + y
        bigint_svec  => | x, y | x + y
        biguint_svec => | x, y | x + y
        rug          => | x, y | RugInteger::from(x + y)
        ramp         => | x, y | x + y
        ibig         => | x, y | x + y
        ubig         => | x, y | x + y
    );
}

fn adda_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench_mut!( group, bits,
        uint         => | x, y | *x = x.saturating_add(*y)
        bigint       => | x, y | *x += y
        biguint      => | x, y | *x += y
        bigint_svec  => | x, y | *x += y
        biguint_svec => | x, y | *x += y
        rug          => | x, y | *x += y
        ramp         => | x, y | *x += y
        ibig         => | x, y | *x += y
        ubig         => | x, y | *x += y
    );
}

fn div_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench!( group, bits,
        uint         => | x, y | x.checked_div(*y)
        bigint       => | x, y | x / y
        biguint      => | x, y | x / y
        bigint_svec  => | x, y | x / y
        biguint_svec => | x, y | x / y
        rug          => | x, y | RugInteger::from(x / y)
        ramp         => | x, y | x / y
        ibig         => | x, y | x / y
        ubig         => | x, y | x / y
    );
}

fn cmp_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench!( group, bits,
        uint         => | x, y | (&*x).cmp(y)
        bigint       => | x, y | x.cmp(y)
        biguint      => | x, y | x.cmp(y)
        bigint_svec  => | x, y | x.cmp(y)
        biguint_svec => | x, y | x.cmp(y)
        rug          => | x, y | x.cmp(y)
        ramp         => | x, y | x.cmp(y)
        ibig         => | x, y | x.cmp(y)
        ubig         => | x, y | x.cmp(y)
    );
}

fn clone_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bench!( group, bits,
        uint         => | x, _y | x.clone()
        bigint       => | x, _y | x.clone()
        biguint      => | x, _y | x.clone()
        bigint_svec  => | x, _y | x.clone()
        biguint_svec => | x, _y | x.clone()
        rug          => | x, _y | x.clone()
        ramp         => | x, _y | x.clone()
        ibig         => | x, _y | x.clone()
        ubig         => | x, _y | x.clone()
    );
}

fn shra_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    const SHIFT: u32 = 11;

    bench_mut!( group, bits,
        uint         => | x, _y | *x >>= SHIFT
        bigint       => | x, _y | *x >>= SHIFT
        biguint      => | x, _y | *x >>= SHIFT
        bigint_svec  => | x, _y | *x >>= SHIFT
        biguint_svec => | x, _y | *x >>= SHIFT
        rug          => | x, _y | *x >>= SHIFT
        ramp         => | x, _y | *x >>= SHIFT as usize
        ibig         => | x, _y | *x >>= SHIFT as usize
        ubig         => | x, _y | *x >>= SHIFT as usize
    );
}

///////////////////////////////////////////////////////////////////////////////
// Benchmarks

fn benchmarks(c: &mut Criterion) {
    mk_benchmark(c, "gcd", gcd_group);
    mk_benchmark(c, "to_dec", decimal_group);
    mk_benchmark(c, "to_hex", hex_group);
    mk_benchmark(c, "mul", mul_group);
    mk_benchmark(c, "mula", mula_group);
    mk_benchmark(c, "div", div_group);
    mk_benchmark(c, "add", add_group);
    mk_benchmark(c, "adda", adda_group);
    mk_benchmark(c, "shra", shra_group);
    mk_benchmark(c, "cmp", cmp_group);
    mk_benchmark(c, "clone", clone_group);
}

criterion_group!(benches, benchmarks,);
criterion_main!(benches);
