use crate::{evaluator, lexer, parser};
use indoc::indoc;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

const CROW_IMAGE: &str = indoc! {
    "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⡟⠋⢻⣷⣄⡀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣾⣿⣷⣿⣿⣿⣿⣿⣶⣾⣿⣿⠿⠿⠿⠶⠄⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠉⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⣿⣿⣿⠟⠻⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⣿⣿⣿⣿⣆⣤⠿⢶⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠑⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠸⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠙⠛⠋⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
  "
};

fn run_help_interactive() {
    println!("\nWelcome to the Carrion Language Help System!");
    println!("Type 'topics' to see available help topics, or 'exit' to return to REPL.\n");

    let mut rl = DefaultEditor::new().expect("Failed to create help line editor");

    loop {
        let readline = rl.readline("help>>> ");
        match readline {
            Ok(line) => {
                let input = line.trim().to_lowercase();
                rl.add_history_entry(&line).ok();

                match input.as_str() {
                    "exit" | "quit" | "back" => {
                        println!("Returning to REPL...\n");
                        break;
                    }
                    "topics" | "help" | "" => {
                        print_help_topics();
                    }
                    "commands" | "1" => {
                        print_commands_help();
                    }
                    "syntax" | "2" => {
                        print_syntax_help();
                    }
                    "variables" | "3" => {
                        print_variables_help();
                    }
                    "functions" | "4" => {
                        print_functions_help();
                    }
                    "control" | "5" => {
                        print_control_flow_help();
                    }
                    "data" | "6" => {
                        print_data_structures_help();
                    }
                    "builtins" | "7" => {
                        print_builtins_help();
                    }
                    _ => {
                        println!(
                            "Unknown topic: '{}'. Type 'topics' to see available topics.",
                            input
                        );
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\nReturning to REPL...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("\nReturning to REPL...");
                break;
            }
            Err(err) => {
                eprintln!("Input error: {:?}", err);
                break;
            }
        }
    }
}

fn print_help_topics() {
    println!("\nAvailable help topics:");
    println!("  1. commands  - REPL commands");
    println!("  2. syntax    - Basic language syntax");
    println!("  3. variables - Variable declarations");
    println!("  4. functions - Function definitions and calls");
    println!("  5. control   - Control flow (if/else, loops)");
    println!("  6. data      - Data structures (arrays, hashes)");
    println!("  7. builtins  - Built-in functions");
    println!("\nType a topic name or number to learn more.\n");
}

fn print_commands_help() {
    println!("\n=== REPL Commands ===");
    println!("  help, scry  - Enter interactive help system");
    println!("  quit, exit  - Exit the REPL");
    println!("  Ctrl+C      - Interrupt current input");
    println!("  Ctrl+D      - Exit the REPL");
    println!("  Up/Down     - Navigate command history\n");
}

fn print_syntax_help() {
    println!("\n=== Basic Syntax ===");
    println!("  - Statements end with semicolons");
    println!("  - Comments start with // (single line)");
    println!("  - Blocks are enclosed in {{ }}");
    println!("  - Identifiers: letters, numbers, underscores");
    println!("  - Numbers: integers and floats supported\n");
}

fn print_variables_help() {
    println!("\n=== Variables ===");
    println!("  Declaration:");
    println!("    let x = 5;");
    println!("    let name = \"Odin\";");
    println!("    let is_raven = true;");
    println!("\n  Variables are immutable by default.");
    println!("  Reassignment creates a new binding.\n");
}

fn print_functions_help() {
    println!("\n=== Functions ===");
    println!("  Definition:");
    println!("    let add = fn(x, y) {{ x + y }};");
    println!("    let greet = fn(name) {{ \"Hello, \" + name }};");
    println!("\n  Calling:");
    println!("    add(3, 4);        // returns 7");
    println!("    greet(\"Thor\");    // returns \"Hello, Thor\"");
    println!("\n  Functions are first-class values.\n");
}

fn print_control_flow_help() {
    println!("\n=== Control Flow ===");
    println!("  If/Else:");
    println!("    if (x > 5) {{");
    println!("        \"greater\";");
    println!("    }} else {{");
    println!("        \"lesser\";");
    println!("    }}");
    println!("\n  Conditionals are expressions that return values.\n");
}

fn print_data_structures_help() {
    println!("\n=== Data Structures ===");
    println!("  Arrays:");
    println!("    let numbers = [1, 2, 3, 4, 5];");
    println!("    let mixed = [1, \"two\", true];");
    println!("\n  Hashes:");
    println!("    let person = {{\"name\": \"Loki\", \"age\": 1000}};");
    println!("    let config = {{\"debug\": true, \"port\": 8080}};");
    println!("\n  Access elements with indexing:\n    numbers[0], person[\"name\"]\n");
}

fn print_builtins_help() {
    println!("\n=== Built-in Functions ===");
    println!("  print(value, ...)  - Print values to stdout");
    println!("    Example: print(\"Hello\", \"World\");");
    println!("\n  More built-in functions coming soon!\n");
}

// ───── Interactive REPL ───────────────────────────────────────────────
pub fn run_repl() {
    println!("Welcome to The Carrion Language Repl!");
    println!("{CROW_IMAGE}");
    println!("Type type 'help' or 'scry' for help and 'quit' or 'exit' to leave.\n");

    // Create a new Rustyline Editor with history support
    let mut rl = DefaultEditor::new().expect("Failed to create line editor");

    // Optionally load history from a file
    let history_path = ".carrion_history";
    let _ = rl.load_history(history_path);

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                let input = line.trim();

                // Add to history
                rl.add_history_entry(&line).ok();

                if matches!(input, "quit" | "exit") {
                    println!("Farewell. May the All-Father bless your travels!");
                    break;
                }

                if matches!(input, "help" | "scry") {
                    run_help_interactive();
                    continue;
                }

                if input.is_empty() {
                    continue;
                }

                // --- The Full Pipeline ---
                let mut lexer = lexer::Lexer::new(input.to_owned(), "<stdin>".into());
                let tokens = lexer.scan_tokens();

                let mut parser = parser::Parser::new(tokens);
                let program = parser.parse_program();

                if !parser.errors().is_empty() {
                    eprintln!("Parsing Error(s):");
                    for err in parser.errors() {
                        eprintln!("\t{}", err);
                    }
                    continue; // Go to next loop iteration
                }

                match evaluator::eval(&program) {
                    Ok(evaluated) => println!("{}", evaluated),
                    Err(e) => eprintln!("Evaluation Error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Input error: {:?}", err);
                break;
            }
        }
    }

    // Save history on exit
    rl.save_history(history_path).ok();
}
