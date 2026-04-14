.PHONY: help build test install run

help:
	@echo "Available targets:"
	@echo "  make build    - Build the project"
	@echo "  make test     - Run tests"
	@echo "  make install  - Install/update the global llme binary"
	@echo "  make run      - Run llme locally; pass args with ARGS='...'"

build:
	cargo build

test:
	cargo test

install:
	cargo install --path . --force

run:
	cargo run -- $(ARGS)
