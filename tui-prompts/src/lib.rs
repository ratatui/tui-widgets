//! A [Ratatui] widget set for friendly prompts and input flows. Part of the [tui-widgets] suite by
//! [Joshka].
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
//! cargo add ratatui tui-prompts crossterm
//! ```
//!
//! # Usage
//!
//! Pick a prompt type, keep its state, and render it inside your UI.
//!
//! ## Text Prompt
//!
//! <details>
//! <summary>Code</summary>
//!
//! ```rust
//! use ratatui::layout::{Constraint, Direction, Layout, Rect};
//! use ratatui::Frame;
//! use tui_prompts::{Prompt, TextPrompt, TextRenderStyle, TextState};
//!
//! struct App<'a> {
//!     username_state: TextState<'a>,
//!     password_state: TextState<'a>,
//!     invisible_state: TextState<'a>,
//! }
//!
//! impl<'a> App<'a> {
//!     fn draw_ui(&mut self, frame: &mut Frame) {
//!         let (username_area, password_area, invisible_area) = split_layout(frame.area());
//!
//!         TextPrompt::from("Username")
//!             .draw(frame, username_area, &mut self.username_state);
//!
//!         TextPrompt::from("Password")
//!             .with_render_style(TextRenderStyle::Password)
//!             .draw(frame, password_area, &mut self.password_state);
//!
//!         TextPrompt::from("Invisible")
//!             .with_render_style(TextRenderStyle::Invisible)
//!             .draw(frame, invisible_area, &mut self.invisible_state);
//!     }
//! }
//!
//! fn split_layout(area: Rect) -> (Rect, Rect, Rect) {
//!     let rows = Layout::default()
//!         .direction(Direction::Vertical)
//!         .constraints([
//!             Constraint::Length(1),
//!             Constraint::Length(1),
//!             Constraint::Length(1),
//!         ])
//!         .split(area);
//!     (rows[0], rows[1], rows[2])
//! }
//! ```
//!
//! </details>
//!
//! ![Text Prompt](https://vhs.charm.sh/vhs-7gLcGtWJWDlQZqcMlhrpRG.gif)
//!
//! See the [text example] for more details.
//!
//! ## Soft Wrapping
//!
//! Text is automatically character wrapped to fit in the render area.
//!
//! ![Multi-line](https://vhs.charm.sh/vhs-5zzgSyRXy6IjBahoe1esDi.gif)
//!
//! See the [multi line example] for more details.
//!
//! # Features
//!
//! - [x] Text prompt
//! - [x] Password prompt
//! - [x] Invisible prompt
//! - [x] Readline / emacs style Key Bindings
//! - [x] Crossterm backend
//! - [x] Soft wrapping single lines
//! - [ ] Multi-line input
//! - [ ] Scrolling
//! - [ ] More prompt types:
//!   - [ ] Number
//!   - [ ] Confirm
//!   - [ ] List
//!   - [ ] Toggle
//!   - [ ] Select
//!   - [ ] Multi-select
//!   - [ ] Autocomplete
//!   - [ ] Autocomplete multi-select
//!   - [ ] Date
//! - [ ] Bracketed paste
//! - [ ] Validation
//! - [ ] Default initial value
//! - [ ] Custom style
//! - [ ] Themes
//! - [ ] Custom formatting
//! - [ ] Backend agnostic keyboard event handling
//!   - [Termion]
//!   - [Termwiz]
//! - [ ] Customizable key bindings
//! - [ ] Handle more advanced multi-key bindings e.g. `^[b` and `^[f`
//! - [ ] Prompt chaining
//!
//! # Key Bindings
//!
//! | Key | Action |
//! | --- | --- |
//! | Home, Ctrl+A | Move cursor to beginning of line |
//! | End, Ctrl+E | Move cursor to end of line |
//! | Left, Ctrl+B | Move cursor one character left |
//! | Right, Ctrl+F | Move cursor one character right |
//! | Backspace (Delete on Mac), Ctrl+H | Delete character before cursor |
//! | Delete (Fn+Delete on Mac), Ctrl+D | Delete character at cursor |
//! | Ctrl+K | Delete all characters from the cursor to the end of line |
//! | Ctrl+U | Delete the entire line |
//! | Enter | Complete the prompt |
//! | Escape, Ctrl+C | Abort the prompt |
//!
//! # More widgets
//!
//! For the full suite of widgets, see [tui-widgets].
//!
//! [Joshka]: https://github.com/joshka
//! [tui-widgets]: https://crates.io/crates/tui-widgets
//! [Crate]: https://crates.io/crates/tui-prompts
//! [Docs]: https://docs.rs/tui-prompts/
//! [Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
//! [Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//! [GitHub Repository]: https://github.com/ratatui/tui-widgets
//! [API Docs]: https://docs.rs/tui-prompts/
//! [Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-prompts/examples
//! [text example]: https://github.com/ratatui/tui-widgets/tree/main/tui-prompts/examples/text.rs
//! [multi line example]: https://github.com/ratatui/tui-widgets/tree/main/tui-prompts/examples/multi_line.rs
//! [Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-prompts/CHANGELOG.md
//! [Contributing]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md
//! [Crate badge]: https://img.shields.io/crates/v/tui-prompts?logo=rust&style=flat
//! [Docs Badge]: https://img.shields.io/docsrs/tui-prompts?logo=rust&style=flat
//! [Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
//! [License Badge]: https://img.shields.io/crates/l/tui-prompts?style=flat
//! [License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
//! [Coverage Badge]:
//!     https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
//! [Discord Badge]:
//!     https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat
//! [Ratatui]: https://crates.io/crates/ratatui
//! [Termion]: https://crates.io/crates/termion
//! [Termwiz]: https://crates.io/crates/termwiz

mod prompt;
mod status;

mod text_prompt;
mod text_state;

pub use prompt::*;
pub use status::*;
pub use text_prompt::*;
pub use text_state::*;

pub mod prelude {
    pub use crate::{FocusState, Prompt, State, Status, TextPrompt, TextRenderStyle, TextState};
}
