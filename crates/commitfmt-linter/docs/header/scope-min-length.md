# `header` `scope-min-length`

Source: [src/rules/header/scope/scope_min_length.rs](../../src/rules/header/scope/scope_min_length.rs)

## What it does
Checks for scope minimum length.

## Why is this bad?
Insufficient Scope can make it difficult to understand the domain of change

## Example
```git-commit
feat(db, core): my feature
```

Use instead:
```git-commit
feat(db-core, ui-core): my feature
```
