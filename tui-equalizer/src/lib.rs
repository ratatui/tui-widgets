//! An equalizer widget for [Ratatui] with multiple frequency bands.
//!
//! The equalizer is a vertical bar chart where each band represents a frequency range. Each band
//! can display a value from 0.0 to 1.0, where 1.0 is the maximum value.
//!
//! ![Made with VHS](https://vhs.charm.sh/vhs-FiRQkkDAUEnH2BrPbUx5i.gif)
//!
//! This demo can be found in the examples folder in the git repo.
//!
//! ```shell
//! cargo run --example demo
//! ```
//!
//! Inspired by [a comment in the ratatui
//! repo](https://github.com/ratatui/ratatui/issues/1325#issuecomment-2335095486).
//!
//! # Example
//!
//! ```rust
//! # use ratatui::{widgets::Widget, layout::Rect, buffer::Buffer};
//! # let area = Rect::default();
//! # let mut buf = Buffer::empty(area);
//! use tui_equalizer::{Band, Equalizer};
//!
//! let equalizer = Equalizer {
//!     bands: vec![Band::from(0.5), Band::from(0.8), Band::from(0.3)],
//!     brightness: 1.0,
//! };
//! equalizer.render(area, &mut buf);
//! ```
//!
//! # License
//!
//! Copyright (c) Josh McKinney
//!
//! This project is licensed under either of:
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! [LICENSE-APACHE]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/ratatui/tui-widgets/blob/main/LICENSE-MIT
//!
//! [Ratatui]: https://crates.io/crates/ratatui

use std::iter::zip;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Layout, Rect};
use ratatui_core::style::Color;
use ratatui_core::symbols;
use ratatui_core::widgets::Widget;

/// An equalizer widget with multiple frequency bands.
///
/// The equalizer is a vertical bar chart where each band represents a frequency range.
///
/// # Example
///
/// ```
/// # use ratatui::widgets::Widget;
/// # let area = ratatui::layout::Rect::default();
/// # let mut buf = ratatui::buffer::Buffer::empty(area);
/// use tui_equalizer::{Band, Equalizer};
///
/// let equalizer = Equalizer {
///     bands: vec![Band::from(0.5), Band::from(0.8), Band::from(0.3)],
///     brightness: 1.0,
/// };
/// equalizer.render(area, &mut buf);
/// ```
#[derive(Debug, Clone)]
pub struct Equalizer {
    /// A vector of `Band` structs representing each frequency band.
    pub bands: Vec<Band>,
    pub brightness: f64,
}

/// A struct representing a single frequency band in the equalizer.
#[derive(Debug, Clone)]
pub struct Band {
    /// The normalized value of the band, where the maximum is 1.0.
    pub value: f64,
}

impl From<f64> for Band {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl Widget for Equalizer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &Equalizer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::horizontal(vec![Constraint::Length(2); self.bands.len()]).split(area);
        for (band, area) in zip(&self.bands, areas.iter()) {
            band.render(*area, buf, self.brightness);
        }
    }
}

impl Band {
    fn render(&self, area: Rect, buf: &mut Buffer, brightness: f64) {
        let value = self.value.clamp(0.0, 1.0);
        let height = (value * area.height as f64) as u16;

        // Calculate the color gradient step
        let color_step = 1.0 / area.height as f64;

        // Iterate over each segment and render it with the corresponding color
        for i in 0..height {
            // Green to Yellow to Red gradient
            let v = i as f64 * color_step;
            let vv = 1.0 - v;
            let br = brightness.clamp(0.0, 1.0) * 255.0;
            let r = if v < 0.5 { v * 2.0 * br } else { br } as u8;
            let g = if v < 0.5 { br } else { vv * 2.0 * br } as u8;
            let b = 0;
            let color = Color::Rgb(r, g, b);
            buf[(area.left(), area.bottom().saturating_sub(i + 1))]
                .set_fg(color)
                .set_symbol(symbols::bar::HALF);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_by_reference_matches_owned_render() {
        let equalizer = Equalizer {
            bands: vec![Band::from(0.25), Band::from(0.5), Band::from(1.0)],
            brightness: 1.0,
        };
        let mut owned_buf = Buffer::empty(Rect::new(0, 0, 6, 4));
        let mut borrowed_buf = Buffer::empty(Rect::new(0, 0, 6, 4));

        equalizer.clone().render(owned_buf.area, &mut owned_buf);
        (&equalizer).render(borrowed_buf.area, &mut borrowed_buf);

        assert_eq!(borrowed_buf, owned_buf);
    }
}
