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

#[test]
fn test_assignment_statements() {
    // Test integer assignments
    let int_tests = vec![
        ("x = 5\nx", 5),
        ("y = 10\ny", 10),
        ("a = 5 * 5\na", 25),
        ("b = 10 + 5 * 2\nb", 20),
        ("x = 5\nx = 10\nx", 10),
        ("x = 5\nx = x + 5\nx", 10),
    ];

    for (input, expected) in int_tests {
        match run_eval(input) {
            Ok(Object::Integer(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected Integer, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }

    // Test string assignments
    let string_tests = vec![
        ("s = \"hello\"\ns", "hello"),
        ("msg = \"world\"\nmsg", "world"),
    ];

    for (input, expected) in string_tests {
        match run_eval(input) {
            Ok(Object::String(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected String, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }

    // Test boolean assignments
    let bool_tests = vec![
        ("t = True\nt", true),
        ("f = False\nf", false),
    ];

    for (input, expected) in bool_tests {
        match run_eval(input) {
            Ok(Object::Boolean(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected Boolean, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}

#[test]
fn test_compound_assignment_statements() {
    let tests = vec![
        ("x = 10\nx += 5\nx", 15),
        ("x = 20\nx -= 5\nx", 15),
        ("x = 5\nx *= 3\nx", 15),
        ("x = 20\nx /= 4\nx", 5),
    ];

    for (input, expected) in tests {
        match run_eval(input) {
            Ok(Object::Integer(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected Integer, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}

#[test]
fn test_multiple_assignment() {
    // Test multiple assignment with different values
    let tests = vec![
        ("x, y = 10, 20\nx", 10),
        ("x, y = 10, 20\ny", 20),
        ("a, b, c = 1, 2, 3\na", 1),
        ("a, b, c = 1, 2, 3\nb", 2),
        ("a, b, c = 1, 2, 3\nc", 3),
        ("x, y = 5 * 2, 3 + 4\nx", 10),
        ("x, y = 5 * 2, 3 + 4\ny", 7),
    ];

    for (input, expected) in tests {
        match run_eval(input) {
            Ok(Object::Integer(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected Integer, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }

    // Test multiple assignment with strings
    let string_tests = vec![
        ("s1, s2 = \"hello\", \"world\"\ns1", "hello"),
        ("s1, s2 = \"hello\", \"world\"\ns2", "world"),
    ];

    for (input, expected) in string_tests {
        match run_eval(input) {
            Ok(Object::String(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected String, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}

#[test]
fn test_list_expressions() {
    let tests = vec![
        ("[]", Vec::<Object>::new()),
        ("[1]", vec![Object::Integer(1)]),
        ("[1, 2, 3]", vec![Object::Integer(1), Object::Integer(2), Object::Integer(3)]),
        ("[1, \"hello\", True]", vec![Object::Integer(1), Object::String("hello".to_string()), Object::Boolean(true)]),
    ];

    for (input, expected) in tests {
        match run_eval(input) {
            Ok(Object::List(elements)) => {
                assert_eq!(elements, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected List, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}

#[test]
fn test_list_indexing() {
    let tests = vec![
        ("[1, 2, 3][0]", 1),
        ("[1, 2, 3][1]", 2),
        ("[1, 2, 3][2]", 3),
        ("[10, 20, 30][1]", 20),
    ];

    for (input, expected) in tests {
        match run_eval(input) {
            Ok(Object::Integer(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected Integer, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }

    // Test string elements
    let string_tests = vec![
        ("[\"hello\", \"world\"][0]", "hello"),
        ("[\"hello\", \"world\"][1]", "world"),
    ];

    for (input, expected) in string_tests {
        match run_eval(input) {
            Ok(Object::String(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected String, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}

#[test]
fn test_dictionary_expressions() {
    let tests = vec![
        ("{}", std::collections::HashMap::<String, Object>::new()),
        ("{\"name\": \"John\"}", {
            let mut map = std::collections::HashMap::new();
            map.insert("name".to_string(), Object::String("John".to_string()));
            map
        }),
        ("{\"age\": 30, \"name\": \"John\"}", {
            let mut map = std::collections::HashMap::new();
            map.insert("age".to_string(), Object::Integer(30));
            map.insert("name".to_string(), Object::String("John".to_string()));
            map
        }),
    ];

    for (input, expected) in tests {
        match run_eval(input) {
            Ok(Object::Dict(dict)) => {
                assert_eq!(dict, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected Dict, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}

#[test]
fn test_dictionary_indexing() {
    let tests = vec![
        ("{\"name\": \"John\"}[\"name\"]", "John"),
        ("{\"city\": \"New York\", \"country\": \"USA\"}[\"city\"]", "New York"),
        ("{\"city\": \"New York\", \"country\": \"USA\"}[\"country\"]", "USA"),
    ];

    for (input, expected) in tests {
        match run_eval(input) {
            Ok(Object::String(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected String, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }

    // Test integer values
    let int_tests = vec![
        ("{\"age\": 30}[\"age\"]", 30),
        ("{\"score\": 95, \"rank\": 1}[\"score\"]", 95),
        ("{\"score\": 95, \"rank\": 1}[\"rank\"]", 1),
    ];

    for (input, expected) in int_tests {
        match run_eval(input) {
            Ok(Object::Integer(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected Integer, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}

#[test]
fn test_builtin_functions() {
    // Test len function
    let len_tests = vec![
        ("len([1, 2, 3])", 3),
        ("len([])", 0),
        ("len([\"hello\", \"world\"])", 2),
        ("len({\"a\": 1, \"b\": 2})", 2),
        ("len({})", 0),
        ("len(\"hello\")", 5),
    ];

    for (input, expected) in len_tests {
        match run_eval(input) {
            Ok(Object::Integer(val)) => {
                assert_eq!(val, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected Integer, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }

    // Test push function
    let push_tests = vec![
        ("push([1, 2], 3)", vec![Object::Integer(1), Object::Integer(2), Object::Integer(3)]),
        ("push([], 42)", vec![Object::Integer(42)]),
        ("push([\"hello\"], \"world\")", vec![Object::String("hello".to_string()), Object::String("world".to_string())]),
    ];

    for (input, expected) in push_tests {
        match run_eval(input) {
            Ok(Object::List(elements)) => {
                assert_eq!(elements, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected List, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }

    // Test pop function
    let pop_tests = vec![
        ("pop([1, 2, 3])", vec![Object::Integer(1), Object::Integer(2)]),
        ("pop([42])", Vec::<Object>::new()),
        ("pop([\"hello\", \"world\"])", vec![Object::String("hello".to_string())]),
    ];

    for (input, expected) in pop_tests {
        match run_eval(input) {
            Ok(Object::List(elements)) => {
                assert_eq!(elements, expected, "Failed for input: {}", input);
            }
            Ok(other) => panic!("Expected List, got {:?} for input '{}'", other, input),
            Err(e) => panic!("Evaluation failed for input '{}': {}", input, e),
        }
    }
}
