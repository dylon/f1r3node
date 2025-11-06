use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use models::rhoapi::{Par, Send, Receive, New, Expr, Match, Bundle};
use rholang::rust::interpreter::matcher::{par_count::ParCount, sub_pars::sub_pars};

fn create_test_par(
    n_sends: usize,
    n_receives: usize,
    n_news: usize,
    n_exprs: usize,
    n_matches: usize,
    n_unforgeables: usize,
    n_bundles: usize,
) -> Par {
    Par {
        sends: vec![Send::default(); n_sends],
        receives: vec![Receive::default(); n_receives],
        news: vec![New::default(); n_news],
        exprs: vec![Expr::default(); n_exprs],
        matches: vec![Match::default(); n_matches],
        unforgeables: vec![Default::default(); n_unforgeables],
        bundles: vec![Bundle::default(); n_bundles],
        connectives: Vec::default(),
        locally_free: Vec::default(),
        connective_used: false,
    }
}

fn bench_sub_pars_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub_pars_small");

    let test_cases = vec![
        (1, 1, 1, 1, 0, 0, 0, "1-1-1-1-0-0-0"),
        (2, 1, 1, 1, 0, 0, 0, "2-1-1-1-0-0-0"),
        (2, 2, 1, 1, 0, 0, 0, "2-2-1-1-0-0-0"),
        (2, 2, 2, 1, 0, 0, 0, "2-2-2-1-0-0-0"),
        (2, 2, 2, 2, 0, 0, 0, "2-2-2-2-0-0-0"),
    ];

    for (sends, receives, news, exprs, matches, unfs, bundles, name) in test_cases {
        let par = create_test_par(sends, receives, news, exprs, matches, unfs, bundles);
        let min = ParCount {
            sends: 0,
            receives: 0,
            news: 0,
            exprs: 0,
            matches: 0,
            unforgeables: 0,
            bundles: 0,
        };
        let max = ParCount {
            sends,
            receives,
            news,
            exprs,
            matches,
            unforgeables: unfs,
            bundles,
        };
        let min_prune = ParCount {
            sends: 0,
            receives: 0,
            news: 0,
            exprs: 0,
            matches: 0,
            unforgeables: 0,
            bundles: 0,
        };
        let max_prune = ParCount {
            sends: 0,
            receives: 0,
            news: 0,
            exprs: 0,
            matches: 0,
            unforgeables: 0,
            bundles: 0,
        };

        group.bench_with_input(BenchmarkId::from_parameter(name), &par, |b, par| {
            b.iter(|| {
                let result: Vec<_> = sub_pars(
                    black_box(par),
                    black_box(&min),
                    black_box(&max),
                    black_box(&min_prune),
                    black_box(&max_prune),
                ).collect();
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_sub_pars_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub_pars_medium");
    group.sample_size(10);

    let test_cases = vec![
        (3, 2, 2, 2, 1, 0, 0, "3-2-2-2-1-0-0"),
        (3, 3, 2, 2, 1, 0, 0, "3-3-2-2-1-0-0"),
        (3, 3, 3, 2, 1, 0, 0, "3-3-3-2-1-0-0"),
        (4, 3, 2, 2, 1, 0, 0, "4-3-2-2-1-0-0"),
    ];

    for (sends, receives, news, exprs, matches, unfs, bundles, name) in test_cases {
        let par = create_test_par(sends, receives, news, exprs, matches, unfs, bundles);
        let min = ParCount {
            sends: 0,
            receives: 0,
            news: 0,
            exprs: 0,
            matches: 0,
            unforgeables: 0,
            bundles: 0,
        };
        let max = ParCount {
            sends,
            receives,
            news,
            exprs,
            matches,
            unforgeables: unfs,
            bundles,
        };
        let min_prune = ParCount {
            sends: 0,
            receives: 0,
            news: 0,
            exprs: 0,
            matches: 0,
            unforgeables: 0,
            bundles: 0,
        };
        let max_prune = ParCount {
            sends: 0,
            receives: 0,
            news: 0,
            exprs: 0,
            matches: 0,
            unforgeables: 0,
            bundles: 0,
        };

        group.bench_with_input(BenchmarkId::from_parameter(name), &par, |b, par| {
            b.iter(|| {
                let result: Vec<_> = sub_pars(
                    black_box(par),
                    black_box(&min),
                    black_box(&max),
                    black_box(&min_prune),
                    black_box(&max_prune),
                ).collect();
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_sub_pars_realistic(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub_pars_realistic");
    group.sample_size(10);

    let par = create_test_par(5, 5, 3, 8, 2, 1, 1);

    let min = ParCount {
        sends: 0,
        receives: 0,
        news: 0,
        exprs: 0,
        matches: 0,
        unforgeables: 0,
        bundles: 0,
    };
    let max = ParCount {
        sends: 5,
        receives: 5,
        news: 3,
        exprs: 8,
        matches: 2,
        unforgeables: 1,
        bundles: 1,
    };
    let min_prune = ParCount {
        sends: 0,
        receives: 0,
        news: 0,
        exprs: 0,
        matches: 0,
        unforgeables: 0,
        bundles: 0,
    };
    let max_prune = ParCount {
        sends: 0,
        receives: 0,
        news: 0,
        exprs: 0,
        matches: 0,
        unforgeables: 0,
        bundles: 0,
    };

    group.bench_function("realistic_5-5-3-8-2-1-1_full", |b| {
        b.iter(|| {
            let mut count = 0;
            for pair in sub_pars(
                black_box(&par),
                black_box(&min),
                black_box(&max),
                black_box(&min_prune),
                black_box(&max_prune),
            ) {
                count += 1;
                black_box(pair);
            }
            black_box(count)
        });
    });

    let constrained_max = ParCount {
        sends: 2,
        receives: 2,
        news: 1,
        exprs: 3,
        matches: 1,
        unforgeables: 0,
        bundles: 0,
    };

    group.bench_function("realistic_5-5-3-8-2-1-1_constrained", |b| {
        b.iter(|| {
            let mut count = 0;
            for pair in sub_pars(
                black_box(&par),
                black_box(&min),
                black_box(&constrained_max),
                black_box(&min_prune),
                black_box(&max_prune),
            ) {
                count += 1;
                black_box(pair);
            }
            black_box(count)
        });
    });

    group.finish();
}

fn bench_sub_pars_with_constraints(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub_pars_constraints");
    group.sample_size(10);

    let par = create_test_par(4, 4, 2, 4, 1, 0, 0);

    let min_prune = ParCount {
        sends: 0,
        receives: 0,
        news: 0,
        exprs: 0,
        matches: 0,
        unforgeables: 0,
        bundles: 0,
    };
    let max_prune = ParCount {
        sends: 0,
        receives: 0,
        news: 0,
        exprs: 0,
        matches: 0,
        unforgeables: 0,
        bundles: 0,
    };

    let test_cases = vec![
        (
            ParCount { sends: 1, receives: 1, news: 0, exprs: 1, matches: 0, unforgeables: 0, bundles: 0 },
            ParCount { sends: 2, receives: 2, news: 1, exprs: 2, matches: 1, unforgeables: 0, bundles: 0 },
            "min_1-1-0-1-0-0-0_max_2-2-1-2-1-0-0"
        ),
        (
            ParCount { sends: 0, receives: 0, news: 0, exprs: 0, matches: 0, unforgeables: 0, bundles: 0 },
            ParCount { sends: 2, receives: 2, news: 1, exprs: 2, matches: 1, unforgeables: 0, bundles: 0 },
            "min_0-0-0-0-0-0-0_max_2-2-1-2-1-0-0"
        ),
        (
            ParCount { sends: 2, receives: 2, news: 1, exprs: 2, matches: 0, unforgeables: 0, bundles: 0 },
            ParCount { sends: 2, receives: 2, news: 1, exprs: 2, matches: 0, unforgeables: 0, bundles: 0 },
            "exact_2-2-1-2-0-0-0"
        ),
    ];

    for (min, max, name) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &(&min, &max), |b, (min, max)| {
            b.iter(|| {
                let mut count = 0;
                for pair in sub_pars(
                    black_box(&par),
                    black_box(*min),
                    black_box(*max),
                    black_box(&min_prune),
                    black_box(&max_prune),
                ) {
                    count += 1;
                    black_box(pair);
                }
                black_box(count)
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_sub_pars_small,
    bench_sub_pars_medium,
    bench_sub_pars_realistic,
    bench_sub_pars_with_constraints
);
criterion_main!(benches);
