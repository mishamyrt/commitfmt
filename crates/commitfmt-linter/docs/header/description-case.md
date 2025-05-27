# `header` `description-case`

Source: [src/rules/header/description/description_case.rs](../../src/rules/header/description/description_case.rs)

## What it does
Checks that the character case of the commit description is consistent

## Why is this bad?
The commit description is primarily used by automated tools to generate
the changelog so it is important that the descriptions are consistent

## Example
```git-commit
feat: My feature
```

Use instead:
```git-commit
feat: my feature
```
