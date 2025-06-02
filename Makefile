VERSION = 0.4.0

.PHONY: setup
setup:
# TODO: remove dependency on Go
	@go install github.com/evilmartians/lefthook@latest

	lefthook install

.PHONY: test
test:
	cargo nextest run

.PHONY: test-coverage
test-coverage:
	cargo llvm-cov --html nextest

publish:
	@python3 ./scripts/update_version.py "$(VERSION)"
	@cargo update -p commitfmt
	@git add \
		Makefile \
		Cargo.lock \
		crates/commitfmt/Cargo.toml \
		packaging/npm \
		packaging/pypi
	@git commit -m "chore: release v$(VERSION) 🔥"
	@git tag v$(VERSION)
	@git-cliff -o CHANGELOG.md
	@git tag -d v$(VERSION)
	@git add CHANGELOG.md
	@git commit --amend --no-edit
	@git tag -a v$(VERSION) -m "release v$(VERSION)"
	@git push
	@git push --tags

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
	cargo clippy --all
