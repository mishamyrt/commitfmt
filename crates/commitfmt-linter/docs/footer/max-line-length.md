# `footer` `max-line-length`

Source: [src/rules/footer/max_line_length.rs](../../src/rules/footer/max_line_length.rs)

## What it does
Checks for too long lines in footers.

## Why is this bad?
Lines that are too long may not look good in the limited space of the terminal.

## Example
```git-commit
feat: my feature

BREAKING CHANGES: I had to heavily rework several modules. Compatibility of TreeView and Card components may be broken due to the library update.
```

Use instead:
```git-commit
feat: my feature

BREAKING CHANGES: I had to heavily rework several modules.
Compatibility of TreeView and Card components may be broken
due to the library update.
```
