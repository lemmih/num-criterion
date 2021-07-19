use criterion::{
    criterion_group,
    criterion_main,
    measurement::WallTime,
    Bencher,
    BenchmarkGroup,
    Criterion,
    // Throughput,
};

use num_bigint::{BigInt, RandBigInt};
use num_integer::Integer;

#[cfg(feature = "rug")]
use rug::Integer as RugInteger;

#[cfg(feature = "ramp")]
use ramp::int::Int as RampInt;

mod rng;
use rand::RngCore;
use rng::get_rng;

type BenchGroup = fn(_: &mut BenchmarkGroup<'_, WallTime>, bits: u64);

fn mk_benchmark(c: &mut Criterion, name: &str, run_group: BenchGroup, native: fn(b: &mut Bencher)) {
    let mut group = c.benchmark_group(format!("{}/64bits", name));
    group.bench_function("native", native);
    run_group(&mut group, 64);
    group.finish();

    for &bits in [128, 1024, 4096, 32768].iter() {
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
        let x = rng.gen_bigint(bits);
        let y = rng.gen_bigint(bits);

        b.iter(|| run(&x, &y))
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
        let x = bigint_to_rug(rng.gen_bigint(bits));
        let y = bigint_to_rug(rng.gen_bigint(bits));
        b.iter(|| run(&x, &y))
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
        let x = bigint_to_ramp(rng.gen_bigint(bits));
        let y = bigint_to_ramp(rng.gen_bigint(bits));
        b.iter(|| run(&x, &y))
    });
}

///////////////////////////////////////////////////////////////////////////////
// Multiply benches

fn mul_bench_native(b: &mut Bencher) {
    let mut rng = get_rng();
    b.iter(|| rng.next_u64() * rng.next_u64());
}

///////////////////////////////////////////////////////////////////////////////
// Division benches

fn div_bench_native(b: &mut Bencher) {
    let mut rng = get_rng();
    b.iter(|| rng.next_u64() / rng.next_u64());
}

///////////////////////////////////////////////////////////////////////////////
// Addition benches

fn add_bench_native(b: &mut Bencher) {
    let mut rng = get_rng();
    b.iter(|| rng.next_u64() + rng.next_u64());
}

///////////////////////////////////////////////////////////////////////////////
// GCD benches

fn gcd_bench_native(b: &mut Bencher) {
    let mut rng = get_rng();
    let x = rng.next_u64();
    let y = rng.next_u64();

    b.iter(|| x.gcd(&y));
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
    // if bits == 1024 {
    //     group.bench_function("uint", |b| {
    //         let mut rng = get_rng();
    //         let x = U1024::from_str_radix(&rng.gen_biguint(bits).to_str_radix(16), 16).unwrap();
    //         let y = U1024::from_str_radix(&rng.gen_biguint(bits).to_str_radix(16), 16).unwrap();
    //         b.iter(|| x.overflowing_mul(y))
    //     });
    // }
    bigint(group, bits, |x, y| x * y);
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x * y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x * y);
}

fn add_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bigint(group, bits, |x, y| x + y);
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x + y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x + y);
}

fn div_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    bigint(group, bits, |x, y| x / y);
    #[cfg(feature = "rug")]
    rug(group, bits, |x, y| RugInteger::from(x / y));
    #[cfg(feature = "ramp")]
    ramp(group, bits, |x, y| x / y);
}

///////////////////////////////////////////////////////////////////////////////
// Benchmarks

fn gcd_benchmark(c: &mut Criterion) {
    mk_benchmark(c, "gcd", gcd_group, gcd_bench_native);
}

fn mul_benchmark(c: &mut Criterion) {
    mk_benchmark(c, "mul", mul_group, mul_bench_native);
}

fn div_benchmark(c: &mut Criterion) {
    mk_benchmark(c, "div", div_group, div_bench_native);
}

fn add_benchmark(c: &mut Criterion) {
    mk_benchmark(c, "add", add_group, add_bench_native);
}

criterion_group!(
    benches,
    gcd_benchmark,
    mul_benchmark,
    div_benchmark,
    add_benchmark
);
criterion_main!(benches);
