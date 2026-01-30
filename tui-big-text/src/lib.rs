//! A [Ratatui] widget to render gloriously oversized pixel text using glyphs from the [font8x8]
//! crate. Part of the [tui-widgets] suite by [Joshka].
//!
//! ![Demo](https://vhs.charm.sh/vhs-7DFJFGwBEnUjjLCFSqwEm9.gif)
//!
//! [![Crate badge]][Crate]
//! [![Docs Badge]][Docs]
//! [![Deps Badge]][Dependency Status]
//! [![License Badge]][License]
//! [![Coverage Badge]][Coverage]
//! [![Discord Badge]][Ratatui Discord]
//!
//! [GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]
//!
//! # Installation
//!
//! ```shell
//! cargo add ratatui tui-big-text
//! ```
//!
//! # Usage
//!
//! Create a [`BigText`] widget using [`BigText::builder`] and pass it to [`render_widget`]. The
//! builder allows you to customize the [`Style`] of the widget and the [`PixelSize`] of the
//! glyphs.
//!
//! # Examples
//!
//! ```rust
//! use ratatui::prelude::{Frame, Style, Stylize};
//! use tui_big_text::{BigText, PixelSize};
//!
//! fn render(frame: &mut Frame) {
//!     let big_text = BigText::builder()
//!         .pixel_size(PixelSize::Full)
//!         .style(Style::new().blue())
//!         .lines(vec![
//!             "Hello".red().into(),
//!             "World".white().into(),
//!             "~~~~~".into(),
//!         ])
//!         .build();
//!     frame.render_widget(big_text, frame.size());
//! }
//! ```
//!
//! The [`PixelSize`] can be used to control how many character cells are used to represent a single
//! pixel of the 8x8 font. It has six variants:
//!
//! - `Full` (default) - Each pixel is represented by a single character cell.
//! - `HalfHeight` - Each pixel is represented by half the height of a character cell.
//! - `HalfWidth` - Each pixel is represented by half the width of a character cell.
//! - `Quadrant` - Each pixel is represented by a quarter of a character cell.
//! - `ThirdHeight` - Each pixel is represented by a third of the height of a character cell.
//! - `Sextant` - Each pixel is represented by a sixth of a character cell.
//! - `QuarterHeight` - Each pixel is represented by a quarter of the height of a character cell.
//! - `Octant` - Each pixel is represented by an eighth of a character cell.
//!
//! ```rust
//! # use tui_big_text::*;
//! BigText::builder().pixel_size(PixelSize::Full);
//! BigText::builder().pixel_size(PixelSize::HalfHeight);
//! BigText::builder().pixel_size(PixelSize::Quadrant);
//! ```
//!
//! ![Pixel Size](https://vhs.charm.sh/vhs-6xzOJiPofPMWtUypzw1tg7.gif)
//!
//! Text can be aligned to the Left / Right / Center using the `alignment` methods.
//!
//! ```rust
//! # use tui_big_text::*;
//! BigText::builder().left_aligned();
//! BigText::builder().centered();
//! BigText::builder().right_aligned();
//! ```
//!
//! ![Alignment Example](https://vhs.charm.sh/vhs-2GdJCPpXfnOCTsykSPr7AW.gif)
//!
//! # More widgets
//!
//! For the full suite of widgets, see [tui-widgets].
//!
//! [tui-big-text]: https://crates.io/crates/tui-big-text
//! [Ratatui]: https://crates.io/crates/ratatui
//! [font8x8]: https://crates.io/crates/font8x8
//!
//! <!-- Note that these links are sensitive to breaking with cargo-rdme -->
//! [`BigText`]: https://docs.rs/tui-big-text/tui_big_text/big_text/struct.BigText.html
//! [`BigText::builder`]:
//!     https://docs.rs/tui-big-text/tui_big_text/big_text/struct.BigText.html#method.builder
//! [`PixelSize`]: https://docs.rs/tui-big-text/tui_big_text/pixel_size/enum.PixelSize.html
//! [`render_widget`]: https://docs.rs/ratatui/ratatui/struct.Frame.html#method.render_widget
//! [`Style`]: https://docs.rs/ratatui/ratatui/style/struct.Style.html
//!
//! [Crate]: https://crates.io/crates/tui-big-text
//! [Docs]: https://docs.rs/tui-big-text/
//! [Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
//! [Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//! [Crate badge]: https://img.shields.io/crates/v/tui-big-text?logo=rust&style=flat
//! [Docs Badge]: https://img.shields.io/docsrs/tui-big-text?logo=rust&style=flat
//! [Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
//! [License Badge]: https://img.shields.io/crates/l/tui-big-text?style=flat
//! [License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
//! [Coverage Badge]:
//!     https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat
//!
//! [GitHub Repository]: https://github.com/ratatui/tui-widgets
//! [API Docs]: https://docs.rs/tui-big-text/
//! [Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-big-text/examples
//! [Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-big-text/CHANGELOG.md
//! [Contributing]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md
//!
//! [Joshka]: https://github.com/joshka
//! [tui-widgets]: https://crates.io/crates/tui-widgets

#![no_std]

extern crate alloc;

mod big_text;
mod pixel_size;

pub use big_text::{BigText, BigTextBuilder};
pub use pixel_size::PixelSize;
