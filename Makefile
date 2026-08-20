VERSION = 1.2.0
COMPARISON_BENCHMARK_DIR = crates/commitfmt-benchmark/comparison
COMMITLINT_BIN = $(COMPARISON_BENCHMARK_DIR)/node_modules/.bin/commitlint

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

$(COMMITLINT_BIN): $(COMPARISON_BENCHMARK_DIR)/package.json $(COMPARISON_BENCHMARK_DIR)/package-lock.json
	npm --prefix "$(COMPARISON_BENCHMARK_DIR)" ci

.PHONY: benchmark-comparison
benchmark-comparison: $(COMMITLINT_BIN)
	cargo build --profile dist -p commitfmt
	cargo bench -p commitfmt-benchmark --bench comparison --features comparison-benchmark

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy --all
