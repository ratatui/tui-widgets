# Changelog

All notable changes to this project will be documented in this file.

## [0.3.1] - 2025-12-27

### 📚 Documentation

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  > Standardize widget crate docs and README layouts.
  > Unify badges, links, and license references.
  > Add consistent usage sections and link style updates.


## [0.3.0] - 2025-12-27

### 🚀 Features

- *(tui-bar-graph)* [**breaking**] Add block octant characters ([#116](https://github.com/ratatui/tui-widgets/issues/116))
  > Since Unicode 16.0 was published on September 10, 2024, support for
  > block octant characters (U+1CD00 to U+1CDE5,
  > [PDF](https://www.unicode.org/charts/PDF/Unicode-16.0/U160-1CC00.pdf))
  > has been improving in fonts. We should enable users of `tui-widgets` to
  > use these characters in addition to existing options.

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


## [0.2.0] - 2025-11-02

### 🚀 Features

- *(tui-bar-graph)* [**breaking**] Support boxed gradients ([#66](https://github.com/ratatui/tui-widgets/issues/66))
  > This patch adds support for boxed gradients in the `BarGraph` widget.
  > This makes it possible to choose gradients of different types at runtime
  > without having to change the type of the `BarGraph` struct.

- *(tui-bar-graph)* Add Quadrant style ([#80](https://github.com/ratatui/tui-widgets/issues/80))
  > This style uses the block drawing 2x2 quadrant characters.
  > In contrast to the braille style, it renders solid rather than dots.
  > In contrast to the solid style, it renders two columns and rows per bar.
  >
  > ![Quadrant Magma](https://vhs.charm.sh/vhs-1rx6XQ9mLiO8qybSBXRGwn.gif)

### 🐛 Bug Fixes

- Broken bar graph test

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

- Use f64:midpoint ([#83](https://github.com/ratatui/tui-widgets/issues/83))
  > MSRV is now 1.87

### 🚜 Refactor

- Simplify color / gradient handling logic

### 🎨 Styling

- Add rustfmt and reformat code

### Other

- Bump msrv to 1.82.0 ([#74](https://github.com/ratatui/tui-widgets/issues/74))


## [tui-bar-graph-v0.1.1] - 2025-03-05

### 🚜 Refactor

- Simplify BarGraph rendering logic

## [tui-bar-graph-v0.1.0] - 2025-03-04

### 🚀 Features

- Add new tui-bar-graph crate ([#63](https://github.com/ratatui/tui-widgets/issues/63))
  > ![Braille demo](https://vhs.charm.sh/vhs-3H7bFj0M1kj0GoHcc4EIJ4.gif)
  >
  > ![Solid demo](https://vhs.charm.sh/vhs-5XMtSFgX3vqOhKcKl8fEQK.gif)
  >
  > ```rust
  > use tui_bar_graph::{BarGraph, BarStyle, ColorMode};
  >
  > let data = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
  > let bar_graph = BarGraph::new(data)
  >     .with_gradient(colorgrad::preset::turbo())
  >     .with_bar_style(BarStyle::Braille)
  >     .with_color_mode(ColorMode::VerticalGradient);
  > frame.render_widget(bar_graph, area);
  > ```
