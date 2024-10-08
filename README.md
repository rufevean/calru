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
Top-level statements
Statement   → LetDecl
            | PrintStmt
            | IfStmt
            | PopStmt
            | PushStmt
            | LoopStmt
            | BreakStmt

Declaration of variables
LetDecl     → 'let' Identifier ':' Type AssignExpr ';'

Expression assignment
AssignExpr  → ':=' Expression

Print statement
PrintStmt   → 'stdout' '(' Expression ')'

If statement
IfStmt      → 'if' Condition 'then' Statement ('else' Statement)? 'end' ';'

Push statement
PushStmt    → Identifier '.' 'push' '(' Expression ')' ';'

Pop statement
PopStmt     → Identifier '.' 'pop' '(' ')' ';'

Loop statement
LoopStmt    → 'loop' Statement 'end' ';'

Break statement
BreakStmt   → 'break' ';'

Conditions
Condition   → Expression ('<' | '>' | '==' | '!=') Expression

Expressions and terms
Expression  → Term (('+' | '-') Term)*

Term        → Factor (('*' | '/') Factor)*

Factors and operands
Factor      → Number
            | Identifier
            | '(' Expression ')'
            | List
            | ListIndex

List        → '[' (Expression (',' Expression)*)? ']'


Fetch       → List '.' 'fetch' '(' Index ')'
 ```


## Latest Update
- Add Loop and break statement

## Next Steps

- Add len method for Lists.

## Language Features

-   **Types**: Only floating-point numbers are supported. Integer types are considered as floats for simplicity.
-   **Operations**: Basic arithmetic operations (`+`, `-`, `*`, `/`).
-   **Statements**: Variable declarations with type, assignments, and print statements.
-   **Control Flow**: Basic `if` statements with conditions.
-   **Error Handling**: Comprehensive error reporting for syntax and semantic issues.
-   **List Operations**: Push and pop operations for lists.
-   **Looping**: Basic loop functionality with a `loop` statement.

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
let i :int := 0;
loop{
    i := i + 1;
    if (i == 10) then
        break;
    end
}
stdout(i);
```
### Explanation:

-  The code declares four floating-point variables `num1`, `num2`, `num3`, and `num4`.
-  The product of these variables is calculated and stored in the `product` variable.
-  The product is printed to the console.
-  An `if` statement checks if the product is less than `10.0`.
-  If the condition is true, `10` is printed; otherwise, `1` is printed.



## Future Plans

-   **Add CLI Options**: Separate lexer, parser, and other components with command-line options.
-   **Expand Language Features**: Include trigonometric functions and other advanced math features.
-  **Add list indexing**: Support indexing into lists to access elements.
-  **Add Loops**: Implement loop functionality to support iteration. 
-   **perform more tests**: Add more tests to ensure the correctness of the compiler.
			
## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you'd like to contribute to the development of Calru.


