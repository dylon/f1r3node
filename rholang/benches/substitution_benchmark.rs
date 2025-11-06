use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use models::rhoapi::{Par, Send, Receive, New, Expr, Match, Bundle};
use rholang::rust::interpreter::accounting::_cost;
use rholang::rust::interpreter::accounting::costs::Cost;
use rholang::rust::interpreter::env::Env;
use rholang::rust::interpreter::substitute::Substitute;

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

fn create_test_send() -> Send {
    Send {
        chan: Some(Par::default()),
        data: Vec::new(),
        persistent: false,
        locally_free: Vec::new(),
        connective_used: false,
    }
}

fn create_test_receive() -> Receive {
    Receive {
        binds: Vec::new(),
        body: Some(Par::default()),
        persistent: false,
        peek: false,
        bind_count: 0,
        locally_free: Vec::new(),
        connective_used: false,
    }
}

fn bench_substitute_and_charge_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("substitute_and_charge_small");

    let test_cases = vec![
        (1, 1, 1, 1, 0, 0, 0, "1-1-1-1-0-0-0"),
        (2, 1, 1, 1, 0, 0, 0, "2-1-1-1-0-0-0"),
        (2, 2, 1, 1, 0, 0, 0, "2-2-1-1-0-0-0"),
        (2, 2, 2, 1, 0, 0, 0, "2-2-2-1-0-0-0"),
        (2, 2, 2, 2, 0, 0, 0, "2-2-2-2-0-0-0"),
    ];

    for (sends, receives, news, exprs, matches, unfs, bundles, name) in test_cases {
        let par = create_test_par(sends, receives, news, exprs, matches, unfs, bundles);
        let env = Env::<Par>::new();

        group.bench_with_input(BenchmarkId::from_parameter(name), &par, |b, par| {
            b.iter(|| {
                let sub = Substitute { cost: _cost::new(Cost::unsafe_max(), 1000) };
                let result = sub.substitute_and_charge(
                    black_box(par.clone()),
                    black_box(0),
                    black_box(&env),
                );
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_substitute_and_charge_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("substitute_and_charge_medium");
    group.sample_size(10);

    let test_cases = vec![
        (3, 2, 2, 2, 1, 0, 0, "3-2-2-2-1-0-0"),
        (3, 3, 2, 2, 1, 0, 0, "3-3-2-2-1-0-0"),
        (4, 3, 3, 2, 1, 0, 0, "4-3-3-2-1-0-0"),
        (5, 4, 3, 3, 2, 0, 0, "5-4-3-3-2-0-0"),
    ];

    for (sends, receives, news, exprs, matches, unfs, bundles, name) in test_cases {
        let par = create_test_par(sends, receives, news, exprs, matches, unfs, bundles);
        let env = Env::<Par>::new();

        group.bench_with_input(BenchmarkId::from_parameter(name), &par, |b, par| {
            b.iter(|| {
                let sub = Substitute { cost: _cost::new(Cost::unsafe_max(), 1000) };
                let result = sub.substitute_and_charge(
                    black_box(par.clone()),
                    black_box(0),
                    black_box(&env),
                );
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_substitute_and_charge_realistic(c: &mut Criterion) {
    let mut group = c.benchmark_group("substitute_and_charge_realistic");
    group.sample_size(10);

    let par = create_test_par(5, 5, 3, 8, 2, 1, 1);
    let env = Env::<Par>::new();

    group.bench_function("realistic_5-5-3-8-2-1-1", |b| {
        b.iter(|| {
            let sub = Substitute { cost: _cost::new(Cost::unsafe_max(), 1000) };
            let result = sub.substitute_and_charge(
                black_box(par.clone()),
                black_box(0),
                black_box(&env),
            );
            black_box(result)
        });
    });

    let large_par = create_test_par(10, 10, 5, 15, 5, 2, 2);

    group.bench_function("realistic_10-10-5-15-5-2-2", |b| {
        b.iter(|| {
            let sub = Substitute { cost: _cost::new(Cost::unsafe_max(), 1000) };
            let result = sub.substitute_and_charge(
                black_box(large_par.clone()),
                black_box(0),
                black_box(&env),
            );
            black_box(result)
        });
    });

    group.finish();
}

fn bench_substitute_no_sort_and_charge(c: &mut Criterion) {
    let mut group = c.benchmark_group("substitute_no_sort_and_charge");
    group.sample_size(10);

    let test_cases = vec![
        (2, 2, 2, 2, 0, 0, 0, "2-2-2-2-0-0-0"),
        (3, 3, 2, 2, 1, 0, 0, "3-3-2-2-1-0-0"),
        (5, 5, 3, 8, 2, 1, 1, "5-5-3-8-2-1-1"),
    ];

    for (sends, receives, news, exprs, matches, unfs, bundles, name) in test_cases {
        let par = create_test_par(sends, receives, news, exprs, matches, unfs, bundles);
        let env = Env::<Par>::new();

        group.bench_with_input(BenchmarkId::from_parameter(name), &par, |b, par| {
            b.iter(|| {
                let sub = Substitute { cost: _cost::new(Cost::unsafe_max(), 1000) };
                let result = sub.substitute_no_sort_and_charge(
                    black_box(par.clone()),
                    black_box(0),
                    black_box(&env),
                );
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_send_substitution(c: &mut Criterion) {
    let mut group = c.benchmark_group("send_substitution");
    group.sample_size(10);

    let send = create_test_send();
    let env = Env::<Par>::new();

    group.bench_function("send_substitute_and_charge", |b| {
        b.iter(|| {
            let sub = Substitute { cost: _cost::new(Cost::unsafe_max(), 1000) };
            let result = sub.substitute_and_charge(
                black_box(send.clone()),
                black_box(0),
                black_box(&env),
            );
            black_box(result)
        });
    });

    group.finish();
}

fn bench_receive_substitution(c: &mut Criterion) {
    let mut group = c.benchmark_group("receive_substitution");
    group.sample_size(10);

    let receive = create_test_receive();
    let env = Env::<Par>::new();

    group.bench_function("receive_substitute_and_charge", |b| {
        b.iter(|| {
            let sub = Substitute { cost: _cost::new(Cost::unsafe_max(), 1000) };
            let result = sub.substitute_and_charge(
                black_box(receive.clone()),
                black_box(0),
                black_box(&env),
            );
            black_box(result)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_substitute_and_charge_small,
    bench_substitute_and_charge_medium,
    bench_substitute_and_charge_realistic,
    bench_substitute_no_sort_and_charge,
    bench_send_substitution,
    bench_receive_substitution
);
criterion_main!(benches);
