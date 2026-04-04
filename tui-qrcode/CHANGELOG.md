# Changelog

All notable changes to this project will be documented in this file.

## [0.2.4] - 2026-04-04

### ⚙️ Miscellaneous Tasks

- *(deps)* Lower dependency floors and reduce dependabot noise ([#211](https://github.com/ratatui/tui-widgets/issues/211))
  > ## Summary
  >
  > - lower direct dependency requirements to the broadest semver ranges the
  > workspace actually supports
  > - keep `Cargo.lock` on current compatible releases, including the direct
  > `clap`, `tokio`, `futures`, and `rand` updates that fit this PR's scope
  > - configure Dependabot to group Cargo and GitHub Actions updates and use
  > `increase-if-necessary` to reduce manifest churn
  >
  > ## Details
  >
  > This change validates dependency floors with `cargo minimal-versions` in
  > `--direct` mode so the library manifests reflect honest direct
  > requirements instead of transitive minimum noise.
  >
  > Notable outcomes:
  >
  > - broadened requirements such as `clap = "4"` and `tokio = "1"` after
  > verifying the workspace still compiles and tests against their earliest
  > compatible direct versions
  > - kept real floors where required, such as `crossterm = "0.29"`,
  > `document-features = "0.2.11"`, and `derive_setters = "0.1.9"`
  > - kept the direct `rand` update to `0.10` and adjusted the
  > `tui-bar-graph` examples to generate random `Vec<f64>` values in a `rand
  > 0.10`-compatible way
  > - kept transitive duplicate major versions where they are still required
  > by downstream crates like the Ratatui stack or `lipsum`
  >
  > Dependabot should now produce less noise because grouped update PRs can
  > primarily refresh `Cargo.lock` while leaving `Cargo.toml` alone unless a
  > broader range is truly needed.
  >
  > ## Validation
  >
  > - `cargo minimal-versions check --workspace --direct`
  > - `cargo check --all-features --workspace`
  > - `cargo test --all-features --workspace`
  > - `cargo minimal-versions test --workspace --all-features --direct`
  > - `cargo outdated --workspace --root-deps-only`
  > - `cargo test -p tui-bar-graph --all-features --examples`
  >
  > ## Supersedes
  >
  > This PR should supersede and allow closing the related Dependabot PRs:


## [0.2.3] - 2026-03-27

### ⚙️ Miscellaneous Tasks

- *(project)* Update the repository link


## [0.2.2] - 2025-12-27

### 📚 Documentation

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  > Standardize widget crate docs and README layouts.
  > Unify badges, links, and license references.
  > Add consistent usage sections and link style updates.


## [0.2.1] - 2025-12-27

### ⚙️ Miscellaneous Tasks

- Refresh readmes and rdme check ([#140](https://github.com/ratatui/tui-widgets/issues/140))
  > Regenerate crate READMEs via cargo-rdme and add a CI check to keep
  > workspace readmes in sync.


## [0.2.0] - 2025-12-27

### 🚀 Features

- [**breaking**] Migrate to ratatui 0.30 ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  > feat!: migrate to ratatui 0.30
  >
  > - Update workspace deps to ratatui 0.30, ratatui-core, ratatui-widgets,
  > crossterm 0.29
  > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  > needed
  > - Update tui-popup/tui-prompts event handling to use crossterm types
  > - Revise tui-popup rendering/ref semantics and docs to match reference
  > rendering rules
  > - Add rolling breaking changes doc and markdownlint config
  > - Bump direct deps needed for minimal-versions and examples
  > (document-features, colorgrad, unicode-width)


## [0.1.3] - 2025-11-02

### 🚀 Features

- Calculate area of QRCodeWidget ([#68](https://github.com/ratatui/tui-widgets/issues/68))

### 🐛 Bug Fixes

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

### 🎨 Styling

- Format doc comments

- Add rustfmt and reformat code


## [0.1.2] - 2025-11-02

### 🚀 Features

- Calculate area of QRCodeWidget ([#68](https://github.com/ratatui/tui-widgets/issues/68))

### 🐛 Bug Fixes

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

### 🎨 Styling

- Format doc comments

- Add rustfmt and reformat code

