# `body` `max-line-length`

Source: [src/rules/body/max_line_length.rs](../../src/rules/body/max_line_length.rs)

## What it does
Checks for long body lines.

## Why is this bad?
Long body lines can make it hard to read and parse.

## Example
```git-commit
feat: my feature

My super long body, which is longer than 72 characters and should be split into multiple lines
```

Use instead:
```git-commit
feat: my feature

My super long body, which is longer than 72 characters
and should be split into multiple lines
```
