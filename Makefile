.PHONY: test
test:
	cargo test

.PHONY: build
build:
	cargo build

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy
