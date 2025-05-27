# `header` `description-max-length`

Source: [src/rules/header/description/description_max_length.rs](../../src/rules/header/description/description_max_length.rs)

## What it does
Checks for description maximum length.

## Why is this bad?
Long description will be truncated when displayed in the logs.

## Example
```git-commit
feat: my feature description where i added some bugs and fixed some others which are longer than 72 characters
```

Use instead:
```git-commit
feat: my feature description
```
