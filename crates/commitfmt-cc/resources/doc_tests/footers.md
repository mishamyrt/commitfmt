# Footers

This file contains tests for git commit message footers. Footers are key-value pairs separated by colon.
They contain additional information about the commit, that can be useful for other tools.

In git terminology, footers are called trailers.

## References

- [git trailer config](https://github.com/git/git/blob/master/Documentation/config/trailer.adoc)
- [git-interpret-trailers documentation](https://git-scm.com/docs/git-interpret-trailers)

## Technical details

### Position and grouping

<!-- <DOC_TEST> -->
Footers can only be at the end of a post and must be in a row. If the end of the post is something like:

```git-commit
feat: my cool feature

Footer-1: value-1
Footer-2: value-2
Not a footer
```

Then all footers should be treated as part of the body.

```toml
body = """
Footer-1: value-1
Footer-2: vale-2
Not a footer"""
footers = []
```
<!-- </DOC_TEST> -->

### Key

The key consists of letters and may include a dash `-` as a separator between words.
The case can be any, but it is recommended to write in Pascal-Kebab-Case.

### Separator

The separator can be any character with any number of spaces.
The default delimiter is a colon `:`.

git uses the `trailer.separators` config to define the available separators. A string of possible characters is written there.

## Cases

### Single

<!-- <DOC_TEST> -->

```git-commit
my cool feature

Authored-By: Co Mitter <comitter@example.com>
```

```toml
description = "my cool feature"
footers = [
  { key = "Authored-By", value = "Co Mitter <comitter@example.com>" }
]
```
<!-- </DOC_TEST> -->

### Multiple 

<!-- <DOC_TEST> -->
```git-commit
my cool feature

Authored-By: Co Mitter <comitter@example.com>
Reviewed-By: Re Viewer <reviewer@example.com>
```

```toml
description = "my cool feature"
footers = [
  { key = "Authored-By", value = "Co Mitter <comitter@example.com>" },
  { key = "Reviewed-By", value = "Re Viewer <reviewer@example.com>" }
]
```
<!-- </DOC_TEST> -->

### Multiline 

<!-- <DOC_TEST> -->
```git-commit
my cool feature

Authored-By: Co Mitter <comitter@example.com>
Multiline-Details: First
 Second
 Third
```

```toml
description = "my cool feature"
footers = [
  { key = "Authored-By", value = "Co Mitter <comitter@example.com>" },
  { key = "Multiline-Details", value = "First\nSecond\nThird" }
]
```
<!-- </DOC_TEST> -->

### After comments

<!-- <DOC_TEST> -->
```git-commit
my cool feature

# This is a comment
# This is another comment

Authored-By: Co Mitter <comitter@example.com>
```

```toml
body = "\n# This is a comment\n# This is another comment"
description = "my cool feature"
footers = [
  { key = "Authored-By", value = "Co Mitter <comitter@example.com>" }
]
```
<!-- </DOC_TEST> -->

### Before comments

<!-- <DOC_TEST> -->
```git-commit
my cool feature

Authored-By: Co Mitter <comitter@example.com>

# This is a comment
# This is another comment
```

```toml
description = "my cool feature"
footers = [
  { key = "Authored-By", value = "Co Mitter <comitter@example.com>" }
]
```
<!-- </DOC_TEST> -->

### Right before comments

<!-- <DOC_TEST> -->
```git-commit
my cool feature

Authored-By: Co Mitter <comitter@example.com>
# This is a comment
# This is another comment
```

```toml
description = "my cool feature"
footers = [
  { key = "Authored-By", value = "Co Mitter <comitter@example.com>" }
]
```
<!-- </DOC_TEST> -->

### With body after comments

<!-- <DOC_TEST> -->
```git-commit
my cool feature

# This is a comment
# This is another comment

Body content

Authored-By: Co Mitter <comitter@example.com>
```

```toml
description = "my cool feature"
body = "\n# This is a comment\n# This is another comment\n\nBody content"
footers = [
  { key = "Authored-By", value = "Co Mitter <comitter@example.com>" }
]
```
<!-- </DOC_TEST> -->

### With body before comments

<!-- <DOC_TEST> -->
```git-commit
my cool feature

Body content

# This is a comment
# This is another comment

Authored-By: Co Mitter <comitter@example.com>
```

```toml
description = "my cool feature"
body = "\nBody content\n\n# This is a comment\n# This is another comment"
footers = [
  { key = "Authored-By", value = "Co Mitter <comitter@example.com>" }
]
```
<!-- </DOC_TEST> -->
