# Getting Started with Shard

## Welcome to Shard!

Shard is a modern programming language that combines the best features of Ruby, Crystal, Zig, and Rust while avoiding their pitfalls.

## Why Shard?

### Problems We Solved

**Crystal's Problems:**
- Slow compilation → Fast C backend
- Tiny ecosystem → Built-in AI/ML support
- Abandoned libraries → Active standard library
- No production use → Production-ready

**Rust's Problems:**
- Complex borrow checker → Simple memory model
- Steep learning curve → Ruby-like syntax
- Cargo complexity → Simple tooling

**Zig's Problems:**
- No standard library → Batteries included
- Manual everything → Smart defaults
- Steep learning → Easy syntax

**Ruby's Problems:**
- Slow execution → Native compilation
- No low-level access → Manual memory when needed
- GIL limitations → True parallelism

## Installation

### Quick Install

```bash
git clone https://github.com/MelvinMod/Shard.git
cd Shard
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

### Verify Installation

```bash
shard version
```

## Your First Program

### Step 1: Create File

Create `hello.shard`:

```shard
fn main
  say "Hello, World!"
~
```

### Step 2: Run It

```bash
shard run hello.shard
```

Output:
```
Hello, World!
```

### Step 3: Compile It

```bash
shard compile hello.shard -o hello --backend native
./hello
```

## Basic Concepts

### Variables

```shard
let name = "Shard"
let age = 1
let active = true
```

### Functions

```shard
func greet(name)
  say "Hello, #{name}!"
~

greet("World")
```

### Conditionals

```shard
when age >= 18
  say "Adult"
else
  say "Minor"
~
```

### Loops

```shard
each i in 1..5
  say i
~
```

## Next Steps

### Day 1: Basics

1. Read **BEGINNER_01.md** - Your first words
2. Read **BEGINNER_02.md** - Words and text
3. Try examples in **examples/01_hello.shard**

### Day 2: Control Flow

1. Read **BEGINNER_03.md** - Making decisions
2. Read **BEGINNER_04.md** - Repeating things
3. Try **examples/03_loops.shard**

### Day 3: Functions

1. Read **BEGINNER_05.md** - Functions
2. Try **examples/05_functions.shard**

### Day 4: Objects

1. Read **BEGINNER_06.md** - Objects
2. Try **examples/06_entities.shard**

### Day 5: AI

1. Read **AI_GUIDE.md** - AI & ML
2. Try **examples/07_simple_ai.shard**

### Day 6: Projects

1. Read **PROJECT_01.md** - Make a game
2. Read **PROJECT_02.md** - Make an AI
3. Read **PROJECT_03.md** - Make a website

## Resources

### Documentation
- **README.md** - Main documentation
- **SYNTAX.md** - Complete syntax reference
- **FEATURES.md** - Feature overview
- **AI_GUIDE.md** - AI & ML guide
- **LIBRARY_GUIDE.md** - Library loading

### Examples
- **examples/01_hello.shard** - Hello World
- **examples/02_calculator.shard** - Math
- **examples/03_loops.shard** - Loops
- **examples/04_conditionals.shard** - If/else
- **examples/05_functions.shard** - Functions
- **examples/06_entities.shard** - Objects
- **examples/07_simple_ai.shard** - AI
- **examples/08_data_structures.shard** - Collections

### Guides
- **BEGINNER_01.md** to **BEGINNER_06.md** - Lessons
- **PROJECT_01.md** to **PROJECT_03.md** - Projects

## Commands Reference

```bash
# Run a program
shard run program.shard

# Compile to executable
shard compile program.shard -o program

# Check for errors
shard check program.shard

# Show version
shard version

# Help
shard --help
```

## Backend Options

### Native Backend (C)

```bash
shard compile program.shard -o program --backend native
```

**Pros:**
- Fast compilation
- No dependencies
- Cross-platform

**Cons:**
- Limited library support

### Rust FFI Backend

```bash
shard compile program.shard -o program --backend rust
```

**Pros:**
- Use Rust libraries
- Type safety
- Rich ecosystem

**Cons:**
- Requires Rust installed
- Slower compilation

## Common Tasks

### Create New Project

```bash
mkdir my_project
cd my_project
mkdir src
touch src/main.shard
```

### Run Multiple Files

```shard
# main.shard
use mylib

fn main
  mylib.greet()
~

# mylib.shard
fn greet
  say "Hello from library!"
~
```

### Use External Libraries

```shard
use lib_loader

let loader = LibraryLoader.new("/usr/lib/libm.so")
loader.load()
```

## Tips

1. **Start Small** - Begin with simple programs
2. **Read Examples** - Study the examples directory
3. **Experiment** - Try modifying code
4. **Use Both Backends** - Compare native vs Rust
5. **Join Community** - Get help and share ideas

## Troubleshooting

### Compilation Errors

```bash
# Check syntax
shard check program.shard

# See detailed errors
shard compile program.shard 2>&1
```

### Runtime Errors

```bash
# Run with debug info
shard run program.shard 2>&1

# Check if executable exists
ls -la a.out
```

### Library Loading

```bash
# Check library exists
ldd /usr/lib/libm.so

# Check dependencies
ldd ./my_program
```

## Community

- GitHub: [MelvinMod/Shard](https://github.com/MelvinMod/Shard)
- Issues: Report bugs and request features
- Examples: Share your programs

## License

MIT License

---

**Author:** MelvinSGjr (MelvinMod)  
**Version:** 0.1.0  
**Goal:** Make programming accessible to everyone
