.PHONY: all build check clean fmt lint test acceptance check-package install-dev

WASM_TARGET = wasm32-wasip2

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

# Opt-in real-server acceptance suite (tests/llsp_stdio.rs): drives a real
# llsp binary over stdio, selected by LLSP_BINARY or `llsp` on PATH. Kept
# out of `make test` so the ordinary run stays hermetic; the historical
# detector is invoked separately with LLSP_HISTORICAL_BINARY set.
acceptance:
	$(TIMEOUT) $(TEST_TIMEOUT_SECONDS)s \
		cargo test --test llsp_stdio -- --ignored --skip historical_language_id_regression_detected

check-package:
	@python3 scripts/check_package.py

install-dev: build
	@echo "Build complete. Install in Zed with:"
	@echo "  1. Open Zed"
	@echo "  2. Cmd/Ctrl+Shift+P → 'Install Dev Extension'"
	@echo "  3. Select this directory"
