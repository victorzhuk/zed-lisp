.PHONY: all build check clean fmt lint test check-package

WASM_TARGET = wasm32-wasip1

# Bounded verification (change: add-lispico-development-support). Test
# commands get a finite wall-clock limit and explicit worker caps; the same
# limits apply locally and in CI. A failing test or a timeout both fail the
# wrapper.
CARGO_BUILD_JOBS ?= 4
RUST_TEST_THREADS ?= 4
BUILD_TIMEOUT_SECONDS ?= 300
TEST_TIMEOUT_SECONDS ?= 300
TIMEOUT ?= timeout

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

test: check-package
	CARGO_BUILD_JOBS=$(CARGO_BUILD_JOBS) $(TIMEOUT) $(BUILD_TIMEOUT_SECONDS)s \
		cargo build --tests
	CARGO_BUILD_JOBS=$(CARGO_BUILD_JOBS) $(TIMEOUT) $(TEST_TIMEOUT_SECONDS)s \
		env RUST_TEST_THREADS=$(RUST_TEST_THREADS) cargo test -- --test-threads=$(RUST_TEST_THREADS)

check-package:
	@python3 scripts/check_package.py

install-dev: build
	@echo "Build complete. Install in Zed with:"
	@echo "  1. Open Zed"
	@echo "  2. Cmd/Ctrl+Shift+P → 'Install Dev Extension'"
	@echo "  3. Select this directory"
