# Header

This file contains tests for git commit message header.
The header is the first line of the commit, which necessarily contains a description,
and can also include type and scope.

> Though not required, it’s a good idea to begin the commit message with a single short
> (no more than 50 characters) line summarizing the change, followed by a blank line
> and then a more thorough description.
> The text up to the first blank line in a commit message is treated as the commit title,
> and that title is used throughout Git.

## References

- [conventional commits summary](https://www.conventionalcommits.org/en/v1.0.0/#summary)
- [git-commit documentation](https://git-scm.com/docs/git-commit)

## Cases

### Description only

<!-- <DOC_TEST> -->
```git-commit
my cool feature
```

```toml
description = "my cool feature"
```
<!-- </DOC_TEST> -->

### With type

<!-- <DOC_TEST> -->
```git-commit
feat: my cool feature
```

```toml
kind = "feat"
description = " my cool feature"
```
<!-- </DOC_TEST> -->

### With scope

<!-- <DOC_TEST> -->
```git-commit
feat(ui): my cool feature
```

```toml
kind = "feat"
description = " my cool feature"
scope = ["ui"]
```
<!-- </DOC_TEST> -->

### With exclamation mark

<!-- <DOC_TEST> -->
```git-commit
feat!: my cool feature
```

```toml
kind = "feat"
description = " my cool feature"
breaking = true
```
<!-- </DOC_TEST> -->

### With scope and exclamation mark

<!-- <DOC_TEST> -->
```git-commit
feat(ui)!: my cool feature
```

```toml
kind = "feat"
description = " my cool feature"
scope = ["ui"]
breaking = true
```
<!-- </DOC_TEST> -->
