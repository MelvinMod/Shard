# Shard Programming Language

**Easy to learn. Powerful to use.**

## Author
**MelvinSGjr** (GitHub: [MelvinMod](https://github.com/MelvinMod))

## What is Shard?

Shard is a programming language that is:
- **Easy to learn** - Start in one day
- **Beautiful** - Clean, readable code
- **Fast** - Runs quickly
- **Powerful** - Build anything

## Quick Start

### Install Shard

```bash
git clone https://github.com/MelvinMod/Shard.git
cd Shard
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

### Your First Program

Create `hello.shard`:

```shard
say "Hello, World!"
```

Run it:

```bash
shard run hello.shard
```

Output:
```
Hello, World!
```

## Why Shard?

### For Beginners
- Easy syntax
- Clear error messages
- Good documentation
- Learn in one week

### For Experts
- Fast execution
- Low-level control
- Manual memory when needed
- Build anything

### For AI Enthusiasts
- Build neural networks
- Train models
- Ready-to-use libraries

### For Game Developers
- Fast execution
- Graphics support
- Game engines

## What Can You Build?

- **AI Models** - Neural networks, machine learning
- **Games** - 2D and 3D games
- **Applications** - Desktop apps, tools
- **Websites** - Web servers, APIs
- **Scripts** - Automation, data processing

## Learning Path

### Start Here (when it will be accessible)

1. **START_HERE.md** - Welcome guide
2. **BEGINNER_01.md** - Your first words
3. **BEGINNER_02.md** - Words and text
4. **BEGINNER_03.md** - Making decisions
5. **BEGINNER_04.md** - Repeating things
6. **BEGINNER_05.md** - Functions
7. **BEGINNER_06.md** - Objects

### Projects (when it will be accessible)

1. **PROJECT_01.md** - Make a game
2. **PROJECT_02.md** - Make an AI
3. **PROJECT_03.md** - Make a website

## Language Features

### Variables

```shard
let name = "Alice"
let age = 25
say "Hello, #{name}"
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

### Objects

```shard
entity Person
  field name
  field age
  
  func new(name, age)
    @name = name
    @age = age
  ~
~

let person = Person.new("Alice", 25)
```

## Standard Library

- **core** - Core types and functions
- **io** - File I/O
- **http** - Web server
- **json** - JSON parsing
- **ai** - Machine learning
- **game** - Game development

## Compiler Commands

```bash
# Run a program
shard run program.shard

# Compile
shard compile program.shard -o program

# Check
shard check program.shard

# Build
shard build --release
```

## File Extension

Use `.shard` for your files:

```
hello.shard
game.shard
website.shard
```

## Community

- GitHub: [MelvinMod/Shard](https://github.com/MelvinMod/Shard)
- Examples: `examples/` directory
- Guides: `*.md` files

## License

MIT License

---

**Author:** MelvinSGjr (MelvinMod)  
**Version:** 0.1.0  
**Goal:** Make programming accessible to everyone
