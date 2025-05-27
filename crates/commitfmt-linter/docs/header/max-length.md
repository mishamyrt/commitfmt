# `header` `max-length`

Source: [src/rules/header/max_length.rs](../../src/rules/header/max_length.rs)

## What it does
Checks for long header.

## Why is this bad?
Long commit messages will be truncated when displayed in the logs.

## Example
```git-commit
feat: my super feature with description which is longer than 72 characters and should be split into multiple lines
```

Use instead:
```git-commit
feat: my super feature

Description which is longer than 72 characters
and should be split into multiple lines.
```
