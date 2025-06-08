use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

mod ast;
mod error;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

fn main() {
    let mut args = env::args();
    args.next(); // Skip the program name

    if let Some(file_path) = args.next() {
        if args.next().is_some() {
            eprintln!("Error: Expected 0 or 1 arguments (path to file), but received more.");
            eprintln!("Usage: carrion [file_path]");
            process::exit(1);
        }
        let path = PathBuf::from(file_path);
        if let Err(e) = run_file(&path) {
            eprintln!("Error running file: {}", e);
            process::exit(1);
        }
    } else {
        println!("Welcome to the Carrion REPL!");
        repl::run_repl();
    }
}

fn run_file(file_path: &PathBuf) -> io::Result<()> {
    let source = fs::read_to_string(file_path)?;

    // 1. Lexing
    let mut lexer = lexer::Lexer::new(source, file_path.clone());
    let tokens = lexer.scan_tokens();

    // 2. Parsing
    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse_program();

    if !parser.errors().is_empty() {
        eprintln!("Encountered parsing errors:");
        for err in parser.errors() {
            eprintln!("\t{}", err);
        }
        return Ok(()); // Don't proceed to evaluation if parsing fails
    }

    // 3. Evaluation
    match evaluator::eval(&program) {
        Ok(evaluated) => {
            // Only print if the final result isn't 'None'
            if evaluated != object::Object::None {
                println!("{}", evaluated);
            }
        }
        Err(e) => {
            eprintln!("Evaluation Error: {}", e);
        }
    }

    Ok(())
}
