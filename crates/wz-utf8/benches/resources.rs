use criterion::{black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use wz_core::Counter;

const LOREM: &[u8] = include_bytes!("../tests/resources/Lorem_big.txt");

fn counter_benchmark(b: &mut Bencher, mut counter: impl Counter<usize>) {
    b.iter(|| counter.count(black_box(LOREM)))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new(std::any::type_name::<wz_utf8::Bytes>(), "Default"),
        &wz_utf8::Bytes::default(),
        |b, i| counter_benchmark(b, i.clone()),
    );

    c.bench_with_input(
        BenchmarkId::new(std::any::type_name::<wz_utf8::Lines>(), "LF"),
        &wz_utf8::Lines::with_linebreak(b'\n'),
        |b, i| counter_benchmark(b, i.clone()),
    );

    c.bench_with_input(
        BenchmarkId::new(std::any::type_name::<wz_utf8::Chars>(), "Default"),
        &wz_utf8::Chars::default(),
        |b, i| counter_benchmark(b, i.clone()),
    );

    c.bench_with_input(
        BenchmarkId::new(std::any::type_name::<wz_utf8::Words>(), "Default"),
        &wz_utf8::Words::default(),
        |b, i| counter_benchmark(b, i.clone()),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
