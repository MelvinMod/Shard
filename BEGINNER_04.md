# BEGINNER 04: Loops and Repetition

Learn how to repeat actions with loops.

## What You'll Learn

- `each` loops
- `while` loops
- `break` and `continue`
- Loop with ranges

## The `each` Loop

Repeat for each item in a collection:

```shard
each i in 1..5
  say i
~
```

Output:
```
1
2
3
4
5
```

## Ranges

Ranges define a start and end:

```shard
1..5      # 1, 2, 3, 4, 5 (inclusive)
1..5      # 1, 2, 3, 4 (exclusive on right)
```

## Looping Through Lists

```shard
let fruits = ["apple", "banana", "cherry"]

each fruit in fruits
  say fruit
~
```

Output:
```
apple
banana
cherry
```

### With Index

```shard
let colors = ["red", "green", "blue"]

each index, color in colors
  say "#{index}: #{color}"
~
```

Output:
```
0: red
1: green
2: blue
```

## The `while` Loop

Repeat while a condition is true:

```shard
let count = 1

while count <= 5
  say count
  count = count + 1
~
```

Output:
```
1
2
3
4
5
```

### Countdown Example

```shard
let count = 5

while count > 0
  say "Countdown: #{count}"
  count = count - 1
~

say "Blast off!"
```

## Break - Stop Early

Exit a loop with `break`:

```shard
each i in 1..100
  if i == 10
    break
  say i
~
```

Only prints 1 through 9.

## Continue - Skip One

Skip the current iteration:

```shard
each i in 1..5
  if i == 3
    continue
  say i
~
```

Output: `1, 2, 4, 5` (skips 3)

## Nested Loops

Loops inside loops:

```shard
each i in 1..3
  each j in 1..3
    say "i=#{i}, j=#{j}"
  ~
~
```

## Examples

### Example 1: Sum of Numbers

```shard
fn main()
  let total = 0
  each i in 1..10
    total = total + i
  ~
  say "Sum: #{total}"  # Sum: 55
~
```

### Example 2: Multiplication Table

```shard
fn main()
  let number = 7
  
  each i in 1..10
    say "#{number} x #{i} = #{number * i}"
  ~
~
```

### Example 3: Find First Even

```shard
fn main()
  let numbers = [1, 3, 5, 8, 9]
  
  each num in numbers
    if num % 2 == 0
      say "First even: #{num}"
      break
    ~
  ~
~
```

### Example 4: Skip Odd Numbers

```shard
fn main()
  each i in 1..10
    if i % 2 != 0
      continue
    say i
  ~
~
```

Output: `2, 4, 6, 8, 10`

### Example 5: Factorial

```shard
fn main()
  let n = 5
  let factorial = 1
  
  each i in 1..n
    factorial = factorial * i
  ~
  
  say "5! = #{factorial}"  # 5! = 120
~
```

## Infinite Loop (Careful!)

```shard
mut count = 0

while true
  say count
  count = count + 1
  if count >= 5
    break
  ~
~
```

Always use `break` to avoid infinite loops!

## Exercises

1. Print all even numbers from 1 to 20
2. Calculate the sum of numbers from 1 to 100
3. Print a countdown from 10 to 1, then "Happy New Year!"
4. Find the first number divisible by 7 in a list

## Next Lesson

Go to **BEGINNER_05.md** to learn about functions and code organization.

## Summary

- `each item in collection` loops through items
- `while condition` loops while true
- `break` exits the loop
- `continue` skips to next iteration
- Use ranges like `1..10` for number sequences
- Always ensure loops can terminate!