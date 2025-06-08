use std::env;
use std::path::PathBuf;
use std::process;

mod ast;
mod error;
mod evaluator;
mod lexer;
mod object;
mod repl;
mod token;
fn main() {
    let mut args = env::args();
    args.next();

    let carrion_file = args.next();

    if args.next().is_some() {
        eprintln!("Error: Expected 0 or 1 arguments (path to file), but recieved_more.");
        eprintln!("Run Carrion to run interactive repl or Carrion [file_path]");
        process::exit(1);
    }

    match carrion_file {
        Some(file_path) => {
            let path = PathBuf::from(file_path);
            let _ = repl::read_and_tokenize_file(&path);
        }
        None => {
            repl::run_repl();
        }
    }
}
