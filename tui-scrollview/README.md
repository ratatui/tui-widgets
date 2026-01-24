# tui-scrollview

<!-- cargo-rdme start -->

A [Ratatui] widget to build smooth scrollable views. Part of the [tui-widgets] suite by [Joshka].

![Demo](https://vhs.charm.sh/vhs-6PuT3pdwSTp4aTvKrCBx9F.gif)

(Note: a github bug stops the example gif above being displayed, but you can view it at:
<https://vhs.charm.sh/vhs-6PuT3pdwSTp4aTvKrCBx9F.gif>)

[![Crate badge]][Crate]
[![Docs Badge]][Docs]
[![Deps Badge]][Dependency Status]
[![License Badge]][License]
[![Coverage Badge]][Coverage]
[![Discord Badge]][Ratatui Discord]

[GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]

## Installation

```shell
cargo add tui-scrollview
```

## Usage

```rust
use std::iter;

use ratatui::layout::Size;
use ratatui::prelude::*;
use ratatui::widgets::*;
use tui_scrollview::{ScrollView, ScrollViewState};

struct MyScrollableWidget;

impl StatefulWidget for MyScrollableWidget {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // 100 lines of text
        let line_numbers = (1..=100).map(|i| format!("{:>3} ", i)).collect::<String>();
        let content =
            iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
                .take(100)
                .collect::<String>();

        let content_size = Size::new(100, 30);
        let mut scroll_view = ScrollView::new(content_size);

        // the layout doesn't have to be hardcoded like this, this is just an example
        scroll_view.render_widget(Paragraph::new(line_numbers), Rect::new(0, 0, 5, 100));
        scroll_view.render_widget(Paragraph::new(content), Rect::new(5, 0, 95, 100));

        scroll_view.render(buf.area, buf, state);
    }
}
```

## Full Example

A full example can be found in the [examples directory].
[scrollview.rs]

This example shows a scrollable view with two paragraphs of text, one for the line numbers and
one for the text. On top of this a Gauge widget is rendered to show that this can be used in
combination with any other widget.

## More widgets

For the full suite of widgets, see [tui-widgets].

[Crate]: https://crates.io/crates/tui-scrollview
[Docs]: https://docs.rs/tui-scrollview/
[Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
[Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
[Crate badge]: https://img.shields.io/crates/v/tui-scrollview?logo=rust&style=flat
[Docs Badge]: https://img.shields.io/docsrs/tui-scrollview?logo=rust&style=flat
[Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
[License Badge]: https://img.shields.io/crates/l/tui-scrollview?style=flat
[License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
[Coverage Badge]:
    https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat

[GitHub Repository]: https://github.com/ratatui/tui-widgets
[API Docs]: https://docs.rs/tui-scrollview/
[Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-scrollview/examples
[examples directory]: https://github.com/ratatui/tui-widgets/tree/main/tui-scrollview/examples
[scrollview.rs]:
    https://github.com/ratatui/tui-widgets/tree/main/tui-scrollview/examples/scrollview.rs
[Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-scrollview/CHANGELOG.md
[Contributing]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md

[Ratatui]: https://crates.io/crates/ratatui

[Joshka]: https://github.com/joshka
[tui-widgets]: https://crates.io/crates/tui-widgets

<!-- cargo-rdme end -->

## TODO

- [ ] Conditionally show scrollbar
- [x] Implement horizontal scrolling and bar

## License

Copyright (c) Josh McKinney

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

[LICENSE-APACHE]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
[CONTRIBUTING.md]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

See [CONTRIBUTING.md].
