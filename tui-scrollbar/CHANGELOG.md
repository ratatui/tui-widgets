# Changelog

All notable changes to this project will be documented in this file.

## [0.2.7] - 2026-06-14

### 📚 Documentation

- Modernize tui-scrollbar examples ([#283](https://github.com/ratatui/tui-widgets/issues/283))
  > ## Summary
  > - use ratatui::run in the tui-scrollbar examples
  > - borrow DefaultTerminal in example run loops
  > - preserve mouse capture setup while using current key press helpers
  > - add q/Esc exit consistency and vim motion keys for the mouse example
  >
  > ## Validation
  > - cargo +nightly fmt --all
  > - cargo check -p tui-scrollbar --examples --all-features
  > - cargo clippy -p tui-scrollbar --examples --all-features -- -D warnings
  > - just rdme-check

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


## [0.2.6] - 2026-06-11

### 📚 Documentation

- *(scrollbar)* Improve API discovery ([#251](https://github.com/ratatui/tui-widgets/issues/251))
  > ## Summary
  >
  > - Improve `tui-scrollbar` crate docs for API discovery, styling
  > behavior, glyph selection, and interaction flow.
  > - Add a small `scrollbar_styled` example that shows distinct track,
  > thumb, and arrow styles for vertical and horizontal scrollbars.
  > - Configure docs.rs example scraping and add a regression test for
  > custom thumb styling on full and partial thumb cells.
  >
  > ## Details
  >
  > This is a non-functional docs/examples/tests change. It reorganizes the
  > crate-level docs so important defaults and caveats are visible earlier,
  > replaces the loose `ScrollBar` “Key methods” list with a grouped method
  > map, and adds canonical examples for the main builder methods.
  >
  > The styling docs now spell out how `fg` and `bg` apply to terminal glyph
  > cells. In particular, they call out that the default minimal track
  > renders spaces, so empty track cells show background color, while
  > visible glyph sets can use foreground color for the track line. The docs
  > also note the partial-thumb caveat: when using visible tracks such as
  > `GlyphSet::box_drawing`, thumb background can show at partial glyph
  > edges, so matching it to the track background is usually less
  > surprising.
  >
  > The glyph docs keep repeated `Symbols for Legacy Computing` context
  > where readers may land directly, rather than relying on linear reading
  > through the crate docs.
  >
  > Related context:
  >
  > - Issue #193: https://github.com/ratatui/tui-widgets/issues/193
  > - PR #201: https://github.com/ratatui/tui-widgets/pull/201
  >
  > ## Validation
  >
  > - `cargo test -p tui-scrollbar --all-features`
  > - `cargo test -p tui-scrollbar --doc --all-features`
  > - `cargo check -p tui-scrollbar --examples --all-features`
  > - `cargo clippy -p tui-scrollbar --all-targets --all-features`
  > - `RUSTDOCFLAGS='-D warnings' cargo doc -p tui-scrollbar --no-deps
  > --all-features`
  > - `just fmt-check`
  > - `cargo rdme --check --manifest-path tui-scrollbar/Cargo.toml`
  > - `markdownlint-cli2 tui-scrollbar/README.md`


## [0.2.5] - 2026-06-11

### 🐛 Bug Fixes

- Restore stable clippy baseline ([#233](https://github.com/ratatui/tui-widgets/issues/233))


## [0.2.4] - 2026-04-04

### Other

- [codex] Vendor CI workflows into this repository ([#212](https://github.com/ratatui/tui-widgets/issues/212))


## [0.2.3] - 2026-03-29

### ⚙️ Miscellaneous Tasks

- *(project)* Update the repository link


## [0.2.2] - 2026-01-05

### 🚀 Features

- *(scrollbar)* Support crossterm 0.28 ([#172](https://github.com/ratatui/tui-widgets/issues/172))
  > Add versioned crossterm feature flags and re-export the selected version
  > as `tui_scrollbar::crossterm`.
  >
  > Add CI checks for the feature matrix and a docs.rs-style build.
  >
  > ---------


## [0.2.1] - 2026-01-05

### 🚀 Features

- *(scrollbar)* Update glyph previews and tests ([#169](https://github.com/ratatui/tui-widgets/issues/169))
  > Default ScrollBar renders without arrow endcaps and uses a dark gray
  > background with a blank (space) track.
  >
  > Add glyph set variants and improve the Unicode-only fallback. Document
  > glyph sets with a 1/8-step horizontal thumb walk, and add snapshot
  > render tests to keep the glyph combinations stable.


## [0.2.0] - 2025-12-28

### 🚀 Features

- *(scrollbar)* Add tui-scrollbar crate ([#164](https://github.com/ratatui/tui-widgets/issues/164))
  > ## Summary
  >
  > Introduce `tui-scrollbar`, a new widget crate for Ratatui that renders
  > smooth, fractional scrollbars
  > with precise thumb feedback and stateless input handling. The crate
  > focuses on clear geometry via
  > `ScrollMetrics`, configurable glyph sets (including legacy computing
  > symbols), and ergonomic
  > examples for keyboard and mouse interaction.
  >
  > ![ScrollBar demo](https://vhs.charm.sh/vhs-7c6j0GG4Up47YEHK5XLKoR.gif)
  >
  > ## Why
  >
  > Ratatui’s built-in scrollbar favors full-cell glyphs and stateful use.
  > This crate prioritizes
  > fractional thumbs for more accurate feedback, exposes pure metrics for
  > testing/composition, and
  > keeps scroll state in the application for predictable input behavior.
  >
  > ## Docs and Examples
  >
  > The crate-level docs include a quick start, API map, and input-handling
  > guidance. Two examples
  > show the fractional glyph sweep and an interactive mouse/keyboard demo.
  >
  > ```rust
  > use ratatui_core::buffer::Buffer;
  > use ratatui_core::layout::Rect;
  > use ratatui_core::widgets::Widget;
  > use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
  >
  > let area = Rect::new(0, 0, 1, 6);
  > let lengths = ScrollLengths {
  >     content_len: 120,
  >     viewport_len: 30,
  > };
  > let scrollbar = ScrollBar::vertical(lengths)
  >     .arrows(ScrollBarArrows::Both)
  >     .offset(45);
  >
  > let mut buffer = Buffer::empty(area);
  > scrollbar.render(area, &mut buffer);
  > ```


## [0.1.0] - 2025-12-27

### Features

- Initial release.
