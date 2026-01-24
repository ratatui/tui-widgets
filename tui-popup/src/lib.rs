//! A [Ratatui] widget to show a snappy popup overlay. Part of the [tui-widgets] suite by [Joshka].
//!
//! ![demo](https://vhs.charm.sh/vhs-q5Kz0QP3zmrBlQ6dofjMh.gif)
//!
//! The popup widget is a simple widget that renders a popup in the center of the screen.
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
//! cargo add tui-popup
//! ```
//!
//! # Usage
//!
//! Build a `Popup` with content and render it over your frame.
//!
//! ```rust
//! use ratatui::style::{Style, Stylize};
//! use ratatui::Frame;
//! use tui_popup::Popup;
//!
//! fn render_popup(frame: &mut Frame) {
//!     let popup = Popup::new("Press any key to exit")
//!         .title("tui-popup demo")
//!         .style(Style::new().white().on_blue());
//!     frame.render_widget(popup, frame.area());
//! }
//! ```
//!
//! # State
//!
//! The widget supports storing the position of the popup in `PopupState`. This is experimental and
//! the exact API for this will likely change.
//!
//! ```rust
//! # #[cfg(feature = "crossterm")]
//! # {
//! use crossterm::event::{KeyCode, KeyEvent};
//! use ratatui::style::{Style, Stylize};
//! use ratatui::Frame;
//! use tui_popup::{Popup, PopupState};
//!
//! fn render_stateful_popup(frame: &mut Frame, popup_state: &mut PopupState) {
//!     let popup = Popup::new("Press any key to exit")
//!         .title("tui-popup demo")
//!         .style(Style::new().white().on_blue());
//!     frame.render_stateful_widget(popup, frame.area(), popup_state);
//! }
//!
//! fn handle_key(event: KeyEvent, state: &mut PopupState) {
//!     match event.code {
//!         KeyCode::Up => state.move_up(1),
//!         KeyCode::Down => state.move_down(1),
//!         KeyCode::Left => state.move_left(1),
//!         KeyCode::Right => state.move_right(1),
//!         _ => {}
//!     }
//! }
//! # }
//! ```
//!
//! The popup can automatically handle being moved around by the mouse, by passing in the column and
//! row of mouse up, down, or drag events.
//!
//! ```rust
//! # #[cfg(feature = "crossterm")]
//! # {
//! use crossterm::event::{Event, MouseButton, MouseEventKind};
//! use tui_popup::PopupState;
//!
//! fn handle_mouse(event: Event, popup_state: &mut PopupState) {
//!     if let Event::Mouse(event) = event {
//!         match event.kind {
//!             MouseEventKind::Down(MouseButton::Left) => {
//!                 popup_state.mouse_down(event.column, event.row)
//!             }
//!             MouseEventKind::Up(MouseButton::Left) => {
//!                 popup_state.mouse_up(event.column, event.row);
//!             }
//!             MouseEventKind::Drag(MouseButton::Left) => {
//!                 popup_state.mouse_drag(event.column, event.row);
//!             }
//!             _ => {}
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! The popup also supports rendering arbitrary widgets by implementing [`KnownSize`] (or wrapping
//! them with [`KnownSizeWrapper`]). This makes it possible to support wrapping and scrolling in a
//! `Paragraph` widget, or scrolling any amount of widgets using [tui-scrollview].
//!
//! ```rust
//! use ratatui::prelude::*;
//! use ratatui::text::{Span, Text};
//! use ratatui::widgets::Paragraph;
//! use tui_popup::{KnownSizeWrapper, Popup};
//!
//! fn render_scrollable_popup(frame: &mut Frame, scroll: u16) {
//!     let lines: Text = (0..10).map(|i| Span::raw(format!("Line {}", i))).collect();
//!     let paragraph = Paragraph::new(lines).scroll((scroll, 0));
//!     let sized_paragraph = KnownSizeWrapper {
//!         inner: paragraph,
//!         width: 21,
//!         height: 5,
//!     };
//!     let popup = Popup::new(sized_paragraph)
//!         .title("scroll: ↑/↓ quit: Esc")
//!         .style(Style::new().white().on_blue());
//!     frame.render_widget(popup, frame.area());
//! }
//! ```
//!
//! ![paragraph example](https://vhs.charm.sh/vhs-A3mwcn9IngIc0hpl2AsXM.gif)
//!
//! # Features
//!
//! - [x] automatically centers
//! - [x] automatically sizes to content
//! - [x] style popup
//! - [x] move the popup (using state)
//! - [x] handle mouse events for dragging
//! - [x] move to position
//! - [ ] resize
//! - [ ] set border set / style
//! - [ ] add close button
//! - [ ] add nicer styling of header etc.
//! - [ ] configure text wrapping in body to conform to a specific size
//!
//! # More widgets
//!
//! For the full suite of widgets, see [tui-widgets].
//!
//!
//! [Crate]: https://crates.io/crates/tui-popup
//! [Docs]: https://docs.rs/tui-popup/
//! [Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
//! [Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//! [Crate badge]: https://img.shields.io/crates/v/tui-popup?logo=rust&style=flat
//! [Docs Badge]: https://img.shields.io/docsrs/tui-popup?logo=rust&style=flat
//! [Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
//! [License Badge]: https://img.shields.io/crates/l/tui-popup?style=flat
//! [License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
//! [Coverage Badge]:
//!     https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat
//!
//! [GitHub Repository]: https://github.com/ratatui/tui-widgets
//! [API Docs]: https://docs.rs/tui-popup/
//! [Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-popup/examples
//! [Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-popup/CHANGELOG.md
//! [Contributing]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md
//! [KnownSize]: https://docs.rs/tui-popup/latest/tui_popup/trait.KnownSize.html
//! [KnownSizeWrapper]: https://docs.rs/tui-popup/latest/tui_popup/struct.KnownSizeWrapper.html
//! [tui-scrollview]: https://crates.io/crates/tui-scrollview
//!
//! [Joshka]: https://github.com/joshka
//! [tui-widgets]: https://crates.io/crates/tui-widgets
#![cfg_attr(docsrs, doc = "\n# Feature flags\n")]
#![cfg_attr(docsrs, doc = document_features::document_features!())]

mod known_size;
mod known_size_wrapper;
mod popup;
mod popup_state;

pub use crate::known_size::KnownSize;
pub use crate::known_size_wrapper::KnownSizeWrapper;
pub use crate::popup::Popup;
pub use crate::popup_state::{DragState, PopupState};
