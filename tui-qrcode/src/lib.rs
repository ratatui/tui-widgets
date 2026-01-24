//! A [Ratatui] widget to render crisp, scan-happy QR codes in the terminal. Part of the
//! [tui-widgets] suite by [Joshka].
//!
//! ![Demo](https://vhs.charm.sh/vhs-nUpcmCP1igCcGoJ5iio07.gif)
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
//! Add qrcode and tui-qrcode to your Cargo.toml. You can disable the default features of qrcode as
//! we don't need the code which renders the QR code to an image.
//!
//! ```shell
//! cargo add qrcode tui-qrcode --no-default-features
//! ```
//!
//! # Usage
//!
//! This example can be found in the `examples` directory of the repository.
//!
//! ```no_run
//! use qrcode::QrCode;
//! use ratatui::crossterm::event;
//! use ratatui::{DefaultTerminal, Frame};
//! use tui_qrcode::{Colors, QrCodeWidget};
//!
//! fn main() -> color_eyre::Result<()> {
//!     color_eyre::install()?;
//!     let terminal = ratatui::init();
//!     let result = run(terminal);
//!     ratatui::restore();
//!     result
//! }
//!
//! fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
//!     loop {
//!         terminal.draw(render)?;
//!         if matches!(event::read()?, event::Event::Key(_)) {
//!             break Ok(());
//!         }
//!     }
//! }
//!
//! fn render(frame: &mut Frame) {
//!     let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
//!     let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted);
//!     frame.render_widget(widget, frame.area());
//! }
//! ```
//!
//! Renders the following QR code:
//!
//! ```text
//! █████████████████████████████████
//! █████████████████████████████████
//! ████ ▄▄▄▄▄ █▄ ▄▄▄ ████ ▄▄▄▄▄ ████
//! ████ █   █ █▄▄▄█▀▄██ █ █   █ ████
//! ████ █▄▄▄█ █▀   ▄▀ ███ █▄▄▄█ ████
//! ████▄▄▄▄▄▄▄█▄▀▄█ ▀▄▀ █▄▄▄▄▄▄▄████
//! ████ █▄▀▀▀▄▄▀▄▄  ▄█▀▄█▀ █▀▄▀ ████
//! ██████▀█  ▄▀▄▄▀▀ ▄ ▄█ ▄▄█ ▄█▄████
//! ████▄▀▀▀▄▄▄▄▀█▄▄█  ▀ ▀ ▀███▀ ████
//! ████▄▄ ▀█▄▄▀▄▄ ▄█▀█▄▀█▄▀▀ ▄█▄████
//! ████▄▄█▄██▄█ ▄▀▄ ▄█  ▄▄▄ ██▄▀████
//! ████ ▄▄▄▄▄ █▄▄▄▀ ▄ ▀ █▄█ ███ ████
//! ████ █   █ ██ ███  ▄▄ ▄▄ █▀ ▄████
//! ████ █▄▄▄█ █▄▀ ▄█▀█▀ ▄█  ▄█▄▄████
//! ████▄▄▄▄▄▄▄█▄▄█▄▄▄██▄█▄██▄██▄████
//! █████████████████████████████████
//! ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
//! ```
//!
//! # More widgets
//!
//! For the full suite of widgets, see [tui-widgets].
//!
//! [Ratatui]: https://crates.io/crates/ratatui
//! [Crate]: https://crates.io/crates/tui-qrcode
//! [Docs]: https://docs.rs/tui-qrcode/
//! [Dependency Status]: https://deps.rs/repo/github/ratatui/tui-widgets
//! [Coverage]: https://app.codecov.io/gh/ratatui/tui-widgets
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//! [Crate badge]: https://img.shields.io/crates/v/tui-qrcode.svg?logo=rust&style=flat
//! [Docs Badge]: https://img.shields.io/docsrs/tui-qrcode?logo=rust&style=flat
//! [Deps Badge]: https://deps.rs/repo/github/ratatui/tui-widgets/status.svg?style=flat
//! [License Badge]: https://img.shields.io/crates/l/tui-qrcode?style=flat
//! [License]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
//! [Coverage Badge]:
//!     https://img.shields.io/codecov/c/github/ratatui/tui-widgets?logo=codecov&style=flat
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?logo=discord&style=flat
//! [GitHub Repository]: https://github.com/ratatui/tui-widgets
//! [API Docs]: https://docs.rs/tui-qrcode/
//! [Examples]: https://github.com/ratatui/tui-widgets/tree/main/tui-qrcode/examples
//! [Changelog]: https://github.com/ratatui/tui-widgets/blob/main/tui-qrcode/CHANGELOG.md
//! [Contributing]: https://github.com/ratatui/tui-widgets/blob/main/CONTRIBUTING.md
//!
//! [Joshka]: https://github.com/joshka
//! [tui-widgets]: https://crates.io/crates/tui-widgets

use qrcode::render::unicode::Dense1x2;
use qrcode::QrCode;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Rect, Size};
use ratatui_core::style::{Style, Styled};
use ratatui_core::text::Text;
use ratatui_core::widgets::Widget;

/// A [Ratatui] widget that renders a QR code.
///
/// This widget can be used to render a QR code in a terminal. It uses the [qrcode] crate to
/// generate the QR code.
///
/// # Examples
///
/// ```no_run
/// use qrcode::QrCode;
/// use tui_qrcode::QrCodeWidget;
///
/// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
/// let widget = QrCodeWidget::new(qr_code);
/// ```
///
/// The widget can be customized using the `quiet_zone`, `scaling`, `colors`, and `style` methods.
/// Additionally, the widget implements the [`Styled`] trait, so all the methods from Ratatui's
/// [`ratatui_core::style::Stylize`] trait can be used.
///
/// ```no_run
/// use qrcode::QrCode;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::Frame;
/// use tui_qrcode::{Colors, QrCodeWidget, QuietZone, Scaling};
///
/// fn render(frame: &mut Frame) {
///     let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
///     let widget = QrCodeWidget::new(qr_code)
///         .quiet_zone(QuietZone::Disabled)
///         .scaling(Scaling::Max)
///         .colors(Colors::Inverted)
///         .red()
///         .on_light_yellow();
///     frame.render_widget(widget, frame.area());
/// }
/// ```
///
/// [Ratatui]: https://crates.io/crates/ratatui
pub struct QrCodeWidget {
    qr_code: QrCode,
    quiet_zone: QuietZone,
    scaling: Scaling,
    colors: Colors,
    style: Style,
}

/// The quiet zone (border) of a QR code.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum QuietZone {
    /// The quiet zone is enabled.
    #[default]
    Enabled,
    /// The quiet zone is disabled.
    Disabled,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Scaling {
    /// The QR code will be scaled to at least the size of the widget.
    ///
    /// Note that this usually results in a QR code that is larger than the widget, which is not
    /// ideal.
    Min,

    /// The QR code will be scaled to be at most the size of the widget.
    ///
    /// Note that this may result in a QR code which is scaled more horizontally or vertically than
    /// the other, which may not be ideal.
    Max,

    /// The QR code will be scaled so each pixel is the size of the given dimensions.
    ///
    /// The minimum dimensions are 1x1 (width x height).
    Exact(u16, u16),
}

impl Default for Scaling {
    fn default() -> Self {
        Self::Exact(1, 1)
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Colors {
    /// The default colors. (Black on white)
    #[default]
    Normal,

    /// The colors are inverted. (White on black)
    Inverted,
}

impl QrCodeWidget {
    /// Create a new QR code widget.
    #[must_use]
    pub fn new(qr_code: QrCode) -> Self {
        Self {
            qr_code,
            quiet_zone: QuietZone::default(),
            scaling: Scaling::default(),
            colors: Colors::default(),
            style: Style::default(),
        }
    }

    /// Set whether the QR code should have a quiet zone.
    ///
    /// This is the white border around the QR code. By default, the quiet zone is enabled.
    ///
    /// # Example
    ///
    /// ```
    /// use qrcode::QrCode;
    /// use tui_qrcode::{QrCodeWidget, QuietZone};
    ///
    /// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    /// let widget = QrCodeWidget::new(qr_code).quiet_zone(QuietZone::Disabled);
    /// ```
    #[must_use]
    pub const fn quiet_zone(mut self, quiet_zone: QuietZone) -> Self {
        self.quiet_zone = quiet_zone;
        self
    }

    /// Set how the QR code should be scaled.
    ///
    /// By default, the QR code will be scaled so each pixel is 1x1.
    ///
    /// The `Min` variant will scale the QR code so it is at least the size of the widget. This may
    /// result in a QR code that is larger than the widget, which is not ideal. The `Max` variant
    /// will scale the QR code so it is at most the size of the widget. This may result in a QR code
    /// which is scaled more horizontally or vertically than the other, which may not be ideal. The
    /// `Exact` variant will scale the QR code so each pixel is the size of the given dimensions.
    /// The minimum scaling is 1x1 (width x height).
    ///
    /// # Example
    ///
    /// ```
    /// use qrcode::QrCode;
    /// use tui_qrcode::{QrCodeWidget, Scaling};
    ///
    /// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    /// let widget = QrCodeWidget::new(qr_code).scaling(Scaling::Max);
    /// ```
    #[must_use]
    pub const fn scaling(mut self, scaling: Scaling) -> Self {
        self.scaling = scaling;
        self
    }

    /// Set the colors of the QR code.
    ///
    /// By default, the colors are normal (light on dark).
    ///
    /// The `Normal` variant will use the default colors. The `Inverted` variant will invert the
    /// colors (dark on light).
    ///
    /// To set the foreground and background colors of the widget, use the `style` method.
    ///
    /// # Example
    ///
    /// ```
    /// use qrcode::QrCode;
    /// use tui_qrcode::{Colors, QrCodeWidget};
    ///
    /// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    /// let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted);
    /// ```
    #[must_use]
    pub const fn colors(mut self, colors: Colors) -> Self {
        self.colors = colors;
        self
    }

    /// Set the style of the widget.
    ///
    /// This will set the foreground and background colors of the widget.
    ///
    /// # Example
    ///
    /// ```
    /// use qrcode::QrCode;
    /// use ratatui::style::{Style, Stylize};
    /// use tui_qrcode::QrCodeWidget;
    ///
    /// let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    /// let style = Style::new().red().on_light_yellow();
    /// let widget = QrCodeWidget::new(qr_code).style(style);
    /// ```
    #[must_use]
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }

    /// The theoretical size of the QR code if rendered into `area`.
    ///
    /// Note that if the QR code does not fit into `area`, the resulting [`Size`] might be larger
    /// than the size of `area`.
    ///
    /// # Example
    /// ```
    /// use qrcode::QrCode;
    /// use ratatui::layout::Rect;
    /// use tui_qrcode::{QrCodeWidget, Scaling};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let qr_code = QrCode::new("https://ratatui.rs")?;
    /// let widget = QrCodeWidget::new(qr_code).scaling(Scaling::Min);
    /// let area = Rect::new(0, 0, 50, 50);
    /// let widget_size = widget.size(area);
    ///
    /// assert_eq!(widget_size.width, 66);
    /// assert_eq!(widget_size.height, 66);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn size(&self, area: Rect) -> Size {
        let qr_width: u16 = match self.quiet_zone {
            QuietZone::Enabled => 8,
            QuietZone::Disabled => 0,
        } + self.qr_code.width() as u16;

        let (x, y) = match self.scaling {
            Scaling::Exact(x, y) => (x, y),
            Scaling::Min => {
                let x = area.width.div_ceil(qr_width);
                let y = (area.height * 2).div_ceil(qr_width);
                (x, y)
            }
            Scaling::Max => {
                let x = area.width / qr_width;
                let y = (area.height * 2) / qr_width;
                (x, y)
            }
        };
        let (x, y) = (x.max(1), y.max(1));
        let width = qr_width * x;
        let height = (qr_width * y).div_ceil(2);
        Size::new(width, height)
    }
}

impl Styled for QrCodeWidget {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for QrCodeWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &QrCodeWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut renderer = self.qr_code.render::<Dense1x2>();
        match self.quiet_zone {
            QuietZone::Enabled => renderer.quiet_zone(true),
            QuietZone::Disabled => renderer.quiet_zone(false),
        };
        match self.scaling {
            Scaling::Min => renderer.min_dimensions(area.width as u32, area.height as u32 * 2),
            Scaling::Max => renderer.max_dimensions(area.width as u32, area.height as u32 * 2),
            Scaling::Exact(width, height) => {
                renderer.module_dimensions(width as u32, height as u32)
            }
        };
        match self.colors {
            Colors::Normal => renderer
                .dark_color(Dense1x2::Dark)
                .light_color(Dense1x2::Light),
            Colors::Inverted => renderer
                .dark_color(Dense1x2::Light)
                .light_color(Dense1x2::Dark),
        };
        Text::raw(renderer.build())
            .style(self.style)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    /// Creates an empty QR code widget. The basic dimensions of the QR code are 21x21 or 29x29 with
    /// a quiet zone.
    #[fixture]
    fn empty_widget() -> QrCodeWidget {
        let empty_qr = QrCode::new("").expect("failed to create QR code");
        QrCodeWidget::new(empty_qr).quiet_zone(QuietZone::Disabled)
    }

    #[rstest]
    /// Cases where the QR code is smaller (21x10.5) than the area (22, 12)
    #[case::smaller_exact((22,12), Scaling::Exact(1, 1), (21, 11))]
    #[case::smaller_max((22, 12), Scaling::Max, (21, 11))]
    #[case::smaller_min((22,12),Scaling::Min, (42, 21))]
    /// Cases where the QR code is the same size (21x10.5) as the area (21, 11)
    #[case::same_exact((21, 11), Scaling::Exact(1, 1), (21, 11))]
    #[case::same_max((21, 11), Scaling::Max, (21, 11))]
    /// Exception: height would be 10.5, so height is doubled to 21
    #[case::same_min((21, 11), Scaling::Min, (21, 21))]
    /// Cases where the QR code is larger (21x10.5) than the area (20, 10)
    #[rstest]
    #[case::larger_exact((20, 10), Scaling::Exact(1, 1), (21, 11))]
    #[case::larger_max((20, 10), Scaling::Max, (21, 11))]
    #[case::larger_min((20, 10), Scaling::Min, (21, 11))]
    /// Cases where the QR code is much smaller (21x10.5) than the area (71, 71).
    #[rstest]
    #[case::huge_exact((71, 71), Scaling::Exact(1, 1), (21, 11))]
    #[case::huge_max((71, 71), Scaling::Max,(63, 63))]
    #[case::huge_min((71, 71), Scaling::Min, (84, 74))]
    fn size(
        empty_widget: QrCodeWidget,
        #[case] rect: (u16, u16),
        #[case] scaling: Scaling,
        #[case] expected: (u16, u16),
    ) {
        let rect = Rect::new(0, 0, rect.0, rect.1);
        let widget = empty_widget.scaling(scaling);
        assert_eq!(widget.size(rect), Size::from(expected));
    }

    /// Testing that a QR code with a quiet zone (29x14.5) is scaled correctly into a large area
    /// (71x71).
    #[rstest]
    #[case::huge_exact(Scaling::Exact(1, 1), (29, 15))]
    #[case::huge_max(Scaling::Max, (58, 58))]
    #[case::huge_min(Scaling::Min, (87, 73))]
    fn size_with_quiet_zone(
        empty_widget: QrCodeWidget,
        #[case] scaling: Scaling,
        #[case] expected: (u16, u16),
    ) {
        let rect = Rect::new(0, 0, 71, 71);
        let widget = empty_widget.quiet_zone(QuietZone::Enabled).scaling(scaling);
        assert_eq!(widget.size(rect), Size::from(expected));
    }

    /// The QR code fits into the area without scaling
    #[rstest]
    fn render_exact_into_fitting_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 21, 11));
        empty_widget.render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀█",
                "█ ███ █ █▀▀ ▀ █ ███ █",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ █",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀▀",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄ ",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄ ",
                "▀ ▀ ▀▀▀ █▄█ █  █  █  ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀█",
                "█ ███ █ █▀██▄█▄ ▀█▀▀▀",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄   ",
                "▀▀▀▀▀▀▀ ▀   ▀  ▀  ▀  ",
            ])
        );
    }

    /// The QR code fits into the area without scaling
    #[rstest]
    fn render_max_into_fitting_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 21, 11));
        empty_widget
            .scaling(Scaling::Max)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀█",
                "█ ███ █ █▀▀ ▀ █ ███ █",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ █",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀▀",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄ ",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄ ",
                "▀ ▀ ▀▀▀ █▄█ █  █  █  ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀█",
                "█ ███ █ █▀██▄█▄ ▀█▀▀▀",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄   ",
                "▀▀▀▀▀▀▀ ▀   ▀  ▀  ▀  ",
            ])
        );
    }

    // The QR code is doubled vertically as the min scaling means this needs to render at least
    // 21x10.5 but the buffer is 21x11
    ///
    /// Note: this is an instance where the square aspect ratio of the QR code is not preserved
    /// correctly. This doesn't align with the documentation of the qrcode crate.
    #[rstest]
    fn render_min_into_fitting_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 21, 11));
        empty_widget
            .scaling(Scaling::Min)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "███████  ██ █ ███████",
                "█     █    ██ █     █",
                "█ ███ █ ███ █ █ ███ █",
                "█ ███ █ █     █ ███ █",
                "█ ███ █ ██  █ █ ███ █",
                "█     █ ████  █     █",
                "███████ █ █ █ ███████",
                "        ██           ",
                "█ █████  ███  █████  ",
                "   █ █ █  █████  █ █ ",
                "      ████  █ ██     ",
            ])
        );
    }

    /// The QR code fits into the area without scaling
    #[rstest]
    fn render_exact_into_larger_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 22, 12));
        empty_widget.render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀█ ",
                "█ ███ █ █▀▀ ▀ █ ███ █ ",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ █ ",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀▀ ",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄  ",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄  ",
                "▀ ▀ ▀▀▀ █▄█ █  █  █   ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀█ ",
                "█ ███ █ █▀██▄█▄ ▀█▀▀▀ ",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄    ",
                "▀▀▀▀▀▀▀ ▀   ▀  ▀  ▀   ",
                "                      ",
            ])
        );
    }

    /// The QR code fits into the area without scaling
    #[rstest]
    fn render_max_into_larger_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 22, 12));
        empty_widget
            .scaling(Scaling::Max)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀█ ",
                "█ ███ █ █▀▀ ▀ █ ███ █ ",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ █ ",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀▀ ",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄  ",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄  ",
                "▀ ▀ ▀▀▀ █▄█ █  █  █   ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀█ ",
                "█ ███ █ █▀██▄█▄ ▀█▀▀▀ ",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄    ",
                "▀▀▀▀▀▀▀ ▀   ▀  ▀  ▀   ",
                "                      ",
            ])
        );
    }

    /// The QR code is doubled vertically and horizontall as the min scaling means this needs to
    /// render at least 21x10.5 but the buffer is 22x12
    #[rstest]
    fn render_min_into_larger_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 22, 12));
        empty_widget
            .scaling(Scaling::Min)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "██████████████    ████",
                "██          ██        ",
                "██  ██████  ██  ██████",
                "██  ██████  ██  ██    ",
                "██  ██████  ██  ████  ",
                "██          ██  ██████",
                "██████████████  ██  ██",
                "                ████  ",
                "██  ██████████    ████",
                "      ██  ██  ██    ██",
                "            ████████  ",
                "██████        ████  ██",
            ])
        );
    }

    /// The QR code is truncated as the area is smaller than the QR code
    #[rstest]
    fn render_exact_into_smaler_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 10));
        empty_widget.render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀",
                "█ ███ █ █▀▀ ▀ █ ███ ",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ ",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄",
                "▀ ▀ ▀▀▀ █▄█ █  █  █ ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀",
                "█ ███ █ █▀██▄█▄ ▀█▀▀",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄  ",
            ])
        );
    }

    /// The QR code is truncated as the max scaling means this needs to render at most 21x10.5 but
    /// the buffer is 20x10
    #[rstest]
    fn render_max_into_smaller_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 10));
        empty_widget
            .scaling(Scaling::Max)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀",
                "█ ███ █ █▀▀ ▀ █ ███ ",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ ",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄",
                "▀ ▀ ▀▀▀ █▄█ █  █  █ ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀",
                "█ ███ █ █▀██▄█▄ ▀█▀▀",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄  ",
            ])
        );
    }

    /// The QR code is truncated as the min scaling means this needs to render at least 21x10.5 but
    /// the buffer is already too small
    #[rstest]
    fn render_min_into_smaller_area(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 10));
        empty_widget
            .scaling(Scaling::Min)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀",
                "█ ███ █ █▀▀ ▀ █ ███ ",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ ",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄",
                "▀ ▀ ▀▀▀ █▄█ █  █  █ ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀",
                "█ ███ █ █▀██▄█▄ ▀█▀▀",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄  ",
            ])
        );
    }

    /// Exact scaling doesn't scale the QR code
    #[rstest]
    fn render_exact_double_height(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 21, 21));
        empty_widget.render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀█",
                "█ ███ █ █▀▀ ▀ █ ███ █",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ █",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀▀",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄ ",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄ ",
                "▀ ▀ ▀▀▀ █▄█ █  █  █  ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀█",
                "█ ███ █ █▀██▄█▄ ▀█▀▀▀",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄   ",
                "▀▀▀▀▀▀▀ ▀   ▀  ▀  ▀  ",
                "                     ",
                "                     ",
                "                     ",
                "                     ",
                "                     ",
                "                     ",
                "                     ",
                "                     ",
                "                     ",
                "                     ",
            ])
        );
    }

    /// The QR code is doubled vertically
    ///
    /// Note: this is an instance where the square aspect ratio of the QR code is not preserved
    /// correctly. This doesn't align with the documentation of the qrcode crate.
    #[rstest]
    fn render_max_double_height(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 21, 21));
        empty_widget
            .scaling(Scaling::Max)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "███████  ██ █ ███████",
                "█     █    ██ █     █",
                "█ ███ █ ███ █ █ ███ █",
                "█ ███ █ █     █ ███ █",
                "█ ███ █ ██  █ █ ███ █",
                "█     █ ████  █     █",
                "███████ █ █ █ ███████",
                "        ██           ",
                "█ █████  ███  █████  ",
                "   █ █ █  █████  █ █ ",
                "      ████  █ ██     ",
                "███    ██ █████  █ █ ",
                "█ █ ███ █ █ █  █  █  ",
                "        ███ █  █  █  ",
                "███████  ███ █  █████",
                "█     █ ███    ██ █ █",
                "█ ███ █ ████ █  █████",
                "█ ███ █ █ █████  █   ",
                "█ ███ █ █   █ ██     ",
                "█     █    ████  █   ",
                "███████ █   █  █  █  ",
            ])
        );
    }

    /// The QR code is doubled vertically
    ///
    /// Note: this is an instance where the square aspect ratio of the QR code is not preserved
    /// correctly. This doesn't align with the documentation of the qrcode crate.
    #[rstest]
    fn render_min_double_height(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 21, 21));
        empty_widget
            .scaling(Scaling::Min)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "███████  ██ █ ███████",
                "█     █    ██ █     █",
                "█ ███ █ ███ █ █ ███ █",
                "█ ███ █ █     █ ███ █",
                "█ ███ █ ██  █ █ ███ █",
                "█     █ ████  █     █",
                "███████ █ █ █ ███████",
                "        ██           ",
                "█ █████  ███  █████  ",
                "   █ █ █  █████  █ █ ",
                "      ████  █ ██     ",
                "███    ██ █████  █ █ ",
                "█ █ ███ █ █ █  █  █  ",
                "        ███ █  █  █  ",
                "███████  ███ █  █████",
                "█     █ ███    ██ █ █",
                "█ ███ █ ████ █  █████",
                "█ ███ █ █ █████  █   ",
                "█ ███ █ █   █ ██     ",
                "█     █    ████  █   ",
                "███████ █   █  █  █  ",
            ])
        );
    }

    #[rstest]
    fn render_exact_double_width(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 42, 11));
        empty_widget.render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀█                     ",
                "█ ███ █ █▀▀ ▀ █ ███ █                     ",
                "█ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ █                     ",
                "▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀▀                     ",
                "▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄                      ",
                "▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄                      ",
                "▀ ▀ ▀▀▀ █▄█ █  █  █                       ",
                "█▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀█                     ",
                "█ ███ █ █▀██▄█▄ ▀█▀▀▀                     ",
                "█ ▀▀▀ █ ▀  ▄█▄█▀ ▄                        ",
                "▀▀▀▀▀▀▀ ▀   ▀  ▀  ▀                       ",
            ])
        );
    }

    /// The QR code is doubled horizontally as the max scaling means this needs to render at most
    /// 42x10.5 but the buffer is 42x11
    ///
    /// Note: this is an instance where the square aspect ratio of the QR code is not preserved
    /// correctly. This doesn't align with the documentation of the qrcode crate.
    #[rstest]
    fn render_max_double_width(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 42, 11));
        empty_widget
            .scaling(Scaling::Max)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "██▀▀▀▀▀▀▀▀▀▀██    ▀▀▀▀▄▄██  ██▀▀▀▀▀▀▀▀▀▀██",
                "██  ██████  ██  ██▀▀▀▀  ▀▀  ██  ██████  ██",
                "██  ▀▀▀▀▀▀  ██  ████▄▄▄▄▀▀  ██  ▀▀▀▀▀▀  ██",
                "▀▀▀▀▀▀▀▀▀▀▀▀▀▀  ██▄▄▀▀  ▀▀  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀",
                "▀▀  ▀▀██▀▀██▀▀▄▄  ▀▀████▄▄▄▄██▀▀▀▀██▀▀▄▄  ",
                "▄▄▄▄▄▄      ▀▀████▀▀▄▄▄▄██▄▄██▀▀  ▄▄  ▄▄  ",
                "▀▀  ▀▀  ▀▀▀▀▀▀  ██▄▄██  ██    ██    ██    ",
                "██▀▀▀▀▀▀▀▀▀▀██  ▄▄████▀▀  ▀▀  ▄▄██▀▀██▀▀██",
                "██  ██████  ██  ██▀▀████▄▄██▄▄  ▀▀██▀▀▀▀▀▀",
                "██  ▀▀▀▀▀▀  ██  ▀▀    ▄▄██▄▄██▀▀  ▄▄      ",
                "▀▀▀▀▀▀▀▀▀▀▀▀▀▀  ▀▀      ▀▀    ▀▀    ▀▀    ",
            ])
        );
    }

    /// Both the width and height are doubled because the min scaling means the QR code needs to be
    /// at least 42x10.5 but the buffer is 42x11
    #[rstest]
    fn render_min_double_width(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 42, 11));
        empty_widget
            .scaling(Scaling::Min)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "██████████████    ████  ██  ██████████████",
                "██          ██        ████  ██          ██",
                "██  ██████  ██  ██████  ██  ██  ██████  ██",
                "██  ██████  ██  ██          ██  ██████  ██",
                "██  ██████  ██  ████    ██  ██  ██████  ██",
                "██          ██  ████████    ██          ██",
                "██████████████  ██  ██  ██  ██████████████",
                "                ████                      ",
                "██  ██████████    ██████    ██████████    ",
                "      ██  ██  ██    ██████████    ██  ██  ",
                "            ████████    ██  ████          ",
            ])
        );
    }

    #[rstest]
    fn render_inverted(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 21, 11));
        empty_widget
            .colors(Colors::Inverted)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                " ▄▄▄▄▄ ██▄▄▀ █ ▄▄▄▄▄ ",
                " █   █ █ ▄▄█▄█ █   █ ",
                " █▄▄▄█ █  ▀▀▄█ █▄▄▄█ ",
                "▄▄▄▄▄▄▄█ ▀▄█▄█▄▄▄▄▄▄▄",
                "▄█▄ ▄ ▄▀█▄  ▀▀ ▄▄ ▄▀█",
                "▀▀▀███▄  ▄▀▀ ▀ ▄█▀█▀█",
                "▄█▄█▄▄▄█ ▀ █ ██ ██ ██",
                " ▄▄▄▄▄ █▀  ▄█▄█▀ ▄ ▄ ",
                " █   █ █ ▄  ▀ ▀█▄ ▄▄▄",
                " █▄▄▄█ █▄██▀ ▀ ▄█▀███",
                "       ▀ ▀▀▀ ▀▀ ▀▀ ▀▀",
            ])
        );
    }

    #[rstest]
    fn render_with_quiet_zone(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 29, 15));
        empty_widget
            .quiet_zone(QuietZone::Enabled)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "                             ",
                "                             ",
                "    █▀▀▀▀▀█  ▀▀▄█ █▀▀▀▀▀█    ",
                "    █ ███ █ █▀▀ ▀ █ ███ █    ",
                "    █ ▀▀▀ █ ██▄▄▀ █ ▀▀▀ █    ",
                "    ▀▀▀▀▀▀▀ █▄▀ ▀ ▀▀▀▀▀▀▀    ",
                "    ▀ ▀█▀█▀▄ ▀██▄▄█▀▀█▀▄     ",
                "    ▄▄▄   ▀██▀▄▄█▄█▀ ▄ ▄     ",
                "    ▀ ▀ ▀▀▀ █▄█ █  █  █      ",
                "    █▀▀▀▀▀█ ▄██▀ ▀ ▄█▀█▀█    ",
                "    █ ███ █ █▀██▄█▄ ▀█▀▀▀    ",
                "    █ ▀▀▀ █ ▀  ▄█▄█▀ ▄       ",
                "    ▀▀▀▀▀▀▀ ▀   ▀  ▀  ▀      ",
                "                             ",
                "                             ",
            ])
        );
    }

    #[rstest]
    fn render_with_quiet_zone_and_inverted(empty_widget: QrCodeWidget) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 29, 15));
        empty_widget
            .quiet_zone(QuietZone::Enabled)
            .colors(Colors::Inverted)
            .render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█████████████████████████████",
                "█████████████████████████████",
                "████ ▄▄▄▄▄ ██▄▄▀ █ ▄▄▄▄▄ ████",
                "████ █   █ █ ▄▄█▄█ █   █ ████",
                "████ █▄▄▄█ █  ▀▀▄█ █▄▄▄█ ████",
                "████▄▄▄▄▄▄▄█ ▀▄█▄█▄▄▄▄▄▄▄████",
                "████▄█▄ ▄ ▄▀█▄  ▀▀ ▄▄ ▄▀█████",
                "████▀▀▀███▄  ▄▀▀ ▀ ▄█▀█▀█████",
                "████▄█▄█▄▄▄█ ▀ █ ██ ██ ██████",
                "████ ▄▄▄▄▄ █▀  ▄█▄█▀ ▄ ▄ ████",
                "████ █   █ █ ▄  ▀ ▀█▄ ▄▄▄████",
                "████ █▄▄▄█ █▄██▀ ▀ ▄█▀███████",
                "████▄▄▄▄▄▄▄█▄███▄██▄██▄██████",
                "█████████████████████████████",
                "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀",
            ])
        );
    }
}
