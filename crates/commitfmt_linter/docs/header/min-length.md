# `header` `min-length`

Source: [src/rules/header/min_length.rs](../../src/rules/header/min_length.rs)

## What it does
Checks for short header.

## Why is this bad?
A commit header that is too short can hide the nature of what is happening in it.

## Example
```git-commit
test: add
```

Use instead:
```git-commit
test: add more cases for parser
```
