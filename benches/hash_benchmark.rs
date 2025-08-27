//! Benchmarks for hash functionality

use atlas_common::hash::{
    calculate_hash, calculate_hash_with_algorithm, combine_hashes, verify_hash, HashAlgorithm,
    HashBuilder, Hasher,
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_hash_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_algorithms");

    // Test with different data sizes
    let sizes = vec![
        ("small", 32),
        ("medium", 1024),
        ("large", 1024 * 1024),
        ("xlarge", 10 * 1024 * 1024),
    ];

    for (name, size) in sizes {
        let data = vec![0u8; size];
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("sha256", name), &data, |b, data| {
            b.iter(|| calculate_hash_with_algorithm(black_box(data), &HashAlgorithm::Sha256));
        });

        group.bench_with_input(BenchmarkId::new("sha384", name), &data, |b, data| {
            b.iter(|| calculate_hash_with_algorithm(black_box(data), &HashAlgorithm::Sha384));
        });

        group.bench_with_input(BenchmarkId::new("sha512", name), &data, |b, data| {
            b.iter(|| calculate_hash_with_algorithm(black_box(data), &HashAlgorithm::Sha512));
        });
    }

    group.finish();
}

fn bench_hash_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_builder");

    let chunk_sizes = vec![
        ("small_chunks", 16),
        ("medium_chunks", 256),
        ("large_chunks", 4096),
    ];

    let total_size = 1024 * 1024; // 1 MB total

    for (name, chunk_size) in chunk_sizes {
        let chunks: Vec<Vec<u8>> = (0..total_size / chunk_size)
            .map(|_| vec![0u8; chunk_size])
            .collect();

        group.throughput(Throughput::Bytes(total_size as u64));

        group.bench_with_input(
            BenchmarkId::new("incremental", name),
            &chunks,
            |b, chunks| {
                b.iter(|| {
                    let mut builder = HashBuilder::new(HashAlgorithm::Sha384);
                    for chunk in chunks {
                        builder.update(black_box(chunk));
                    }
                    builder.finalize()
                });
            },
        );
    }

    group.finish();
}

fn bench_hasher_trait(c: &mut Criterion) {
    let mut group = c.benchmark_group("hasher_trait");

    let text_small = "Hello, World!";
    let text_medium = "a".repeat(1000);
    let text_large = "b".repeat(100_000);

    group.bench_function("str_small", |b| {
        b.iter(|| black_box(text_small).hash(HashAlgorithm::Sha384));
    });

    group.bench_function("string_medium", |b| {
        b.iter(|| black_box(&text_medium).hash(HashAlgorithm::Sha384));
    });

    group.bench_function("bytes_large", |b| {
        let bytes = text_large.as_bytes();
        b.iter(|| black_box(bytes).hash(HashAlgorithm::Sha384));
    });

    group.finish();
}

fn bench_hash_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_verification");

    let data_sizes = vec![("small", 100), ("medium", 10_000), ("large", 1_000_000)];

    for (name, size) in data_sizes {
        let data = vec![42u8; size];
        let hash = calculate_hash(&data);

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("verify", name),
            &(&data, &hash),
            |b, (data, hash)| {
                b.iter(|| verify_hash(black_box(data), black_box(hash)));
            },
        );
    }

    group.finish();
}

fn bench_combine_hashes(c: &mut Criterion) {
    let mut group = c.benchmark_group("combine_hashes");

    let hash1 = calculate_hash(b"data1");
    let hash2 = calculate_hash(b"data2");
    let hash3 = calculate_hash(b"data3");
    let hash4 = calculate_hash(b"data4");
    let hash5 = calculate_hash(b"data5");

    group.bench_function("combine_2", |b| {
        b.iter(|| combine_hashes(black_box(&[&hash1, &hash2])));
    });

    group.bench_function("combine_3", |b| {
        b.iter(|| combine_hashes(black_box(&[&hash1, &hash2, &hash3])));
    });

    group.bench_function("combine_5", |b| {
        b.iter(|| combine_hashes(black_box(&[&hash1, &hash2, &hash3, &hash4, &hash5])));
    });

    group.finish();
}

fn bench_default_vs_specific(c: &mut Criterion) {
    let mut group = c.benchmark_group("default_vs_specific");

    let data = vec![0u8; 10_000];

    group.bench_function("default_sha384", |b| {
        b.iter(|| calculate_hash(black_box(&data)));
    });

    group.bench_function("explicit_sha384", |b| {
        b.iter(|| calculate_hash_with_algorithm(black_box(&data), &HashAlgorithm::Sha384));
    });

    group.finish();
}

fn bench_algorithm_detection(c: &mut Criterion) {
    use atlas_common::hash::detect_hash_algorithm;

    let sha256_hash = "a".repeat(64);
    let sha384_hash = "b".repeat(96);
    let sha512_hash = "c".repeat(128);

    c.bench_function("detect_sha256", |b| {
        b.iter(|| detect_hash_algorithm(black_box(&sha256_hash)));
    });

    c.bench_function("detect_sha384", |b| {
        b.iter(|| detect_hash_algorithm(black_box(&sha384_hash)));
    });

    c.bench_function("detect_sha512", |b| {
        b.iter(|| detect_hash_algorithm(black_box(&sha512_hash)));
    });
}

fn bench_hash_validation(c: &mut Criterion) {
    use atlas_common::hash::validate_hash_format;

    let valid_hash = "a".repeat(96);
    let invalid_hash = "xyz123";

    c.bench_function("validate_valid", |b| {
        b.iter(|| validate_hash_format(black_box(&valid_hash)));
    });

    c.bench_function("validate_invalid", |b| {
        b.iter(|| validate_hash_format(black_box(&invalid_hash)));
    });
}

criterion_group!(
    benches,
    bench_hash_algorithms,
    bench_hash_builder,
    bench_hasher_trait,
    bench_hash_verification,
    bench_combine_hashes,
    bench_default_vs_specific,
    bench_algorithm_detection,
    bench_hash_validation,
);

criterion_main!(benches);
