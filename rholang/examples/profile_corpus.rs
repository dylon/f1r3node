use rholang::rust::interpreter::compiler::compiler::Compiler;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let corpus_dir = "/home/dylon/Workspace/f1r3fly.io/rholang-rs/rholang-parser/tests/corpus/";

    println!("Profiling Rholang contracts from corpus...\n");

    let mut rho_files = Vec::new();
    find_rho_files(Path::new(corpus_dir), &mut rho_files);

    println!("Found {} Rholang files\n", rho_files.len());

    let mut total_time = 0u128;
    let mut successful = 0;
    let mut failed = 0;

    let files_to_profile: Vec<_> = rho_files.iter()
        .filter(|f| {
            ![
                "failing", "syntax_error", "invalid"
            ].iter().any(|skip| f.to_string_lossy().contains(skip))
        })
        .collect();

    println!("Profiling {} files (skipping known failures)...\n", files_to_profile.len());

    for path in &files_to_profile {
        let file_name = path.file_name().unwrap().to_string_lossy();

        match fs::read_to_string(path) {
            Ok(source) => {
                let start = Instant::now();
                match Compiler::source_to_adt(&source) {
                    Ok(_) => {
                        let elapsed = start.elapsed().as_micros();
                        total_time += elapsed;
                        successful += 1;

                        if elapsed > 1000 {
                            println!("✓ {} ({} µs)", file_name, elapsed);
                        }
                    }
                    Err(e) => {
                        failed += 1;
                        println!("✗ {} - Error: {:?}", file_name, e);
                    }
                }
            }
            Err(e) => {
                println!("✗ {} - Read error: {}", file_name, e);
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Total files: {}", files_to_profile.len());
    println!("Successful: {}", successful);
    println!("Failed: {}", failed);
    println!("Total time: {} ms ({} µs)", total_time / 1000, total_time);
    if successful > 0 {
        println!("Average time: {} µs per file", total_time / successful as u128);
    }
}

fn find_rho_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_rho_files(&path, files);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rho") {
                files.push(path);
            }
        }
    }
}
