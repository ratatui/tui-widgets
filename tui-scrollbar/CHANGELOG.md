# Changelog

All notable changes to this project will be documented in this file.

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
