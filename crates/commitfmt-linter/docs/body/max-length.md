# `body` `max-length`

Source: [src/rules/body/max_length.rs](../../src/rules/body/max_length.rs)

## What it does
Checks for long body.

## Why is this bad?
If feature or fix needs huge description, maybe it indicates something wrong.

## Example
```git-commit
feat: my feature

My super long body, which is longer than 72 characters and should be split into multiple lines
```

Use instead:
```git-commit
feat: my feature

My body
```
