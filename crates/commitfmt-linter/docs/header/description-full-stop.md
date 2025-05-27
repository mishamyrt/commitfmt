# `header` `description-full-stop`

Source: [src/rules/header/description/description_full_stop.rs](../../src/rules/header/description/description_full_stop.rs)

## What it does
Checks for header not ending with full stop

## Why is this bad?
Automatically generated changelogs can be hard to read
if the header ends with a full stop.

## Example
```git-commit
feat: my feature.
```

Use instead:
```git-commit
feat: my feature
```
