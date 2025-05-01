# `body` `case`

Source: [src/rules/body/case.rs](../../src/rules/body/case.rs)

## What it does
Checks that the character case of the commit body is consistent

## Why is this bad?
A random case in a generated changelog may not look very pretty.

## Example
```git-commit
feat: my feature

my Feature IS SO COOL
```

Use instead:
```git-commit
feat: my feature

My feature is so cool
```
