//! Smooth, fractional scrollbars for Ratatui. Part of the [tui-widgets] suite by [Joshka].
//!
//! ![ScrollBar demo](https://vhs.charm.sh/vhs-21HzyozMOar6SYjVDBrpOb.gif)
//!
//! [![Crate badge]][Crate]
//! [![Docs Badge]][Docs]
//! [![Deps Badge]][Dependency Status]
//! [![License Badge]][License]
//! [![Coverage Badge]][Coverage]
//! [![Discord Badge]][Ratatui Discord]
//!
//! [GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing] · [Crate source]
//!
//! Use this crate when you want scrollbars that communicate position and size more precisely than
//! full-cell glyphs. The widget renders into a [`Buffer`] for a given [`Rect`] and stays reusable
//! by implementing [`Widget`] for `&ScrollBar`.
//!
//! # Feature highlights
//!
//! - Fractional thumbs: render 1/8th-cell steps for clearer position/size feedback.
//! - Arrow endcaps: optional start/end arrows with click-to-step support.
//! - Backend-agnostic input: handle pointer + wheel events without tying to a backend.
//! - Stateless rendering: render via [`Widget`] for `&ScrollBar` with external state.
//! - Metrics-first: [`ScrollMetrics`] exposes pure geometry for testing and hit testing.
//!
//! # Why not Ratatui's scrollbar?
//!
//! Ratatui's built-in scrollbar favors simple full-cell glyphs and a stateful widget workflow.
//! This crate chooses fractional glyphs for more precise thumbs, keeps rendering stateless, and
//! exposes a small interaction API plus pure metrics so apps can control behavior explicitly.
//!
//! # Installation
//!
//! ```shell
//! cargo add tui-scrollbar
//! ```
//!
//! # Important
//!
//! - Zero lengths are treated as 1.
//! - Arrow endcaps are disabled by default; configure them with [`ScrollBarArrows`].
//! - The default [`GlyphSet`] hides the track using spaces; use [`GlyphSet::box_drawing`] or
//!   [`GlyphSet::unicode`] for a visible track.
//! - The default glyphs use [Symbols for Legacy Computing] for missing upper/right eighth blocks.
//!   Use [`GlyphSet::unicode`] if you need only standard Unicode block elements.
//!
//! # Quick start
//!
//! This example renders a vertical [`ScrollBar`] into a [`Buffer`] using a fixed track size and
//! offset. Use it as a minimal template when you just need a thumb and track on screen.
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::Rect;
//! use ratatui_core::widgets::Widget;
//! use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
//!
//! let area = Rect::new(0, 0, 1, 6);
//! let lengths = ScrollLengths {
//!     content_len: 120,
//!     viewport_len: 30,
//! };
//! let scrollbar = ScrollBar::vertical(lengths)
//!     .arrows(ScrollBarArrows::Both)
//!     .offset(45);
//!
//! let mut buffer = Buffer::empty(area);
//! scrollbar.render(area, &mut buffer);
//! ```
//!
//! # Conceptual overview
//!
//! The scrollbar works in three pieces:
//!
//! 1. Your app owns `content_len`, `viewport_len`, and `offset` (lengths along the scroll axis).
//! 2. [`ScrollMetrics`] converts those values into a thumb position and size.
//! 3. [`ScrollBar`] renders the track + thumb using fractional glyphs.
//!
//! Most apps update `offset` in response to input events and re-render each frame.
//!
//! ## Units and subcell conversions
//!
//! `content_len`, `viewport_len`, and `offset` are measured in logical units along the scroll
//! axis. For many apps, those units are items or lines. The ratio between `viewport_len` and
//! `content_len` is what matters, so any consistent unit works.
//!
//! # Styling
//!
//! Style the track, thumb, and arrow endcaps directly on [`ScrollBar`]. See [`ScrollBar`] for a
//! full method map and more focused examples.
//!
//! Scrollbar glyphs are terminal characters. For visible track glyphs, thumb blocks, and arrow
//! symbols, `Style::fg` colors the glyph itself and `Style::bg` colors the cell behind it. The
//! default [`GlyphSet::minimal`] track renders spaces, so only the track background is visible in
//! empty track cells. Visible track glyph sets, such as [`GlyphSet::box_drawing`] and
//! [`GlyphSet::unicode`], can use foreground color for the track line. Thumb glyphs are block
//! characters, so `Style::fg` is usually the useful knob for thumb color; `Style::bg` still colors
//! the rest of the cell. With partial thumb glyphs, especially on a visible line track such as
//! [`GlyphSet::box_drawing`], that background can show at the ends of the thumb. Match the thumb
//! background to the track background unless that contrast is intentional.
//!
//! ```rust
//! use ratatui_core::style::{Color, Style};
//! use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
//!
//! let lengths = ScrollLengths {
//!     content_len: 120,
//!     viewport_len: 30,
//! };
//! let scrollbar = ScrollBar::vertical(lengths)
//!     .arrows(ScrollBarArrows::Both)
//!     .track_style(Style::new().bg(Color::Black))
//!     .thumb_style(Style::new().fg(Color::Rgb(255, 158, 100)))
//!     .arrow_style(Style::new().fg(Color::Yellow).bg(Color::Black));
//! ```
//!
//! # Layout integration
//!
//! This example shows how to reserve a column for a vertical [`ScrollBar`] alongside your content.
//! Use the same pattern for a horizontal [`ScrollBar`] by splitting rows instead of columns.
//!
//! ```rust,no_run
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::{Constraint, Layout, Rect};
//! use ratatui_core::widgets::Widget;
//! use tui_scrollbar::{ScrollBar, ScrollLengths};
//!
//! let area = Rect::new(0, 0, 12, 6);
//! let [content_area, bar_area] = area.layout(&Layout::horizontal([
//!     Constraint::Fill(1),
//!     Constraint::Length(1),
//! ]));
//!
//! let lengths = ScrollLengths {
//!     content_len: 400,
//!     viewport_len: 80,
//! };
//! let scrollbar = ScrollBar::vertical(lengths).offset(0);
//!
//! let mut buffer = Buffer::empty(area);
//! scrollbar.render(bar_area, &mut buffer);
//! ```
//!
//! # Interaction loop
//!
//! This pattern assumes you have enabled mouse capture in your terminal backend and have the
//! scrollbar [`Rect`] (`bar_area`) from your layout each frame. Keep a [`ScrollBarInteraction`] in
//! your app state so drag operations persist across draws. Mouse events are handled via
//! [`ScrollBar::handle_mouse_event`], which returns a [`ScrollCommand`] to apply.
//!
//! ```rust,no_run
//! use ratatui_core::layout::Rect;
//! use tui_scrollbar::{ScrollBar, ScrollBarInteraction, ScrollCommand, ScrollLengths};
//!
//! let bar_area = Rect::new(0, 0, 1, 10);
//! let lengths = ScrollLengths {
//!     content_len: 400,
//!     viewport_len: 80,
//! };
//! let scrollbar = ScrollBar::vertical(lengths).offset(0);
//! let mut interaction = ScrollBarInteraction::new();
//! let mut offset = 0;
//!
//! # #[cfg(any(feature = "crossterm_0_28", feature = "crossterm_0_29"))]
//! # {
//! # use tui_scrollbar::crossterm::event::{self, Event};
//! if let Event::Mouse(event) = event::read()? {
//!     if let Some(ScrollCommand::SetOffset(next)) =
//!         scrollbar.handle_mouse_event(bar_area, event, &mut interaction)
//!     {
//!         offset = next;
//!     }
//! }
//! # }
//! # let _ = offset;
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! # Metrics-first workflow
//!
//! This example shows how to compute thumb geometry without rendering via [`ScrollMetrics`]. It's
//! useful for testing, hit testing, or when you want to inspect thumb sizing directly.
//!
//! ```rust
//! use tui_scrollbar::{ScrollLengths, ScrollMetrics, SUBCELL};
//!
//! let track_cells = 12;
//! let viewport_len = track_cells * SUBCELL;
//! let content_len = viewport_len * 6;
//! let lengths = ScrollLengths {
//!     content_len,
//!     viewport_len,
//! };
//! let metrics = ScrollMetrics::new(lengths, 0, track_cells as u16);
//! assert!(metrics.thumb_len() >= SUBCELL);
//! ```
//!
//! # Glyph selection
//!
//! [`GlyphSet`] controls the track and thumb characters. The default glyphs include
//! [Symbols for Legacy Computing] so the thumb can render upper/right partial fills that are
//! missing from the standard block set. Use [`GlyphSet::box_drawing`] for a visible line track, or
//! [`GlyphSet::unicode`] when the terminal font should avoid [Symbols for Legacy Computing]
//! glyphs.
//!
//! ```rust
//! use tui_scrollbar::{GlyphSet, ScrollBar, ScrollLengths};
//!
//! let lengths = ScrollLengths {
//!     content_len: 10,
//!     viewport_len: 5,
//! };
//! let scrollbar = ScrollBar::vertical(lengths).glyph_set(GlyphSet::unicode());
//! ```
//!
//! # API map
//!
//! ## Widgets
//!
//! - [`ScrollBar`]: renders vertical or horizontal scrollbars with fractional thumbs.
//!
//! ## Supporting types
//!
//! - [`ScrollBarInteraction`]: drag capture state for pointer interaction.
//! - [`ScrollMetrics`]: pure math for thumb sizing and hit testing.
//! - [`GlyphSet`]: glyph selection for track and thumb rendering.
//! - [`ScrollBarArrows`]: arrow endcap configuration.
//!
//! ## Enums and events
//!
//! - [`ScrollBarOrientation`], [`ScrollBarArrows`], [`TrackClickBehavior`]
//! - [`ScrollEvent`], [`ScrollCommand`]
//! - [`PointerEvent`], [`PointerEventKind`], [`PointerButton`]
//! - [`ScrollWheel`], [`ScrollAxis`]
//!
//! # See also
//!
//! - [tui-scrollbar examples]
//! - [`scrollbar_styled` example]
//! - [`scrollbar_mouse` example]
//! - [`scrollbar` example]
//! - [`Widget`]
//! - [Ratatui]
//!
//! # More widgets
//!
//! For the full suite of widgets, see [tui-widgets].
//!
//! [Ratatui]: https://crates.io/crates/ratatui
//! [Crate]: https://crates.io/crates/tui-scrollbar
//! [Docs]: https://docs.rs/tui-scrollbar/
//! [Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
//! [Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//! [Crate badge]: https://img.shields.io/crates/v/tui-scrollbar?logo=rust&style=flat
//! [Docs Badge]: https://img.shields.io/docsrs/tui-scrollbar?logo=rust&style=flat
//! [Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
//! [License Badge]: https://img.shields.io/crates/l/tui-scrollbar?style=flat
//! [License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
//! [Coverage Badge]:
//!     https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat
//! [GitHub Repository]: https://github.com/ratatui/tui-widgets
//! [API Docs]: https://docs.rs/tui-scrollbar/
//! [Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-scrollbar/examples
//! [Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-scrollbar/CHANGELOG.md
//! [Contributing]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md
//! [Crate source]: https://github.com/ratatui/tui-widgets/blob/main/tui-scrollbar/src/lib.rs
//! [`scrollbar_mouse` example]: https://github.com/ratatui/tui-widgets/tree/main/tui-scrollbar/examples/scrollbar_mouse.rs
//! [`scrollbar_styled` example]: https://github.com/ratatui/tui-widgets/tree/main/tui-scrollbar/examples/scrollbar_styled.rs
//! [`scrollbar` example]: https://github.com/ratatui/tui-widgets/tree/main/tui-scrollbar/examples/scrollbar.rs
//! [tui-scrollbar examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-scrollbar/examples
//! [`Buffer`]: ratatui_core::buffer::Buffer
//! [`Rect`]: ratatui_core::layout::Rect
//! [`Widget`]: ratatui_core::widgets::Widget
//! [Symbols for Legacy Computing]: https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing
//!
//! [Joshka]: https://github.com/joshka
//! [tui-widgets]: https://crates.io/crates/tui-widgets
#![cfg_attr(docsrs, doc = "\n# Feature flags\n")]
#![cfg_attr(docsrs, doc = document_features::document_features!())]
#![deny(missing_docs)]

mod glyphs;
mod input;
mod lengths;
mod metrics;
mod scrollbar;

/// Re-export of the selected crossterm version.
///
/// See `tui_scrollbar::crossterm` for the version selection rules.
#[cfg(all(feature = "crossterm_0_28", not(feature = "crossterm_0_29")))]
pub use ::crossterm_0_28 as crossterm;
/// Re-export of the selected crossterm version.
///
/// This crate supports multiple crossterm versions via feature flags:
///
/// - `crossterm` selects the latest supported crossterm version (currently 0.29).
/// - `crossterm_0_28` selects `crossterm` 0.28.
/// - `crossterm_0_29` selects `crossterm` 0.29.
///
/// When both 0.28 and 0.29 are enabled, this re-export points to 0.29. Downstream code can use
/// `tui_scrollbar::crossterm::event::*` without needing to match the dependency name/version
/// selection logic.
#[cfg(feature = "crossterm_0_29")]
pub use ::crossterm_0_29 as crossterm;

pub use crate::glyphs::GlyphSet;
pub use crate::input::{
    PointerButton, PointerEvent, PointerEventKind, ScrollAxis, ScrollBarInteraction, ScrollCommand,
    ScrollEvent, ScrollWheel,
};
pub use crate::lengths::ScrollLengths;
pub use crate::metrics::{CellFill, HitTest, ScrollMetrics, SUBCELL};
pub use crate::scrollbar::{ScrollBar, ScrollBarArrows, ScrollBarOrientation, TrackClickBehavior};
