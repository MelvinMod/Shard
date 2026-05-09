# Shard Quick Reference

## Basic Syntax

### Comments
```shard
# Single line comment
```

### Variables
```shard
let x = 10           # Immutable
mut y = 20           # Mutable
const MAX = 100      # Constant
```

### Types
```shard
Int                  # Integer
Float                # Floating point
Bool                 # true/false
String               # Text
Char                 # Single character
```

## Functions

### Definition
```shard
func name(param1, param2)
  # Body
  return value
~
```

### With Types
```shard
func add(a: Int, b: Int) : Int
  a + b
~
```

### Default Arguments
```shard
func greet(name, greeting = "Hello")
  say "#{greeting}, #{name}!"
~
```

## Control Flow

### If/Else
```shard
when condition
  # Code
else
  # Other code
~
```

### Elsif
```shard
when x > 10
  say "Greater"
elsif x > 5
  say "Medium"
else
  say "Small"
~
```

### Unless
```shard
unless valid
  say "Invalid"
~
```

### Match
```shard
match value
  1 => say "One"
  2 => say "Two"
  _ => say "Other"
~
```

## Loops

### Each Loop
```shard
each i in 1..10
  say i
~
```

### While Loop
```shard
while count < 10
  count = count + 1
~
```

### Loop with Break
```shard
loop
  break when done
~
```

### For Each
```shard
each item in items
  say item
~
```

## Entities (Objects)

### Definition
```shard
entity Person
  field name
  field age
  
  func new(name, age)
    @name = name
    @age = age
  ~
  
  func greet
    "Hello, I'm #{@name}"
  ~
~
```

### Usage
```shard
let person = Person.new("Alice", 25)
say person.greet()
```

## Collections

### Arrays
```shard
let numbers = [1, 2, 3, 4, 5]

# Operations
numbers.map { |n| n * 2 }
numbers.select { |n| n.even? }
numbers.reduce(0, &:+)
```

### Hashes
```shard
let person = {
  name: "Alice",
  age: 25
}

say person[:name]
```

## Error Handling

### Try/Catch
```shard
try
  result = risky_operation()
catch Error as e
  say "Error: #{e.message}"
finally
  cleanup()
~
```

## Modules

### Definition
```shard
module MyModule
  func helper
    # Code
  ~
~
```

### Include
```shard
entity MyClass
  include MyModule
~
```

### Use
```shard
use mymodule

mymodule.helper()
```

## Libraries

### Load Library
```shard
use lib_loader

let loader = LibraryLoader.new("/path/to/library.so")
loader.load()
```

### Call Function
```shard
let result = call_function("func_name", arg1, arg2)
```

## AI & ML

### ONNX
```shard
use ai

let session = OnnxSession.new("model.onnx")
let output = session.predict(input)
```

### LLM
```shard
use ai

let llm = LLM.new("model.gguf")
let response = llm.generate(prompt, max_tokens=200)
```

### Computer Vision
```shard
use ai

let cv = ComputerVision.new("yolov8.onnx")
let objects = cv.detect_objects("image.jpg")
```

### Neural Network
```shard
use ai

let nn = NeuralNetwork.new()
  .add_layer(Dense(784, 128))
  .add_layer(Dense(128, 10))

nn.compile(optimizer="adam", loss="cross_entropy")
nn.train(dataset, epochs=50)
```

## I/O

### File Reading
```shard
let content = File.read("file.txt")
```

### File Writing
```shard
File.write("file.txt", "content")
```

### Print
```shard
say "Hello"
say "Value: #{x}"
```

## Math

### Basic Operations
```shard
a + b              # Addition
a - b              # Subtraction
a * b              # Multiplication
a / b              # Division
a % b              # Modulo
a ** b             # Power
```

### Functions
```shard
Math.sqrt(x)
Math.abs(x)
Math.max(a, b)
Math.min(a, b)
Math.random()
```

## Concurrency

### Spawn
```shard
spawn do
  # Background task
~
```

### Async/Await
```shard
async func fetch_data
  # Async code
~

let data = await fetch_data()
```

## Pointers

### Allocation
```shard
let ptr = alloc(Int)
*ptr = 42
free(ptr)
```

### References
```shard
let ref = &variable
```

## Macros

### Definition
```shard
macro my_macro
  # Generated code
~
```

### Usage
```shard
my_macro()
```

## Keywords

| Keyword | Purpose |
|---------|---------|
| `let` | Immutable variable |
| `mut` | Mutable variable |
| `const` | Constant |
| `func` | Function definition |
| `return` | Return value |
| `when` | If condition |
| `else` | Else branch |
| `unless` | Negative if |
| `each` | For loop |
| `while` | While loop |
| `loop` | Infinite loop |
| `break` | Exit loop |
| `continue` | Next iteration |
| `match` | Pattern matching |
| `entity` | Class definition |
| `field` | Object property |
| `new` | Constructor |
| `self` | Current object |
| `super` | Parent method |
| `use` | Import module |
| `include` | Mixin |
| `try` | Error handling |
| `catch` | Catch error |
| `async` | Async function |
| `await` | Wait for async |
| `unsafe` | Unsafe code |
| `~` | End block |

---

**Author:** MelvinSGjr (MelvinMod)
