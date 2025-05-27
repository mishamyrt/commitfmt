# `body` `leading-new-line`

Source: [src/rules/body/leading_newline.rs](../../src/rules/body/leading_newline.rs)

## What it does
Checks for missing newlines at the start of the body

## Why is this bad?
Missing newlines at the start of the body can make it hard to read and parse.

## Example
```git-commit
feat: my feature
body
```

Use instead:
```git-commit
feat: my feature

body
```
