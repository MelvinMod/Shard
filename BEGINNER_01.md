# BEGINNER 01: Your First Words

Welcome to Shard! In this lesson, you'll learn the absolute basics.

## What You'll Learn

- How to print text
- What variables are
- Basic data types
- Comments

## Printing Text

The easiest way to start is with `say`:

```shard
say "Hello, World!"
```

This prints: `Hello, World!`

You can say multiple things:

```shard
say "Line 1"
say "Line 2"
say "Line 3"
```

Output:
```
Line 1
Line 2
Line 3
```

## Variables

Variables store data. Use `let` to create them:

```shard
let name = "Alice"
let age = 25
```

Now you can use them:

```shard
let name = "Alice"
say name
```

Output: `Alice`

### Mutable Variables

By default, variables can't change. Use `mut` if you need to:

```shard
mut score = 0
score = 10
score = 100
say score
```

Output: `100`

## Data Types

Shard has several basic types:

### Strings (Text)

```shard
let greeting = "Hello"
let name = 'World'  # Also works
```

### Integers (Whole Numbers)

```shard
let age = 25
let price = 1999
let hex = 0xFF      # Hexadecimal: 255
let binary = 0b1010 # Binary: 10
```

### Floats (Decimal Numbers)

```shard
let pi = 3.14159
let temperature = -5.5
```

### Booleans (True/False)

```shard
let is_active = true
let is_done = false
```

### Characters (Single Letter)

```shard
let letter = 'A'
let symbol = '@'
```

## String Interpolation

Put variables inside strings with `#{}`:

```shard
let name = "Alice"
let age = 25
say "My name is #{name} and I'm #{age} years old"
```

Output: `My name is Alice and I'm 25 years old`

## Comments

Add notes with `#`:

```shard
# This is a comment
let x = 42  # This is also a comment

say x  # Say the value of x
```

Multi-line comments:

```shard
/*
  This is a multi-line comment.
  You can write multiple lines.
*/
```

## Your First Program

Here's a complete program:

```shard
# My first Shard program
fn main()
  let name = "Shard"
  say "Welcome to #{name}!"
  say "Let's learn programming together."
  
  mut count = 0
  count = count + 1
  say "Count is now: #{count}"
~
```

## Exercises

Try these:

1. Print your name
2. Create a variable with your age
3. Print a message using your name and age
4. Add a comment explaining what your program does

## Next Lesson

Go to **BEGINNER_02.md** to learn more about strings and text manipulation.

## Summary

- `say "text"` prints to the screen
- `let name = value` creates an immutable variable
- `mut name = value` creates a mutable variable
- Basic types: String, Int, Float, Bool, Char
- Use `#{variable}` inside strings
- `#` starts a comment