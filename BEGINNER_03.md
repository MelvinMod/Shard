# BEGINNER 03: Making Decisions

Learn how to make your programs smart with conditionals.

## What You'll Learn

- if/else statements
- Comparison operators
- Logical operators
- Match expressions

## if Statements

The simplest decision:

```shard
let age = 18

if age >= 18
  say "You are an adult"
~
```

Only runs if the condition is true.

## if/else

Choose between two options:

```shard
let age = 15

if age >= 18
  say "You can vote"
else
  say "You cannot vote yet"
~
```

## elsif

Multiple conditions:

```shard
let score = 85

if score >= 90
  say "A"
elsif score >= 80
  say "B"
elsif score >= 70
  say "C"
else
  say "F"
~
```

## Comparison Operators

Compare values:

```shard
==  # Equal
!=  # Not equal
>   # Greater than
<   # Less than
>=  # Greater than or equal
<=  # Less than or equal
```

Examples:

```shard
let x = 10
let y = 20

say x == y   # false
say x != y   # true
say x < y    # true
say x > y    # false
say x <= 10  # true
say y >= 20  # true
```

## Logical Operators

Combine conditions:

```shard
and  # Both must be true
or   # At least one must be true
not  # Negate
```

Examples:

```shard
let age = 25
let has_license = true

if age >= 18 and has_license
  say "You can drive"
~

let is_weekend = false
let is_holiday = true

if is_weekend or is_holiday
  say "No work today!"
~

let is_raining = true

if not is_raining
  say "Go outside!"
~
```

## Nested if

Put if inside if:

```shard
let age = 20
let has_money = true

if age >= 18
  if has_money
    say "You can buy the game"
  else
    say "You need money"
  ~
else
  say "You are too young"
~
```

## Ternary Operator

Short if/else:

```shard
let age = 20
let status = if age >= 18 "adult" else "minor"
say status
```

## match Expression

Match against values:

```shard
let day = 3

match day
  case 1 => say "Monday"
  case 2 => say "Tuesday"
  case 3 => say "Wednesday"
  case 4 => say "Thursday"
  case 5 => say "Friday"
  case 6 | 7 => say "Weekend"
  case _ => say "Invalid day"
~
```

The `_` is a catch-all for any other value.

## match with Ranges

```shard
let score = 85

match score
  case 90..100 => say "A"
  case 80..89  => say "B"
  case 70..79  => say "C"
  case _       => say "F"
~
```

## match with Types

```shard
let value: Int = 42

match value
  case 0 => say "Zero"
  case 1..=10 => say "Small number"
  case 11..=100 => say "Medium number"
  case _ => say "Large number"
~
```

## Boolean Values

```shard
let is_true = true
let is_false = false

if is_true
  say "This runs"
~

if not is_false
  say "This also runs"
~
```

## Examples

### Example 1: Login System

```shard
fn main()
  let username = "admin"
  let password = "secret123"
  
  if username == "admin" and password == "secret123"
    say "Login successful!"
  else
    say "Invalid credentials"
  ~
~
```

### Example 2: Grade Calculator

```shard
fn main()
  let score = 78
  
  if score >= 90
    say "A - Excellent!"
  elsif score >= 80
    say "B - Good job"
  elsif score >= 70
    say "C - Average"
  elsif score >= 60
    say "D - Passing"
  else
    say "F - Needs improvement"
  ~
~
```

### Example 3: Day of Week

```shard
fn main()
  let day = 5
  
  match day
    case 1 => say "Start of the week"
    case 2 | 3 | 4 | 5 => say "Weekday"
    case 6 | 7 => say "Weekend"
    case _ => say "Invalid day number"
  ~
~
```

### Example 4: Even or Odd

```shard
fn main()
  let num = 7
  
  if num % 2 == 0
    say "#{num} is even"
  else
    say "#{num} is odd"
  ~
~
```

## Exercises

1. Write a program that checks if a number is positive, negative, or zero
2. Create a simple login system with username and password
3. Write a program that converts a numeric grade to a letter grade
4. Use match to determine the season based on a month number (1-12)

## Next Lesson

Go to **BEGINNER_04.md** to learn about loops and repetition.

## Summary

- `if`, `elsif`, `else` for branching
- Comparison: `==`, `!=`, `>`, `<`, `>=`, `<=`
- Logical: `and`, `or`, `not`
- `match` for multiple cases
- `_` is the catch-all in match
- Ternary: `if condition "yes" else "no"`