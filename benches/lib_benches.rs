use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tokio::runtime::Runtime;
use template::{sync_test, async_test};

pub fn sync_bench(c: &mut Criterion) {
    c.bench_function("test 15", |b| b.iter(|| sync_test(black_box("argument"))));
}

fn async_bench(c: &mut Criterion) {
    let size: usize = 1024;

    c.bench_with_input(BenchmarkId::new("async_test", size), &size, |b, &s| {
        b.to_async(Runtime::new().unwrap()).iter(||async_test(s));
    });
}

criterion_group!(sync_benches, sync_bench);
criterion_group!(async_benches, async_bench);

criterion_main!(sync_benches, async_benches);
