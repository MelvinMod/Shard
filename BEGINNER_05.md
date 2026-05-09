# BEGINNER 05: Functions

Learn how to organize your code with functions.

## What You'll Learn

- Creating functions
- Parameters and arguments
- Return values
- Function scope

## What is a Function?

A function is a reusable block of code:

```shard
fn say_hello()
  say "Hello!"
~

say_hello()
```

Output: `Hello!`

## Defining Functions

Use `fn` to create a function:

```shard
fn greet()
  say "Welcome to Shard!"
~
```

Call it by name:

```shard
greet()
```

## Parameters

Functions can accept inputs:

```shard
fn greet(name)
  say "Hello, #{name}!"
~

greet("Alice")
greet("Bob")
```

Output:
```
Hello, Alice!
Hello, Bob!
```

### Multiple Parameters

```shard
fn introduce(first_name, last_name, age)
  say "I'm #{first_name} #{last_name}, #{age} years old"
~

introduce("Alice", "Smith", 25)
```

### Typed Parameters

```shard
fn greet(name: String)
  say "Hello, #{name}!"
~

fn add(a: Int, b: Int)
  say a + b
~
```

## Return Values

Functions can return values:

```shard
fn add(a: Int, b: Int) -> Int
  return a + b
~

let result = add(5, 3)
say result  # 8
```

### Implicit Return

You can omit `return` for the last expression:

```shard
fn add(a: Int, b: Int) -> Int
  a + b
~
```

## Return Type

Specify the return type:

```shard
fn multiply(a: Int, b: Int) -> Int
  a * b
~

fn get_name() -> String
  "Anonymous"
~

fn is_adult(age: Int) -> Bool
  age >= 18
~
```

## Function Scope

Variables inside functions are local:

```shard
fn my_function()
  let x = 10  # Local variable
  say x
~

my_function()
# say x  # This would error - x doesn't exist here
```

## Default Parameters

```shard
fn greet(name: String, greeting: String = "Hello")
  say "#{greeting}, #{name}!"
~

greet("Alice")           # Hello, Alice!
greet("Bob", "Hi")       # Hi, Bob!
```

## Examples

### Example 1: Calculator Functions

```shard
fn add(a: Int, b: Int) -> Int
  a + b
~

fn subtract(a: Int, b: Int) -> Int
  a - b
~

fn multiply(a: Int, b: Int) -> Int
  a * b
~

fn divide(a: Int, b: Int) -> Int
  a / b
~

say add(10, 5)        # 15
say subtract(10, 5)   # 5
say multiply(10, 5)   # 50
say divide(10, 5)     # 2
```

### Example 2: String Utilities

```shard
fn capitalize(word: String) -> String
  word.to_upper()
~

fn reverse_string(text: String) -> String
  text.reverse()
~

say capitalize("hello")    # HELLO
say reverse_string("abc")  # cba
```

### Example 3: Temperature Converter

```shard
fn celsius_to_fahrenheit(c: Float) -> Float
  c * 9.0 / 5.0 + 32.0
~

fn fahrenheit_to_celsius(f: Float) -> Float
  (f - 32.0) * 5.0 / 9.0
~

say celsius_to_fahrenheit(0.0)    # 32.0
say fahrenheit_to_celsius(32.0)   # 0.0
```

### Example 4: Factorial

```shard
fn factorial(n: Int) -> Int
  if n <= 1
    return 1
  ~
  n * factorial(n - 1)
~

say factorial(5)  # 120
```

### Example 5: Check Prime

```shard
fn is_prime(n: Int) -> Bool
  if n <= 1
    return false
  ~
  if n <= 3
    return true
  ~
  if n % 2 == 0 or n % 3 == 0
    return false
  ~
  
  mut i = 5
  while i * i <= n
    if n % i == 0 or n % (i + 2) == 0
      return false
    ~
    i = i + 6
  ~
  
  true
~

say is_prime(17)  # true
say is_prime(10)  # false
```

## Recursion

Functions can call themselves:

```shard
fn fibonacci(n: Int) -> Int
  if n <= 1
    return n
  ~
  fibonacci(n - 1) + fibonacci(n - 2)
~

each i in 0..10
  say fibonacci(i)
~
```

Output: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34

## Void Functions

Functions that don't return a value:

```shard
fn print_banner()
  say "======"
  say "Banner"
  say "======"
~

print_banner()
```

## Exercises

1. Create a function that calculates the area of a rectangle
2. Write a function that checks if a string is a palindrome
3. Create a function that converts minutes to hours and minutes
4. Write a recursive function to calculate powers (x^n)

## Next Lesson

Go to **BEGINNER_06.md** to learn about structs and organizing data.

## Summary

- `fn name(params) -> Type` defines a function
- Use `return` to return a value
- Last expression can be implicit return
- Variables inside functions are local
- Functions can call themselves (recursion)
- Default parameters: `fn f(x: Int = 5)`