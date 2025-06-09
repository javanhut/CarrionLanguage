# CarrionLanguage

Carrion Language is a dynamic, interpreted programming language written in Rust. It features Python-like indentation syntax with modern features and is built with a clean, pipelined architecture consisting of a lexer, a Pratt parser for handling complex operator precedence, and a tree-walking evaluator.

This project serves as a practical implementation of a full-featured interpreter, from source code scanning to runtime evaluation.

Based on [TheCarrionLanguage](https://github.com/javanhut/TheCarrionLanguage) built in Go.

## Features (Current Implementation)

The Carrion interpreter currently supports the following features:

- **Data Types**:
  - `Integer` (64-bit signed)
  - `Float` (64-bit double precision)
  - `Boolean` (`True`, `False`)
  - `String` (UTF-8 strings with `"` or `'` quotes)
  - `List` (dynamic arrays with mixed types)
  - `Dict` (hash maps with any hashable key)

- **Operators**:
  - Arithmetic: `+`, `-`, `*`, `/`, `%`, `**` (exponent)
  - Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
  - Logical: `and`, `or`, `not`
  - Assignment: `=`, `+=`, `-=`, `*=`, `/=`
  - Prefix/Postfix: `++`, `--`

- **Variables & Assignment**:
  - Simple assignment: `x = 42`
  - Multiple assignment: `a, b, c = 1, 2, 3`
  - Compound assignment: `count += 1`

- **Control Flow** (Production-Ready):
  - **If/Otherwise/Else statements** with indentation-based blocks
  - Multiple `otherwise` clauses supported (Python's `elif` equivalent)
  - Nested conditionals with proper scope handling
  - Safety limits to prevent infinite loops and stack overflow

- **Data Structures**:
  - **Lists**: `[1, 2, "hello", True]` with indexing `list[0]`
  - **Dictionaries**: `{"name": "Alice", "age": 30}` with key access `dict["name"]`

- **Built-in Functions**:
  - `print()` - output values to console
  - `len()` - get length of lists/dicts/strings
  - `type()` - get type information

- **Interactive Features**:
  - **REPL** with command history and help system
  - **File execution** support
  - **Comprehensive help system** with interactive topics

## Getting Started

The project is built with Cargo, the Rust package manager.

### Running the REPL

To start the interactive Read-Eval-Print Loop (REPL), run the project without any arguments:

```sh
cargo run
```

This will drop you into the Carrion REPL, where you can enter expressions for immediate evaluation.

```
Welcome to The Carrion Language Repl!
Type type 'help' or 'scry' for help and 'quit' or 'exit' to leave.

>>> 5 * (2 + 10)
60
>>> True == (5 < 10)
True
>>> print("Hello, Carrion!")
Hello, Carrion!
```

### Running a File

You can execute a Carrion source file (conventionally with a `.crl` extension) by passing the file path as an argument:

```sh
cargo run example.crl
```

## Language Syntax Examples

### Basic Data Types and Variables

```carrion
# Numbers
age = 25
pi = 3.14159
score = 95.5

# Strings
name = "Alice"
message = 'Hello, World!'

# Booleans
is_active = True
is_complete = False

# Lists
numbers = [1, 2, 3, 4, 5]
mixed = [42, "hello", True, 3.14]

# Dictionaries
person = {"name": "Bob", "age": 30, "city": "New York"}
scores = {"math": 95, "science": 87, "english": 92}
```

### Control Flow

```carrion
# If/Otherwise/Else statements
score = 85
if score >= 90:
    print("A grade")
otherwise score >= 80:
    print("B grade")
otherwise score >= 70:
    print("C grade")
otherwise score >= 60:
    print("D grade")
else:
    print("F grade")

# Nested conditionals
weather = "sunny"
temperature = 75

if weather == "sunny":
    if temperature > 70:
        print("Perfect beach day!")
    else:
        print("Sunny but chilly")
else:
    print("Not sunny today")
```

### Data Structure Operations

```carrion
# List operations
fruits = ["apple", "banana", "cherry"]
print(fruits[0])        # apple
print(len(fruits))      # 3

# Dictionary operations
student = {"name": "Charlie", "grade": 85}
print(student["name"])  # Charlie
print(student["grade"]) # 85

# Mixed data structures
data = [
    {"name": "Alice", "scores": [95, 87, 92]},
    {"name": "Bob", "scores": [78, 84, 90]}
]
print(data[0]["name"])          # Alice
print(data[1]["scores"][0])     # 78
```

### Assignment Operations

```carrion
# Basic assignment
x = 10
y = x

# Multiple assignment
a, b, c = 1, 2, 3
coordinates = [10, 20]
x, y = coordinates

# Compound assignment
counter = 0
counter += 1    # counter is now 1
counter *= 2    # counter is now 2
counter -= 1    # counter is now 1
```

### Running Tests

The project includes a suite of integration tests to verify the correctness of the evaluator. To run them:

```sh
cargo test
```

## Language Keywords

The following keywords are currently implemented in Carrion:

| Keyword      | Purpose                   | Status            |
| ------------ | ------------------------- | ----------------- |
| `if`         | Conditional logic         | **✅ Implemented** |
| `otherwise`  | Else-if clause            | **✅ Implemented** |
| `else`       | Final else clause         | **✅ Implemented** |
| `True`       | Boolean literal           | **✅ Implemented** |
| `False`      | Boolean literal           | **✅ Implemented** |
| `and`        | Logical AND operator      | **✅ Implemented** |
| `or`         | Logical OR operator       | **✅ Implemented** |
| `not`        | Logical NOT operator      | **✅ Implemented** |
| `return`     | Return from function      | **✅ Implemented** |

### Planned Keywords

| Keyword     | Purpose (Planned)       | Status          |
| ----------- | ----------------------- | --------------- |
| `spell`     | Function definition     | 🔄 Planned      |
| `grim`      | Class/Struct definition | 🔄 Planned      |
| `for`       | For-in loop             | 🔄 Planned      |
| `while`     | While loop              | 🔄 Planned      |
| `in`        | Membership test         | 🔄 Planned      |
| `none`      | None/Null value         | 🔄 Planned      |

## Production Features

### Safety & Performance
- **Memory safety**: Rust's ownership system prevents memory leaks and crashes
- **Error handling**: Comprehensive error messages with line numbers and context
- **Loop detection**: Automatic detection and prevention of infinite parsing loops
- **Production limits**: Configurable limits on nesting depth and complexity
- **Robust recovery**: Parser continues after errors for better development experience

### Development Experience
- **Interactive REPL**: Full-featured REPL with history and help system
- **Comprehensive help**: Built-in documentation with examples and syntax guides
- **Clear error messages**: Detailed error reporting for debugging
- **File execution**: Direct execution of `.crl` source files
- **Test suite**: Comprehensive test coverage for reliability
