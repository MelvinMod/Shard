# Project 3: Make a Simple Website

## What You Will Make

A simple web server that shows pages.

## What is a Web Server?

A web server is a program that:
1. Waits for requests
2. Sends back pages

## Step 1: Start a Server

```shard
use http

entity Server
  field port
  
  func new(port)
    @port = port
  ~
  
  func start
    say "Server starting on port #{@port}"
    # Server code here
  ~
~

let server = Server.new(8080)
server.start()
```

## Step 2: Handle Requests

```shard
use http

entity Server
  field port
  
  func new(port)
    @port = port
  ~
  
  func start
    say "Server starting on port #{@port}"
    
    server = HTTP::Server.new do |request, response|
      let path = request.path
      
      when path == "/"
        response.content_type = "text/html"
        response.print "<h1>Welcome to Shard!</h1>"
      else
        response.status_code = 404
        response.print "Page not found"
      ~
    ~
    
    server.bind_tcp("localhost", @port)
    server.listen
  ~
~

let server = Server.new(8080)
server.start()
```

## Step 3: Multiple Pages

```shard
use http

entity Server
  field port
  
  func new(port)
    @port = port
  ~
  
  func start
    say "Server starting on port #{@port}"
    
    server = HTTP::Server.new do |request, response|
      let path = request.path
      
      when path == "/"
        response.content_type = "text/html"
        response.print """
<!DOCTYPE html>
<html>
<head>
  <title>Home</title>
</head>
<body>
  <h1>Welcome to My Website</h1>
  <p>This is the home page</p>
  <a href="/about">About</a>
  <a href="/contact">Contact</a>
</body>
</html>
"""
      when path == "/about"
        response.content_type = "text/html"
        response.print """
<!DOCTYPE html>
<html>
<head>
  <title>About</title>
</head>
<body>
  <h1>About Us</h1>
  <p>We make websites with Shard</p>
  <a href="/">Home</a>
</body>
</html>
"""
      when path == "/contact"
        response.content_type = "text/html"
        response.print """
<!DOCTYPE html>
<html>
<head>
  <title>Contact</title>
</head>
<body>
  <h1>Contact Us</h1>
  <p>Email: info@example.com</p>
  <a href="/">Home</a>
</body>
</html>
"""
      else
        response.status_code = 404
        response.print "<h1>404 - Page Not Found</h1>"
      ~
    ~
    
    server.bind_tcp("localhost", @port)
    server.listen
  ~
~

let server = Server.new(8080)
server.start()
```

## Step 4: API Endpoints

```shard
use http

entity Server
  field port
  
  func new(port)
    @port = port
  ~
  
  func start
    say "Server starting on port #{@port}"
    
    server = HTTP::Server.new do |request, response|
      let path = request.path
      
      when path == "/api/time"
        response.content_type = "application/json"
        response.print """{"time": "12:00:00"}"""
      when path == "/api/health"
        response.content_type = "application/json"
        response.print """{"status": "ok"}"""
      when path == "/api/message"
        response.content_type = "application/json"
        response.print """{"message": "Hello from server!"}"""
      else
        response.status_code = 404
        response.print """{"error": "Not found"}"""
      ~
    ~
    
    server.bind_tcp("localhost", @port)
    server.listen
  ~
~

let server = Server.new(8080)
server.start()
```

## Complete Website Example

```shard
use http

entity Website
  field port
  
  func new(port)
    @port = port
  ~
  
  func start
    say "=== My Website ==="
    say "Opening http://localhost:#{@port}"
    
    server = HTTP::Server.new do |request, response|
      let path = request.path
      
      when path == "/"
        response.content_type = "text/html"
        response.print """
<!DOCTYPE html>
<html>
<head>
  <title>My Website</title>
  <style>
    body { font-family: Arial; max-width: 800px; margin: 50px auto; }
    h1 { color: #333; }
    nav a { margin-right: 15px; }
  </style>
</head>
<body>
  <h1>Welcome to My Website</h1>
  <nav>
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/api">API</a>
  </nav>
  <p>This website is made with Shard!</p>
</body>
</html>
"""
      when path == "/about"
        response.content_type = "text/html"
        response.print """
<!DOCTYPE html>
<html>
<head>
  <title>About</title>
</head>
<body>
  <h1>About This Website</h1>
  <p>Made with Shard programming language</p>
  <a href="/">Back to Home</a>
</body>
</html>
"""
      when path == "/api"
        response.content_type = "application/json"
        response.print """
{
  "endpoints": [
    "/api/time",
    "/api/health",
    "/api/message"
  ]
}
"""
      else
        response.status_code = 404
        response.print "<h1>404 - Not Found</h1>"
      ~
    ~
    
    server.bind_tcp("localhost", @port)
    server.listen
  ~
~

let website = Website.new(8080)
website.start()
```

## How to Use

1. Run the server
2. Open browser to http://localhost:8080
3. Click links to see different pages

## What You Learned

1. Web servers wait for requests
2. You can serve HTML pages
3. You can serve JSON for APIs
4. Different paths show different content

## Make It Better

1. Add more pages
2. Add forms
3. Add database
4. Add user login

---

**Congratulations!** You made a website!

You now know how to make:
- Games ✓
- AI ✓
- Websites ✓

You are a complete programmer!
