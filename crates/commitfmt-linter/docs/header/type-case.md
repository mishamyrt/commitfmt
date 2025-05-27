# `header` `type-case`

Source: [src/rules/header/kind/type_case.rs](../../src/rules/header/kind/type_case.rs)

## What it does
Checks that the character case of the commit type is consistent

## Why is this bad?
Type is a completely technical field. Different spellings of the same type
can confuse automatic documentation generation utilities.

## Example
```git-commit
Feat: my feature
```

Use instead:
```git-commit
feat: my feature
```
