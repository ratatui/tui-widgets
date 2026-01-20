//! A [Ratatui] widget to render charming playing cards in the terminal. Part of the [tui-widgets]
//! suite by [Joshka].
//!
//! ![demo](https://vhs.charm.sh/vhs-34mhPM1Juk2XnnLTGpOtE9.gif)
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
//! # Usage
//!
//! Create a `Card` and render it directly in a frame.
//!
//! ```no_run
//! use tui_cards::{Card, CardSize, Rank, Suit};
//!
//! # fn draw(frame: &mut ratatui::Frame) {
//! let card = Card::new(Rank::Ace, Suit::Spades, CardSize::Normal);
//! frame.render_widget(&card, frame.area());
//! # }
//! ```
//!
//! # Demo
//!
//! ```shell
//! cargo run --example card
//! ```
//!
//! # More widgets
//!
//! For the full suite of widgets, see [tui-widgets].
//!
//! [Crate]: https://crates.io/crates/tui-cards
//! [Docs]: https://docs.rs/tui-cards/
//! [Dependency Status]: https://deps.rs/repo/github/joshka/tui-widgets
//! [Coverage]: https://app.codecov.io/gh/joshka/tui-widgets
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//! [Crate badge]: https://img.shields.io/crates/v/tui-cards?logo=rust&style=flat
//! [Docs Badge]: https://img.shields.io/docsrs/tui-cards?logo=rust&style=flat
//! [Deps Badge]: https://deps.rs/repo/github/joshka/tui-widgets/status.svg?style=flat
//! [License Badge]: https://img.shields.io/crates/l/tui-cards?style=flat
//! [License]: https://github.com/joshka/tui-widgets/blob/main/LICENSE-MIT
//! [Coverage Badge]:
//!     https://img.shields.io/codecov/c/github/joshka/tui-widgets?logo=codecov&style=flat
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat
//!
//! [GitHub Repository]: https://github.com/joshka/tui-widgets
//! [API Docs]: https://docs.rs/tui-cards/
//! [Examples]: https://github.com/joshka/tui-widgets/tree/main/tui-cards/examples
//! [Changelog]: https://github.com/joshka/tui-widgets/blob/main/tui-cards/CHANGELOG.md
//! [Contributing]: https://github.com/joshka/tui-widgets/blob/main/CONTRIBUTING.md
//! [Joshka]: https://github.com/joshka
//! [tui-widgets]: https://crates.io/crates/tui-widgets
use std::iter::zip;

use indoc::indoc;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Style, Stylize};
use ratatui_core::widgets::Widget;
use strum::{Display, EnumIter};

/// A playing card.
///
/// Card dimensions depend on the size:
/// - `CardSize::Normal`: 14 characters wide × 9 lines tall
/// - `CardSize::Small`: 8 characters wide × 5 lines tall
///
/// # Example
///
/// ```rust
/// use tui_cards::{Card, CardSize, Rank, Suit};
/// # fn draw(frame: &mut ratatui::Frame) {
/// let card = Card::new(Rank::Ace, Suit::Spades, CardSize::Normal);
/// frame.render_widget(&card, frame.area());
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub size: CardSize,
    pub style: Style,
}

/// The size of a card when rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardSize {
    /// Small card: 8 characters wide × 5 lines tall.
    Small,
    /// Normal card: 14 characters wide × 9 lines tall.
    #[default]
    Normal,
}

impl CardSize {
    /// Returns the dimensions (width, height) of the card.
    pub const fn dimensions(self) -> (u16, u16) {
        match self {
            Self::Small => (8, 5),
            Self::Normal => (14, 9),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Card {
    pub const fn new(rank: Rank, suit: Suit, size: CardSize) -> Self {
        Self {
            rank,
            suit,
            size,
            style: Style::new(),
        }
    }

    /// Sets the style of the card.
    ///
    /// The foreground color defaults to the suit color if not specified.
    /// The background defaults to transparent (`Color::Reset`) if not specified.
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn as_colored_symbol(&self) -> String {
        format!(
            "{}{}",
            self.rank.as_symbol(),
            self.suit.as_four_color_symbol()
        )
    }
}

impl Rank {
    pub const fn as_symbol(self) -> char {
        match self {
            Self::Ace => 'A',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Ten => 'T',
            Self::Jack => 'J',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }
}

impl Suit {
    pub const fn color(self) -> Color {
        match self {
            Self::Clubs => Color::Green,
            Self::Diamonds => Color::Blue,
            Self::Hearts => Color::Red,
            Self::Spades => Color::Black,
        }
    }

    pub const fn as_symbol(self) -> char {
        match self {
            Self::Clubs => '♣',
            Self::Diamonds => '♦',
            Self::Hearts => '♥',
            Self::Spades => '♠',
        }
    }

    pub const fn as_colored_symbol(self) -> &'static str {
        match self {
            Self::Clubs => "\u{2663}\u{FE0F}",
            Self::Diamonds => "\u{2666}\u{FE0F}",
            Self::Hearts => "\u{2665}\u{FE0F}",
            Self::Spades => "\u{2660}\u{FE0F}",
        }
    }

    pub const fn as_four_color_symbol(self) -> &'static str {
        match self {
            Self::Clubs => "\u{2618}\u{FE0F}",     // shamrock
            Self::Diamonds => "\u{1F537}\u{FE0F}", // blue diamond
            Self::Hearts => "\u{2665}\u{FE0F}",
            Self::Spades => "\u{2660}\u{FE0F}",
        }
    }
}

impl Rank {
    /// Returns the template for the given card size.
    pub const fn template(self, size: CardSize) -> &'static str {
        match size {
            CardSize::Small => self.small_template(),
            CardSize::Normal => self.normal_template(),
        }
    }

    /// Returns the small template (8 wide × 5 tall).
    pub const fn small_template(self) -> &'static str {
        match self {
            Self::Ace => indoc! {"
                ╭──────╮
                │Ax  x │
                │      │
                │ x  xA│
                ╰──────╯"},
            Self::Two => indoc! {"
                ╭──────╮
                │2x  x │
                │      │
                │ x  x2│
                ╰──────╯"},
            Self::Three => indoc! {"
                ╭──────╮
                │3x  x │
                │      │
                │ x  x3│
                ╰──────╯"},
            Self::Four => indoc! {"
                ╭──────╮
                │4x  x │
                │      │
                │ x  x4│
                ╰──────╯"},
            Self::Five => indoc! {"
                ╭──────╮
                │5x  x │
                │      │
                │ x  x5│
                ╰──────╯"},
            Self::Six => indoc! {"
                ╭──────╮
                │6x  x │
                │      │
                │ x  x6│
                ╰──────╯"},
            Self::Seven => indoc! {"
                ╭──────╮
                │7x  x │
                │      │
                │ x  x7│
                ╰──────╯"},
            Self::Eight => indoc! {"
                ╭──────╮
                │8x  x │
                │      │
                │ x  x8│
                ╰──────╯"},
            Self::Nine => indoc! {"
                ╭──────╮
                │9x  x │
                │      │
                │ x  x9│
                ╰──────╯"},
            Self::Ten => indoc! {"
                ╭──────╮
                │10  x │
                │      │
                │ x  10│
                ╰──────╯"},
            Self::Jack => indoc! {"
                ╭──────╮
                │Jx    │
                │  JJ  │
                │    xJ│
                ╰──────╯"},
            Self::Queen => indoc! {"
                ╭──────╮
                │Qx    │
                │  QQ  │
                │    xQ│
                ╰──────╯"},
            Self::King => indoc! {"
                ╭──────╮
                │Kx    │
                │  KK  │
                │    xK│
                ╰──────╯"},
        }
    }

    /// Returns the normal template (14 wide × 9 tall).
    pub const fn normal_template(self) -> &'static str {
        match self {
            Self::Ace => indoc! {"
                ╭────────────╮
                │ A          │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │          A │
                ╰────────────╯"},
            Self::Two => indoc! {"
                ╭────────────╮
                │ 2   xx     │
                │            │
                │            │
                │            │
                │            │
                │            │
                │     xx   2 │
                ╰────────────╯"},
            Self::Three => indoc! {"
                ╭────────────╮
                │ 3   xx     │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │     xx   3 │
                ╰────────────╯"},
            Self::Four => indoc! {"
                ╭────────────╮
                │ 4xx    xx  │
                │            │
                │            │
                │            │
                │            │
                │            │
                │  xx    xx4 │
                ╰────────────╯"},
            Self::Five => indoc! {"
                ╭────────────╮
                │ 5xx    xx  │
                │            │
                │            │
                │     xx     │
                │            │
                │            │
                │  xx    xx5 │
                ╰────────────╯"},
            Self::Six => indoc! {"
                ╭────────────╮
                │ 6xx    xx  │
                │            │
                │            │
                │  xx    xx  │
                │            │
                │            │
                │  xx    xx6 │
                ╰────────────╯"},
            Self::Seven => indoc! {"
                ╭────────────╮
                │ 7xx    xx  │
                │            │
                │     xx     │
                │  xx    xx  │
                │            │
                │            │
                │  xx    xx7 │
                ╰────────────╯"},
            Self::Eight => indoc! {"
                ╭────────────╮
                │ 8xx    xx  │
                │            │
                │     xx     │
                │  xx    xx  │
                │     xx     │
                │            │
                │  xx    xx8 │
                ╰────────────╯"},
            Self::Nine => indoc! {"
                ╭────────────╮
                │ 9xx    xx  │
                │            │
                │  xx    xx  │
                │     xx     │
                │  xx    xx  │
                │            │
                │  xx    xx9 │
                ╰────────────╯
                "},
            Self::Ten => indoc! {"
                ╭────────────╮
                │10xx    xx  │
                │     xx     │
                │  xx    xx  │
                │            │
                │  xx    xx  │
                │     xx     │
                │  xx    xx10│
                ╰────────────╯"},
            Self::Jack => indoc! {"
                ╭────────────╮
                │ Jxx        │
                │       JJ   │
                │       JJ   │
                │       JJ   │
                │  JJ   JJ   │
                │   JJJJJ    │
                │        xxJ │
                ╰────────────╯"},
            Self::Queen => indoc! {"
                ╭────────────╮
                │ Qxx        │
                │   QQQQQ    │
                │  QQ   QQ   │
                │  QQ   QQ   │
                │  QQ   QQ   │
                │   QQQQ  Q  │
                │        xxQ │
                ╰────────────╯
            "},
            Self::King => indoc! {"
                ╭────────────╮
                │ Kxx        │
                │  KK    KK  │
                │  KK   KK   │
                │  KK KK     │
                │  KK   KK   │
                │  KK    KK  │
                │        xxK │
                ╰────────────╯"},
        }
    }
}

impl Widget for &Card {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let template = self.rank.template(self.size);
        let card = match self.size {
            CardSize::Small => {
                let symbol = self.suit.as_symbol();
                template.replace('x', &symbol.to_string())
            }
            CardSize::Normal => {
                let symbol = self.suit.as_four_color_symbol();
                template.replace("xx", symbol)
            }
        };
        let fg = self.style.fg.unwrap_or(self.suit.color());
        let bg = self.style.bg.unwrap_or(Color::Reset);
        for (line, row) in zip(card.lines(), area.rows()) {
            let span = line.fg(fg).bg(bg);
            span.render(row, buf);
        }
    }
}
