# CarrionLanguage

Carrion Language is a dynamic, interpreted programming language written in Rust. It features a C-like syntax with modern features and is built with a clean, pipelined architecture consisting of a lexer, a Pratt parser for handling complex operator precedence, and a tree-walking evaluator.

This project serves as a practical implementation of a full-featured interpreter, from source code scanning to runtime evaluation.

Based on [TheCarrionLanguage](https://github.com/javanhut/TheCarrionLanguage) built in Go.

## Features (Current Implementation)

The Carrion interpreter currently supports the following features:

- **Data Types**:
  - `Integer` (64-bit)
  - `Float` (64-bit)
  - `Boolean` (`true`, `false`)
  - `String` (e.g., `"hello"`)
- **Operators**:
  - Arithmetic: `+`, `-`, `*`, `/`
  - Comparison: `==`, `!=`, `<`, `>`
  - Logical: `not` (as a prefix operator)
  - Prefix Operators: `-` (negation), `not` (logical not)
- **Expressions**:
  - The evaluator correctly handles **operator precedence** (e.g., `*` before `+`).
  - **Grouped expressions** using parentheses `()` are supported.
- **Statements**:
  - **Expression Statements**: Any expression can be used as a statement.
  - **Return Statements**: `return <expression>` is parsed and handled by the evaluator.

## Getting Started

The project is built with Cargo, the Rust package manager.

### Running the REPL

To start the interactive Read-Eval-Print Loop (REPL), run the project without any arguments:

```sh
cargo run
```

This will drop you into the Carrion REPL, where you can enter expressions for immediate evaluation.

```
Welcome to the Carrion REPL!
Type 'quit' or 'exit' to leave.

>>> 5 * (2 + 10)
60
>>> true == (5 < 10)
true
```

### Running a File

You can execute a Carrion source file (conventionally with a `.crl` extension) by passing the file path as an argument:

```sh
# Given a file example.crl with content "50 / 2 * 2 + 10"
cargo run -- example.crl
```

The final result of the program will be printed to the console.

### Running Tests

The project includes a suite of integration tests to verify the correctness of the evaluator. To run them:

```sh
cargo test
```

## Language Keywords

The following keywords are defined in the lexer, with plans for future implementation:

| Keyword     | Purpose (Planned)       | Status          |
| ----------- | ----------------------- | --------------- |
| `spell`     | Function definition     | Not Implemented |
| `grim`      | Class/Struct definition | Not Implemented |
| `if`        | Conditional logic       | Not Implemented |
| `else`      | Conditional logic       | Not Implemented |
| `for`       | For-in loop             | Not Implemented |
| `while`     | While loop              | Not Implemented |
| `return`    | Return from function    | **Implemented** |
| `true`      | Boolean literal         | **Implemented** |
| `false`     | Boolean literal         | **Implemented** |
| `none`      | None/Null value         | Not Implemented |
| `and`, `or` | Logical operators       | Not Implemented |
| `not`       | Logical prefix operator | **Implemented** |
