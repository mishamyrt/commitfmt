# `body` `full-stop`

Source: [src/rules/body/full_stop.rs](../../src/rules/body/full_stop.rs)

## What it does
Checks for body ending with full stop

## Why is this bad?
Automatically generated changelogs can be hard to read
if the body not ends with a full stop.

## Example
```git-commit
feat: my feature

My feature is so cool. I can't even describe it
```

Use instead:
```git-commit
feat: my feature

My feature is so cool. I can't even describe it.
```
