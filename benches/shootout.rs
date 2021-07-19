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
    let (sign, digits) = big.to_u64_digits();
    let rug = RugInteger::from_digits(&digits, rug::integer::Order::Lsf);
    if sign == num_bigint::Sign::Minus {
        -rug
    } else {
        rug
    }
}

#[cfg(feature = "ramp")]
fn bigint_to_ramp(big: BigInt) -> RampInt {
    RampInt::from_str_radix(&big.to_str_radix(16), 16).unwrap()
}

///////////////////////////////////////////////////////////////////////////////
// Multiply benches

fn mul_bench_bigint(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = rng.gen_bigint(bits);
    let y = rng.gen_bigint(bits);

    b.iter(|| &x * &y);
}

#[cfg(feature = "rug")]
fn mul_bench_rug(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = bigint_to_rug(rng.gen_bigint(bits));
    let y = bigint_to_rug(rng.gen_bigint(bits));
    b.iter(|| RugInteger::from(&x * &y));
}

#[cfg(feature = "ramp")]
fn mul_bench_ramp(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = bigint_to_ramp(rng.gen_bigint(bits));
    let y = bigint_to_ramp(rng.gen_bigint(bits));
    b.iter(|| &x * &y);
}

fn mul_bench_native(b: &mut Bencher) {
    let mut rng = get_rng();
    b.iter(|| rng.next_u64() * rng.next_u64());
}

///////////////////////////////////////////////////////////////////////////////
// Division benches

fn div_bench_bigint(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = rng.gen_bigint(bits);
    let y = rng.gen_bigint(bits);

    b.iter(|| &x / &y);
}

#[cfg(feature = "rug")]
fn div_bench_rug(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = bigint_to_rug(rng.gen_bigint(bits));
    let y = bigint_to_rug(rng.gen_bigint(bits));
    b.iter(|| RugInteger::from(&x / &y));
}

#[cfg(feature = "ramp")]
fn div_bench_ramp(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = bigint_to_ramp(rng.gen_bigint(bits));
    let y = bigint_to_ramp(rng.gen_bigint(bits));

    b.iter(|| &x / &y);
}

fn div_bench_native(b: &mut Bencher) {
    let mut rng = get_rng();
    b.iter(|| rng.next_u64() / rng.next_u64());
}

///////////////////////////////////////////////////////////////////////////////
// Addition benches

fn add_bench_bigint(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = rng.gen_bigint(bits);
    let y = rng.gen_bigint(bits);

    b.iter(|| &x + &y);
}

#[cfg(feature = "rug")]
fn add_bench_rug(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = bigint_to_rug(rng.gen_bigint(bits));
    let y = bigint_to_rug(rng.gen_bigint(bits));
    b.iter(|| RugInteger::from(&x + &y));
}

#[cfg(feature = "ramp")]
fn add_bench_ramp(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = bigint_to_ramp(rng.gen_bigint(bits));
    let y = bigint_to_ramp(rng.gen_bigint(bits));
    b.iter(|| &x + &y);
}

fn add_bench_native(b: &mut Bencher) {
    let mut rng = get_rng();
    b.iter(|| rng.next_u64() + rng.next_u64());
}

///////////////////////////////////////////////////////////////////////////////
// GCD benches

fn gcd_bench_bigint(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = rng.gen_bigint(bits);
    let y = rng.gen_bigint(bits);

    b.iter(|| BigInt::gcd(&x, &y));
}

#[cfg(feature = "rug")]
fn gcd_bench_rug(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = bigint_to_rug(rng.gen_bigint(bits));
    let y = bigint_to_rug(rng.gen_bigint(bits));
    b.iter(|| RugInteger::from(RugInteger::gcd_ref(&x, &y)));
}

#[cfg(feature = "ramp")]
fn gcd_bench_ramp(b: &mut Bencher, bits: u64) {
    let mut rng = get_rng();
    let x = bigint_to_ramp(rng.gen_bigint(bits));
    let y = bigint_to_ramp(rng.gen_bigint(bits));
    b.iter(|| RampInt::gcd(&x, &y));
}

fn gcd_bench_native(b: &mut Bencher) {
    let mut rng = get_rng();
    let x = rng.next_u64();
    let y = rng.next_u64();

    b.iter(|| x.gcd(&y));
}

///////////////////////////////////////////////////////////////////////////////
// Groups

fn gcd_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    group.bench_function("num", |b| gcd_bench_bigint(b, bits));
    #[cfg(feature = "rug")]
    group.bench_function("rug", |b| gcd_bench_rug(b, bits));
    #[cfg(feature = "ramp")]
    group.bench_function("ramp", |b| gcd_bench_ramp(b, bits));
}

fn mul_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    group.bench_function("num", |b| mul_bench_bigint(b, bits));
    #[cfg(feature = "rug")]
    group.bench_function("rug", |b| mul_bench_rug(b, bits));
    #[cfg(feature = "ramp")]
    group.bench_function("ramp", |b| mul_bench_ramp(b, bits));
}

fn add_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    group.bench_function("num", |b| add_bench_bigint(b, bits));
    #[cfg(feature = "rug")]
    group.bench_function("rug", |b| add_bench_rug(b, bits));
    #[cfg(feature = "ramp")]
    group.bench_function("ramp", |b| add_bench_ramp(b, bits));
}

fn div_group(group: &mut BenchmarkGroup<'_, WallTime>, bits: u64) {
    group.bench_function("num", |b| div_bench_bigint(b, bits));
    #[cfg(feature = "rug")]
    group.bench_function("rug", |b| div_bench_rug(b, bits));
    #[cfg(feature = "ramp")]
    group.bench_function("ramp", |b| div_bench_ramp(b, bits));
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
