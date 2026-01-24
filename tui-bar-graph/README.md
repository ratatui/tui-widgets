# Tui-bar-graph

<!-- cargo-rdme start -->

A [Ratatui] widget to render bold, colorful bar graphs. Part of the [tui-widgets] suite by
[Joshka].

![Braille Rainbow](https://vhs.charm.sh/vhs-1sx9Ht6NzU6e28Cl51jJVv.gif)
![Solid Plasma](https://vhs.charm.sh/vhs-7pWuLtZpzrz1OVD04cMt1a.gif)
![Quadrant Magma](https://vhs.charm.sh/vhs-1rx6XQ9mLiO8qybSBXRGwn.gif)
![Octant Viridis](https://vhs.charm.sh/vhs-7BevtFvn5S7j8jcAJrxl1F.gif)

<details><summary>More examples</summary>

![Braille Magma](https://vhs.charm.sh/vhs-4RDwcz9DApA90iJYMQXHXd.gif)
![Braille Viridis](https://vhs.charm.sh/vhs-5ylsZAdKGPiHUYboOpZFZL.gif)
![Solid Inferno](https://vhs.charm.sh/vhs-4z1gbmJ50KGz2TPej3mnVf.gif)
![Solid Sinebow](https://vhs.charm.sh/vhs-63aAmMhcfMT8CnWCV20dsn.gif)
![Quadrant Plasma](https://vhs.charm.sh/vhs-5o8AfNgQZAT1U4hOaLtY7m.gif)
![Quadrant Sinebow](https://vhs.charm.sh/vhs-1zAyLkSvNGTKL1SGHRyZFD.gif)
![Octant Inferno](https://vhs.charm.sh/vhs-3bwxZkh1WcSFUkVzpBXWb9.gif)
![Octant Rainbow](https://vhs.charm.sh/vhs-6eDjdEbRK4xWNtVpHuTkIh.gif)

</details>

Uses the [Colorgrad] crate for gradient coloring.

[![Crate badge]][Crate]
[![Docs Badge]][Docs]
[![Deps Badge]][Dependency Status]
[![License Badge]][License]
[![Coverage Badge]][Coverage]
[![Discord Badge]][Ratatui Discord]

[GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]

## Installation

```shell
cargo add ratatui tui-bar-graph
```

## Usage

Build a `BarGraph` with your data and render it in a widget area.

```rust
use tui_bar_graph::{BarGraph, BarStyle, ColorMode};

let data = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
let bar_graph = BarGraph::new(data)
    .with_gradient(colorgrad::preset::turbo())
    .with_bar_style(BarStyle::Braille)
    .with_color_mode(ColorMode::VerticalGradient);
frame.render_widget(bar_graph, area);
```

## More widgets

For the full suite of widgets, see [tui-widgets].

[Colorgrad]: https://crates.io/crates/colorgrad
[Ratatui]: https://crates.io/crates/ratatui
[Crate]: https://crates.io/crates/tui-bar-graph
[Docs]: https://docs.rs/tui-bar-graph/
[Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
[Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
[Crate badge]: https://img.shields.io/crates/v/tui-bar-graph.svg?logo=rust&style=flat
[Docs Badge]: https://img.shields.io/docsrs/tui-bar-graph?logo=rust&style=flat
[Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
[License Badge]: https://img.shields.io/crates/l/tui-bar-graph.svg?style=flat
[License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
[Coverage Badge]:
    https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat

[GitHub Repository]: https://github.com/ratatui/tui-widgets
[API Docs]: https://docs.rs/tui-bar-graph/
[Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-bar-graph/examples
[Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-bar-graph/CHANGELOG.md
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
