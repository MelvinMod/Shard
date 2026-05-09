# BEGINNER 02: Strings and Text

In this lesson, you'll learn how to work with text in Shard.

## What You'll Learn

- String creation and manipulation
- String methods
- Escaping special characters
- Multi-line strings

## Creating Strings

You can create strings with double or single quotes:

```shard
let greeting = "Hello"
let name = 'World'
```

Both work the same way.

## String Concatenation

Combine strings with `+`:

```shard
let first = "Hello"
let second = "World"
let combined = first + " " + second
say combined
```

Output: `Hello World`

## String Interpolation

Insert variables into strings with `#{}`:

```shard
let name = "Alice"
let age = 25
say "Name: #{name}, Age: #{age}"
```

Output: `Name: Alice, Age: 25`

You can also use expressions:

```shard
let x = 10
let y = 20
say "Sum: #{x + y}"
```

Output: `Sum: 30`

## String Length

Get the length of a string:

```shard
let text = "Hello"
say text.length
```

Output: `5`

## Accessing Characters

Get characters by index (starts at 0):

```shard
let word = "Hello"
say word[0]  # H
say word[1]  # e
say word[4]  # o
```

## String Methods

### to_upper / to_lower

```shard
let text = "Hello"
say text.to_upper()  # HELLO
say text.to_lower()  # hello
```

### trim

Remove whitespace:

```shard
let text = "  Hello  "
say text.trim()  # "Hello"
```

### replace

Replace parts of a string:

```shard
let text = "Hello World"
say text.replace("World", "Shard")  # "Hello Shard"
```

### starts_with / ends_with

```shard
let filename = "document.pdf"
say filename.starts_with("doc")  # true
say filename.ends_with(".pdf")   # true
```

### contains

Check if a string contains another:

```shard
let text = "The quick brown fox"
say text.contains("quick")  # true
say text.contains("dog")    # false
```

### split

Split a string into parts:

```shard
let text = "apple,banana,cherry"
let parts = text.split(",")
say parts[0]  # apple
say parts[1]  # banana
```

## Escaping Characters

Use `\` for special characters:

```shard
# Newline
say "Line 1\nLine 2"

# Tab
say "Column1\tColumn2"

# Quote
say "She said \"Hello\""

# Backslash
say "Path\\to\\file"

# Carriage return
say "Overwrite\rDone"
```

## Multi-line Strings

Use triple quotes for multi-line text:

```shard
let poem = """
  Roses are red
  Violets are blue
  Shard is great
  Programming's cool too
"""
say poem
```

## String Comparison

Compare strings:

```shard
let a = "hello"
let b = "hello"
let c = "world"

say a == b  # true
say a == c  # false
say a != c  # true
```

## Empty String

Check if a string is empty:

```shard
let empty = ""
say empty.is_empty()  # true

let text = "Hello"
say text.is_empty()   # false
```

## Converting to String

Convert other types to strings:

```shard
let number = 42
say number.to_string()  # "42"

let pi = 3.14
say pi.to_string()      # "3.14"

let flag = true
say flag.to_string()    # "true"
```

## Examples

### Example 1: Greeting

```shard
fn main()
  let first_name = "John"
  let last_name = "Doe"
  let full_name = first_name + " " + last_name
  say "Hello, #{full_name}!"
~
```

### Example 2: Formatted Output

```shard
fn main()
  let product = "Laptop"
  let price = 999.99
  say "Product: #{product}"
  say "Price: $#{price}"
~
```

### Example 3: Text Processing

```shard
fn main()
  let text = "  Hello, World!  "
say text.trim()
  say text.trim().to_upper()
~
```

## Exercises

1. Create your name and print it in uppercase
2. Take a sentence and split it into words
3. Check if a string contains a specific word
4. Create a multi-line message about yourself

## Next Lesson

Go to **BEGINNER_03.md** to learn about making decisions with if/else.

## Summary

- Use `#{}` for string interpolation
- Common methods: `length`, `to_upper()`, `to_lower()`, `trim()`, `replace()`
- Escape characters with `\`: `\n`, `\t`, `\"`, `\\`
- Use `"""` for multi-line strings
- Convert types with `to_string()`