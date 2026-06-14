//! A [Ratatui] widget to draw delightfully boxy text with line-drawing characters. Part of the
//! [tui-widgets] suite by [Joshka].
//!
//! ![Demo](https://vhs.charm.sh/vhs-6ldj2r9v3mIaSzk8H7Jp8t.gif)
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
//! Create a `BoxChar` and render it into a region of your frame.
//!
//! ```rust
//! use tui_box_text::BoxChar;
//!
//! # fn draw(frame: &mut ratatui::Frame) {
//! let letter = BoxChar::new('A');
//! frame.render_widget(&letter, frame.area());
//! # }
//! ```
//!
//! # More widgets
//!
//! For the full suite of widgets, see [tui-widgets].
//!
//! [Crate]: https://crates.io/crates/tui-box-text
//! [Docs]: https://docs.rs/tui-box-text/
//! [Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
//! [Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//! [Crate badge]: https://img.shields.io/crates/v/tui-box-text?logo=rust&style=flat
//! [Docs Badge]: https://img.shields.io/docsrs/tui-box-text?logo=rust&style=flat
//! [Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
//! [License Badge]: https://img.shields.io/crates/l/tui-box-text?style=flat
//! [License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
//! [Coverage Badge]:
//!     https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat
//!
//! [GitHub Repository]: https://github.com/ratatui/tui-widgets
//! [API Docs]: https://docs.rs/tui-box-text/
//! [Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-box-text/examples
//! [Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-box-text/CHANGELOG.md
//! [Contributing]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md
//! [Joshka]: https://github.com/joshka
//! [tui-widgets]: https://crates.io/crates/tui-widgets

use std::collections::HashMap;
use std::iter::zip;
use std::sync::LazyLock;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::Widget;

pub struct BoxChar(char);

impl BoxChar {
    pub const fn new(c: char) -> Self {
        Self(c)
    }
}

impl Widget for &BoxChar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let c = self
            .0
            .to_uppercase() // TODO: add support for lower case characters
            .next()
            .and_then(|c| CHARS.get(&c))
            .unwrap_or(&" ");
        let lines = c.lines().collect::<Vec<_>>();
        for (line, row) in zip(lines, area.rows()) {
            for (char, cell) in zip(line.chars(), row.columns()) {
                buf[cell.as_position()].set_symbol(&char.to_string());
            }
        }
    }
}

/// A macro for creating a hash table that maps single characters to strings.
macro_rules! char_table {
    ( $($char:expr_2021 => $repr:expr_2021),* $(,)? ) => {
        {
            let mut table = ::std::collections::HashMap::new();
            $(
                table.insert($char, ::indoc::indoc! {$repr});
            )*
            table
        }
    };
}

/// A hash table that maps single characters to strings that are 3 lines high and made up of box
/// drawing characters.
static CHARS: LazyLock<HashMap<char, &str>> = LazyLock::new(|| {
    char_table!(
        ' ' => " ",
        '!' => "│
                ╵
                ╵",
        '"' => "╭╭",
        '#' => "┼─┼
                ┼─┼",
        '$' => "╭┼╴
                └┼┐
                ╶┼╯",
        '%' => "o╱
                ╱o",
        '&' => "╭─╮
                ╭╲╯
                ╰─╲",
        '\'' => "╭",
        '(' => "╭
                │
                ╰",
        ')' => "╮
                │
                ╯",
        '*' => "
        
                *
                ",
        '+' => "
                 ╷
                ╶┼╴
                 ╵",
        ',' => "

                
                ╯",
        '-' => "

                ──
                 ",
        '.' => "

                .
                 ",
        '/' => "
                 ╱
                ╱
                ",
        '0' => "╭─╮
                │╱│
                ╰─╯",
        '1' => "
                 ╶┐
                  │
                 ─┴─",
        '2' => "╶─╮
                ┌─┘
                └─╴",
        '3' => "╶─╮
                ╶─┤
                ╶─╯",
        '4' => "╷ ╷
                ╰─┤
                  ╵",
        '5' => "┌─╴
                └─╮
                ╰─╯",
        '6' => "╭─╴
                ├─╮
                ╰─╯",
        '7' => "╶─┐
                 ╱
                ╵  ",
        '8' => "╭─╮
                ├─┤
                ╰─╯",
        '9' => "╭─╮
                ╰─┤
                ╶─╯",
        ':' => "╷
                ╵
                │
                 ",
        ';' => "╷
                ╵
                ╯",
        '<' => "
                 ╱
                 ╲
                 ",
        '=' => "
                ──
                ──",
        '>' => "
                 ╲
                 ╱
                 ",
        '?' => "
                ╶─╮
                 ╭╯
                 ╷",
        '@' => "╭─╮
                ╭╮│
                ╰┴╯",
        'A' => "╭─╮
                ├─┤
                ╵ ╵",
        'B' => "┌╮
                ├┴╮
                ╰─╯",
        'C' => "╭─╮
                │
                ╰─╯",
        'D' => "┌─╮
                │ │
                └─╯",
        'E' => "┌─╴
                ├─
                └─╴",
        'F' => "┌─╴
                ├─
                ╵  ",
        'G' => "╭─╮
                │─╮
                ╰─╯",
        'H' => "╷ ╷
                ├─┤
                ╵ ╵",
        'I' => "╶┬╴
                 │
                ╶┴╴",
        'J' => " ╶┐
                  │
                ╰─╯",
        'K' => "╷╭
                ├┴╮
                ╵ ╵",
        'L' => "╷
                │
                └──",
        'M' => "╭┬╮
                │││
                ╵╵╵",
        'N' => "╭─╮
                │ │
                ╵ ╵",
        'O' => "╭─╮
                │ │
                ╰─╯",
        'P' => "┌─╮
                ├─╯
                ╵  ",
        'Q' => "╭─╮
                │ │
                ╰─╳",
        'R' => "┌─╮
                ├┬╯
                ╵╰ ",
        'S' => "╭─╮
                ╰─╮ 
                ╰─╯",
        'T' => "
                ╶┬╴
                 │
                 ╵",
        'U' => "╷ ╷
                │ │
                ╰─╯",
        'V' => "╷ ╷
                │ │
                └─╯",
        'W' => "╷╷╷
                │││
                ╰┴╯",
        'X' => "╮ ╭
                ╰─╮ 
                ╯ ╰",
        'Y' => "╮ ╭
                ╰┬╯ 
                 ╵",
        'Z' => "╶─╮
                 ╱
                ╰─╴",
        '[' => "┌─
                │
                └─",
        '\\' => "
                 ╲
                  ╲
                ",
        ']' => "─┐
                 │
                ─┘",
        '^' => "╱╲",
        '_' => "

                ──",
        '`' => "╮",
        '{' => "
                ╭
                ┤
                ╰",
        '|' => "│
                │
                │",
        '}' => "╮
                ├
                ╯",
        '~' => "
                ╭╮
                 ╰╯",
    )
});
