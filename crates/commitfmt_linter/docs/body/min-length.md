# `body` `min-length`

Source: [src/rules/body/min_length.rs](../../src/rules/body/min_length.rs)

## What it does
Checks for short body.

## Why is this bad?
Short body can make it hard to understand.

## Example
```git-commit
feat: my feature
```

Use instead:
```git-commit
feat: my feature

My feature description
```
