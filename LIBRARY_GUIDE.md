# Library Loading in Shard

## Overview

Shard supports loading external libraries dynamically, allowing you to:

- Use C libraries
- Load shared objects (.so, .dll, .dylib)
- Import Rust libraries
- Extend functionality with existing code

## Loading Libraries

### Basic Library Loading

```shard
use lib_loader

let loader = LibraryLoader.new("/usr/lib/libm.so")
loader.load()
```

### Windows Libraries

```shard
use lib_loader

let loader = LibraryLoader.new("C:\\Windows\\System32\\kernel32.dll")
loader.load()
```

### macOS Libraries

```shard
use lib_loader

let loader = LibraryLoader.new("/usr/lib/libSystem.dylib")
loader.load()
```

## Using C Libraries

### Math Library

```shard
use lib_loader

let loader = LibraryLoader.new("/usr/lib/libm.so")
loader.load()

# Call math functions
let sqrt_result = call_function("sqrt", 16.0)
say "sqrt(16) = #{sqrt_result}"
```

### Network Library

```shard
use lib_loader

let loader = LibraryLoader.new("/usr/lib/libcurl.so")
loader.load()

# Use curl functions
let curl = call_function("curl_easy_init")
```

## Rust Library Integration

### Loading Rust Libraries

```shard
use lib_loader

let loader = LibraryLoader.new("./target/release/libmylib.so")
loader.load()

# Call Rust functions
let result = call_function("my_function", arg1, arg2)
```

### FFI Declarations

```shard
# Declare external function
extern fn rust_function(arg1: Int, arg2: String) -> Int

# Use it
let result = rust_function(42, "hello")
```

## Dynamic Loading

### Load at Runtime

```shard
use lib_loader

fn load_plugin(plugin_path: String)
  let loader = LibraryLoader.new(plugin_path)
  if loader.load()
    say "Plugin loaded successfully"
    return true
  else
    say "Failed to load plugin"
    return false
  ~
~

load_plugin("./plugins/my_plugin.so")
```

### Plugin System

```shard
use lib_loader

entity PluginManager
  field plugins : Array[LibraryLoader]
  
  func new
    @plugins = []
  ~
  
  func load_plugin(path: String)
    let loader = LibraryLoader.new(path)
    if loader.load()
      @plugins.push(loader)
      return true
    else
      return false
    ~
  ~
~

let manager = PluginManager.new()
manager.load_plugin("./plugins/plugin1.so")
manager.load_plugin("./plugins/plugin2.so")
```

## Common Libraries

### Math

```bash
# Linux
/usr/lib/libm.so

# macOS
/usr/lib/libSystem.dylib

# Windows
C:\Windows\System32\msvcrt.dll
```

### Networking

```bash
# Linux
/usr/lib/libcurl.so
/usr/lib/libssl.so
/usr/lib/libcrypto.so

# Windows
C:\Windows\System32\ws2_32.dll
```

### Graphics

```bash
# Linux
/usr/lib/libGL.so
/usr/lib/libSDL2.so
/usr/lib/libglfw.so

# Windows
C:\Windows\System32\opengl32.dll
```

## Error Handling

```shard
use lib_loader

fn safe_load_library(path: String)
  let loader = LibraryLoader.new(path)
  try
    loader.load()
    say "Library loaded: #{path}"
  catch LibraryLoadError as e
    say "Failed to load #{path}: #{e.message}"
  ~
~

safe_load_library("/usr/lib/libm.so")
safe_load_library("/nonexistent/library.so")
```

## Best Practices

1. **Check Library Existence** - Verify library exists before loading
2. **Handle Errors** - Always catch loading errors
3. **Clean Up** - Unload libraries when done
4. **Document Dependencies** - List required libraries in your project

## Example: Complete Application

```shard
use lib_loader
use ai

fn main
  # Load AI library
  let ai_loader = LibraryLoader.new("./lib/ai_backend.so")
  ai_loader.load()
  
  # Load math library
  let math_loader = LibraryLoader.new("/usr/lib/libm.so")
  math_loader.load()
  
  # Use both libraries
  let session = OnnxSession.new("model.onnx")
  let result = session.predict([1.0, 2.0, 3.0])
  
  let math_result = call_function("sqrt", 144.0)
  
  say "AI Result: #{result}"
  say "Math Result: #{math_result}"
~

main()
```

---

**Author:** MelvinSGjr (MelvinMod)
