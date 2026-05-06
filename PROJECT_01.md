# Project 1: Make a Simple Game

## What You Will Make

A guessing game where the computer picks a number and you guess it.

## What You Need

You already know everything! Just:
- Variables
- Functions
- Loops
- If/else

## Step 1: The Computer Picks a Number

```shard
let secret_number = 7    # Later we'll make it random
say "I picked a number from 1 to 10"
```

## Step 2: Ask the User

```shard
say "Guess the number:"
let guess = 5    # For now, we'll use this
```

## Step 3: Check the Guess

```shard
when guess == secret_number
  say "You won!"
else
  say "Wrong! The number was #{secret_number}"
~
```

## Step 4: Let Them Try Again

```shard
let secret_number = 7
let guess = 0
let attempts = 0

loop
  say "Guess the number (1-10):"
  guess = 5    # Replace with user input later
  
  attempts = attempts + 1
  
  when guess == secret_number
    say "You won in #{attempts} attempts!"
    break
  when guess < secret_number
    say "Too low!"
  else
    say "Too high!"
  ~
~
```

## Complete Game

```shard
# Number Guessing Game

func play_game
  let secret_number = 7
  let guess = 0
  let attempts = 0
  
  say "=== Number Guessing Game ==="
  say "I picked a number from 1 to 10"
  say ""
  
  loop
    say "Guess the number (1-10):"
    guess = 5    # Replace with user input
    
    attempts = attempts + 1
    
    when guess == secret_number
      say ""
      say "You won!"
      say "It took you #{attempts} attempts"
      break
    when guess < secret_number
      say "Too low! Try again"
    else
      say "Too high! Try again"
    ~
    
    say ""
  ~
~

play_game()
```

## Make It Better

### Add Random Numbers

```shard
let secret_number = random(1, 10)
```

### Add Multiple Levels

```shard
func play_game
  let difficulty = 3    # Easy, Medium, Hard
  
  when difficulty == 1
    let max = 10
  when difficulty == 2
    let max = 50
  else
    let max = 100
  ~
  
  let secret_number = random(1, max)
  # ... rest of the game
~
```

### Add Hints

```shard
when guess < secret_number
  say "Too low! (at least #{guess + 1})"
else
  say "Too high! (at most #{guess - 1})"
~
```

## Your Turn

Add these features:
1. Count how many attempts
2. Say "You won!" when they guess right
3. Ask if they want to play again

## Summary

You made a complete game! You learned:
- Games are just programs with rules
- Loops let players try again
- If/else checks if they win

---

**Next Project:** Go to PROJECT_02.md to make an AI!
