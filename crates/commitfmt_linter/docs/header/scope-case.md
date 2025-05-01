# `header` `scope-case`

Source: [src/rules/header/scope/scope_case.rs](../../src/rules/header/scope/scope_case.rs)

## What it does
Checks that the character case of the commit scope is consistent

## Why is this bad?
Scopes are used to categorize commits into groups based on the domain of the change.
If you write them differently, automatic tools will not be able to match commits

## Example
```git-commit
feat(DB-Core, ui_core, reqInternal): my feature
```

Use instead:
```git-commit
feat(db-core, ui-core, req-internal): my feature
```
