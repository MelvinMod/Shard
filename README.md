# Shard Programming Language

**Easy as Ruby. Simple as Lua. Beautiful as Crystal. Powerful as Zig. Smart as Python.**

## Author
**MelvinSGjr** (GitHub: [MelvinMod](https://github.com/MelvinMod))

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
fn main
  say "Hello, World!"
~
```

Run:
```bash
shard run hello.shard
```

## Two Backends

### Native Backend
- Compiles to C
- Fast executables
- No dependencies

```bash
shard compile program.shard -o program --backend native
```

### Rust FFI Backend (Rust Support)
- Compiles to Rust
- Use Rust libraries
- Type safety

```bash
shard compile program.shard -o program --backend rust
```

## AI & Machine Learning

### ONNX Support
```shard
use ai

let session = OnnxSession.new("model.onnx")
let output = session.predict(input)
```

### LLM Support
```shard
use ai

let llm = LLM.new("llama-2.gguf")
let response = llm.generate("What is AI?", 200)
```

### Computer Vision
```shard
use ai

let cv = ComputerVision.new("yolov8.onnx")
let objects = cv.detect_objects("photo.jpg")
```

### Neural Networks
```shard
use ai

let nn = NeuralNetwork.new()
  .add_layer(Dense(784, 128))
  .add_layer(Dense(128, 10))

nn.compile(optimizer="adam", loss="cross_entropy")
nn.train(dataset, epochs=50)
```

## Library Loading

```shard
use lib_loader

let loader = LibraryLoader.new("/usr/lib/libm.so")
loader.load()
```

## What You Can Build

- **AI Models** - LLMs, computer vision, neural networks
- **Games** - 2D/3D games, game engines
- **Web Apps** - Servers, APIs, full-stack
- **Systems** - OS, drivers, embedded
- **Scripts** - Automation, data processing

## Documentation

### Getting Started
- **GETTING_STARTED.md** - Complete beginner guide
- **QUICK_REFERENCE.md** - Syntax quick reference
- **SYNTAX_GUIDE.md** - Complete syntax reference

### Learning
- **BEGINNER_01.md** to **06.md** - Step-by-step lessons
- **PROJECT_01.md** to **03.md** - Real projects

### Advanced
- **AI_GUIDE.md** - AI & ML complete guide
- **LIBRARY_GUIDE.md** - Library loading guide
- **FEATURES.md** - Feature overview

### Examples
- **[examples](https://github.com/MelvinMod/Shard/tree/main/examples)** - Working programs

## Examples Directory

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
shard run program.shard      # Run
shard compile file.shard -o program  # Compile
shard check program.shard    # Check syntax
shard version                # Show version
```

## File Extension

Use `.shard` for source files.

## License

MIT License

---

**Author:** MelvinSGjr (MelvinMod)  
**Version:** 0.1.0  
**Goal:** Make programming available to me and perhaps everyone
