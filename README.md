<p align="center">
  <img width="350" src="./docs/assets/logo.svg" alt="commitfmt logo" />
  <br />
  <br />
  Utility for formatting and verifying commit messages.
</p>

---

<p align="center">
  <a href="https://github.com/mishamyrt/commitfmt/actions/workflows/qa.yaml">
    <img src="https://github.com/mishamyrt/commitfmt/actions/workflows/qa.yaml/badge.svg" alt="Quality Assurance" />
  </a>
  <a href="https://npmjs.com/package/commitfmt">
    <img src="https://img.shields.io/npm/v/commitfmt.svg?color=red" alt="NPM Version" />
  </a>
  <a href="https://pypi.org/project/commitfmt/">
    <img src="https://img.shields.io/pypi/v/commitfmt.svg?color=blue" alt="PyPI Version" />
  </a>
</p>

It's not a linter. At least not a complete replacement for [commitlint](https://commitlint.js.org), because commitfmt can't prevent you from writing a body or force you to write a description in uppercase (I don't know why you might want to do that), but it will help keep git history clean and readable.

By design, commitfmt runs on the `prepare-commit-msg` hook and formats the message according to git standards and [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) in particular.

## Features

### Formatting

commitfmt by default transforms a message like this:

```
feat ( scope     ,    scope  )  : add new feature.
body description
```

into well-formatted message:

```
feat(scope, scope): add new feature

body description
```

### Linting

commitfmt can check that developers follow the rules set by the project.

For example, check that only allowed types and scopes are used. To do this, add the following to the <nobr>[configuration file](#configuration)</nobr>:

```toml
[lint.header]
# Check allowed commit type
type-enum = ["chore", "ci", "feat", "fix", "refactor", "style", "test"]
# Check allowed commit scopes
scope-enum = ["cc", "config", "git", "linter"]

[lint.footer]
# Check required footers
exists = ["Issue-ID", "Authored-By"]
```

### Performance

commitfmt is very fast because its code is written in Rust with memory consumption and performance in mind.

It natively supports following platforms:

| OS | Architecture |
| --- | --- |
| macOS | x86_64, arm64 |
| Windows | x86_64, i686 |
| Linux | x86_64, i686, arm64 |

## Installation

### Script

You can use a simple [script](https://github.com/mishamyrt/commitfmt/blob/refs/heads/main/scripts/install.sh) to install commitfmt.
It will download the latest version of the binary and install it to the system.

```bash
# Install latest version
curl -sSfL https://raw.githubusercontent.com/mishamyrt/commitfmt/refs/heads/main/scripts/install.sh | bash
```

### pnpm

```bash
pnpm add --save-dev commitfmt
```

### npm

```bash
npm install --save-dev commitfmt
```

### yarn

```bash
yarn add --dev commitfmt
```

### pip

```bash
pip install commitfmt
```

## Hook

After installing the package, you need to add a hook to the `prepare-commit-msg` event. You can use any hook manager.

> **Important:** if you are using a pnpm, yarn or any other package manager, you need to run `pnpm commitfmt`, `yarn commitfmt`, etc. instead of `commitfmt`.

### Script

You can use a simple script to add a hook.

```bash
echo "#!/bin/sh" > .git/hooks/prepare-commit-msg
echo "commitfmt" >> .git/hooks/prepare-commit-msg
chmod +x .git/hooks/prepare-commit-msg
```

### [Lefthook](https://github.com/evilmartians/lefthook)

Add to your `lefthook.yml` file:

```yaml
prepare-commit-msg:
  - name: format commit message
    run: commitfmt
```

### [Husky](https://github.com/typicode/husky)

Add to your `.husky` folder `prepare-commit-msg` file with the following content:

```bash
#!/bin/sh
commitfmt
```

## Configuration

In commitfmt, you cannot customize basic formatting rules such as extra spaces removal.

It is an opinionated formatter and the author has established best practices that should not harm anyone.

### Linting

Most of the linting rules are disabled by default. Default config contains 2 rules as they can be safely auto-fixed:

```toml
[lint.header]
full-stop = true

[lint.footer]
breaking-exclamation = true
```

To enable more rules, create a `commitfmt.toml` or (`.commitfmt.toml`) file in the root of your project. Available lint rules can be found in the [rules.md](https://github.com/mishamyrt/commitfmt/blob/main/crates/commitfmt-linter/docs/rules.md) file.

If there is a problem with an enabled rule and it cannot be automatically fixed, the commit process will be aborted.

#### Unsafe fixes

Some rules may be fixed, but in certain contexts this fix may not be what is desired. For example, adding a full stop to the end of body will be useful in most cases, if there is a log at the end of the message, the period may distort it. You can see which rules have unsafe patches in the same `rules.md` file mentioned above.

To enable unsafe fixes, add the following to your config file:

```toml
[lint]
unsafe-fixes = true
```

### Extending

You can extend the configuration of the parent project by adding the `extends` key to your config file:

```toml
extends = "node_modules/commitfmt-config-standard/commitfmt.toml"
```

Extension is only possible for the current configuration. If the current configuration extends another configuration, which in turn extends a third configuration, commitfmt will throw an error when trying to load such a configuration.

### Parser Configuration

commitfmt can be configured to use custom footer separators and comment symbols for parsing commit messages.

#### Footer separators

By default, commitfmt uses git's `trailer.separators` configuration to determine which characters separate footer keys from values. You can override this in your config file:

```toml
footer-separators = ":#"
```

This allows footers like `Issue-ID: 123` or `Issue-ID #123` to be recognized.

#### Comment symbol

By default, commitfmt uses git's `core.commentChar` or `core.commentString` configuration to identify comment lines in commit messages. You can override this:

```toml
comment-symbol = "//"
```

Lines starting with the comment symbol will be ignored during parsing.

### Additional footers

commitfmt can add additional footers to the commit message.

#### Value template

You can use a template to format the value of the footer. Inside of template expression you can use any shell command available at the `PATH`.

For example, to add the `Authored-By` footer with the current user name, add the following to your config file:

```toml
[additional-footers]
key = "Authored-By"
value-template = "{{ echo $USER }}"
```

#### Branch value pattern

You can also add the ticket number from the task tracker to the footer if it is in the branch name:

```toml
[additional-footers]
key = "Ticket-ID"
branch-value-pattern = "(?:.*)/([A-Z0-9-]+)/?(?:.*)"
```

For example, if your branch name is `feature/CC-123/add-new-feature` or `feature/CC-123`, the `Ticket-ID` footer will be added to the commit message with the value `CC-123`.

If the ticket number is not found in the branch name, footer will be skipped.

You can use [rustexp](https://rustexp.lpil.uk) to test your pattern.

##### Recipes

Examples of patterns for branch names in git flow format:

- Jira/YouTrack: `(?:.*)/([A-Z0-9-]+)/?(?:.*)`
  - `feature/CFMT-123`
  - `feature/CFMT-123/add-new-feature`
- GitHub: `(?:.*)/#?([0-9-]+)/?(?:.*)`
  - `feature/#123`
  - `feature/123`
  - `feature/#123/add-new-feature`

#### On conflict

If the footer already exists in the commit message, you can specify what to do with it. By default, the footer will be skipped.

```toml
[additional-footers]
key = "Ticket-ID"
branch-value-pattern = "(?:.*)/([A-Z0-9-]+)/?(?:.*)"
on-conflict = "skip" # skip, append, error
```

Available options:

- `skip` - skip the footer if it already exists
- `append` - append the footer to the end of the footer list
- `error` - abort the commit

## Testing

To test the configuration and the work of commitfmt, run the following command:

```bash
echo "chore ( test ) : test commit" | commitfmt
# or
cat commit_text.txt | commitfmt
```

## History testing

To test the history of commits, run the following command:

```bash
commitfmt --from HEAD~20
# or
commitfmt --from 1234567890 --to 1234567890
```
