use morello_rs::*;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench(c: &mut Criterion) {
    let spec = Spec::new(100, 100, 100);
    let bsize = 10; // optimum block size for the 100*100*100 spec

    let mut group = c.benchmark_group("DP");
    group.sample_size(10); // 10 is minimum required; default is 100
    #[allow(clippy::single_element_loop)]
    for parameter in [bsize].iter() {
        group.throughput(Throughput::Elements(*parameter as u64));
        group.bench_with_input(
            BenchmarkId::new("Parallel", parameter),
            parameter,
            |b, par| b.iter(|| dp(spec.clone(), *par)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
