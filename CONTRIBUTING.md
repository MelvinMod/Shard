# Contributing to Shard

Thank you for your interest in contributing to Shard!

## How to Contribute

### Reporting Bugs

1. Check if the bug already exists in issues
2. Provide clear reproduction steps
3. Include your environment (OS, compiler version, etc.)

### Suggesting Features

1. Open an issue with your feature suggestion
2. Describe the use case
3. Discuss the implementation approach

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests: `cargo test`
5. Run formatter: `cargo fmt`
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## Coding Standards

### Rust Code

- Follow Rust style guidelines
- Use `rustfmt` for formatting
- Write tests for new features
- Document public APIs

### Shard Code

- Use clear, descriptive names
- Keep functions small and focused
- Add comments for complex logic

## Development Setup

```bash
# Clone the repository
git clone https://github.com/MelvinMod/Shard.git
cd Shard

# Build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy
```

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --test integration
```

### Manual Testing

```bash
# Test a Shard program
cargo run -- compile examples/hello.shard -o /tmp/hello
/tmp/hello
```

## Code Review

- Be respectful and constructive
- Explain your reasoning
- Reference issues when applicable

## Questions?

Feel free to ask in the issues or join our discussion forum.

---

**Author:** MelvinSGjr (MelvinMod)
