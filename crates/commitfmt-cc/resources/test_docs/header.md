# Header

This file contains tests for conventional commit message headers. Header is the first line of the commit message that provides a concise description of the change.

## Structure

According to conventional commits specification, header has the following structure:

```
<type>(<scope>): <description>
```

Where:
- `type` - the type of change (required)
- `scope` - the area of change (optional)
- `description` - a brief description of the change (required)

## References

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [Angular Commit Message Format](https://github.com/angular/angular/blob/main/CONTRIBUTING.md#commit)

## Technical details

### Type

Type describes the kind of change being made.
Type must be written in lowercase and contain only letters, no numbers, special characters or spaces.

<!--<test-case id="type-feat">-->

<!--<test-input>-->
```
feat: add user authentication
```

<!--<test-result>-->
```toml
type = "feat"
description = "add user authentication"
```
<!--</test-case>-->

If the title does not contain a type, then all text is considered to be a description

<!--<test-case id="type-less">-->

<!--<test-input>-->
```
add user authentication
```

<!--<test-result>-->
```toml
description = "add user authentication"
```
<!--</test-case>-->

If type is incorrect, then all text is considered to be a description

<!--<test-case id="type-incorrect">-->

<!--<test-input>-->
```
feat ure123: add user authentication
```

<!--<test-result>-->
```toml
description = "feat ure123: add user authentication"
```
<!--</test-case>-->

### Scope

Scope is an optional part of the header that describes the area of change.
Scope must be written in lowercase and contain only letters, numbers, underscores and hyphens.

<!--<test-case id="scope-simple">-->

<!--<test-input>-->
```
feat(auth): add OAuth2 support
```

<!--<test-result>-->
```toml
type = "feat"
scope = ["auth"]
description = "add OAuth2 support"
```

<!--</test-case>-->

Scope name can contain dashes as word separators.

<!--<test-case id="scope-multiple-words">-->

<!--<test-input>-->
```
fix(user-profile): validate email format
```

<!--<test-result>-->
```toml
type = "fix"
scope = ["user-profile"]
description = "validate email format"
```
<!--</test-case>-->

Scope name can contain numbers.

<!--<test-case id="scope-numbers">-->

<!--<test-input>-->
```
feat(api-v2): implement new endpoints
```

<!--<test-result>-->
```toml
type = "feat"
scope = ["api-v2"]
description = "implement new endpoints"
```
<!--</test-case>-->

If a commit refers to multiple scopes, they are listed comma-separated.

<!--<test-case id="multiple-scopes">-->

<!--<test-input>-->
```
fix(auth, api): handle null pointer exception
```

<!--<test-result>-->
```toml
type = "fix"
scope = ["auth", "api"]
description = "handle null pointer exception"
```
<!--</test-case>-->

### Breaking changes

Breaking changes are indicated by the `!` suffix.

<!--<test-case id="breaking-changes">-->

<!--<test-input>-->
```
fix!: add new feature
```

<!--<test-result>-->
```toml
type = "fix"
description = "add new feature"
breaking = true
```
<!--</test-case>-->

If header contains scope and breaking changes, then scope must be placed before the breaking changes.

<!--<test-case id="breaking-changes-with-scope">-->

<!--<test-input>-->
```
fix(auth)!: add new feature
```

<!--<test-result>-->
```toml
type = "fix"
scope = ["auth"]
description = "add new feature"
breaking = true
```
<!--</test-case>-->
