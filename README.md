# commitfmt [![Quality Assurance](https://github.com/mishamyrt/commitfmt/actions/workflows/qa.yaml/badge.svg)](https://github.com/mishamyrt/commitfmt/actions/workflows/qa.yaml)

Utility for formatting and verifying the commit message.

It's not a linter. At least not a complete replacement for [commitlint](https://commitlint.js.org), because commitfmt can't prevent you from writing a body or force you to write a description in uppercase (I don't know why you might want to do that), but it will help keep the story high quality.

By design, commitfmt runs on the `prepare-commit-msg` hook and formats the message according to git standards and [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) in particular.

A message like this:

```
feat ( scope     ,    scope  )  : add new feature.
body description
```

Will be formatted to:

```
feat(scope, scope): add new feature

body description
```

Additionally, you can customize checks, such as limiting the list of available types and scopes. To do this, create a [configuration file](#configuration).

## Installation

### pnpm

```bash
pnpm add --save-dev commitfmt
```

### npm

```bash
npm install --save-dev commitfmt
```

### yarn

```bash
yarn add --dev commitfmt
```

### pip

```bash
pip install commitfmt
```

## Configuration

### TOML

Create a `commitfmt.toml` or (`.commitfmt.toml`) file in the root of your project.

```toml
[lint.body]
full-stop = false

[lint.header]
scope-case = "lower"
scope-enum = ["cc", "config", "git", "linter"]
type-enum = ["build", "chore", "ci", "docs", "feat", "fix", "perf", "refactor", "revert", "style", "test"]
```
