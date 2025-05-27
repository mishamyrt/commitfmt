# Footers

This file contains tests for git commit message footers. Footers are key-value pairs separated by colon.
They contain additional information about the commit, that can be useful for other tools.

In git terminology, footers are called trailers.

## References

- [git trailer config](https://github.com/git/git/blob/master/Documentation/config/trailer.adoc)
- [git-interpret-trailers documentation](https://git-scm.com/docs/git-interpret-trailers)

## Technical details

### Location

Footers are always placed at the end of the “meaningful part” of the message and are separated from the header or body by two line breaks `\n\n`. commitfmt is kinder in terms of parsing and can recognize the footer on the next line after the header.

<!--<test-case id="trailing-junk-breakage">-->
Footers must be in a row. If the end of the commit message looks like this:

<!--<test-input>-->
```
my cool feature

Footer-1: value-1
Footer-2: value-2
Not a footer
```

Then all footers is treated as part of the body.

<!--<test-result>-->
```toml
description = "my cool feature"
body = """

Footer-1: value-1
Footer-2: value-2
Not a footer
"""
footers = []
```
<!--</test-case>-->

#### Comments

<!--<test-case id="before-comments">-->
Since comments aren't part of the “meaningful part” of the post, if message has trailing comments right after the footers:

<!--<test-input>-->
```
my cool feature

Footer-1: value-1
# This is a comment
# This is another comment
```

they are ignored and footers are treated correctly.

<!--<test-result>-->
```toml
description = "my cool feature"

[[footers]]
key = "Footer-1"
value = "value-1"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

<!--<test-case id="after-comments">-->
Same goes for leading comments.

<!--<test-input>-->
```
my cool feature

# This is a comment
# This is another comment
Footer-1: value-1
```

<!--<test-result>-->
```toml
description = "my cool feature"

[[footers]]
key = "Footer-1"
value = "value-1"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

<!--<test-case id="no-newline">-->
Even if there is no empty line between header and footers. `git interpret-trailers` is discarding this case,
but commitfmt is treating it as a footer. 

<!--<test-input>-->
```
my cool feature
# This is a comment
Footer-1: value-1
```

<!--<test-result>-->
```toml
description = "my cool feature"

[[footers]]
key = "Footer-1"
value = "value-1"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

### Key

The key consists of letters and numbers and may include a dash `-` as a separator between words.
The case can be any, but it is recommended to write in `Train-Case` and `UPPER-TRAIN-CASE` for `BREAKING-CHANGES`.

### Separator

The separator can be any character with any number of spaces.
The default delimiter is a colon `:`.

git uses the `trailer.separators` config to define the available separators. A string of possible characters is written there.

## Cases

### Single

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

### Multiple 

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

### Multiline 

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

### After comments

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

### Before comments

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

### Right before comments

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

### With body after comments

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

### With body before comments

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
