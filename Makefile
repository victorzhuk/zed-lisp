.PHONY: all build check clean fmt lint test install-dev

WASM_TARGET = wasm32-wasip1

all: build

build:
	cargo build --release --target $(WASM_TARGET)

check:
	cargo check

clean:
	cargo clean
	$(RM) -rf target/

fmt:
	cargo fmt

lint:
	cargo clippy --target $(WASM_TARGET) -- -D warnings

test:
	cargo test

install-dev: build
	@echo "Build complete. Install in Zed with:"
	@echo "  1. Open Zed"
	@echo "  2. Cmd/Ctrl+Shift+P → 'Install Dev Extension'"
	@echo "  3. Select this directory"
