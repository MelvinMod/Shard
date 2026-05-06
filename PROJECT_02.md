# Project 2: Make a Simple AI

## What You Will Make

A simple AI that learns from examples.

## What is AI?

AI is a program that learns patterns from data.

## Step 1: Simple Pattern Learning

```shard
# This AI learns simple rules

entity SimpleAI
  field rules
  
  func new
    @rules = {}
  ~
  
  func learn(input, output)
    @rules[input] = output
    say "Learned: #{input} -> #{output}"
  ~
  
  func predict(input)
    when @rules[input]
      return @rules[input]
    else
      return "I don't know"
    ~
  ~
~

let ai = SimpleAI.new()

ai.learn("yes", "I will do it")
ai.learn("no", "I will not do it")

say ai.predict("yes")    # Says "I will do it"
say ai.predict("no")     # Says "I will not do it"
say ai.predict("maybe")  # Says "I don't know"
```

## Step 2: Number Pattern AI

```shard
entity NumberAI
  field patterns
  
  func new
    @patterns = []
  ~
  
  func learn(input, output)
    @patterns.push({input: input, output: output})
    say "Learned: #{input} -> #{output}"
  ~
  
  func predict(input)
    each pattern in @patterns
      when pattern.input == input
        return pattern.output
      ~
    ~
    
    return "I don't know"
  ~
~

let ai = NumberAI.new()

ai.learn(1, 2)
ai.learn(2, 4)
ai.learn(3, 6)
ai.learn(4, 8)

say ai.predict(2)    # Says 4
say ai.predict(5)    # Says "I don't know"
```

## Step 3: Simple Chat Bot

```shard
entity ChatBot
  field responses
  
  func new
    @responses = {
      "hello": "Hello! How can I help you?",
      "how are you": "I am doing great! Thanks for asking.",
      "bye": "Goodbye! Have a nice day!",
      "what is your name": "I am ShardBot v1.0"
    }
  ~
  
  func chat(message)
    let lower_message = message.lower
    when @responses[lower_message]
      return @responses[lower_message]
    else
      return "I don't understand. Try saying: hello, how are you, bye"
    ~
  ~
~

let bot = ChatBot.new()

say bot.chat("Hello")           # Hello! How can I help you?
say bot.chat("how are you")     # I am doing great!
say bot.chat("what is your name")  # I am ShardBot v1.0
say bot.chat("unknown")         # I don't understand
```

## Step 4: Learning Chat Bot

```shard
entity LearningBot
  field responses
  field learning_mode
  
  func new
    @responses = {}
    @learning_mode = false
  ~
  
  func chat(message)
    let lower_message = message.lower
    
    when @responses[lower_message]
      return @responses[lower_message]
    when @learning_mode
      say "What should I say when someone says '#{message}'?"
      let response = "placeholder"    # Replace with user input
      @responses[lower_message] = response
      say "I learned it!"
      return response
    else
      @learning_mode = true
      say "Teach me! What should I say when someone says '#{message}'?"
      let response = "placeholder"    # Replace with user input
      @responses[lower_message] = response
      @learning_mode = false
      say "I learned it!"
      return response
    ~
  ~
~

let bot = LearningBot.new()

say bot.chat("hello")
# Teaches: "Hello! Nice to meet you!"

say bot.chat("hello")
# Says: "Hello! Nice to meet you!"
```

## Complete Example

```shard
# Simple Chat Bot

entity SimpleChatBot
  field responses
  
  func new
    @responses = {
      "hi": "Hello!",
      "hello": "Hi there!",
      "how are you": "I'm doing great!",
      "what is shard": "Shard is a programming language",
      "bye": "Goodbye!",
      "thank you": "You're welcome!"
    }
  ~
  
  func chat(message)
    let lower_message = message.lower
    
    when @responses[lower_message]
      return @responses[lower_message]
    else
      return "I don't understand that. Try: hi, hello, how are you, bye"
    ~
  ~
~

func main
  let bot = SimpleChatBot.new()
  
  say "=== Simple Chat Bot ==="
  say "Type 'bye' to exit"
  say ""
  
  loop
    say "You: "
    let message = "hello"    # Replace with user input
    
    when message.lower == "bye"
      say "Bot: Goodbye!"
      break
    ~
    
    let response = bot.chat(message)
    say "Bot: #{response}"
    say ""
  ~
~

main()
```

## What You Learned

1. AI learns from examples
2. AI matches patterns
3. AI can get better with more data

## Make It Better

1. Add more responses
2. Add fuzzy matching (understand similar words)
3. Add learning mode (user teaches the bot)

---

**Next Project:** Go to PROJECT_03.md to make a website!
