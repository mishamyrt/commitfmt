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

commitfmt is an opinionated formatter and configurable linter for Git commit messages. Designed primarily for use in a Git `prepare-commit-msg` hook, it helps keep commit history clean and readable.

## Quick start

The example below installs commitfmt as a local npm development dependency. See [Installation](#installation) for standalone and Python alternatives.

```bash
npm install --save-dev commitfmt
```

Try the default formatting mode:

```bash
printf '%s\n' "feat ( parser ) : add new option." | npm exec -- commitfmt
# feat(parser): add new option
```

Then add `npm exec -- commitfmt` to a Git `prepare-commit-msg` hook using one of the [hook examples](#hook). No configuration is required for formatting. To enforce project-specific rules, add a [configuration file](#configuration) to the repository root.

See [Modes](#modes) for the difference between formatting a message, linting a message, and checking commit history.

## Features

### Formatting

By default, commitfmt transforms a message like this:

```
feat ( scope     ,    scope  )  : add new feature.
body description
```

into a well-formatted message:

```
feat(scope, scope): add new feature

body description
```

### Linting

commitfmt can enforce project-specific commit rules.

For example, to restrict commit types and scopes, add the following to the [configuration file](#configuration):

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

commitfmt is designed for low-overhead local hooks. The [comparison benchmark](docs/benchmark.md) records mean latency and throughput for commitfmt and commitlint when checking the current commit and a 10-commit history range.

## Installation

Prebuilt binaries are available for the following platforms and installation methods:

| OS | Installation script | npm or pip |
| --- | --- | --- |
| macOS | x64, arm64 | x64, arm64 |
| Windows | x64, x86, arm64 | x64, arm64 |
| Linux | x64, x86, arm64 | x64, arm64 |

### Script

You can use a simple [script](https://github.com/mishamyrt/commitfmt/blob/refs/heads/main/scripts/install.sh) to install commitfmt.
It downloads and installs the latest prebuilt binary.

The installer requires Bash, `curl` or `wget`, and the appropriate archive tools: `tar` on macOS and Linux, with `xz` or `unxz` as a fallback, or `unzip` on Windows. On Windows, run it from Git Bash, MSYS2, or Cygwin. The binary is installed in `/usr/local/bin` when that directory is writable; otherwise, it is installed in `~/.local/bin`.

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

After installing commitfmt, add a Git `prepare-commit-msg` hook. You can configure it manually or use any hook manager.

> **Command used in hooks:** the examples below assume an installation made with the script or pip and use the direct `commitfmt` command. For a local Node.js development dependency, replace it with `pnpm exec commitfmt`, `npm exec -- commitfmt`, or `yarn exec commitfmt`.

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
  jobs:
    - name: format commit message
      run: commitfmt
      args: ""
```

`args` is intentionally empty: Lefthook otherwise forwards Git hook arguments, while commitfmt locates Git's `COMMIT_EDITMSG` itself.

### [Husky](https://github.com/typicode/husky)

Create `.husky/prepare-commit-msg` with the following content:

```bash
#!/bin/sh
commitfmt
```

## Configuration

Core formatting behavior, such as removing extra whitespace, is intentionally not configurable. This keeps formatted commit messages consistent and predictable across a project. Project-specific constraints can be configured using the lint rules below.

### Linting

Most linting rules are disabled by default. Two rules are enabled because the formatter can fix them safely:

```toml
[lint.header]
description-full-stop = true # Remove trailing periods from header descriptions

[lint.footer]
breaking-exclamation = true # Add ! for BREAKING CHANGE or BREAKING-CHANGE
```

Breaking-change footer keys are case-sensitive. commitfmt recognizes `BREAKING CHANGE` and `BREAKING-CHANGE`; `BREAKING CHANGES` is not supported.

To enable more rules, create a `.commitfmt.toml` or `commitfmt.toml` file in the root of your Git repository. commitfmt loads the configuration from the repository root even when it is run from a subdirectory. If both files exist, `.commitfmt.toml` takes precedence. Available lint rules can be found in the [rules.md](https://github.com/mishamyrt/commitfmt/blob/main/crates/commitfmt-linter/docs/rules.md) file.

In the default formatting mode, safe fixes are applied automatically. The command fails on violations that cannot be fixed; when used as a hook, this aborts the commit. In `--lint` mode, the message is never modified and any violation causes a non-zero exit code.

#### Unsafe fixes

Some rules have fixes that may be undesirable in certain contexts. For example, adding a period to the end of a body may distort an embedded log. Rules with unsafe fixes are marked in the same `rules.md` file.

To enable unsafe fixes, add the following to your config file:

```toml
[lint]
unsafe-fixes = true
```

### Extending

To reuse another configuration, add `extends` with a path relative to the current configuration file:

```toml
extends = "node_modules/commitfmt-config-standard/commitfmt.toml"
```

Only one inheritance level is supported. If the referenced configuration also contains `extends`, commitfmt returns an error.

Configuration inheritance uses a shallow merge:

- top-level parser options from the current file override inherited values
- redefining `[lint.header]`, `[lint.body]`, or `[lint.footer]` replaces the entire inherited group; groups omitted from the current file remain inherited
- `[lint]` options from the current file replace inherited `[lint]` options, so repeat `unsafe-fixes = true` when the current file defines lint settings and still needs unsafe fixes
- additional footers from the current file are appended after inherited footers

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

Comment lines immediately after the header and at the end of the commit message are ignored during parsing. Comment lines elsewhere are preserved as message content, and the header is always parsed as content.

### Additional footers

commitfmt can add additional footers to the commit message.

#### Static value

You can add a footer with a static value:

```toml
[[additional-footers]]
key = "Authored-By"
value = "John Doe"
```

#### Shell commands

You can use shell commands to dynamically generate footer values:

```toml
[[additional-footers]]
key = "Authored-By"
value = "{{ printf %s \"$USER\" }}"
```

Commands are executed with `sh -c`. The `sh` executable and any external commands must be available through `PATH`; shell built-ins such as `printf` require no separate executable. A non-zero exit code aborts formatting, and trailing line endings are removed from standard output before it is used as the footer value.

> **Security:** shell templates execute arbitrary commands. Use configuration files only from trusted sources.

#### Branch value pattern

You can also add the ticket number from the task tracker to the footer if it is in the branch name:

```toml
[[additional-footers]]
key = "Ticket-ID"
branch-pattern = "^(?:[^/]+/)*(?<TICKET_ID>[A-Z][A-Z0-9]*-[0-9]+)(?:/.*)?$"
value = "${{ TICKET_ID }}"
```

For example, if your branch name is `feature/CC-123/add-new-feature` or `feature/CC-123`, the `Ticket-ID` footer will be added to the commit message with the value `CC-123`.

The named capture is available to the value template as `TICKET_ID`. If the current branch is unavailable or does not match the pattern, the footer is skipped.

Branch patterns use the [regex-lite syntax](https://docs.rs/regex-lite/latest/regex_lite/#syntax).
Unicode character classes and Unicode-aware case folding are not supported.

##### Patterns

Examples of patterns for branch names in git flow format:

- Jira/YouTrack: `^(?:[^/]+/)*(?<TICKET_ID>[A-Z][A-Z0-9]*-[0-9]+)(?:/.*)?$`
  - `feature/CFMT-123`
  - `feature/CFMT-123/add-new-feature`
- GitHub: `^(?:[^/]+/)*(?<ISSUE_ID>[0-9]+)(?:/.*)?$`
  - `feature/123`
  - `feature/123/add-new-feature`

#### On conflict

If the message already contains a footer with the same key, `on-conflict` determines whether commitfmt adds the configured footer. The existing footer is never removed.

```toml
[[additional-footers]]
key = "Ticket-ID"
branch-pattern = "^(?:[^/]+/)*(?<TICKET_ID>[A-Z][A-Z0-9]*-[0-9]+)(?:/.*)?$"
value = "${{ TICKET_ID }}"
on-conflict = "error" # optional; default: skip
```

Available options:

- `skip` — keep the existing footer and do not add the configured one (default)
- `append` — keep the existing footer and add the configured one at the end
- `error` — return an error without writing the formatted message; when running as a hook, this aborts the commit

#### Footer formatting

You can customize how footers are formatted using `separator` and `alignment`. The separator must be a single character.

```toml
[[additional-footers]]
key = "Ticket-ID"
value = "CFMT-123"
separator = "#"
alignment = "right"
```

Available alignment options:

- `left` — no space before the separator and one after it (default): `Ticket-ID# CFMT-123`
- `right` — one space before the separator and none after it: `Ticket-ID #CFMT-123`

### Recipe

To enforce conventional commits, you can use the following configuration:

```toml
[lint.header]
type-enum = ["chore", "ci", "feat", "fix", "refactor", "style", "test", "docs", "revert"]
description-case = "lower-first"
description-max-length = 72
description-full-stop = true
type-required = true
# scope-enum = ["cc", "config", "git", "linter"] # optional

[lint.body]
max-line-length = 72
case = "upper-first"

[lint.footer]
breaking-exclamation = true
```

## Modes

### Formatting mode

Formatting is the default mode. commitfmt normalizes the message structure and applies fixes from enabled rules. Safe fixes are always applied; unsafe fixes are applied only when [`unsafe-fixes`](#unsafe-fixes) is enabled. An unfixable violation returns a non-zero exit status and prevents the formatted message from being written.

When a message is supplied through standard input, `commitfmt` formats it and writes the result to standard output:

```bash
printf '%s\n' "chore ( test ) : test commit." | commitfmt
# chore(test): test commit

# or
commitfmt < commit_text.txt
```

The command exits with a non-zero status if configuration loading, parsing, or an unfixable rule fails.

When run without redirected standard input during an active Git commit, as in a `prepare-commit-msg` hook, commitfmt reads and updates Git's `COMMIT_EDITMSG` file instead of writing the formatted message to standard output.

### Lint mode

Use `--lint` to check a message without modifying it or applying available fixes:

```bash
printf '%s\n' "chore(test): test commit" | commitfmt --lint
```

A valid message produces no output and exits with status 0. Any violation produces a report and a non-zero exit status. When used in a hook, lint mode checks `COMMIT_EDITMSG` but never rewrites it.

### History mode

Use `--from` to lint a Git commit range:

```bash
commitfmt --from HEAD~20
# or
commitfmt --from v1.1.0 --to HEAD
```

The lower boundary is excluded. `--to` defaults to `HEAD` and can only be used together with `--from`. History mode never modifies existing commits and always lints messages, so `--lint` is unnecessary and ignored.

## Ignoring commits

commitfmt skips messages whose text starts with the exact, case-sensitive prefix `Merge` or `Revert`. This avoids rewriting Git-generated merge and revert messages.

The check is based only on the message text, not commit metadata: `merge`, `revert:`, and other differently cased prefixes are not skipped, while any custom message starting with `Merge` or `Revert` is skipped. This applies both when formatting a single message and when linting history.

## License

[MIT](./LICENSE).
