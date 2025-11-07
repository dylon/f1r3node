use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rholang::rust::interpreter::compiler::normalize::{normalize_ann_proc, ProcVisitInputs};
use rholang_parser::RholangParser;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use validated::Validated;

const CASPER_PROD_PATH: &str = "/var/tmp/debug/f1r3node/casper/src/main/resources";
const CASPER_TEST_PATH: &str = "/var/tmp/debug/f1r3node/casper/src/test/resources";

const PRODUCTION_ORDER: &[&str] = &[
    "Registry.rho",
    "ListOps.rho",
    "NonNegativeNumber.rho",
    "AuthKey.rho",
    "match_example.rho",
    "RegistryRealLifeTest.rho",
    "Either.rho",
    "MakeMint.rho",
    "RevVault.rho",
    "MultiSigRevVault.rho",
];

fn load_casper_production_contracts() -> Vec<(String, String)> {
    let mut contracts = Vec::new();

    for filename in PRODUCTION_ORDER {
        let path = format!("{}/{}", CASPER_PROD_PATH, filename);
        match fs::read_to_string(&path) {
            Ok(source) => {
                contracts.push((filename.to_string(), source));
            }
            Err(e) => {
                eprintln!("Warning: Failed to load {}: {}", filename, e);
            }
        }
    }

    contracts
}

fn get_valid_test_files() -> Vec<(String, String)> {
    let mut valid_files = Vec::new();

    if let Ok(entries) = fs::read_dir(CASPER_TEST_PATH) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "rho").unwrap_or(false) {
                if let Ok(source) = fs::read_to_string(&path) {
                    let parser = RholangParser::new();
                    let parsed = parser.parse(&source);

                    match parsed {
                        Validated::Good(procs) => {
                            let mut can_normalize = true;
                            for ann_proc in &procs {
                                if normalize_ann_proc(
                                    ann_proc,
                                    ProcVisitInputs::new(),
                                    &HashMap::new(),
                                    &parser,
                                )
                                .is_err()
                                {
                                    can_normalize = false;
                                    break;
                                }
                            }

                            if can_normalize {
                                if let Some(filename) = path.file_name() {
                                    valid_files.push((filename.to_string_lossy().to_string(), source));
                                }
                            }
                        }
                        Validated::Fail(_) => {
                        }
                    }
                }
            }
        }
    }

    valid_files.sort_by(|a, b| a.0.cmp(&b.0));
    valid_files
}

fn benchmark_casper_production_cascade(c: &mut Criterion) {
    let contracts = load_casper_production_contracts();

    if contracts.is_empty() {
        eprintln!("Warning: No Casper production contracts loaded");
        return;
    }

    c.bench_function("casper_production_cascade_total", |b| {
        b.iter(|| {
            let parser = RholangParser::new();

            for (filename, source) in &contracts {
                let parsed = parser.parse(source);

                match parsed {
                    Validated::Good(procs) => {
                        for ann_proc in &procs {
                            let normalized = normalize_ann_proc(
                                black_box(ann_proc),
                                black_box(ProcVisitInputs::new()),
                                black_box(&HashMap::new()),
                                black_box(&parser),
                            );

                            if let Ok(result) = normalized {
                                black_box(result);
                            } else {
                                eprintln!("Failed to normalize {} in cascade", filename);
                            }
                        }
                    }
                    Validated::Fail(_) => {
                        eprintln!("Failed to parse {} in cascade", filename);
                    }
                }
            }
        });
    });

    let mut group = c.benchmark_group("casper_production_per_contract");

    for (filename, source) in &contracts {
        group.bench_function(filename.as_str(), |b| {
            b.iter(|| {
                let parser = RholangParser::new();
                let parsed = parser.parse(source);

                match parsed {
                    Validated::Good(procs) => {
                        for ann_proc in &procs {
                            let normalized = normalize_ann_proc(
                                black_box(ann_proc),
                                black_box(ProcVisitInputs::new()),
                                black_box(&HashMap::new()),
                                black_box(&parser),
                            )
                            .expect("Failed to normalize");

                            black_box(normalized);
                        }
                    }
                    Validated::Fail(_) => panic!("File should have been validated: {}", filename),
                }
            });
        });
    }

    group.finish();
}

fn benchmark_casper_test_contracts(c: &mut Criterion) {
    let test_files = get_valid_test_files();

    if test_files.is_empty() {
        eprintln!("Warning: No valid Casper test contracts found");
        return;
    }

    let mut group = c.benchmark_group("casper_test_contracts");

    for (file_name, source) in &test_files {
        group.bench_function(file_name.as_str(), |b| {
            b.iter(|| {
                let parser = RholangParser::new();
                let parsed = parser.parse(source);

                match parsed {
                    Validated::Good(procs) => {
                        for ann_proc in &procs {
                            let normalized = normalize_ann_proc(
                                black_box(ann_proc),
                                black_box(ProcVisitInputs::new()),
                                black_box(&HashMap::new()),
                                black_box(&parser),
                            )
                            .expect("Failed to normalize");

                            black_box(normalized);
                        }
                    }
                    Validated::Fail(_) => panic!("File should have been pre-validated"),
                }
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_casper_production_cascade,
    benchmark_casper_test_contracts
);
criterion_main!(benches);
