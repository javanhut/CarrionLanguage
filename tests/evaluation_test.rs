use the_carrion_language::{evaluator, lexer, object::Object, parser};

fn run_eval(input: &str) -> Result<Object, String> {
    let mut lexer = lexer::Lexer::new(input.to_owned(), "<test>".into());
    let tokens = lexer.scan_tokens();
    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse_program();

    if !parser.errors().is_empty() {
        return Err(format!("Parser errors: {:?}", parser.errors()));
    }

    evaluator::eval(&program)
}

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        ("5", 5, "Expected 5, but the parser failed"), // Note: This will fail until assignment is parsed
        ("10", 10, "Expected 10, but the parser failed"),
        ("-5", -5, "Expected -5, but the parser failed"),
        ("-10", -10, "Expected -10, but the parser failed"),
        (
            "5 + 5 + 5 + 5 - 10",
            10,
            "Failed to evaluate addition/subtraction",
        ),
        ("2 * 2 * 2 * 2 * 2", 32, "Failed to evaluate multiplication"),
        (
            "-50 + 100 + -50",
            0,
            "Failed to evaluate with negative numbers",
        ),
        ("5 * 2 + 10", 20, "Failed to respect operator precedence"),
        ("5 + 2 * 10", 25, "Failed to respect operator precedence"),
        (
            "20 + 2 * -10",
            0,
            "Failed to handle negative numbers in precedence",
        ),
        ("50 / 2 * 2 + 10", 60, "Failed with division"),
        ("2 * (5 + 10)", 30, "Failed to handle parentheses"),
        (
            "(5 + 10 * 2 + 15 / 3) * 2 + -10",
            50,
            "Failed a complex expression",
        ),
    ];

    for (input, expected_val, msg) in tests {
        match run_eval(input) {
            Ok(Object::Integer(actual)) => {
                assert_eq!(actual, expected_val, "Input: '{}'. {}", input, msg);
            }
            Ok(other) => panic!("Expected Integer, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}

#[test]
fn test_boolean_expressions() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
    ];

    for (input, expected_val) in tests {
        match run_eval(input) {
            Ok(Object::Boolean(actual)) => {
                assert_eq!(actual, expected_val, "Input: '{}'", input);
            }
            Ok(other) => panic!("Expected Boolean, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", e, input),
        }
    }
}
