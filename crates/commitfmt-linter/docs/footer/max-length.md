# `footer` `max-length`

Source: [src/rules/footer/max_length.rs](../../src/rules/footer/max_length.rs)

## What it does
Checks for too long footers.

## Why is this bad?
If the footer contains a lot of information, something probably
didn't go according to plan. Maybe it should be in the body?

## Example
```git-commit
feat: my feature

BREAKING CHANGES: I had to heavily rework several modules.
Compatibility of TreeView and Card components may be broken
due to the library update.
```

Use instead:
```git-commit
feat: my feature

I had to heavily rework several modules. Compatibility
of TreeView and Card components may be broken due
to the library update.

BREAKING CHANGES: TreeView and Card APIs
```
