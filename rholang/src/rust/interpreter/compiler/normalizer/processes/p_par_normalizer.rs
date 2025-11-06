use crate::rust::interpreter::compiler::exports::{ProcVisitInputs, ProcVisitOutputs};
use crate::rust::interpreter::errors::InterpreterError;
use models::rhoapi::Par;
use std::collections::HashMap;

use crate::rust::interpreter::compiler::normalize::normalize_ann_proc;
use rholang_parser::ast::{AnnProc, Proc};

fn flatten_par<'ast>(root: &'ast AnnProc<'ast>) -> Vec<&'ast AnnProc<'ast>> {
    let mut result = Vec::new();
    let mut stack = vec![root];

    while let Some(current) = stack.pop() {
        match &current.proc {
            Proc::Par { left, right } => {
                stack.push(right);
                stack.push(left);
            }
            _ => result.push(current),
        }
    }
    result
}

pub fn normalize_p_par<'ast>(
    left: &'ast AnnProc<'ast>,
    right: &'ast AnnProc<'ast>,
    input: ProcVisitInputs,
    env: &HashMap<String, Par>,
    parser: &'ast rholang_parser::RholangParser<'ast>,
) -> Result<ProcVisitOutputs, InterpreterError> {
    use models::rust::utils::union;

    let flattened_left = flatten_par(left);
    let flattened_right = flatten_par(right);

    let mut all_procs = Vec::with_capacity(flattened_left.len() + flattened_right.len());
    all_procs.extend(flattened_left);
    all_procs.extend(flattened_right);

    let mut accumulated_exprs = Vec::new();
    let mut accumulated_sends = Vec::new();
    let mut accumulated_receives = Vec::new();
    let mut accumulated_news = Vec::new();
    let mut accumulated_matches = Vec::new();
    let mut accumulated_unforgeables = Vec::new();
    let mut accumulated_bundles = Vec::new();
    let mut accumulated_connectives = Vec::new();
    let mut accumulated_locally_free = input.par.locally_free.clone();
    let mut accumulated_connective_used = input.par.connective_used;

    let mut accumulated_free_map = input.free_map;
    let bound_map_chain = input.bound_map_chain;

    for proc in all_procs {
        let proc_input = ProcVisitInputs {
            par: Par::default(),
            free_map: accumulated_free_map,
            bound_map_chain: bound_map_chain.clone(),
        };

        let proc_result = normalize_ann_proc(proc, proc_input, env, parser)?;

        accumulated_exprs.extend(proc_result.par.exprs);
        accumulated_sends.extend(proc_result.par.sends);
        accumulated_receives.extend(proc_result.par.receives);
        accumulated_news.extend(proc_result.par.news);
        accumulated_matches.extend(proc_result.par.matches);
        accumulated_unforgeables.extend(proc_result.par.unforgeables);
        accumulated_bundles.extend(proc_result.par.bundles);
        accumulated_connectives.extend(proc_result.par.connectives);
        accumulated_locally_free = union(accumulated_locally_free, proc_result.par.locally_free);
        accumulated_connective_used = accumulated_connective_used || proc_result.par.connective_used;

        accumulated_free_map = proc_result.free_map;
    }

    accumulated_exprs.reverse();
    accumulated_sends.reverse();
    accumulated_receives.reverse();
    accumulated_news.reverse();
    accumulated_matches.reverse();
    accumulated_unforgeables.reverse();
    accumulated_bundles.reverse();
    accumulated_connectives.reverse();

    let final_par = Par {
        exprs: accumulated_exprs,
        sends: accumulated_sends,
        receives: accumulated_receives,
        news: accumulated_news,
        matches: accumulated_matches,
        unforgeables: accumulated_unforgeables,
        bundles: accumulated_bundles,
        connectives: accumulated_connectives,
        locally_free: accumulated_locally_free,
        connective_used: accumulated_connective_used,
    };

    Ok(ProcVisitOutputs {
        par: final_par,
        free_map: accumulated_free_map,
    })
}

// See rholang/src/test/scala/coop/rchain/rholang/interpreter/compiler/normalizer/ProcMatcherSpec.scala
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use models::{
        create_bit_vector,
        rhoapi::Par,
        rust::utils::{new_boundvar_expr, new_freevar_expr, new_gint_expr},
    };

    use crate::rust::interpreter::{
        compiler::{exports::ProcVisitInputs, normalize::VarSort},
        errors::InterpreterError,
        test_utils::utils::proc_visit_inputs_and_env,
    };

    use crate::rust::interpreter::compiler::normalize::normalize_ann_proc;
    use crate::rust::interpreter::test_utils::par_builder_util::ParBuilderUtil;
    use rholang_parser::ast::{Id, Var};
    use rholang_parser::SourcePos;

    #[test]
    fn p_par_should_compile_both_branches_into_a_par_object() {
        let parser = rholang_parser::RholangParser::new();

        let left_proc = ParBuilderUtil::create_ast_long_literal(7, &parser);
        let right_proc = ParBuilderUtil::create_ast_long_literal(8, &parser);
        let par_proc = ParBuilderUtil::create_ast_par(left_proc, right_proc, &parser);

        let result =
            normalize_ann_proc(&par_proc, ProcVisitInputs::new(), &HashMap::new(), &parser);

        assert!(result.is_ok());
        assert_eq!(
            result.clone().unwrap().par,
            Par::default().with_exprs(vec![new_gint_expr(8), new_gint_expr(7)])
        );
        assert_eq!(result.unwrap().free_map, ProcVisitInputs::new().free_map);
    }

    #[test]
    fn p_par_should_compile_both_branches_with_the_same_environment() {
        use std::rc::Rc;

        let parser = rholang_parser::RholangParser::new();

        let left_proc = ParBuilderUtil::create_ast_proc_var_from_var(
            Var::Id(Id {
                name: "x",
                pos: SourcePos { line: 0, col: 0 },
            }),
            &parser,
        );
        let right_proc = ParBuilderUtil::create_ast_proc_var_from_var(
            Var::Id(Id {
                name: "x",
                pos: SourcePos { line: 0, col: 0 },
            }),
            &parser,
        );
        let par_proc = ParBuilderUtil::create_ast_par(left_proc, right_proc, &parser);

        let (mut inputs, env) = proc_visit_inputs_and_env();
        inputs.bound_map_chain = Rc::new(inputs.bound_map_chain.put_pos((
            "x".to_string(),
            VarSort::ProcSort,
            SourcePos { line: 0, col: 0 },
        )));

        let result = normalize_ann_proc(&par_proc, inputs, &env, &parser);

        assert!(result.is_ok());
        assert_eq!(result.clone().unwrap().par, {
            let mut par =
                Par::default().with_exprs(vec![new_boundvar_expr(0), new_boundvar_expr(0)]);
            par.locally_free = create_bit_vector(&vec![0]);
            par
        });
        assert_eq!(result.unwrap().free_map, ProcVisitInputs::new().free_map);
    }

    #[test]
    fn p_par_should_not_compile_if_both_branches_use_the_same_free_variable() {
        let parser = rholang_parser::RholangParser::new();

        let left_proc = ParBuilderUtil::create_ast_proc_var_from_var(
            Var::Id(Id {
                name: "x",
                pos: SourcePos { line: 0, col: 0 },
            }),
            &parser,
        );
        let right_proc = ParBuilderUtil::create_ast_proc_var_from_var(
            Var::Id(Id {
                name: "x",
                pos: SourcePos { line: 0, col: 0 },
            }),
            &parser,
        );
        let par_proc = ParBuilderUtil::create_ast_par(left_proc, right_proc, &parser);

        let result =
            normalize_ann_proc(&par_proc, ProcVisitInputs::new(), &HashMap::new(), &parser);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(InterpreterError::UnexpectedReuseOfProcContextFree {
                var_name,
                first_use: _,
                second_use: _
            }) if var_name == "x"
        ));
    }

    #[test]
    fn p_par_should_accumulate_free_counts_from_both_branches() {
        let parser = rholang_parser::RholangParser::new();

        let left_proc = ParBuilderUtil::create_ast_proc_var_from_var(
            Var::Id(Id {
                name: "x",
                pos: SourcePos { line: 0, col: 0 },
            }),
            &parser,
        );
        let right_proc = ParBuilderUtil::create_ast_proc_var_from_var(
            Var::Id(Id {
                name: "y",
                pos: SourcePos { line: 0, col: 0 },
            }),
            &parser,
        );
        let par_proc = ParBuilderUtil::create_ast_par(left_proc, right_proc, &parser);

        let result =
            normalize_ann_proc(&par_proc, ProcVisitInputs::new(), &HashMap::new(), &parser);

        assert!(result.is_ok());
        assert_eq!(result.clone().unwrap().par, {
            let mut par = Par::default().with_exprs(vec![new_freevar_expr(1), new_freevar_expr(0)]);
            par.connective_used = true;
            par
        });
        assert_eq!(
            result.unwrap().free_map,
            ProcVisitInputs::new().free_map.put_all_pos(vec![
                (
                    "x".to_owned(),
                    VarSort::ProcSort,
                    SourcePos { line: 0, col: 0 }
                ),
                (
                    "y".to_owned(),
                    VarSort::ProcSort,
                    SourcePos { line: 0, col: 0 }
                )
            ])
        )
    }

    #[test]
    fn p_par_should_normalize_without_stack_overflow_error_even_for_huge_program() {
        let parser = rholang_parser::RholangParser::new();

        // Create a huge nested Par with integers 1..50000 using new AST
        fn create_huge_par<'ast>(
            range: std::ops::RangeInclusive<i64>,
            parser: &'ast rholang_parser::RholangParser<'ast>,
        ) -> rholang_parser::ast::AnnProc<'ast> {
            let mut iter = range.into_iter();
            let first = iter.next().unwrap();
            let first_proc = ParBuilderUtil::create_ast_long_literal(first, parser);

            iter.fold(first_proc, |acc, n| {
                let next_proc = ParBuilderUtil::create_ast_long_literal(n, parser);
                ParBuilderUtil::create_ast_par(acc, next_proc, parser)
            })
        }

        let huge_par = create_huge_par(1..=50000, &parser);

        let result =
            normalize_ann_proc(&huge_par, ProcVisitInputs::new(), &HashMap::new(), &parser);
        assert!(result.is_ok());
    }
}
