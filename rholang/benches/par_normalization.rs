use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rholang::rust::interpreter::compiler::normalize::{normalize_ann_proc, ProcVisitInputs};
use rholang_parser::ast::AnnProc;
use rholang_parser::{SourcePos, SourceSpan};
use std::collections::HashMap;

fn create_ast_long_literal<'ast>(
    value: i64,
    parser: &'ast rholang_parser::RholangParser<'ast>,
) -> AnnProc<'ast> {
    AnnProc {
        proc: parser.ast_builder().alloc_long_literal(value),
        span: SourceSpan {
            start: SourcePos { line: 0, col: 0 },
            end: SourcePos { line: 0, col: 0 },
        },
    }
}

fn create_ast_par<'ast>(
    left: AnnProc<'ast>,
    right: AnnProc<'ast>,
    parser: &'ast rholang_parser::RholangParser<'ast>,
) -> AnnProc<'ast> {
    AnnProc {
        proc: parser.ast_builder().alloc_par(left, right),
        span: SourceSpan {
            start: SourcePos { line: 0, col: 0 },
            end: SourcePos { line: 0, col: 0 },
        },
    }
}

fn create_nested_par<'ast>(
    size: i64,
    parser: &'ast rholang_parser::RholangParser<'ast>,
) -> rholang_parser::ast::AnnProc<'ast> {
    let mut iter = 1..=size;
    let first = iter.next().unwrap();
    let first_proc = create_ast_long_literal(first, parser);

    iter.fold(first_proc, |acc, n| {
        let next_proc = create_ast_long_literal(n, parser);
        create_ast_par(acc, next_proc, parser)
    })
}

fn benchmark_par_normalization(c: &mut Criterion) {
    let mut group = c.benchmark_group("par_normalization");

    for size in [100, 1000, 10000, 50000].iter() {
        if *size >= 10000 {
            group.sample_size(10);
        }

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let parser = rholang_parser::RholangParser::new();
            let huge_par = create_nested_par(size, &parser);

            b.iter(|| {
                let result = normalize_ann_proc(
                    black_box(&huge_par),
                    black_box(ProcVisitInputs::new()),
                    black_box(&HashMap::new()),
                    black_box(&parser),
                );
                assert!(result.is_ok());
                result
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_par_normalization);
criterion_main!(benches);
