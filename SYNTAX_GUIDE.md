# Complete Syntax Guide

## Lexical Elements

### Identifiers
```shard
identifier = letter (letter | digit | '_')*
```

### Literals
```shard
# Integer
123
0xFF
0b1010

# Float
3.14
2.5e10

# String
"Hello, World!"
"Line 1\nLine 2"

# Char
'A'
'\n'

# Boolean
true
false
```

### Comments
```shard
# Single line comment
```

## Grammar

### Program
```
program = { statement }
```

### Statement
```
statement = expression
          | let_statement
          | func_statement
          | entity_statement
          | if_statement
          | loop_statement
          | match_statement
          | return_statement
```

### Let Statement
```
let_statement = "let" identifier "=" expression
              | "mut" identifier "=" expression
              | "const" identifier "=" expression
```

### Function Statement
```
func_statement = "func" identifier "(" [ params ] ")" [ ":" type ] block
params = param { "," param }
param = identifier [ ":" type ]
```

### Entity Statement
```
entity_statement = "entity" identifier "{" { entity_member } "}"
entity_member = field_declaration
              | method_declaration
              | constructor
```

### If Statement
```
if_statement = "when" expression block
             | "when" expression block "else" block
             | "when" expression block { "elsif" expression block } "else" block
```

### Unless Statement
```
unless_statement = "unless" expression block
```

### Loop Statements
```
loop_statement = "loop" block
               | "while" expression block
               | "each" identifier "in" expression block
```

### Match Statement
```
match_statement = "match" expression "{" { match_arm } "}"
match_arm = pattern "=>" block
```

### Return Statement
```
return_statement = "return" [ expression ]
```

### Block
```
block = "{" { statement } "}"
```

### Expression
```
expression = assignment
           | logical_or
           | comparison
           | addition
           | multiplication
           | unary
           | call
           | primary
```

### Primary Expression
```
primary = literal
        | identifier
        | "(" expression ")"
        | "[" array_elements "]"
        | "{" hash_pairs "}"
```

### Call Expression
```
call = primary "(" [ args ] ")"
args = expression { "," expression }
```

### Array
```
array = "[" [ expression { "," expression } ] "]"
```

### Hash
```
hash = "{" [ hash_pair { "," hash_pair } ] "}"
hash_pair = key ":" value
```

### Operators

#### Arithmetic
```
+    Addition
-    Subtraction
*    Multiplication
/    Division
%    Modulo
**   Power
```

#### Comparison
```
==   Equal
!=   Not equal
<    Less than
>    Greater than
<=   Less than or equal
>=   Greater than or equal
```

#### Logical
```
and  Logical AND
or   Logical OR
not  Logical NOT
```

#### Assignment
```
=    Assignment
+=   Add and assign
-=   Subtract and assign
*=   Multiply and assign
/=   Divide and assign
%=   Modulo and assign
```

#### Range
```
..   Inclusive range
...  Exclusive range
```

### Types

#### Primitive Types
```
Int       # 64-bit integer
Int8      # 8-bit integer
Int16     # 16-bit integer
Int32     # 32-bit integer
Int64     # 64-bit integer
UInt      # 64-bit unsigned integer
UInt8     # 8-bit unsigned integer
UInt16    # 16-bit unsigned integer
UInt32    # 32-bit unsigned integer
UInt64    # 64-bit unsigned integer
Float     # 64-bit float
Float32   # 32-bit float
Float64   # 64-bit float
Bool      # Boolean
Char      # Character
String    # String
Void      # No value
```

#### Composite Types
```
Array(T)      # Array of T
Hash(K, V)    # Hash map with key K and value V
Tuple(T1, T2) # Tuple of types
Pointer(T)    # Pointer to T
```

### Pattern Matching

#### Patterns
```
pattern = literal
        | identifier
        | "_"
        | pattern "|" pattern
```

### Entity Members

#### Field Declaration
```
field_declaration = "field" identifier [ ":" type ]
```

#### Method Declaration
```
method_declaration = "func" identifier "(" [ params ] ")" [ ":" type ] block
```

#### Constructor
```
constructor = "func" "new" "(" [ params ] ")" block
```

### Visibility

```
pub      # Public
private  # Private (default)
```

### Generics

```
entity Stack(T)
  field items : Array(T)
~

func push(item: T)
  @items.push(item)
~
```

### Traits/Interfaces

```
trait Serializable
  func to_json : String
  func from_json(data: String)
~

entity Person
  include Serializable
  
  field name : String
  
  func to_json
    "{\"name\": \"#{@name}\"}"
  ~
~
```

### Macros

```
macro def_method(name)
  func #{name}
    say "Called #{name}"
  ~
~

def_method(my_method)
```

### Unsafe Code

```shard
unsafe
  let ptr = alloc(Int)
  *ptr = 42
  free(ptr)
~
```

### Async/Await

```shard
async func fetch_data(url: String) : String
  # Async operation
  "Data"
~

let data = await fetch_data("https://example.com")
```

### Concurrency

```shard
spawn do
  # Background task
~

task = spawn async { long_operation() }
let result = await task
```

### FFI

```shard
extern fn c_function(arg1: Int, arg2: String) -> Int
```

### Type Aliases

```shard
alias ID = UInt64
alias Point2D = Tuple(Float, Float)
```

### Modules

```shard
mod mymodule
  pub func helper
    # Code
  ~
~
```

### Imports

```shard
use mymodule
use mymodule.helper
from mymodule import helper
```

### Re-export

```shard
export func public_func
  # Code
~
```

### Attributes

```shard
#[inline]
func fast_function
  # Code
~

#[deprecated("Use new_function instead")]
func old_function
  # Code
~
```

### Documentation

```shard
# Single line documentation
func my_func
  # Code
~

##
# Multi-line documentation
# with multiple lines
##
func another_func
  # Code
~
```

---

**Author:** MelvinSGjr (MelvinMod)
