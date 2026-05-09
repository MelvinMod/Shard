# Shard Programming Language

**Minimalist. Safe. Fast. Powerful.**

# Warning

Shard is a **toy / concept language** that I built for **myself**. It exists
because I kept running into problems with other languages that I otherwise
really like:

* **Zig & D** – Both are solid low‑level languages, but I constantly felt
  the urge to jump ship. Zig’s lack of encapsulation and D’s garbage‑collection
  baggage just never felt right for the kind of code I wanted to write.

* **Ruby** – Ruby’s syntax is a joy, but it’s not a systems language. When
  I need to be close to the metal, Ruby isn’t an option.

* **Crystal** – Crystal was almost perfect. It looks like Ruby, compiles to
  native code, and gives you low‑level control. But the reality hit hard:
  the **ecosystem is tiny**, many libraries are **abandoned or missing**, and
  **compile times** can be excruciatingly slow for anything bigger than a
  script. It never reached the critical mass that would have made it
  practical for my daily work.

* **Rust** – I tried Rust, but the **library system felt weird and chaotic**
  (like it got “drunk” somewhere along the way), and the language itself
  is **hard to grasp when you’re thinking about logic** rather than just
  fighting the borrow checker.

So I ended up creating Shard: a language that is **extremely simple** and
gives me exactly the low‑level feel I want – without the ecosystem nightmares
or the mental overhead.

**Use Shard sparingly.** It’s honestly shitcoded (please fork it and make
something better!), and I wouldn’t recommend it for serious daily tasks.
This is my personal playground, and you’re welcome to play in it too – just
don’t expect a polished product.

## Author
**MelvinSGjr** (GitHub: [MelvinMod](https://github.com/MelvinMod))

## What's New in 0.2.0

### Memory Safety Without GC
- No garbage collector
- Explicit memory management with `alloc/free`
- Ownership system like Rust but simpler
- `defer` for automatic cleanup
- Borrow checker without lifetime annotations

### Error Handling
- Functions return `T` or `!Error`
- `try` operator for error propagation
- No exceptions, no hidden control flow
- Flat error enums

### Comptime Evaluation
- Code executed at compile time
- Generate code, compute constants
- `comptime { ... }` blocks
- Type introspection with `@typeOf`, `@sizeOf`

### ASM Blocks
- Direct assembly access
- `asm { ... }` blocks
- Register-level control
- Map variables to registers

### JIT Compilation
- Built-in REPL with JIT
- Instant feedback
- Prototyping without full compile
- `shard repl` command

### Optional Type Annotations
- Type inference by default
- Explicit types when needed
- `let x: i32 = 42`
- Lambda syntax: `-> x, y { x + y }`

## Quick Start

### Install

```bash
git clone https://github.com/MelvinMod/Shard.git
cd Shard
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

### Hello World

```shard
fn main() -> Void
    say "Hello, World!"
~
```

Run:
```bash
shard run hello.shard
```

## Memory Management

### Basic Allocation

```shard
let ptr: *i32 = alloc(i32)
*ptr = 42
free(ptr)
```

### Ownership

```shard
let data: String = "Hello"
let ref: &String = &data  // Borrow, not copy
say *ref
```

### Defer for Cleanup

```shard
let file = open("test.txt")
defer { close(file) }  // Auto-cleanup
// Use file
// File automatically closed here
```

## Error Handling

### Error Type

```shard
fn read_file(path: []const u8) !String
    if not exists(path) {
        return Err(FileError::NotFound)
    }
    return Ok(load(path))
~
```

### Try Operator

```shard
let content: String = read_file("test.txt") try
// If error, returns early
```

### Match Errors

```shard
match read_file("test.txt") {
    Ok(content) => say(content),
    Err(e) => say("Error: #{e}"),
}
```

## Comptime

### Compile-Time Evaluation

```shard
let x: i32 = comptime { 2 + 2 }
let size: Int = comptime { size_of::<i32>() }
```

### Code Generation

```shard
comptime {
    for (1..5) |i| {
        generate_function(i)
    }
}
```

## ASM Blocks

### Direct Assembly

```shard
fn add(a: i32, b: i32) -> i32
    asm {
        mov eax, a
        add eax, b
        ret
    }
~
```

## REPL

```bash
shard repl
shard> let x: i32 = 42
shard> say x
42
```

## Examples

- `01_hello_v2.shard` - Hello World
- `02_memory_v2.shard` - Memory management
- `03_errors_v2.shard` - Error handling
- `04_comptime_v2.shard` - Comptime & ASM

## Commands

```bash
shard run file.shard          # Run
shard compile file.shard -o program  # Compile
shard check file.shard        # Check types
shard repl                    # Start REPL
shard version                 # Show version
```

## License

MIT

---

**Author:** MelvinSGjr (MelvinMod)  
**Version:** 0.2.0  
**Philosophy:** Simple syntax, powerful features, no surprises
