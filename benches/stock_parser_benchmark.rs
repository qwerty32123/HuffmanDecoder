// Create this as benches/stock_parser_benchmark.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crate::stock_parser::{parse_stock_bytes, parse_stock_iterator, parse_stock_simple};

pub mod stock_parser;
// Replace with your actual crate name

fn generate_test_data(size: usize) -> String {
    let mut result = String::with_capacity(size * 30);
    for i in 0..size {
        let stock = if i % 3 == 0 { "1" } else { "0" };
        result.push_str(&format!("2{:04}-{}-{}-1100000000|", i, stock, i * 10));
    }
    result
}

fn benchmark_parsers(c: &mut Criterion) {
    let small_data = "20067-0-104-1100000000|20069-1-47-1100000000|21021-0-447-1630000000|";
    let large_data = generate_test_data(1000);

    let mut group = c.benchmark_group("stock_parser");

    // Small data benchmarks
    group.bench_function("simple_small", |b| {
        b.iter(|| parse_stock_simple(black_box(small_data)))
    });
    group.bench_function("iterator_small", |b| {
        b.iter(|| parse_stock_iterator(black_box(small_data)))
    });
    group.bench_function("bytes_small", |b| {
        b.iter(|| parse_stock_bytes(black_box(small_data.as_ref())))
    });

    // Large data benchmarks
    group.bench_function("simple_large", |b| {
        b.iter(|| parse_stock_simple(black_box(&large_data)))
    });
    group.bench_function("iterator_large", |b| {
        b.iter(|| parse_stock_iterator(black_box(&large_data)))
    });
    group.bench_function("bytes_large", |b| {
        b.iter(|| parse_stock_bytes(black_box((&large_data).as_ref())))
    });

    group.finish();
}

criterion_group!(benches, benchmark_parsers);
criterion_main!(benches);