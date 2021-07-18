use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use num_bigint::BigInt;
use num_rational::{BigRational, Ratio};

mod rng;
use rng::get_rng;

fn alloc_ratio_bigint_bench(b: &mut Bencher) {
    use rand::RngCore;
    let mut rng = get_rng();
    b.iter(|| {
        let a = BigInt::from(rng.next_u64());
        let b = BigInt::from(rng.next_u64());
        BigRational::new(a, b)
    });
}

fn alloc_ratio_u64_bench(b: &mut Bencher) {
    use rand::RngCore;
    let mut rng = get_rng();
    b.iter(|| {
        let a = rng.next_u64();
        let b = rng.next_u64();
        Ratio::new(a, b)
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("alloc_ratio_bigint_bench", alloc_ratio_bigint_bench);
    c.bench_function("alloc_ratio_u64_bench", alloc_ratio_u64_bench);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
