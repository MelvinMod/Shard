.PHONY: build clean test check format run

BUILD_DIR := target/release
SHARD_BIN := $(BUILD_DIR)/shardc

.PHONY: all
all: build

build:
	cargo build --release

clean:
	cargo clean

test:
	cargo test

check:
	cargo check

format:
	cargo fmt

lint:
	cargo clippy

run: build
	$(SHARD_BIN) run examples/hello.shard

install: build
	install -m 755 $(SHARD_BIN) /usr/local/bin/shardc

uninstall:
	rm -f /usr/local/bin/shardc

help:
	@echo "Shard Compiler Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build     - Build the compiler (release)"
	@echo "  clean     - Remove build artifacts"
	@echo "  test      - Run all tests"
	@echo "  check     - Check for errors without building"
	@echo "  format    - Format code with rustfmt"
	@echo "  lint      - Run clippy linter"
	@echo "  run       - Run hello example"
	@echo "  install   - Install to /usr/local/bin"
	@echo "  uninstall - Remove from /usr/local/bin"
	@echo "  help      - Show this help"
