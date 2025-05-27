# Footers



## References

- [git trailer config](https://github.com/git/git/blob/master/Documentation/config/trailer.adoc)
- [git-interpret-trailers documentation](https://git-scm.com/docs/git-interpret-trailers)

## Cases

### Single

```git-commit
feat: my feature

Authored-By: Co Mitter <comitter@example.com>
```

```toml
kind = feat
scope = []
description = "my feature"
footers = [{
  key = "Authored-By"
  value = "Co Mitter <comitter@example.com>"
}]
```

### Multiple

```git-commit

Authored-By: Co Mitter <comitter@example.com>
Reviewed-By: Re Viewer <reviewer@example.com>
```

```toml
kind = feat
scope = []
description = "my feature"
footers = [
  {
    key = "Authored-By"
    value = "Co Mitter <comitter@example.com>"
  },
  {
    key = "Reviewed-By"
    value = "Re Viewer <reviewer@example.com>"
  }
]
```

### With body

```git-commit
feat: my feature

Description body

Authored-By: Co Mitter <comitter@example.com>
```

```toml
kind = feat
scope = []
body = "\nDescription body"
description = "my feature"
footers = [{
  key = "Authored-By"
  value = "Co Mitter <comitter@example.com>"
}]
```
