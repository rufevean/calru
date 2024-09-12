# calru
Calru is a compiler that parses and interprets a custom language designed to support basic arithmetic operations, variable declarations, and print statements. The language is a work-in-progress with ongoing improvements and features.

## Features

-   **Lexer**: Tokenizes input source code into meaningful symbols.
-   **Parser**: Constructs an Abstract Syntax Tree (AST) from tokens.
-   **Semantic Analysis**: Performs type checking and error detection.
-   **Intermediate Representation (IR)**: Generates assembly-like instructions from the AST.
-   **Interpreter**: Executes the instructions to provide output.
-   **REPL**: An interactive environment to test Calru code.
-   **Unit Tests**: Comprehensive tests for every phase of the compiler to ensure correctness.

## Grammar

The current grammar for the Calru language is as follows:
```
// Top-level statements
Statement   → LetDecl
            | PrintStmt

// Declaration of variables
LetDecl     → 'let' Identifier ':' Type AssignExpr ';'

// Expression assignment
AssignExpr  → ':=' Expression

// Print statement
PrintStmt   → 'stdout' '(' Expression ')'

// Expressions and terms
Expression  → Term (('+' | '-') Term)*

Term        → Factor (('*' | '/') Factor)*

// Factors and operands
Factor      → Number
            | Identifier
 ```

## Language Features

-   **Types**: Only floating-point numbers are supported. Integer types are considered as floats for simplicity.
-   **Operations**: Basic arithmetic operations (`+`, `-`, `*`, `/`).
-   **Statements**: Variable declarations with type, assignments, and print statements.
-   **Error Handling**: Comprehensive error reporting for syntax and semantic issues.

## Usage

### Running the Compiler

To run the Calru compiler:

1.  **Compile the Project**:
```
cargo build
```
2. **Run the Compiler**:
```
cargo run 
```
You can change the input file in `input/main.cru`
### Running Tests
Tests for each phase of the compiler are provided. To run the tests:
```
cargo test
```
## Example
```
let v1:int := 12;
let v2:float := 13.5;
let result:float := v1 + v2;
stdout(result);
```
### Explanation

-   `let v1:int := 12;` declares an integer variable `v1` with a value of `12`.
-   `let v2:float := 13.5;` declares a floating-point variable `v2` with a value of `13.5`.
-   `let result:float := v1 + v2;` declares a floating-point variable `result` and assigns it the sum of `v1` and `v2`.
-   `stdout(result);` prints the value of `result` to the standard output.


## Future Plans

-   **Add CLI Options**: Separate lexer, parser, and other components with command-line options.
-   **Implement Parentheses**: Support for parentheses in expressions.
-   **Add Control Flow**: Introduce `if` statements and boolean values.
-   **Expand Language Features**: Include trigonometric functions and other advanced math features.
			
## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you'd like to contribute to the development of Calru.


