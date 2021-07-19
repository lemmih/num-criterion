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

use num_bigint::{BigInt, RandBigInt};
use num_integer::Integer;

use uint::construct_uint;

#[cfg(feature = "rug")]
use rug::Integer as RugInteger;

#[cfg(feature = "ramp")]
use ramp::int::Int as RampInt;

mod rng;
// use rand::RngCore;
use rng::get_rng;

construct_uint! {
    pub struct U1024(16);
}

// uint doesn't go higher than this.
construct_uint! {
    pub struct U4096(64);
}

macro_rules! uint {
    ( $group: ident, $bits: ident, $ty:ty, $run: expr ) => {
        $group.bench_function("uint", |b| {
            let mut rng = get_rng();
            // let x = <$ty>::from_str_radix(&rng.gen_biguint($bits).to_str_radix(16), 16).unwrap();
            // let y = <$ty>::from_str_radix(&rng.gen_biguint($bits).to_str_radix(16), 16).unwrap();
            b.iter_batched(
                || {
                    let x = <$ty>::from_str_radix(&rng.gen_biguint($bits).to_str_radix(16), 16)
                        .unwrap();
                    let y = <$ty>::from_str_radix(&rng.gen_biguint($bits).to_str_radix(16), 16)
                        .unwrap();
                    (x, y)
                },
                |(x, y)| ($run)(x, y),
                BatchSize::SmallInput,
            )
        });
    };
}

type BenchGroup = fn(_: &mut BenchmarkGroup<'_, WallTime>, bits: u64);

fn mk_benchmark(c: &mut Criterion, name: &str, run_group: BenchGroup) {
    for &bits in [64, 128, 1024, 4096, 32768].iter() {
        let mut group = c.benchmark_group(format!("{}/{}bits", name, bits));
        run_group(&mut group, bits);
        group.finish();
    }
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
    group.bench_function("num", |b| {
        let mut rng = get_rng();
        // let x = rng.gen_bigint(bits);
        // let y = rng.gen_bigint(bits);

        b.iter_batched(
            || (rng.gen_bigint(bits), rng.gen_bigint(bits)),
            |(x, y)| run(&x, &y),
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
    group.bench_function("rug", |b| {
        let mut rng = get_rng();
        // let x = bigint_to_rug(rng.gen_bigint(bits));
        // let y = bigint_to_rug(rng.gen_bigint(bits));
        b.iter_batched(
            || {
                (
                    bigint_to_rug(rng.gen_bigint(bits)),
                    bigint_to_rug(rng.gen_bigint(bits)),
                )
            },
            |(x, y)| run(&x, &y),
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
    group.bench_function("ramp", |b| {
        let mut rng = get_rng();
        // let x = bigint_to_ramp(rng.gen_bigint(bits));
        // let y = bigint_to_ramp(rng.gen_bigint(bits));
        b.iter_batched(
            || {
                (
                    bigint_to_ramp(rng.gen_bigint(bits)),
                    bigint_to_ramp(rng.gen_bigint(bits)),
                )
            },
            |(x, y)| run(&x, &y),
            BatchSize::SmallInput,
        )
    });
}

///////////////////////////////////////////////////////////////////////////////
// Groups

fn gcd_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bigint(group, bits, |x, y| x.gcd(y));
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x.gcd_ref(y)));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x.gcd(y));
}

fn mul_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    if bits == 64 {
        uint!(group, bits, u64, |x: u64, y: u64| x.overflowing_mul(y));
    }
    if bits == 128 {
        uint!(group, bits, u128, |x: u128, y: u128| x.overflowing_mul(y));
    }
    if bits == 1024 {
        uint!(group, bits, U1024, |x: U1024, y: U1024| x
            .overflowing_mul(y));
    }
    if bits == 4096 {
        uint!(group, bits, U4096, |x: U4096, y: U4096| x
            .overflowing_mul(y));
    }
    bigint(group, bits, |x, y| x * y);
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x * y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x * y);
}

fn add_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    if bits == 64 {
        uint!(group, bits, u64, |x: u64, y: u64| x.overflowing_add(y));
    }
    if bits == 128 {
        uint!(group, bits, u128, |x: u128, y: u128| x.overflowing_add(y));
    }
    if bits == 1024 {
        uint!(group, bits, U1024, |x: U1024, y: U1024| x
            .overflowing_add(y));
    }
    if bits == 4096 {
        uint!(group, bits, U4096, |x: U4096, y: U4096| x
            .overflowing_add(y));
    }
    bigint(group, bits, |x, y| x + y);
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x + y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x + y);
}

fn div_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    if bits == 64 {
        uint!(group, bits, u64, |x: u64, y: u64| x.checked_div(y));
    }
    if bits == 128 {
        uint!(group, bits, u128, |x: u128, y: u128| x.checked_div(y));
    }
    if bits == 1024 {
        uint!(group, bits, U1024, |x: U1024, y: U1024| x.checked_div(y));
    }
    if bits == 4096 {
        uint!(group, bits, U4096, |x: U4096, y: U4096| x.checked_div(y));
    }
    bigint(group, bits, |x, y| x / y);
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x / y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x / y);
}

///////////////////////////////////////////////////////////////////////////////
// Benchmarks

fn benchmarks(c: &mut Criterion) {
    mk_benchmark(c, "gcd", gcd_group);
    mk_benchmark(c, "mul", mul_group);
    mk_benchmark(c, "div", div_group);
    mk_benchmark(c, "add", add_group);
}

criterion_group!(benches, benchmarks,);
criterion_main!(benches);
