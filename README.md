## Expression Solver

Solves a mathematical expression while following precedence and associativity.

The crate provides one public api function.

```rs
fn resolve(input_string: String) -> Result<f64, String>
```

This takes mathematical expressions as String, and returns a Result enum with solved value or incase of an error, error string.

### Examples

```rs
use expr_solver::resolve;

// simple binary expression.
resolve("2+2".to_string()); // Ok(2.0)

// follows precendence, 2 + (2 _ 2) and NOT (2 + 2) _ 2
resolve("2+2*2".to_string()); // Ok(6.0);

// unary expression.
resolve("-2".to_string()); // Ok(-2.0)

// even chain them. -(-2)
resolve("--2".to_string()); // Ok(2.0)

// binary and unary in one expression.
resolve("2+-2".to_string()); // Ok(0.0)

// gives syntax error.
resolve("2)2".to_string()); // Err(String);
```

### Inner workings

There are three steps involved

#### 1. Lexical Analysis.

Breaks the input string into indiviual tokens.

#### 2. Parser

This uses a Pratt Parsing technique to parse the stream of tokens into Abstract Syntax Tree (AST).

#### 3. Interpreting

Uses a 'Tree-Walk' interpreter to evalute the AST.
