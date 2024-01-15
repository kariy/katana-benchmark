use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn contract_class_storage_serde(c: &mut Criterion) {
    let sierra_class = include_str!("./resources/sierra/dojo_world_240.json");

    c.bench_function("katana", |b| {
        // b.iter_with_large_drop(|| foo(black_box(class.clone())))
    });

    c.bench_function("papyrus", |b| {
        // b.iter_with_large_drop(|| foo(black_box(class.clone())))
    });
}

criterion_group!(serde, contract_class_storage_serde);
criterion_main!(serde);
