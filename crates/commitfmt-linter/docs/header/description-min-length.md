# `header` `description-min-length`

Source: [src/rules/header/description/description_min_length.rs](../../src/rules/header/description/description_min_length.rs)

## What it does
Checks for short commit description.

## Why is this bad?
A description that is too short can hide the nature of what is happening in it.

## Example
```git-commit
test: add
```

Use instead:
```git-commit
test: add more cases for parser
```
