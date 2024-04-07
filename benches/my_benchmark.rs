use ark_bls12_381::Fq;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::mem;

pub struct MyStruct {
    a: Vec<Fq>,
}

const SIZE: usize = 1 << 20;

impl MyStruct {
    fn new(size: usize) -> Self {
        MyStruct {
            a: (0..size).map(|i| Fq::from(i as u64)).collect(),
        }
    }

    fn take_vector(&mut self) -> Vec<Fq> {
        mem::take(&mut self.a)
    }
}

fn clone_vector_benchmark(c: &mut Criterion) {
    let my_struct = MyStruct::new(SIZE);

    c.bench_function("clone_vector", |b| {
        b.iter(|| {
            let cloned_vector = my_struct.a.clone();
            black_box(cloned_vector);
        })
    });
}

fn take_vector_benchmark(c: &mut Criterion) {
    let mut my_struct = MyStruct::new(SIZE);

    c.bench_function("take_vector", |b| {
        b.iter(|| {
            let taken_vector = my_struct.take_vector();
            black_box(taken_vector);
        })
    });
}

criterion_group!(benches, clone_vector_benchmark, take_vector_benchmark);
criterion_main!(benches);
