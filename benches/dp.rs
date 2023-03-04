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
            BenchmarkId::new("x -> y -> z", parameter),
            parameter,
            |b, par| b.iter(|| dp_xyz(spec.clone(), *par)),
        );
        group.bench_with_input(
            BenchmarkId::new("x -> z -> y", parameter),
            parameter,
            |b, par| b.iter(|| dp_xzy(spec.clone(), *par)),
        );
        group.bench_with_input(
            BenchmarkId::new("y -> x -> z", parameter),
            parameter,
            |b, par| b.iter(|| dp_yxz(spec.clone(), *par)),
        );
        group.bench_with_input(
            BenchmarkId::new("y -> z -> x", parameter),
            parameter,
            |b, par| b.iter(|| dp_yzx(spec.clone(), *par)),
        );
        group.bench_with_input(
            BenchmarkId::new("z -> x -> y", parameter),
            parameter,
            |b, par| b.iter(|| dp_zxy(spec.clone(), *par)),
        );
        group.bench_with_input(
            BenchmarkId::new("z -> y -> x", parameter),
            parameter,
            |b, par| b.iter(|| dp_zyx(spec.clone(), *par)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
