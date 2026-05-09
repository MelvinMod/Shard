# START HERE: Welcome to Shard!

Welcome to the Shard programming language! This guide will help you get started.

## Who Is This For?

- **Complete Beginners** - Never programmed before? Great! Start here.
- **Programmers from Other Languages** - You'll pick it up quickly.
- **AI/ML Enthusiasts** - Build neural networks and models.
- **Game Developers** - Create fast, fun games.

## What is Shard?

Shard is a modern programming language that is:
- **Easy to learn** - Clean, readable syntax
- **Fast** - Compiles to native code
- **Safe** - Memory safety without garbage collection
- **Powerful** - Build anything from scripts to systems

## Installation

### Step 1: Install Rust

Shard is built with Rust, so you need Rust first:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 2: Clone Shard

```bash
git clone https://github.com/MelvinMod/Shard.git
cd Shard
```

### Step 3: Build the Compiler

```bash
cargo build --release
```

The compiler is now at `./target/release/shard`.

### Step 4: Add to PATH (Optional)

```bash
export PATH="$PWD/target/release:$PATH"
```

## Your First Program

Create `hello.shard`:

```shard
fn main()
  say "Hello, World!"
~
```

Run it:

```bash
./target/release/shard run hello.shard
```

See `Hello, World!`? You're a Shard programmer now!

## Learning Path

Follow these lessons in order:

### Beginner Series
1. **BEGINNER_01.md** - Variables, basic types, printing
2. **BEGINNER_02.md** - Strings and text manipulation
3. **BEGINNER_03.md** - Conditionals (if/else, match)
4. **BEGINNER_04.md** - Loops (each, while)
5. **BEGINNER_05.md** - Functions
6. **BEGINNER_06.md** - Structs and enums

### Projects
1. **PROJECT_01.md** - Build a simple game
2. **PROJECT_02.md** - Create an AI model
3. **PROJECT_03.md** - Make a web server

## Quick Reference

### Basic Syntax

```shard
# Variables
let name = "Alice"
mut age = 25

# Functions
fn greet(name: String) -> String
  "Hello, #{name}"
~

# Conditionals
when age >= 18
  say "Adult"
else
  say "Minor"
~

# Loops
each i in 1..5
  say i
~

# Structs
struct Point
  x: Float
  y: Float
~
```

### Common Commands

```bash
# Run a program
shard run program.shard

# Compile
shard compile program.shard -o program

# Type check
shard check program.shard

# Build project
shard build
```

## Tips for Success

1. **Read the docs** - All `.md` files are your friends
2. **Type the examples** - Don't copy-paste, type them yourself
3. **Experiment** - Change things, break things, learn
4. **Build projects** - Apply what you learn immediately
5. **Ask questions** - Check GitHub issues or community

## File Structure

A typical Shard project:

```
my_project/
├── src/
│   └── main.shard      # Entry point
├── lib/
│   └── utils.shard     # Helper functions
├── tests/
│   └── test.shard      # Tests
├── examples/           # Example programs
└── README.md           # Project description
```

## Next Steps

1. Finish reading this file
2. Go to **BEGINNER_01.md** for your first lesson
3. Come back here if you get lost

## Need Help?

- Check other `BEGINNER_*.md` files
- Look at examples in `examples/`
- See **CONTRIBUTING.md** for community info
- Open an issue on GitHub

## You're Ready!

Programming is a journey. Shard makes it enjoyable. Let's begin!

**Next: [BEGINNER_01.md](BEGINNER_01.md) - Your First Words**

---

**Author**: MelvinSGjr (MelvinMod)  
**Version**: 0.1.0  
**Goal**: Make programming accessible to everyone