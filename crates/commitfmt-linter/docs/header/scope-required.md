# `header` `scope-required`

Source: [src/rules/header/scope/scope_required.rs](../../src/rules/header/scope/scope_required.rs)

## What it does
Checks that the commit scope is exists.

## Why is this bad?
Insufficient Scope can make it difficult to understand the domain of change.

## Example
```git-commit
feat: my feature
```

Use instead:
```git-commit
feat(ui): my feature
```
