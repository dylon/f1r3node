use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use models::rhoapi::Par;
use rholang::rust::interpreter::compiler::bound_map_chain::BoundMapChain;
use rholang::rust::interpreter::compiler::free_map::FreeMap;
use rholang::rust::interpreter::env::Env;
use rholang_parser::{SourcePos, SourceSpan};

fn create_test_bindings_span(n: usize) -> Vec<(String, i32, SourceSpan)> {
    (0..n)
        .map(|i| {
            (
                format!("var{}", i),
                i as i32,
                SourceSpan {
                    start: SourcePos { line: i, col: 0 },
                    end: SourcePos { line: i, col: 10 },
                },
            )
        })
        .collect()
}

fn create_test_bindings_pos(n: usize) -> Vec<(String, i32, SourcePos)> {
    (0..n)
        .map(|i| {
            (
                format!("var{}", i),
                i as i32,
                SourcePos { line: i, col: 0 },
            )
        })
        .collect()
}

fn create_populated_free_map(n: usize) -> FreeMap<i32> {
    let bindings = create_test_bindings_span(n);
    let mut free_map = FreeMap::new();
    for binding in bindings {
        free_map = free_map.put_span(binding);
    }
    free_map
}

fn create_populated_bound_map_chain(n: usize, depth: usize) -> BoundMapChain<i32> {
    let mut chain = BoundMapChain::new();
    for _ in 0..depth {
        chain = chain.push();
        let bindings = create_test_bindings_span(n);
        chain = chain.put_all_span(bindings);
    }
    chain
}

fn create_populated_env(n: usize) -> Env<Par> {
    let mut env = Env::new();
    for _ in 0..n {
        env = env.put(Par::default());
    }
    env
}

fn bench_free_map_put_span_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("free_map_put_span_small");

    for size in [1, 5, 10] {
        let free_map = create_populated_free_map(size);
        let binding = (
            "new_var".to_string(),
            100_i32,
            SourceSpan {
                start: SourcePos { line: 100, col: 0 },
                end: SourcePos { line: 100, col: 10 },
            },
        );

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(free_map.put_span(black_box(binding.clone())))
            });
        });
    }

    group.finish();
}

fn bench_free_map_put_span_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("free_map_put_span_medium");
    group.sample_size(10);

    for size in [20, 50, 100] {
        let free_map = create_populated_free_map(size);
        let binding = (
            "new_var".to_string(),
            100_i32,
            SourceSpan {
                start: SourcePos { line: 100, col: 0 },
                end: SourcePos { line: 100, col: 10 },
            },
        );

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(free_map.put_span(black_box(binding.clone())))
            });
        });
    }

    group.finish();
}

fn bench_free_map_put_all_span_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("free_map_put_all_span_small");

    for (existing, new_count) in [(1, 1), (5, 5), (10, 10)] {
        let free_map = create_populated_free_map(existing);
        let new_bindings = create_test_bindings_span(new_count);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}-{}", existing, new_count)),
            &(existing, new_count),
            |b, _| {
                b.iter(|| {
                    black_box(free_map.put_all_span(black_box(new_bindings.clone())))
                });
            },
        );
    }

    group.finish();
}

fn bench_free_map_put_all_span_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("free_map_put_all_span_medium");
    group.sample_size(10);

    for (existing, new_count) in [(20, 20), (50, 50), (100, 100)] {
        let free_map = create_populated_free_map(existing);
        let new_bindings = create_test_bindings_span(new_count);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}-{}", existing, new_count)),
            &(existing, new_count),
            |b, _| {
                b.iter(|| {
                    black_box(free_map.put_all_span(black_box(new_bindings.clone())))
                });
            },
        );
    }

    group.finish();
}

fn bench_free_map_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("free_map_merge");
    group.sample_size(10);

    for size in [5, 20, 50] {
        let free_map1 = create_populated_free_map(size);
        let free_map2 = create_populated_free_map(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(free_map1.merge(black_box(free_map2.clone())))
            });
        });
    }

    group.finish();
}

fn bench_free_map_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("free_map_get");

    for size in [10, 50, 100] {
        let free_map = create_populated_free_map(size);
        let key = format!("var{}", size / 2);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(free_map.get(black_box(&key)))
            });
        });
    }

    group.finish();
}

fn bench_free_map_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("free_map_clone");

    for size in [5, 20, 50, 100, 500] {
        let free_map = create_populated_free_map(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(free_map.clone())
            });
        });
    }

    group.finish();
}

fn bench_bound_map_chain_put_span(c: &mut Criterion) {
    let mut group = c.benchmark_group("bound_map_chain_put_span");

    for (bindings_per_level, depth) in [(5, 1), (5, 5), (10, 10)] {
        let chain = create_populated_bound_map_chain(bindings_per_level, depth);
        let binding = (
            "new_var".to_string(),
            100_i32,
            SourceSpan {
                start: SourcePos { line: 100, col: 0 },
                end: SourcePos { line: 100, col: 10 },
            },
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}-{}", bindings_per_level, depth)),
            &(bindings_per_level, depth),
            |b, _| {
                b.iter(|| {
                    black_box(chain.put_span(black_box(binding.clone())))
                });
            },
        );
    }

    group.finish();
}

fn bench_bound_map_chain_put_all_span(c: &mut Criterion) {
    let mut group = c.benchmark_group("bound_map_chain_put_all_span");
    group.sample_size(10);

    for (bindings_per_level, depth, new_count) in [(5, 5, 5), (10, 10, 10), (20, 20, 20)] {
        let chain = create_populated_bound_map_chain(bindings_per_level, depth);
        let new_bindings = create_test_bindings_span(new_count);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}-{}-{}", bindings_per_level, depth, new_count)),
            &(bindings_per_level, depth, new_count),
            |b, _| {
                b.iter(|| {
                    black_box(chain.put_all_span(black_box(new_bindings.clone())))
                });
            },
        );
    }

    group.finish();
}

fn bench_bound_map_chain_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("bound_map_chain_push");

    for depth in [1, 5, 10, 20, 50] {
        let chain = create_populated_bound_map_chain(10, depth);

        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, _| {
            b.iter(|| {
                black_box(chain.push())
            });
        });
    }

    group.finish();
}

fn bench_bound_map_chain_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("bound_map_chain_find");

    for (bindings_per_level, depth) in [(5, 5), (10, 10), (20, 20)] {
        let chain = create_populated_bound_map_chain(bindings_per_level, depth);
        let key = format!("var{}", bindings_per_level / 2);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}-{}", bindings_per_level, depth)),
            &(bindings_per_level, depth),
            |b, _| {
                b.iter(|| {
                    black_box(chain.find(black_box(&key)))
                });
            },
        );
    }

    group.finish();
}

fn bench_bound_map_chain_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("bound_map_chain_clone");

    for (bindings_per_level, depth) in [(5, 5), (10, 10), (20, 20), (50, 50)] {
        let chain = create_populated_bound_map_chain(bindings_per_level, depth);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}-{}", bindings_per_level, depth)),
            &(bindings_per_level, depth),
            |b, _| {
                b.iter(|| {
                    black_box(chain.clone())
                });
            },
        );
    }

    group.finish();
}

fn bench_env_put(c: &mut Criterion) {
    let mut group = c.benchmark_group("env_put");

    for size in [5, 10, 20, 50, 100] {
        let mut env = create_populated_env(size);
        let value = Par::default();

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(env.put(black_box(value.clone())))
            });
        });
    }

    group.finish();
}

fn bench_env_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("env_get");

    for size in [10, 50, 100] {
        let env = create_populated_env(size);
        let key = (size / 2) as i32;

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(env.get(black_box(&key)))
            });
        });
    }

    group.finish();
}

fn bench_env_shift(c: &mut Criterion) {
    let mut group = c.benchmark_group("env_shift");

    for size in [10, 50, 100] {
        let env = create_populated_env(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(env.shift(black_box(1)))
            });
        });
    }

    group.finish();
}

fn bench_env_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("env_clone");

    for size in [5, 10, 20, 50, 100, 500] {
        let env = create_populated_env(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(env.clone())
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_free_map_put_span_small,
    bench_free_map_put_span_medium,
    bench_free_map_put_all_span_small,
    bench_free_map_put_all_span_medium,
    bench_free_map_merge,
    bench_free_map_get,
    bench_free_map_clone,
    bench_bound_map_chain_put_span,
    bench_bound_map_chain_put_all_span,
    bench_bound_map_chain_push,
    bench_bound_map_chain_find,
    bench_bound_map_chain_clone,
    bench_env_put,
    bench_env_get,
    bench_env_shift,
    bench_env_clone
);
criterion_main!(benches);
