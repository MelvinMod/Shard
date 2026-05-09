# Shard Examples

This directory contains working examples demonstrating Shard programming language features.

## Running Examples

```bash
# Using native backend (C compilation)
shard run examples/01_hello.shard --backend native

# Using Rust FFI backend
shard run examples/01_hello.shard --backend rust

# Compile to executable
shard compile examples/01_hello.shard -o my_program --backend native
./my_program
```

## Example List

### 01_hello.shard
Hello World program - your first Shard program

### 02_calculator.shard
Basic math operations and functions

### 03_loops.shard
For loops, while loops, and array iteration

### 04_conditionals.shard
If/else, unless, and match statements

### 05_functions.shard
Function definitions, parameters, and return values

### 06_entities.shard
Object-oriented programming with entities

### 07_simple_ai.shard
Simple AI pattern learning examples

### 08_data_structures.shard
Arrays, hashes, and string operations

## Backend Options

### Native Backend
- Compiles to C then to native executable
- Fast execution
- No runtime dependencies

### Rust FFI Backend
- Compiles to Rust code
- Can use Rust libraries
- Good for interoperability

## Tips

1. Read examples in order (01, 02, 03, ...)
2. Try modifying the code
3. Run with both backends
4. Build your own programs

---

**Author:** MelvinSGjr (MelvinMod)
