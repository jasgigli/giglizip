use criterion::{criterion_group, criterion_main, Criterion};
use giglizip_compress::compress;

fn bench_compress(c: &mut Criterion) {
    let data = vec![0u8; 1024 * 1024];
    c.bench_function("compress 1MB", |b| b.iter(|| compress(&data)));
}

criterion_group!(benches, bench_compress);
criterion_main!(benches);
