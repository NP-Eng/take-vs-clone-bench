use ark_bls12_381::Fq;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::mem;

const SIZE: usize = 1 << 24;

mod ext {
    use super::*;
    pub struct MyStruct {
        a: Vec<Fq>,
        pub b: String,
    }

    impl MyStruct {
        pub fn a(self) -> Vec<Fq> {
            self.a
        }

        pub fn a_ref(&self) -> &Vec<Fq> {
            &self.a
        }

        pub fn a_mut(&mut self) -> &mut Vec<Fq> {
            &mut self.a
        }
    }

    impl MyStruct {
        pub fn new(size: usize) -> Self {
            MyStruct {
                a: (0..size).map(|i| Fq::from(i as u64)).collect(),
                b: "Hello, World!".to_string(),
            }
        }
    }
}

use ext::MyStruct;

pub fn clone_function(s: MyStruct) -> Vec<Fq> {
    let a: Vec<Fq> = s.a_ref().clone();

    // anonymous closure doing something to `b`, which doesn't use `a`.
    let _ = || s.b;

    a
}

pub fn take_function(mut s: MyStruct) -> Vec<Fq> {
    let a = { mem::take(s.a_mut()) };

    // anonymous closure doing something to `b`, which doesn't use `a`.
    let _ = || s.b;

    a
}

fn clone_vector_benchmark(c: &mut Criterion) {
    c.bench_function("clone_vector", |b| {
        b.iter_batched(
            || MyStruct::new(SIZE),
            |my_struct| clone_function(my_struct),
            BatchSize::LargeInput,
        )
    });
}

fn take_vector_benchmark(c: &mut Criterion) {
    c.bench_function("take_vector", |b| {
        b.iter_batched(
            || MyStruct::new(SIZE),
            |my_struct| take_function(my_struct),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, clone_vector_benchmark, take_vector_benchmark);
criterion_main!(benches);
