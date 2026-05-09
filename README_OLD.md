# Shard Programming Language

**Easy as Ruby. Simple as Lua. Beautiful as Crystal. Powerful as Zig. Smart as Python.**

## Author
**MelvinSGjr** (GitHub: [MelvinMod](https://github.com/MelvinMod))

## Quick Start

### Install

```bash
git clone https://github.com/MelvinMod/Shard.git
cd Shard
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

### Hello World

Create `hello.shard`:

```shard
fn main
  say "Hello, World!"
~
```

Run it:

```bash
shard run hello.shard
```

## Features

### Easy Syntax

```shard
# Variables
let name = "Shard"
let version = 1.0

say "Welcome to #{name} v#{version}"
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
```

## Two Backends

### Native Backend
- Compiles to C
- Fast native executables
- No runtime dependencies

```bash
shard compile program.shard -o program --backend native
```

### Rust FFI Backend
- Compiles to Rust
- Can use Rust libraries
- Good for interoperability

```bash
shard compile program.shard -o program --backend rust
```

## AI & Machine Learning

### ONNX Support

```shard
use ai

let session = OnnxSession.new("model.onnx")
let input = [1.0, 2.0, 3.0]
let output = session.predict(input)
```

### LLM Support

```shard
use ai

let llm = LLM.new("llama-2.gguf")
let response = llm.generate("What is AI?", 200)
say response
```

### Computer Vision

```shard
use ai

let cv = ComputerVision.new("yolov8.onnx")
let objects = cv.detect_objects("photo.jpg")

each obj in objects
  say "Found: #{obj.class_name} (confidence: #{obj.confidence})"
~
```

### Transformers

```shard
use ai

let transformer = Transformer.new("bert-base")
let embedding = transformer.encode("Hello world")
say "Embedding size: #{embedding.len}"
```

## Library Loading

```shard
use lib_loader

let loader = LibraryLoader.new("/path/to/library.so")
loader.load()
```

## What You Can Build

### AI and Machine Learning

- Neural networks
- Computer vision
- Language models
- Recommendation systems

### Games

- 2D and 3D games
- Game engines
- Physics simulations

### Web

- Web servers
- REST APIs
- Full-stack applications

### Systems

- Operating systems
- Device drivers
- Embedded systems

### Scripts

- Automation
- Data processing
- DevOps tools

## Learning Path

### Beginner (1-2 days)

1. **START_HERE.md** - Welcome guide
2. **BEGINNER_01.md** - Your first words
3. **BEGINNER_02.md** - Words and text
4. **BEGINNER_03.md** - Making decisions
5. **BEGINNER_04.md** - Repeating things
6. **BEGINNER_05.md** - Functions
7. **BEGINNER_06.md** - Objects

### Projects (3-5 days)

1. **PROJECT_01.md** - Make a game
2. **PROJECT_02.md** - Make an AI
3. **PROJECT_03.md** - Make a website

## Examples

See `examples/` directory:

- `01_hello.shard` - Hello World
- `02_calculator.shard` - Math operations
- `03_loops.shard` - Loop examples
- `04_conditionals.shard` - If/else/match
- `05_functions.shard` - Functions
- `06_entities.shard` - Objects
- `07_simple_ai.shard` - AI examples
- `08_data_structures.shard` - Collections

## Commands

```bash
# Run
shard run program.shard

# Compile
shard compile program.shard -o program

# Check
shard check program.shard

# Version
shard version
```

## File Extension

Use `.shard` for your source files:

```
main.shard
app.shard
library.shard
```

## Documentation

- **README.md** - This file
- **START_HERE.md** - Beginner's guide
- **BEGINNER_01.md to 06.md** - Lessons
- **PROJECT_01.md to 03.md** - Projects
- **SYNTAX.md** - Complete syntax reference
- **examples/README.md** - Example programs

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
