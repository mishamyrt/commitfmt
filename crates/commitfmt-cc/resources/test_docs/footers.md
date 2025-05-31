# Footers

This file contains tests for git commit message footers. Footers are key-value pairs separated by colon.
They contain additional information about the commit, that can be useful for other tools.

In git terminology, footers are called trailers.

## References

- [git trailer config](https://github.com/git/git/blob/master/Documentation/config/trailer.adoc)
- [git-interpret-trailers documentation](https://git-scm.com/docs/git-interpret-trailers)

## Technical details

### Location

Footers are always placed at the end of the "meaningful part" of the message and are separated from the header or body by two line breaks `\n\n`. commitfmt is kinder in terms of parsing and can recognize the footer on the next line after the header.

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

### Comments

<!--<test-case id="before-comments">-->
Since comments aren't part of the "meaningful part" of the post, if message has trailing comments right after the footers:

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

Comment symbol can be altered by git config.

<!--<test-case id="comment-symbol">-->

<!--<test-input-params>-->
```toml
comment-symbol = "//"
```

<!--<test-input>-->
```
my cool feature

// This is a comment
// This is another comment
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

<!--<test-case id="key-letters-numbers">-->

<!--<test-input>-->
```
add new feature

Footer-123: value
ABC-456: another value
```

<!--<test-result>-->
```toml
description = "add new feature"

[[footers]]
key = "Footer-123"
value = "value"
separator = ":"
alignment = "left"

[[footers]]
key = "ABC-456"
value = "another value"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

<!--<test-case id="key-with-dashes">-->
Keys can contain dashes as word separators.

<!--<test-input>-->
```
fix bug

Co-Authored-By: John Doe <john@example.com>
Reviewed-By: Jane Smith <jane@example.com>
```

<!--<test-result>-->
```toml
description = "fix bug"

[[footers]]
key = "Co-Authored-By"
value = "John Doe <john@example.com>"
separator = ":"
alignment = "left"

[[footers]]
key = "Reviewed-By"
value = "Jane Smith <jane@example.com>"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

#### Case

The case can be any, but it is recommended to write in `Train-Case` and `UPPER-TRAIN-CASE` for `BREAKING-CHANGES`

<!--<test-case id="key-train-case">-->

<!--<test-input>-->
```
implement feature

Signed-Off-By: Developer <dev@example.com>
Related-To: #123
```

<!--<test-result>-->
```toml
description = "implement feature"

[[footers]]
key = "Signed-Off-By"
value = "Developer <dev@example.com>"
separator = ":"
alignment = "left"

[[footers]]
key = "Related-To"
value = "#123"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

<!--<test-case id="key-upper-train-case">-->

<!--<test-input>-->
```
API endpoint removed

SECURITY-FIX: vulnerability patched
```

<!--<test-result>-->
```toml
description = "API endpoint removed"

[[footers]]
key = "SECURITY-FIX"
value = "vulnerability patched"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

<!--<test-case id="key-mixed-case">-->

<!--<test-input>-->
```
update dependencies

camelCase: value1
PascalCase: value2
lowercase: value3
UPPERCASE: value4
```

<!--<test-result>-->
```toml
description = "update dependencies"

[[footers]]
key = "camelCase"
value = "value1"
separator = ":"
alignment = "left"

[[footers]]
key = "PascalCase"
value = "value2"
separator = ":"
alignment = "left"

[[footers]]
key = "lowercase"
value = "value3"
separator = ":"
alignment = "left"

[[footers]]
key = "UPPERCASE"
value = "value4"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

#### Breaking changes

The only exception to the key naming rules is `BREAKING CHANGES`. This key contains a space as a delimiter, but will still be processed correctly

<!--<test-case id="breaking-changes-variants">-->

<!--<test-input>-->
```
major update

Breaking-Changes: removed deprecated API
BreakingChanges: changed response format
BREAKING CHANGES: updated authentication method
```

<!--<test-result>-->
```toml
description = "major update"

[[footers]]
key = "Breaking-Changes"
value = "removed deprecated API"
separator = ":"
alignment = "left"

[[footers]]
key = "BreakingChanges"
value = "changed response format"
separator = ":"
alignment = "left"

[[footers]]
key = "BREAKING CHANGES"
value = "updated authentication method"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

### Separators

The separator is a character that separates the key from the value. The default separator in git trailer is the colon character `:`. 

<!--<test-case id="default-colon-separator">-->

<!--<test-input-params>-->
```toml
separators = ":"
```

<!--<test-input>-->
```
fix issue

Signed-off-by: Developer <dev@example.com>
```

<!--<test-result>-->
```toml
description = "fix issue"

[[footers]]
key = "Signed-off-by"
value = "Developer <dev@example.com>"
separator = ":"
alignment = "left"
```
<!--</test-case>-->

Git supports multiple separators at once.

<!--<test-case id="multiple-separators">-->

<!--<test-input-params>-->
```toml
separators = ":="
```

<!--<test-input>-->
```
fix issue

Signed-off-by: Developer <dev@example.com>
Issue-ID= 123
```

<!--<test-result>-->
```toml
description = "fix issue"

[[footers]]
key = "Signed-off-by"
value = "Developer <dev@example.com>"
separator = ":"
alignment = "left"

[[footers]]
key = "Issue-ID"
value = "123"
separator = "="
alignment = "left"
```

<!--</test-case>-->

Separator can be left or right aligned.

<!--<test-case id="multiple-separators">-->

<!--<test-input-params>-->
```toml
separators = ":#"
```

<!--<test-input>-->
```
fix issue

Signed-off-by: Developer <dev@example.com>
Issue-ID #123
```

<!--<test-result>-->
```toml
description = "fix issue"

[[footers]]
key = "Signed-off-by"
value = "Developer <dev@example.com>"
separator = ":"
alignment = "left"

[[footers]]
key = "Issue-ID"
value = "123"
separator = "#"
alignment = "right"
```

<!--</test-case>-->
