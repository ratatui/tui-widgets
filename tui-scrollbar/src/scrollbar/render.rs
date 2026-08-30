//! Rendering helpers and `Widget` implementation for [`ScrollBar`].
//!
//! The core widget delegates rendering to these helpers so the draw logic is grouped separately
//! from configuration and input handling. Keep rendering changes localized here.

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::Style;
use ratatui_core::widgets::Widget;

use super::{ArrowLayout, ScrollBar, ScrollBarOrientation};
use crate::ScrollLengths;
use crate::metrics::{CellFill, ScrollMetrics};

impl Widget for &ScrollBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_inner(area, buf);
    }
}

impl ScrollBar {
    /// Renders the scrollbar into the provided buffer.
    fn render_inner(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let layout = self.arrow_layout(area);
        self.render_arrows(&layout, buf);
        if layout.track_area.width == 0 || layout.track_area.height == 0 {
            return;
        }

        match self.orientation {
            ScrollBarOrientation::Vertical => {
                self.render_vertical_track(layout.track_area, buf);
            }
            ScrollBarOrientation::Horizontal => {
                self.render_horizontal_track(layout.track_area, buf);
            }
        }
    }

    /// Renders arrow endcaps into the buffer before the thumb/track.
    fn render_arrows(&self, layout: &ArrowLayout, buf: &mut Buffer) {
        let arrow_style = self.style.arrow_style.unwrap_or(self.style.track_style);
        if let Some((x, y)) = layout.start {
            let glyph = match self.orientation {
                ScrollBarOrientation::Vertical => self.glyph_set.arrow_vertical_start,
                ScrollBarOrientation::Horizontal => self.glyph_set.arrow_horizontal_start,
            };
            let cell = &mut buf[(x, y)];
            cell.set_char(glyph);
            cell.set_style(arrow_style);
        }
        if let Some((x, y)) = layout.end {
            let glyph = match self.orientation {
                ScrollBarOrientation::Vertical => self.glyph_set.arrow_vertical_end,
                ScrollBarOrientation::Horizontal => self.glyph_set.arrow_horizontal_end,
            };
            let cell = &mut buf[(x, y)];
            cell.set_char(glyph);
            cell.set_style(arrow_style);
        }
    }

    /// Renders the vertical track and thumb into the provided area.
    fn render_vertical_track(&self, area: Rect, buf: &mut Buffer) {
        let metrics = ScrollMetrics::new(
            ScrollLengths {
                content_len: self.content_len,
                viewport_len: self.viewport_len,
            },
            self.offset,
            area.height,
        );
        let x = area.x;
        for (idx, y) in (area.y..area.y.saturating_add(area.height)).enumerate() {
            let (glyph, style) = self.glyph_for_vertical(metrics.cell_fill(idx));
            let cell = &mut buf[(x, y)];
            cell.set_char(glyph);
            cell.set_style(style);
        }
    }

    /// Renders the horizontal track and thumb into the provided area.
    fn render_horizontal_track(&self, area: Rect, buf: &mut Buffer) {
        let metrics = ScrollMetrics::new(
            ScrollLengths {
                content_len: self.content_len,
                viewport_len: self.viewport_len,
            },
            self.offset,
            area.width,
        );
        let y = area.y;
        for (idx, x) in (area.x..area.x.saturating_add(area.width)).enumerate() {
            let (glyph, style) = self.glyph_for_horizontal(metrics.cell_fill(idx));
            let cell = &mut buf[(x, y)];
            cell.set_char(glyph);
            cell.set_style(style);
        }
    }

    /// Chooses the vertical glyph + style for a track cell fill.
    fn glyph_for_vertical(&self, fill: CellFill) -> (char, Style) {
        match fill {
            CellFill::Empty => (self.glyph_set.track_vertical, self.style.track_style),
            CellFill::Full => (
                self.glyph_set.thumb_vertical_lower[7],
                self.style.thumb_style,
            ),
            CellFill::Partial { start, len } => {
                let index = len.saturating_sub(1) as usize;
                let glyph = if start == 0 {
                    self.glyph_set.thumb_vertical_upper[index]
                } else {
                    self.glyph_set.thumb_vertical_lower[index]
                };
                (glyph, self.style.thumb_style)
            }
        }
    }

    /// Chooses the horizontal glyph + style for a track cell fill.
    fn glyph_for_horizontal(&self, fill: CellFill) -> (char, Style) {
        match fill {
            CellFill::Empty => (self.glyph_set.track_horizontal, self.style.track_style),
            CellFill::Full => (
                self.glyph_set.thumb_horizontal_left[7],
                self.style.thumb_style,
            ),
            CellFill::Partial { start, len } => {
                let index = len.saturating_sub(1) as usize;
                let glyph = if start == 0 {
                    self.glyph_set.thumb_horizontal_left[index]
                } else {
                    self.glyph_set.thumb_horizontal_right[index]
                };
                (glyph, self.style.thumb_style)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;
    use ratatui_core::style::{Color, Style};

    use super::*;
    use crate::{GlyphSet, ScrollBarArrows, ScrollBarStyle, ScrollLengths};

    fn assert_horizontal_thumb_walk(
        glyph_set: GlyphSet,
        track_char: char,
        expected_lines: [&str; 9],
    ) {
        let lengths = ScrollLengths {
            content_len: 8 * crate::SUBCELL,
            viewport_len: 2 * crate::SUBCELL,
        };

        for (offset, expected_line) in expected_lines.into_iter().enumerate() {
            let scrollbar = ScrollBar::horizontal(lengths)
                .arrows(ScrollBarArrows::None)
                .glyph_set(glyph_set.clone())
                .style(ScrollBarStyle::new())
                .offset(offset);
            let mut buf = Buffer::empty(Rect::new(0, 0, 8, 1));
            (&scrollbar).render(buf.area, &mut buf);

            let mut expected = Buffer::with_lines(vec![expected_line]);
            expected.set_style(expected.area, scrollbar.styles().track_style);
            for (x, symbol) in expected_line.chars().enumerate() {
                if symbol != track_char {
                    expected[(x as u16, 0)].set_style(scrollbar.styles().thumb_style);
                }
            }
            assert_eq!(buf, expected);
        }
    }

    #[test]
    fn render_vertical_fractional_thumb() {
        let scrollbar = ScrollBar::vertical(ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        })
        .arrows(ScrollBarArrows::None)
        .offset(1);
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 4));
        (&scrollbar).render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec!["▅", "▀", " ", " "]);
        expected.set_style(expected.area, scrollbar.styles().track_style);
        expected[(0, 0)].set_style(scrollbar.styles().thumb_style);
        expected[(0, 1)].set_style(scrollbar.styles().thumb_style);
        assert_eq!(buf, expected);
    }

    #[test]
    fn render_horizontal_fractional_thumb() {
        let scrollbar = ScrollBar::horizontal(ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        })
        .arrows(ScrollBarArrows::None)
        .offset(1);
        let mut buf = Buffer::empty(Rect::new(0, 0, 4, 1));
        (&scrollbar).render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec!["🮉▌  "]);
        expected.set_style(expected.area, scrollbar.styles().track_style);
        expected[(0, 0)].set_style(scrollbar.styles().thumb_style);
        expected[(1, 0)].set_style(scrollbar.styles().thumb_style);
        assert_eq!(buf, expected);
    }

    #[test]
    fn render_uses_custom_thumb_style_for_full_and_partial_cells() {
        let track_style = Style::new().bg(Color::Rgb(10, 20, 30));
        let thumb_style = Style::new()
            .fg(Color::Rgb(255, 158, 100))
            .bg(Color::Rgb(10, 20, 30));
        let arrow_style = Style::new()
            .fg(Color::Rgb(158, 206, 106))
            .bg(Color::Rgb(10, 20, 30));
        let scrollbar = ScrollBar::horizontal(ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        })
        .arrows(ScrollBarArrows::Both)
        .offset(1)
        .track_style(track_style)
        .thumb_style(thumb_style)
        .arrow_style(arrow_style);

        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
        (&scrollbar).render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec!["◀🮉▌  ▶"]);
        expected.set_style(expected.area, track_style);
        expected[(0, 0)].set_style(arrow_style);
        expected[(1, 0)].set_style(thumb_style);
        expected[(2, 0)].set_style(thumb_style);
        expected[(5, 0)].set_style(arrow_style);
        assert_eq!(buf, expected);
    }

    #[test]
    fn render_horizontal_fractional_thumb_box_drawing_track() {
        let scrollbar = ScrollBar::horizontal(ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        })
        .arrows(ScrollBarArrows::None)
        .offset(1)
        .glyph_set(GlyphSet::box_drawing());
        let mut buf = Buffer::empty(Rect::new(0, 0, 4, 1));
        (&scrollbar).render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec!["🮉▌──"]);
        expected.set_style(expected.area, scrollbar.styles().track_style);
        expected[(0, 0)].set_style(scrollbar.styles().thumb_style);
        expected[(1, 0)].set_style(scrollbar.styles().thumb_style);
        assert_eq!(buf, expected);
    }

    #[test]
    fn render_horizontal_fractional_thumb_unicode_glyphs() {
        let scrollbar = ScrollBar::horizontal(ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        })
        .arrows(ScrollBarArrows::None)
        .offset(1)
        .glyph_set(GlyphSet::unicode());
        let mut buf = Buffer::empty(Rect::new(0, 0, 4, 1));
        (&scrollbar).render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec!["▐▌──"]);
        expected.set_style(expected.area, scrollbar.styles().track_style);
        expected[(0, 0)].set_style(scrollbar.styles().thumb_style);
        expected[(1, 0)].set_style(scrollbar.styles().thumb_style);
        assert_eq!(buf, expected);
    }

    #[test]
    fn render_horizontal_thumb_walk_minimal_glyphs() {
        assert_horizontal_thumb_walk(
            GlyphSet::minimal(),
            ' ',
            [
                "██      ",
                "🮋█▏     ",
                "🮊█▎     ",
                "🮉█▍     ",
                "▐█▌     ",
                "🮈█▋     ",
                "🮇█▊     ",
                "▕█▉     ",
                " ██     ",
            ],
        );
    }

    #[test]
    fn render_horizontal_thumb_walk_legacy_glyphs() {
        assert_horizontal_thumb_walk(
            GlyphSet::symbols_for_legacy_computing(),
            '─',
            [
                "██──────",
                "🮋█▏─────",
                "🮊█▎─────",
                "🮉█▍─────",
                "▐█▌─────",
                "🮈█▋─────",
                "🮇█▊─────",
                "▕█▉─────",
                "─██─────",
            ],
        );
    }

    #[test]
    fn render_horizontal_thumb_walk_unicode_glyphs() {
        assert_horizontal_thumb_walk(
            GlyphSet::unicode(),
            '─',
            [
                "██──────",
                "██▏─────",
                "▐█▎─────",
                "▐█▍─────",
                "▐█▌─────",
                "▐█▋─────",
                "▕█▊─────",
                "▕█▉─────",
                "─██─────",
            ],
        );
    }

    #[test]
    fn render_full_thumb_when_no_scroll() {
        let scrollbar = ScrollBar::vertical(ScrollLengths {
            content_len: 5,
            viewport_len: 10,
        })
        .arrows(ScrollBarArrows::None);
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 3));
        (&scrollbar).render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec!["█", "█", "█"]);
        expected.set_style(expected.area, scrollbar.styles().thumb_style);
        assert_eq!(buf, expected);
    }

    #[test]
    fn render_vertical_arrows() {
        let scrollbar = ScrollBar::vertical(ScrollLengths {
            content_len: 5,
            viewport_len: 2,
        })
        .arrows(ScrollBarArrows::Both);
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 3));
        (&scrollbar).render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec!["▲", "█", "▼"]);
        expected[(0, 0)].set_style(
            scrollbar
                .style
                .arrow_style
                .unwrap_or(scrollbar.styles().track_style),
        );
        expected[(0, 1)].set_style(scrollbar.styles().thumb_style);
        expected[(0, 2)].set_style(
            scrollbar
                .style
                .arrow_style
                .unwrap_or(scrollbar.styles().track_style),
        );
        assert_eq!(buf, expected);
    }

    #[test]
    fn render_horizontal_arrows() {
        let scrollbar = ScrollBar::horizontal(ScrollLengths {
            content_len: 5,
            viewport_len: 2,
        })
        .arrows(ScrollBarArrows::Both);
        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 1));
        (&scrollbar).render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec!["◀█▶"]);
        expected[(0, 0)].set_style(
            scrollbar
                .style
                .arrow_style
                .unwrap_or(scrollbar.styles().track_style),
        );
        expected[(1, 0)].set_style(scrollbar.styles().thumb_style);
        expected[(2, 0)].set_style(
            scrollbar
                .style
                .arrow_style
                .unwrap_or(scrollbar.styles().track_style),
        );
        assert_eq!(buf, expected);
    }
}
