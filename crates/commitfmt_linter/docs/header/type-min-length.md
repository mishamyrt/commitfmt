# `header` `type-min-length`

Source: [src/rules/header/kind/type_min_length.rs](../../src/rules/header/kind/type_min_length.rs)

## What it does
Checks for scope minimum length.

## Why is this bad?
Insufficient Scope can make it difficult to understand the domain of change.

## Example
```git-commit
tests
```

Use instead:
```git-commit
test: add cases
```
