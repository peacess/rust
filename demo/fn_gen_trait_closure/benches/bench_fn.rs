use std::ops::Deref;

use criterion::{Criterion, criterion_group, criterion_main, PlotConfiguration, PlottingBackend};

fn sum(data: &[i64]) -> i64 {
    let mut re = i64::default();
    for i in 0..data.len() {
        re += data[i];
    }
    re
}

trait Summer {
    fn sum(&self, data: &[i64]) -> i64;
}

struct SummerImp {}

impl Summer for SummerImp {
    fn sum(&self, data: &[i64]) -> i64 {
        let mut re = i64::default();
        for i in 0..data.len() {
            re += data[i];
        }
        re
    }
}

fn sum_g<T: std::ops::AddAssign + Copy + Default>(data: &[T]) -> T {
    let mut re = T::default();
    for i in 0..data.len() {
        re += data[i];
    }
    re
}


fn criterion_benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("compare: ");
    let data = vec![1, 90, 76, 6688];
    let mut re = 0;
    c.bench_function("fn", |b| b.iter(|| {
        re = sum(&data);
    }));
    let summer = SummerImp {};
    let sum_trait: &dyn Summer = &summer;
    c.bench_function("trait object", |b| b.iter(|| {
        re = sum_trait.sum(&data);
    }));
    c.bench_function("closure", |b| b.iter(|| {
        re = (|data: &[i64]| {
            let mut re = i64::default();
            for i in 0..data.len() {
                re += data[i];
            }
            re
        })(&data);
    }));

    c.bench_function("closure no parameter", |b| b.iter(|| {
        re = (|| {
            let mut re = i64::default();
            for i in 0..data.len() {
                re += data[i];
            }
            re
        })();
    }));
    let p_sum = sum;
    c.bench_function("fn pointer", |b| b.iter(|| {
        re = p_sum(&data);
    }));

    c.bench_function("generics", |b| b.iter(|| {
        re = p_sum(&data);
    }));
    c.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);