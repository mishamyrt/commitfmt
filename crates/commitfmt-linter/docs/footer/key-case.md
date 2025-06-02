# `footer` `key-case`

Source: [src/rules/footer/key_case.rs](../../src/rules/footer/key_case.rs)

## What it does
Checks that the character case of the footer keys is consistent

## Why is this bad?
Footer keys are used to provide additional metadata about a commit.
If you write them differently, automatic tools will not be able to match footers

## Example
```git-commit
feat: my feature

Fixes: #123
BreakingChange: removed API
Signed_off_by: John Doe
```

Use instead:
```git-commit
feat: my feature

fixes: #123
breaking-change: removed API
signed-off-by: John Doe
```
