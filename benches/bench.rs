use std::hint::black_box;

use rand::prelude::*;
use criterion::{Criterion, criterion_group, criterion_main};

use calculator::*;

fn test1(c: &mut Criterion) {
    c.bench_function("詞法分析", |b| {
        let mut random = rand::rng();
        let x = random.random_range(0..=100);
        let y = random.random_range(0..=100);
        let input = format!("{}+{}", x, y);
        b.iter(|| black_box(lexical_analysis(&input)));
    });
}

fn test2(c: &mut Criterion) {
    let mut group = c.benchmark_group("詞法分析對比");
    // let mut input = "1+1".repeat(1_000_000);
    group.bench_function("0到100(含)的隨機數", |b| {
        let mut random = rand::rng();
        let x = random.random_range(0..=100);
        let y = random.random_range(0..=100);
        let input = format!("{}+{}", x, y);
        // input = format!("{}+{}", x, y);
        b.iter(|| black_box(lexical_analysis(&input)));
    });
    group.bench_function("100到1000(含)的隨機數", |b| {
        let mut random = rand::rng();
        let x = random.random_range(100..=1000);
        let y = random.random_range(100..=1000);
        let input = format!("{}+{}", x, y);
        // input = format!("{}+{}", x, y);
        b.iter(|| black_box(lexical_analysis(&input)));
    });
    group.bench_function("1000到10000(含)的隨機數", |b| {
        let mut random = rand::rng();
        let x = random.random_range(1000..=10000);
        let y = random.random_range(1000..=10000);
        let input = format!("{}+{}", x, y);
        // input = format!("{}+{}", x, y);
        b.iter(|| black_box(lexical_analysis(&input)));
    });
    group.finish();
}

criterion_group!(name = test; config = Criterion::default().warm_up_time(std::time::Duration::from_secs(5)); targets = test2);
criterion_main!(test);
