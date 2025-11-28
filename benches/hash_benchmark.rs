use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::hint::black_box;
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Digest as Sha256Digest};
use sha3::{Sha3_256, Digest as Sha3Digest};

fn benchmark_all_hashes(c: &mut Criterion) {
    let message = "Hello, world!".as_bytes();
    c.bench_function("CRC32", |b| {
        b.iter(|| crc32fast::hash(black_box(message)))
    });
    c.bench_function("MD5", |b| {
        b.iter(|| md5::compute(black_box(message)))
    });
    c.bench_function("SHA-1", |b| {
        b.iter(|| Sha1::digest(black_box(message)))
    });
    c.bench_function("SHA-256", |b| {
        b.iter(|| Sha256::digest(black_box(message)))
    });
    c.bench_function("SHA-3", |b| {
        b.iter(|| Sha3_256::digest(black_box(message)))
    });
}

fn benchmark_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("SHA-256 by size");

    for size in [16, 64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| Sha256::digest(black_box(&data)))
        });
    }

    group.finish();
}

fn benchmark_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("All algorithms");
    let message = b"Hello, world!";

    group.bench_function("CRC32", |b| b.iter(|| crc32fast::hash(black_box(message))));
    group.bench_function("MD5", |b| b.iter(|| md5::compute(black_box(message))));
    group.bench_function("SHA-1", |b| b.iter(|| Sha1::digest(black_box(message))));
    group.bench_function("SHA-256", |b| b.iter(|| Sha256::digest(black_box(message))));
    group.bench_function("SHA-3", |b| b.iter(|| Sha3_256::digest(black_box(message))));

    group.finish();
}

criterion_group!(benches, benchmark_all_hashes, benchmark_different_sizes, benchmark_comparison);
criterion_main!(benches);

#[cfg(test)]
mod simple_benchmark {
    use super::*;
    use std::time::Instant;

    const ITERATIONS: u32 = 100_000;

    #[test]
    fn quick_benchmark() {
        let message = b"Hello, world!";

        println!("\n{}", "=".repeat(60));
        println!("Швидкий бенчмарк ({} ітерацій)", ITERATIONS);
        println!("{}", "=".repeat(60));

        let start = Instant::now();
        for _ in 0..ITERATIONS {
            crc32fast::hash(message);
        }
        let crc_time = start.elapsed();
        println!("CRC32:   {:>10.2?} ({:.0} оп/сек)",
                 crc_time / ITERATIONS,
                 ITERATIONS as f64 / crc_time.as_secs_f64());

        // MD5
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            md5::compute(message);
        }
        let md5_time = start.elapsed();
        println!("MD5:     {:>10.2?} ({:.0} оп/сек)",
                 md5_time / ITERATIONS,
                 ITERATIONS as f64 / md5_time.as_secs_f64());

        // SHA-1
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            Sha1::digest(message);
        }
        let sha1_time = start.elapsed();
        println!("SHA-1:   {:>10.2?} ({:.0} оп/сек)",
                 sha1_time / ITERATIONS,
                 ITERATIONS as f64 / sha1_time.as_secs_f64());

        // SHA-256
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            Sha256::digest(message);
        }
        let sha256_time = start.elapsed();
        println!("SHA-256: {:>10.2?} ({:.0} оп/сек)",
                 sha256_time / ITERATIONS,
                 ITERATIONS as f64 / sha256_time.as_secs_f64());

        // SHA-3
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            Sha3_256::digest(message);
        }
        let sha3_time = start.elapsed();
        println!("SHA-3:   {:>10.2?} ({:.0} оп/сек)",
                 sha3_time / ITERATIONS,
                 ITERATIONS as f64 / sha3_time.as_secs_f64());

        println!("{}", "=".repeat(60));
    }
}