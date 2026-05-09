# BEGINNER 06: Structs and Data Organization

Learn how to organize data with structs and enums.

## What You'll Learn

- Creating structs
- Struct fields and methods
- Creating enums
- Pattern matching with enums

## What is a Struct?

A struct groups related data together:

```shard
struct Person
  name: String
  age: Int
~
```

## Creating Struct Instances

```shard
struct Person
  name: String
  age: Int
~

let person = Person {
  name: "Alice"
  age: 25
}

say person.name  # Alice
say person.age   # 25
```

## Struct Methods

Add functions to structs:

```shard
struct Person
  name: String
  age: Int
  
  fn greet(self)
    say "Hi, I'm #{self.name}"
  ~
  
  fn birthday(self) -> Person
    Person {
      name: self.name
      age: self.age + 1
    }
  ~
~

let person = Person {
  name: "Alice"
  age: 25
}

person.greet()
let older = person.birthday()
say older.age  # 26
```

## Constructor Method

Use `new` as a conventional constructor:

```shard
struct Person
  name: String
  age: Int
  
  fn new(name: String, age: Int) -> Person
    Person {
      name: name
      age: age
    }
  ~
~

let person = Person.new("Alice", 25)
```

## Mutable Structs

```shard
mut struct Person
  name: String
  age: Int
  
  fn aging(self)
    self.age = self.age + 1
  ~
~

mut person = Person.new("Alice", 25)
person.aging()
say person.age  # 26
```

## Enums

Enums define a type with possible values:

```shard
enum Color
  Red
  Green
  Blue
~

let c = Color::Red
```

## Enums with Data

```shard
enum Shape
  Circle(Float)
  Rectangle(Float, Float)
  Square(Float)
~

let circle = Shape::Circle(5.0)
let rect = Shape::Rectangle(10.0, 20.0)
```

## Matching Enums

```shard
fn area(shape: Shape) -> Float
  match shape
    case Shape::Circle(r) => 3.14 * r * r
    case Shape::Rectangle(w, h) => w * h
    case Shape::Square(s) => s * s
  ~
~

say area(Shape::Circle(5.0))       # 78.5
say area(Shape::Rectangle(10, 5))  # 50
```

## Option Type

Handle optional values:

```shard
enum Option<T>
  Some(T)
  None
~

fn find_user(id: Int) -> Option<String>
  if id == 1
    return Option::Some("Alice")
  ~
  Option::None
~

let user = find_user(1)
match user
  case Option::Some(name) => say "Found: #{name}"
  case Option::None => say "Not found"
~
```

## Result Type

Handle errors:

```shard
enum Result<T, E>
  Ok(T)
  Err(E)
~

fn divide(a: Int, b: Int) -> Result<Int, String>
  if b == 0
    return Result::Err("Division by zero")
  ~
  Result::Ok(a / b)
~

let result = divide(10, 2)
match result
  case Result::Ok(value) => say "Result: #{value}"
  case Result::Err(msg) => say "Error: #{msg}"
~
```

## Examples

### Example 1: Point in 2D Space

```shard
struct Point
  x: Float
  y: Float
  
  fn new(x: Float, y: Float) -> Point
    Point { x, y }
  ~
  
  fn distance(self, other: Point) -> Float
    let dx = self.x - other.x
    let dy = self.y - other.y
    (dx * dx + dy * dy).sqrt()
  ~
~

let p1 = Point.new(0.0, 0.0)
let p2 = Point.new(3.0, 4.0)
say p1.distance(p2)  # 5.0
```

### Example 2: Bank Account

```shard
struct BankAccount
  owner: String
  balance: Float
  
  fn new(owner: String) -> BankAccount
    BankAccount {
      owner: owner
      balance: 0.0
    }
  ~
  
  fn deposit(self, amount: Float) -> BankAccount
    BankAccount {
      owner: self.owner
      balance: self.balance + amount
    }
  ~
  
  fn withdraw(self, amount: Float) -> Result<BankAccount, String>
    if amount > self.balance
      return Result::Err("Insufficient funds")
    ~
    Result::Ok(BankAccount {
      owner: self.owner
      balance: self.balance - amount
    })
  ~
~

mut account = BankAccount.new("Alice")
account = account.deposit(100.0)
let result = account.withdraw(50.0)

match result
  case Result::Ok(acc) => {
    account = acc
    say "Withdrawn successfully. Balance: #{account.balance}"
  }
  case Result::Err(msg) => say "Error: #{msg}"
~
```

### Example 3: Traffic Light

```shard
enum TrafficLight
  Red
  Yellow
  Green
  
  fn next(self) -> TrafficLight
    match self
      case TrafficLight::Red => TrafficLight::Green
      case TrafficLight::Yellow => TrafficLight::Red
      case TrafficLight::Green => TrafficLight::Yellow
    ~
  ~
  
  fn action(self) -> String
    match self
      case TrafficLight::Red => "Stop"
      case TrafficLight::Yellow => "Prepare to stop"
      case TrafficLight::Green => "Go"
    ~
  ~
~

mut light = TrafficLight::Red
say light.action()  # Stop

light = light.next()
say light.action()  # Go
```

## Exercises

1. Create a `Rectangle` struct with width and height, and methods for area and perimeter
2. Make an enum for days of the week with a method to return the next day
3. Create a `Book` struct with title, author, and pages, plus a method to check if it's a novel (>200 pages)
4. Implement a simple stack using a struct with push, pop, and is_empty methods

## Summary

- `struct Name { field: Type }` defines a struct
- Access fields with `instance.field`
- Add methods with `fn method(self)`
- `self` refers to the current instance
- `enum Name { Variant1, Variant2 }` defines an enum
- Use `match` to handle enum variants
- Enums can hold data: `Variant(Type)`
- `Option<T>` and `Result<T, E>` are common enum patterns