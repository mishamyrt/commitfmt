# Rules

## Linting

#### `subject-max-length`

Subject can be no longer than N characters.

#### `subject-min-length`

Subject can be no shorter than N characters.

#### `subject-type`

Subject types can be limited. (`feat`, `fix`, etc.)

#### `subject-scope-format`

'camel-case', // camelCase
'kebab-case', // kebab-case
'pascal-case', // PascalCase
'snake-case', // snake_case

#### `body-max-line-length`

Every body line can be no longer than N characters.

#### `body-max-length`

Body can be no longer than N characters.

#### `body-min-length`

Body can be no shorter than N characters.

## Formatting

### Additions

#### `body-ticket-id`

Ticket ID can be added to the body of the commit.

##### Example

```
feat: my feature

Description body

Ref: PRJ-123
```

### Fixes

#### `body-blank-lines`

Body MUST start with a blank line and MUST NOT end with a blank line.

##### Wrong

```
feat: my feature
Description body
```

```
feat: my feature

Description body

```

```
feat: my feature


Description body
```

##### Correct

```
feat: my feature

Description body
```

#### `subject-scope-spaces`

There MUST NOT be a space before or after the comma.

##### Wrong

```
perf(core, repo): avoid allocations
```

##### Correct

```
perf(core,repo): avoid allocations
```

#### `subject-colon-spaces`

There MUST NOT be a space before the colon. There MUST be a space after the colon.

##### Wrong

```
feat : my feature
```

```
feat :my feature
```

#### `subject-trailing-characters`

There MUST NOT be a space or colon at the end of the subject.

##### Wrong

`feat: my feature `

`feat: my feature.`



#### `subject-breaking-changes-exclamation`

Subject MUST contain `!` before `:` if there are `BREAKING CHANGES` in body.

##### Wrong

```
feat: my feature

Description body

BREAKING CHANGES
```

##### Correct

```
feat!: my feature

Description body

BREAKING CHANGES
```

and also

```
feat!: my feature
```

#### `subject-description-lowercase-start`

Subject description MUST start with lowercase letter.

##### Wrong

```
feat: My feature
```

But should be skipped if the first word is a abbreviation.

```
feat: OTP feature
```
