# Changelog

All notable changes to this project will be documented in this file.

## [0.2.3] - 2026-02-14

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

