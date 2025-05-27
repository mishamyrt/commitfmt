# Example of test.md file

Act like a senior rust programmer. Write memory efficient and fail safe parser of conventional commits message. Don't use regex or other slow methods. Maybe good idea will be to write custom lexer.

Rules are following:

- First line of message is header. If header starts with single word and colon, then word should be thought of type.
- Before the colon can be scope. It strings separated by comma.
- After the colon can be description.
- If header format is wrong (starts with something else), then all first line is description.
- After the description can be body. Body can be multiple lines.
- After the body can be footers. Footers are key-value pairs separated by colon. Key MUST be dash separated string or `BREAKING CHANGES`. Value is string.
- Footer's key can't contain colon or space. If footer's key contains colon or space, then it (and all previous footers) is body.
- If there space before colon, then it should be removed from key.


### Example 1

```git-commit
feat: my feature

Description body

BREAKING CHANGES: some breaking changes
```

```toml
type = feat
scope = []
description = "my feature"
body = "\nDescription body"
footers = [{
  key = "BREAKING CHANGES"
  value = "some breaking changes"
}]
```

### Example 2

```commit
i am represent : my : cool : feature

Description body

Fixes #1

Fi xes: #1

Fixes: #1
```

```toml
# type = None
scope = []
description = "i am represent : my : cool : feature"
body = """

Description body
Fixes #1

Fi xes: #1"""
footers = [
  {
    key = "Fixes"
    value = "#1"
  }
]
```

### Example 2

```commit
fix (scope1,scope2) !: my fix

Fixes #1

Fixes: #1

Fi xes: #1

Fixes: #2
BREAKING CHANGES: some breaking changes
```

```toml
# type = None
scope = ["scope1", "scope2"]
description = "i am represent : my : cool : feature"
body = """

Fixes #1

Fixes: #1

Fi xes: #1"""
footers = [
  {
    key = "Fixes"
    value = "#2"
  },
  {
    key = "BREAKING CHANGES"
    value = "some breaking changes"
  }
]
```

### Multiline Footers

```git-commit
fix (scope1,scope2) !: my fix

Fixes #1
Fixes: #1
Fi xes: #1
Signed-By: #2

BREAKING CHANGES: some breaking changes
so many info that it should be in multiple lines
Ticket-ID: PRJ-123
Reviewed-by: Me
```

```toml
# type = None
scope = ["scope1", "scope2"]
description = "i am represent : my : cool : feature"
body = """

Fixes #1

Fixes: #1

Fi xes: #1"""
footers = [
  {
    key = "Fixes"
    value = "#2"
  },
  {
    key = "BREAKING CHANGES"
    value = "some breaking changes"
  }
]
```
