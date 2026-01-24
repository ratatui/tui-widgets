# tui-box-text

<!-- cargo-rdme start -->

A [Ratatui] widget to draw delightfully boxy text with line-drawing characters. Part of the
[tui-widgets] suite by [Joshka].

![Demo](https://vhs.charm.sh/vhs-6ldj2r9v3mIaSzk8H7Jp8t.gif)

[![Crate badge]][Crate]
[![Docs Badge]][Docs]
[![Deps Badge]][Dependency Status]
[![License Badge]][License]
[![Coverage Badge]][Coverage]
[![Discord Badge]][Ratatui Discord]

[GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]

## Usage

Create a `BoxChar` and render it into a region of your frame.

```rust
use tui_box_text::BoxChar;

let letter = BoxChar::new('A');
frame.render_widget(&letter, frame.area());
```

## More widgets

For the full suite of widgets, see [tui-widgets].

[Crate]: https://crates.io/crates/tui-box-text
[Docs]: https://docs.rs/tui-box-text/
[Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
[Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
[Crate badge]: https://img.shields.io/crates/v/tui-box-text?logo=rust&style=flat
[Docs Badge]: https://img.shields.io/docsrs/tui-box-text?logo=rust&style=flat
[Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
[License Badge]: https://img.shields.io/crates/l/tui-box-text?style=flat
[License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
[Coverage Badge]:
    https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat

[GitHub Repository]: https://github.com/ratatui/tui-widgets
[API Docs]: https://docs.rs/tui-box-text/
[Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-box-text/examples
[Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-box-text/CHANGELOG.md
[Contributing]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md
[Joshka]: https://github.com/joshka
[tui-widgets]: https://crates.io/crates/tui-widgets

<!-- cargo-rdme end -->

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
