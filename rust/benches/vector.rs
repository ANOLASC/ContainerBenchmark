use criterion::{criterion_group, criterion_main, Criterion};

use criterion::BenchmarkId;
use criterion::Throughput;

// pub fn vector_insert_front_16bit(insert_size: usize) {
//     let mut bench_vec = Vec::new();
//     for _ in 0..insert_size {
//         bench_vec.insert(0, 1_i16);
//     }
// }

pub fn vector_insert_front_32bit(insert_size: usize) {
    let mut bench_vec = Vec::new();
    for _ in 0..insert_size {
        bench_vec.insert(0, 1);
    }
}

pub fn vector_insert_front_64bit(insert_size: usize) {
    let mut bench_vec = Vec::new();
    for _ in 0..insert_size {
        bench_vec.insert(0, 1_i64);
    }
}

pub fn vector_random_remove() {}

pub fn insert_benchmark(c: &mut Criterion) {
    static K: usize = 1000;

    insert_bench!(16, c);

    // let mut group_16 = c.benchmark_group("vector_insert_front_32bit");
    // for size in [100, K, 10 * K, 100 * K].iter() {
    //     group_16.throughput(Throughput::Bytes(16_u64));
    //     group_16.sample_size(10);
    //     group_16.measurement_time(std::time::Duration::from_millis(100000));
    //     group_16.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
    //         b.iter(|| vector_insert_front_32bit(size));
    //     });
    // }
    // group_16.finish();

    // let mut group_32 = c.benchmark_group("vector_insert_front_32bit");
    // for size in [100, K, 10 * K, 100 * K].iter() {
    //     group_32.throughput(Throughput::Bytes(32_u64));
    //     group_32.sample_size(10);
    //     group_32.measurement_time(std::time::Duration::from_millis(100000));
    //     group_32.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
    //         b.iter(|| vector_insert_front_32bit(size));
    //     });
    // }
    // group_32.finish();

    // let mut group_64 = c.benchmark_group("vector_insert_front_64bit");
    // for size in [100, K, 10 * K, 100 * K].iter() {
    //     group_64.throughput(Throughput::Bytes(64_u64));
    //     group_64.sample_size(10);
    //     group_64.measurement_time(std::time::Duration::from_millis(100000));
    //     group_64.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
    //         b.iter(|| vector_insert_front_32bit(size));
    //     });
    // }
    // group_64.finish();
}

#[macro_export]
macro_rules! insert_bench {
    ($bit: tt, $c: ident) => {
        paste::paste! {
            let mut [<group_ $bit>] = $c.benchmark_group("vector_insert_front_64bit");
            for size in [100, K, 10 * K, 100 * K].iter() {
                [<group_ $bit>].throughput(Throughput::Bytes($bit));
                [<group_ $bit>].sample_size(10);
                [<group_ $bit>].measurement_time(std::time::Duration::from_millis(100000));
                [<group_ $bit>].bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
                    b.iter(|| [<vector_insert_front_ $bit bit>] (size));
                });
            }
            [<group_ $bit>].finish();
        }
    };
}

use paste::paste;
macro_rules! create_vec_insert {
    ($bit: tt) => {
        paste! {
            fn [<vector_insert_front_ $bit bit>](insert_size: usize) {
                let mut [<vec_ $bit>] = Vec::new();
                for _ in 0..insert_size {
                    [<vec_ $bit>].insert(0, 1 as [<u $bit>]);
                }
            }
        }
    };
}

create_vec_insert!(16);

criterion_group!(benches, insert_benchmark);
criterion_main!(benches);
