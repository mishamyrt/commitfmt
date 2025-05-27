# `footer` `breaking-exclamation`

Source: [src/rules/footer/breaking_exclamation.rs](../../src/rules/footer/breaking_exclamation.rs)

## What it does
Checks for the presence of a flag (exclamation mark)
in a message containing `BREAKING CHANGES`.

## Why is this bad?
Some utilities may not check commit footers and count on the presence of an exclamation mark.
And they would be right

## Example
```git-commit
feat: my super feature

BREAKING CHANGES: some breaking changes
```

Use instead:
```git-commit
feat!: my super feature

BREAKING CHANGES: some breaking changes
```
