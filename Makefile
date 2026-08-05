.PHONY: help check build test fmt lint install run

help:
	@echo "Available targets:"
	@echo "  make check    - Run formatting, linting, and tests"
	@echo "  make build    - Build the project"
	@echo "  make test     - Run tests"
	@echo "  make fmt      - Format Rust sources"
	@echo "  make lint     - Run Clippy with warnings denied"
	@echo "  make install  - Install/update the global llme binary"
	@echo "  make run      - Run llme locally; pass args with ARGS='...'"

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all-features

build:
	cargo build

test:
	cargo test --all-features

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

install:
	cargo install --path . --force

run:
	cargo run -- $(ARGS)
