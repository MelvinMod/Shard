# Shard Quick Start Guide

## Installation

### Prerequisites

- Rust 1.70+ installed
- C compiler (gcc, clang, or MSVC)

### Build from Source

```bash
git clone https://github.com/MelvinMod/Shard.git
cd Shard
cargo build --release
```

### Add to PATH

```bash
export PATH="$PWD/target/release:$PATH"
```

## Your First Program

Create `hello.shard`:

```shard
fn main() {
    let message: String = "Hello, World!";
    print(message);
}
```

Compile and run:

```bash
shardc compile hello.shard -o hello
./hello
```

Or run directly:

```bash
shardc run hello.shard
```

## Basic Syntax

### Variables

```shard
let x: Int = 10;        // Immutable
mut y: Int = 20;        // Mutable
const MAX: Int = 100;   // Constant
```

### Functions

```shard
fn add(a: Int, b: Int) -> Int {
    return a + b;
}

fn greet(name: String) {
    print("Hello, ", name);
}
```

### Control Flow

```shard
// If/else
if x > 10 {
    print("Greater");
} else {
    print("Less");
}

// Loop
loop {
    break;
    continue;
}

// For loop
for i in 0..10 {
    print(i);
}

// Match
match value {
    1 => { print("One"); }
    2 => { print("Two"); }
    _ => { print("Other"); }
}
```

### Structs

```shard
struct Point {
    x: Int,
    y: Int,
}

let p: Point = Point { x: 10, y: 20 };
print(p.x);
```

## Examples

See the `examples/` directory:

- `hello.shard` - Hello World
- `fibonacci.shard` - Recursive function
- `arrays.shard` - Array operations
- `structs.shard` - Struct usage
- `game.shard` - Simple game loop

## Next Steps

1. Read `SYNTAX.md` for complete language reference
2. Read `docs/ARCHITECTURE.md` for compiler internals
3. Check `CONTRIBUTING.md` to help develop Shard

## Common Commands

```bash
# Compile
shardc compile program.shard -o program

# Run
shardc run program.shard

# Type check
shardc check program.shard

# Help
shardc --help
```

## Troubleshooting

### "command not found"

Add Shard to your PATH:

```bash
export PATH="$HOME/Shard/target/release:$PATH"
```

### "No main function"

Make sure you have a `main` function:

```shard
fn main() {
    // Your code here
}
```

### Type errors

Check your types match:

```shard
let x: Int = 10;      // Correct
let y: String = "10"; // Correct
// let z: Int = "10"; // Error!
```

---

**Author:** MelvinSGjr (MelvinMod)
