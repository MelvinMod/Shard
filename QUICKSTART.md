# Quick Start with Shard

Welcome to Shard! This guide will get you up and running in minutes.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/MelvinMod/Shard.git
cd Shard

# Build the compiler
cargo build --release

# The binary is now at:
# ./target/release/shard
```

### Add to PATH (Optional)

```bash
export PATH="$PWD/target/release:$PATH"
```

## Your First Program

### Step 1: Create a file

Create `hello.shard`:

```shard
fn main()
  say "Hello, World!"
~
```

### Step 2: Run it

```bash
./target/release/shard run hello.shard
```

Output:
```
Hello, World!
```

### Step 3: Compile it

```bash
./target/release/shard compile hello.shard -o hello
./hello
```

## Basic Syntax

### Variables

```shard
# Immutable
let name = "Alice"

# Mutable
mut age = 25
age = 26

# With type annotation
let pi: Float64 = 3.14159
```

### Functions

```shard
fn greet(name: String) -> String
  return "Hello, #{name}!"
~

# Call it
let message = greet("World")
say message
```

### Conditionals

```shard
when age >= 18
  say "Adult"
elsif age >= 13
  say "Teenager"
else
  say "Child"
~
```

### Loops

```shard
# For loop
each i in 1..5
  say "Count: #{i}"
~

# While loop
while counter > 0
  say counter
  counter -= 1
~
```

### Structs

```shard
struct Person
  name: String
  age: Int
  
  fn new(name: String, age: Int) -> Person
    Person { name, age }
  ~
  
  fn greet(self)
    say "Hi, I'm #{self.name}"
  ~
~

let person = Person.new("Alice", 30)
person.greet()
```

## Common Commands

```bash
# Run a file
shard run program.shard

# Compile to binary
shard compile program.shard -o program

# Check for errors without compiling
shard check program.shard

# Build with optimizations
shard compile program.shard -o program --release

# Show help
shard --help

# Show version
shard --version
```

## File Structure

A typical Shard project:

```
my_project/
├── src/
│   └── main.shard
├── lib/
│   └── utils.shard
├── tests/
│   └── test.shard
└── README.md
```

## Next Steps

1. Read **BEGINNER_01.md** for more basics
2. Try the examples in `examples/`
3. Build a project from **PROJECT_01.md**

## Need Help?

- Check the documentation in `*.md` files
- Look at examples in `examples/`
- See **CONTRIBUTING.md** for community info

Happy coding! 🚀