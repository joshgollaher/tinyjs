This project is a toy interpreter for a subset of JavaScript. It is written in Rust, and supports most pre-ES6 features, including:
- Variables
- Functions
- Loops
- If/While/For statements
- Arrays
- Objects
- Intrinsics (Math, Object, etc.)
- Closures

Inside src are modules for various parts of the interpreter, including:
- Lexer: Takes source code and returns a list of tokens
- Parser: Takes a list of tokens and returns an Abstract Syntax Tree (AST)
- Optim: Performs various optimizations on the AST.
- Runtime: Implements the interpreter and supporting code.

Things I'm still implementing:
- Bytecode generator & VM
- Operator precedence (using shunting yard)
- Classes
- Better error handling (and storing the source span for each AST node)
- A REPL
