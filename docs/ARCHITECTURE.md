# Shard Compiler Architecture

## Overview

Shard is a low-level programming language with a compiler written in Rust. The compiler follows a traditional multi-stage pipeline:

```
Source Code → Lexer → Parser → AST → Type Checker → IR → Code Generator → Executable
```

## Components

### 1. Lexer (`src/lexer.rs`)

Tokenizes the source code into a stream of tokens.

**Input:** Source code as a string  
**Output:** Vec<Token>

Key features:
- Handles all Shard keywords
- Supports string interpolation
- Line/column tracking for error messages
- Unicode support

### 2. Parser (`src/parser/mod.rs`)

Parses tokens into an Abstract Syntax Tree (AST).

**Input:** Vec<Token>  
**Output:** CompilationUnit (AST)

Key features:
- Recursive descent parsing
- Operator precedence handling
- Error recovery
- Full grammar support

### 3. AST (`src/ast.rs`)

Represents the program structure in memory.

Key node types:
- `FunctionDef` - Function declarations
- `VariableDef` - Variable bindings
- `StructDef` - Structure definitions
- `EnumDef` - Enumerations
- `Expression` - All expression types
- `Statement` - All statement types

### 4. Type Checker (`src/typechecker/mod.rs`)

Validates types and performs type inference.

**Input:** AST  
**Output:** Result<(), TypeCheckError>

Key features:
- Static type checking
- Type inference
- Generic type support
- Error reporting with locations

### 5. IR (`src/ir/mod.rs`)

Generates intermediate representation.

**Input:** AST  
**Output:** Module (IR)

Key features:
- SSA-like form
- Basic blocks
- Control flow graph
- Platform-independent

### 6. Code Generator (`src/codegen/mod.rs`)

Generates C code and compiles to native executable.

**Input:** IR Module  
**Output:** Native executable

Key features:
- C code generation
- Cross-platform compilation
- Optimization levels (0-3)
- Automatic cleanup

## Data Flow

```
main.rs
  ├── lexer::lex(source) → Vec<Token>
  ├── parser::parse(tokens) → CompilationUnit
  ├── typechecker::check(ast) → Result<(), Error>
  ├── ir::generate(ast) → Module
  └── codegen::compile(ir, output) → Executable
```

## File Organization

```
Shard/
├── src/
│   ├── main.rs          # CLI and compilation pipeline
│   ├── lexer.rs         # Tokenization
│   ├── ast.rs           # AST definitions
│   ├── parser/
│   │   └── mod.rs       # Parser implementation
│   ├── typechecker/
│   │   └── mod.rs       # Type checking
│   ├── ir/
│   │   └── mod.rs       # IR generation
│   └── codegen/
│       └── mod.rs       # Code generation
├── stdlib/              # Standard library
│   ├── core/
│   ├── io/
│   ├── math/
│   └── os/
└── examples/            # Example programs
```

## Compilation Pipeline

### Stage 1: Lexing
```
"fn main() { print(\"Hello\"); }"
  ↓
[Fn, Ident("main"), LParen, RParen, LBrace, 
 Ident("print"), LParen, StringLiteral("Hello"), 
 RParen, Semicolon, RBrace]
```

### Stage 2: Parsing
```
Tokens
  ↓
AST Node: FunctionDef {
    name: "main",
    params: [],
    body: Block {
        statements: [
            Expression(Call {
                func: "print",
                args: [Literal("Hello")]
            })
        ]
    }
}
```

### Stage 3: Type Checking
```
AST
  ↓
Type-checked AST
  ↓
Result::Ok or Result::Err(TypeCheckError)
```

### Stage 4: IR Generation
```
AST
  ↓
Module {
    functions: [
        Function {
            name: "main",
            blocks: [
                BasicBlock {
                    label: "entry",
                    instructions: [...],
                    terminator: Return
                }
            ]
        }
    ]
}
```

### Stage 5: Code Generation
```
IR Module
  ↓
C Code
  ↓
CC Compilation
  ↓
Native Executable
```

## Error Handling

All stages use Rust's `Result<T, String>` for error handling:

```rust
pub fn lex(source: &str) -> Result<Vec<Token>, String>
pub fn parse(tokens: Vec<Token>) -> Result<CompilationUnit, String>
pub fn check(ast: &CompilationUnit) -> Result<(), String>
pub fn generate(ast: &CompilationUnit) -> Result<Module, String>
pub fn compile(module: &Module, output: &Path) -> Result<(), String>
```

## Platform Support

### Linux
- Compiler: `cc`
- ABI: System V AMD64
- Stdlib: POSIX

### FreeBSD
- Compiler: `cc`
- ABI: System V AMD64
- Stdlib: POSIX

### Windows
- Compiler: `gcc` (MinGW) or `cl` (MSVC)
- ABI: Windows x64
- Stdlib: Win32 API

## Optimization

### Level 0 (-O0)
- No optimization
- Fast compilation
- Debug builds

### Level 1 (-O1)
- Basic optimizations
- Dead code elimination

### Level 2 (-O2) [Default]
- Standard optimizations
- Inlining
- Loop optimization

### Level 3 (-O3)
- Aggressive optimizations
- Vectorization
- Full inlining

## Future Enhancements

1. **Native Code Generation**
   - Direct assembly generation
   - No C dependency

2. **JIT Compilation**
   - Runtime compilation
   - Dynamic optimization

3. **LLVM Backend**
   - Better optimizations
   - More targets

4. **WebAssembly Target**
   - Web deployment
   - Sandbox execution

## Author

**MelvinSGjr** (GitHub: [MelvinMod](https://github.com/MelvinMod))

## License

MIT License
