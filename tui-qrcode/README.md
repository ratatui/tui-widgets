# TUI QR Code

<!-- cargo-rdme start -->

A [Ratatui] widget to render crisp, scan-happy QR codes in the terminal. Part of the
[tui-widgets] suite by [Joshka].

![Demo](https://vhs.charm.sh/vhs-nUpcmCP1igCcGoJ5iio07.gif)

[![Crate badge]][Crate]
[![Docs Badge]][Docs]
[![Deps Badge]][Dependency Status]
[![License Badge]][License]
[![Coverage Badge]][Coverage]
[![Discord Badge]][Ratatui Discord]

[GitHub Repository] · [API Docs] · [Examples] · [Changelog] · [Contributing]

## Installation

Add qrcode and tui-qrcode to your Cargo.toml. You can disable the default features of qrcode as
we don't need the code which renders the QR code to an image.

```shell
cargo add qrcode tui-qrcode --no-default-features
```

## Usage

This example can be found in the `examples` directory of the repository.

```rust
use qrcode::QrCode;
use ratatui::crossterm::event;
use ratatui::{DefaultTerminal, Frame};
use tui_qrcode::{Colors, QrCodeWidget};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, event::Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted);
    frame.render_widget(widget, frame.area());
}
```

Renders the following QR code:

```text
█████████████████████████████████
█████████████████████████████████
████ ▄▄▄▄▄ █▄ ▄▄▄ ████ ▄▄▄▄▄ ████
████ █   █ █▄▄▄█▀▄██ █ █   █ ████
████ █▄▄▄█ █▀   ▄▀ ███ █▄▄▄█ ████
████▄▄▄▄▄▄▄█▄▀▄█ ▀▄▀ █▄▄▄▄▄▄▄████
████ █▄▀▀▀▄▄▀▄▄  ▄█▀▄█▀ █▀▄▀ ████
██████▀█  ▄▀▄▄▀▀ ▄ ▄█ ▄▄█ ▄█▄████
████▄▀▀▀▄▄▄▄▀█▄▄█  ▀ ▀ ▀███▀ ████
████▄▄ ▀█▄▄▀▄▄ ▄█▀█▄▀█▄▀▀ ▄█▄████
████▄▄█▄██▄█ ▄▀▄ ▄█  ▄▄▄ ██▄▀████
████ ▄▄▄▄▄ █▄▄▄▀ ▄ ▀ █▄█ ███ ████
████ █   █ ██ ███  ▄▄ ▄▄ █▀ ▄████
████ █▄▄▄█ █▄▀ ▄█▀█▀ ▄█  ▄█▄▄████
████▄▄▄▄▄▄▄█▄▄█▄▄▄██▄█▄██▄██▄████
█████████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
```

## More widgets

For the full suite of widgets, see [tui-widgets].

[Ratatui]: https://crates.io/crates/ratatui
[Crate]: https://crates.io/crates/tui-qrcode
[Docs]: https://docs.rs/tui-qrcode/
[Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
[Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
[Crate badge]: https://img.shields.io/crates/v/tui-qrcode.svg?logo=rust&style=flat
[Docs Badge]: https://img.shields.io/docsrs/tui-qrcode?logo=rust&style=flat
[Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
[License Badge]: https://img.shields.io/crates/l/tui-qrcode?style=flat
[License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
[Coverage Badge]:
    https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat
[GitHub Repository]: https://github.com/ratatui/tui-widgets
[API Docs]: https://docs.rs/tui-qrcode/
[Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-qrcode/examples
[Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-qrcode/CHANGELOG.md
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
