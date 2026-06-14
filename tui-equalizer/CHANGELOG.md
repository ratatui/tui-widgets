# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2026-06-14

### 📚 Documentation

- Document widget examples ([#286](https://github.com/ratatui/tui-widgets/issues/286))
  > ## Summary
  >
  > - add run commands and widget-specific context to the example module
  > docs
  > - add targeted inline comments for non-obvious example sizing, state,
  > and rendering choices
  > - fix broken crate-root `Ratatui` reference links that surfaced during
  > docs validation
  >
  > ## Validation
  >
  > - `cargo +nightly fmt --all --check`
  > - `cargo clippy --workspace --examples --all-features -- -D warnings`
  > - `cargo doc --workspace --examples --all-features --no-deps`
  >
  > `cargo doc` still reports the existing `tui-bar-graph` example/lib
  > output filename collision.

### ⚙️ Miscellaneous Tasks

- Migrate workspace to Rust 2024 ([#263](https://github.com/ratatui/tui-widgets/issues/263))
  > ## Summary
  > - migrate the workspace package edition from Rust 2021 to Rust 2024
  > - apply Cargo edition fixes for lifetime capture and macro fragment
  > specifiers
  > - apply mechanical Clippy let-chain fixes needed for the stable -D
  > warnings gate
  > - refresh generated README snippets and document the pre-push README
  > check in AGENTS.md
  >
  > ## Validation
  > - cargo fix --edition --all-features --workspace --allow-dirty
  > --allow-staged
  > - just fmt
  > - just clippy-stable
  > - cargo test --all-features --workspace
  > - just rdme-check
  > - markdownlint-cli2 AGENTS.md README.md tui-*/README.md

