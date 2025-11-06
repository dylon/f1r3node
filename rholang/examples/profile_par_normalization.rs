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
) -> AnnProc<'ast> {
    let mut iter = 1..=size;
    let first = iter.next().unwrap();
    let first_proc = create_ast_long_literal(first, parser);

    iter.fold(first_proc, |acc, n| {
        let next_proc = create_ast_long_literal(n, parser);
        create_ast_par(acc, next_proc, parser)
    })
}

fn main() {
    let parser = rholang_parser::RholangParser::new();

    println!("Creating 10,000 element nested Par structure...");
    let huge_par = create_nested_par(10_000, &parser);

    println!("Normalizing...");
    for _ in 0..10 {
        let result = normalize_ann_proc(
            &huge_par,
            ProcVisitInputs::new(),
            &HashMap::new(),
            &parser,
        );
        assert!(result.is_ok());
    }

    println!("Done!");
}
