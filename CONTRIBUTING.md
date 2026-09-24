# Contribution guidelines

First off, thank you for considering contributing to tui-widgets.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/ratatui/tui-widgets/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

## Commit Message Format

This project adheres to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
A specification for adding human and machine readable meaning to commit messages.

### Commit Message Header

```text
<type>(<scope>): <short summary>
  │       │             │
  │       │             └─⫸ Summary in present tense. Not capitalized. No period at the end.
  │       │
  │       └─⫸ Commit Scope
  │
  └─⫸ Commit Type: feat|fix|build|ci|docs|perf
                     |refactor|test|chore
```

#### Type

- feat: Features. A new feature.
- fix: Bug fixes. A bug fix.
- docs: Documentation. Documentation only changes.
- style: Styles. Changes that do not affect meaning (formatting, whitespace, etc).
- refactor: Code refactoring. Change that neither fixes a bug nor adds a feature.
- perf: Performance improvements. Change that improves performance.
- test: Tests. Adding missing tests or correcting existing tests.
- build: Builds. Changes that affect the build system or dependencies (main, serde).
- ci: Continuous integration. Changes to CI configuration or scripts (GitHub Actions).
- chore: Chores. Other changes that don't modify src or test files.
- revert: Reverts. Reverts a previous commit.

## Release notes

Release-plz generates changelogs from squash-merge commit subjects, using the PR title and number.
Write titles that describe the benefit to users and use a widget scope when applicable, such as
`fix(big-text): align non-ASCII text correctly`. The facade changelog uses the same concise entries;
put detailed widget usage in its documentation and link to it from the PR.

Ordinary PR bodies are not copied into changelogs. For a change that needs additional user guidance,
add a short `Release-note:` footer at the end of the PR body and preserve it in the squash commit:

```text
Release-note: Use size() and page_size() to inspect the dimensions from the latest render.
Both return None before the first render.
```

Use `BREAKING CHANGE:` for migration instructions and `!` in the title for breaking changes.
Keep migration footers short and self-contained: name the affected API and the required action.
Put screenshots, extended examples, and implementation details before the footer, and link to the
PR or migration guide for more detail. Avoid references such as "see above", since the rest of the
PR body is omitted. Breaking changes remain visible even when their commit type would normally be
filtered out.
Use `fix(security):` for a security fix, with a concise explanation of the impact in a
`Release-note:` footer. Mentioning security in a PR body does not classify it as a security fix.

Routine dependency updates, CI, chores, formatting, tests, and internal refactors are omitted from
mixed releases. Releases containing only these changes get a single "Dependency updates." or
"Maintenance updates." summary instead. Entries use a compact list without category headings.
For dependency changes that affect consumers, describe the effect with a `fix:` or `feat:` title,
or add a `Release-note:` footer to retain the entry. This includes compatibility changes, minimum
Rust version increases, and dependency requirements needed to guarantee a fix. Keep compatible
lockfile refreshes separate from such changes and retain the widest dependency requirements that
the library supports.

These filters control release notes only; they do not decide which packages receive a release.
Do not edit historical changelogs by hand. Release-plz and git-cliff generate new entries.

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/ratatui/tui-widgets
cd tui-widgets
cargo test
```

### Useful Commands

- Run Clippy:

  ```shell
  cargo clippy --all-targets --all-features --workspace
  ```

- Run all tests:

  ```shell
  cargo test --all-features --workspace
  ```

- Check to see if there are code formatting issues. This uses nightly rustfmt because
  `rustfmt.toml` enables unstable options for comment wrapping and import grouping.

  ```shell
  just fmt-check
  ```

- Format the code in the project. Use the same nightly-backed recipe so local formatting matches
  check mode.

  ```shell
  just fmt
  ```
