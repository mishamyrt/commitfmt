# `header` `type-max-length`

Source: [src/rules/header/kind/type_max_length.rs](../../src/rules/header/kind/type_max_length.rs)

## What it does
Checks for scope maximum length.

## Why is this bad?
While Scopes are useful, they take up space in the header,
taking it away from the description.

## Example
```git-commit
feat(db-core, ui-core, ui-widgets, db-internal): my feature
```

Use instead:
```git-commit
feat(db, ui): my feature
```
