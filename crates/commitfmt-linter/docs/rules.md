# Linter Rules

## `body`

| Rule | Message | Fix Mode |
|------|---------|----------|
| [`case`](body/case.md) | Body case is inconsistent. Expected: `{case}` | Unfixable |
| [`full-stop`](body/full-stop.md) | Body is not ended with a full stop | Unsafe |
| [`max-length`](body/max-length.md) | Body is longer than `{max_length}` characters | Unfixable |
| [`max-line-length`](body/max-line-length.md) | Body line is longer than `{max_length}` characters | Unfixable |
| [`min-length`](body/min-length.md) | Body is shorter than `{length}` characters | Unfixable |

## `footer`

| Rule | Message | Fix Mode |
|------|---------|----------|
| [`breaking-exclamation`](footer/breaking-exclamation.md) | Message contains breaking changes footer but no exclamation mark | Safe |
| [`exists`](footer/exists.md) | Footer '`{key}`' is required but not found | Unfixable |
| [`key-case`](footer/key-case.md) | Footer key case is inconsistent. Expected: `{case}` | Unfixable |
| [`max-length`](footer/max-length.md) | Footer '`{key}`' length is longer than `{length}` characters | Unfixable |
| [`max-line-length`](footer/max-line-length.md) | Footer '`{key}`' contains a line that length is longer than `{length}` characters | Unfixable |
| [`min-length`](footer/min-length.md) | Footer '`{key}`' length is less than `{length}` characters | Unfixable |

## `header`

| Rule | Message | Fix Mode |
|------|---------|----------|
| [`description-case`](header/description-case.md) | Description case is inconsistent. Expected: `{case}` | Unfixable |
| [`description-full-stop`](header/description-full-stop.md) | Header description is ended with a full stop | Safe |
| [`description-max-length`](header/description-max-length.md) | Description is longer than `{length}` characters | Unfixable |
| [`description-min-length`](header/description-min-length.md) | Description is shorter than `{length}` characters | Unfixable |
| [`max-length`](header/max-length.md) | Header is longer than `{max_length}` characters | Unfixable |
| [`min-length`](header/min-length.md) | Header is shorter than `{length}` characters | Unfixable |
| [`scope-case`](header/scope-case.md) | Scope case is inconsistent. Expected: `{case}` | Unfixable |
| [`scope-enum`](header/scope-enum.md) | Scope is not allowed: `{miss}` | Unfixable |
| [`scope-max-length`](header/scope-max-length.md) | Scope is longer than `{length}` characters | Unfixable |
| [`scope-min-length`](header/scope-min-length.md) | Scope is shorter than `{length}` characters | Unfixable |
| [`scope-required`](header/scope-required.md) | Scope is required | Unfixable |
| [`type-case`](header/type-case.md) | Type case is inconsistent. Expected: `{case}` | Unfixable |
| [`type-enum`](header/type-enum.md) | Type is not allowed: `{miss}` | Unfixable |
| [`type-max-length`](header/type-max-length.md) | Type is longer than `{length}` characters | Unfixable |
| [`type-min-length`](header/type-min-length.md) | Type is shorter than `{length}` characters | Unfixable |
| [`type-required`](header/type-required.md) | Commit type is required | Unfixable |

