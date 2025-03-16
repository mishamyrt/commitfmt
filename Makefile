VERSION = 0.0.1-alpha.6

.PHONY: test
test:
	cargo test -- --nocapture

publish:
	@python3 ./scripts/update_version.py "$(VERSION)"
	cargo update -p commitfmt
	git add \
		Makefile \
		Cargo.lock \
		crates/commitfmt/Cargo.toml \
		packaging/npm
	git commit -m "chore: release v$(VERSION) 🔥"
	git tag -a v$(VERSION) -m "release v$(VERSION)"
	git push && git push --tags

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy
