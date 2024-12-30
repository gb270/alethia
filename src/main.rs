use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn print_usage() {
    eprintln!("Usage: alethia <command> [args]");
    eprintln!("Commands:");
    eprintln!("   run <file>    Run an .at file");
    eprintln!("   repl          Start an interactive REPL session");
}

fn run_file(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    if path.extension().and_then(|s| s.to_str()) != Some("at") {
        return Err("File must have .at extension".to_string());
    }

    match fs::read_to_string(path) {
        Ok(contents) => alethia::run_source(contents),
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
}

fn run_repl() -> Result<(), String> {
    use std::io::{self, Write};

    println!("alethia REPL (exit with Ctrl+C");
    let mut interpreter = alethia::Interpreter::new();

    loop {
        print!("> ");
        if let Err(e) = io::stdout().flush(){
            return Err(format!("Error flushing stdout: {}", e));
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().is_empty() {
                    continue;
                }
                if let Err(e) = alethia::run_line(input, &mut interpreter) {
                eprintln!("Error: {}", e);
                }
        }
        Err(e) => return Err(format!("Error reading input: {}", e)),
        
    }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let result = match args[1].as_str() {
        "run" => {
            if args.len() != 3 {
                print_usage();
                process::exit(1);
            }
            run_file(Path::new(&args[2]))
        }
        "repl" => run_repl(),
        _ => {
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
