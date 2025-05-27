# `footer` `exists`

Source: [src/rules/footer/exists.rs](../../src/rules/footer/exists.rs)

## What it does
Checks that footer exists.

## Why is this bad?
Automated tools may require certain footers and their absence can break processes.

## Example
```git-commit
feat: my feature
```

Use instead:
```git-commit
feat: my feature

Issue: PRJ-123
```
