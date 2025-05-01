# `header` `type-required`

Source: [src/rules/header/kind/type_required.rs](../../src/rules/header/kind/type_required.rs)

## What it does
Checks that the commit type is exists

## Why is this bad?
The commit type is necessary for utilities analyzing git logs.
Its absence will prevent them from assigning the commit to a certain group.

## Example
```git-commit
my feature
```

Use instead:
```git-commit
feat: my feature
```
