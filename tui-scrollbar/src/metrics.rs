//! Pure scrollbar geometry and hit testing.
//!
//! This module contains the math behind thumb sizing and positioning. It is backend-agnostic and
//! does not touch terminal rendering, making it suitable for unit tests and hit testing.
//!
//! Use [`ScrollMetrics`] when you need the thumb geometry without rendering a widget. This is
//! especially useful for input handling, layout tests, or validating edge cases such as
//! `content_len <= viewport_len`.
//!
//! ## Subcells
//!
//! A subcell is one eighth of a terminal cell. This module measures content, viewport, and offsets
//! in logical units so fractional thumb sizes can be represented precisely. These lengths are
//! logical values (not pixels); you decide how they map to your data. The track length is still
//! expressed in full cells, then multiplied by [`SUBCELL`] to compute subcell positions.
//!
//! The example below shows a common pattern: convert a track measured in cells into subcell units,
//! then compute a proportional thumb size and position.
//!
//! ```rust
//! use tui_scrollbar::{SUBCELL, ScrollLengths, ScrollMetrics};
//!
//! let track_cells = 8;
//! let viewport_len = track_cells * SUBCELL;
//! let content_len = viewport_len * 4;
//! let lengths = ScrollLengths {
//!     content_len,
//!     viewport_len,
//! };
//! let metrics = ScrollMetrics::new(lengths, 0, track_cells as u16);
//!
//! assert!(metrics.thumb_len() >= SUBCELL);
//! ```

use std::ops::Range;

/// Number of subcells in a single terminal cell.
pub const SUBCELL: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Describes how much of a single cell is covered by the thumb.
///
/// Use this to select track vs thumb glyphs. Partial fills are measured from the start of the
/// cell (top for vertical, left for horizontal).
pub enum CellFill {
    /// No coverage; the track should render.
    Empty,
    /// Entire cell is covered by the thumb.
    Full,
    /// A fractional range inside the cell is covered by the thumb.
    Partial {
        /// Subcell offset within the cell where coverage starts.
        start: u8,
        /// Number of subcells covered within the cell.
        len: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Whether a position lands on the thumb or track.
///
/// Positions are measured in subcells along the track.
pub enum HitTest {
    /// The position is inside the thumb.
    Thumb,
    /// The position is outside the thumb.
    Track,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Precomputed values for proportional scrollbars.
///
/// All positions are tracked in subcell units (1/8 of a terminal cell). Use this type to compute
/// thumb length, travel, and hit testing without rendering anything. The inputs are:
///
/// - `content_len` and `viewport_len` in logical units (zero treated as 1)
/// - `track_cells` in terminal cells
pub struct ScrollMetrics {
    content_len: usize,
    viewport_len: usize,
    offset: usize,
    track_cells: usize,
    track_len: usize,
    thumb_len: usize,
    thumb_start: usize,
}

impl ScrollMetrics {
    /// Build metrics using a [`crate::ScrollLengths`] helper.
    pub fn from_lengths(lengths: crate::ScrollLengths, offset: usize, track_cells: u16) -> Self {
        Self::new(lengths, offset, track_cells)
    }

    /// Build metrics for the given content and viewport lengths.
    ///
    /// The `track_cells` parameter is the number of terminal cells available for the track
    /// (height for vertical scrollbars, width for horizontal). The lengths are logical units.
    /// When `content_len` is smaller than `viewport_len`, the thumb fills the track to indicate no
    /// scrolling. Zero lengths are treated as 1.
    pub fn new(lengths: crate::ScrollLengths, offset: usize, track_cells: u16) -> Self {
        let track_cells = track_cells as usize;
        let track_len = track_cells.saturating_mul(SUBCELL);

        if track_len == 0 {
            return Self {
                content_len: lengths.content_len,
                viewport_len: lengths.viewport_len,
                offset,
                track_cells,
                track_len,
                thumb_len: 0,
                thumb_start: 0,
            };
        }

        let content_len = lengths.content_len.max(1);
        let viewport_len = lengths.viewport_len.min(content_len).max(1);
        let max_offset = content_len.saturating_sub(viewport_len);
        let offset = offset.min(max_offset);

        let thumb_len = (track_len.saturating_mul(viewport_len) / content_len)
            .max(SUBCELL)
            .min(track_len);
        let thumb_travel = track_len.saturating_sub(thumb_len);
        let thumb_start = thumb_travel
            .saturating_mul(offset)
            .checked_div(max_offset)
            .unwrap_or(0);

        Self {
            content_len,
            viewport_len,
            offset,
            track_cells,
            track_len,
            thumb_len,
            thumb_start,
        }
    }

    /// Returns the current content length in logical units.
    pub const fn content_len(&self) -> usize {
        self.content_len
    }

    /// Returns the current viewport length in logical units.
    pub const fn viewport_len(&self) -> usize {
        self.viewport_len
    }

    /// Returns the current content offset in logical units.
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the track length in terminal cells.
    pub const fn track_cells(&self) -> usize {
        self.track_cells
    }

    /// Returns the track length in subcells.
    pub const fn track_len(&self) -> usize {
        self.track_len
    }

    /// Returns the thumb length in subcells.
    pub const fn thumb_len(&self) -> usize {
        self.thumb_len
    }

    /// Returns the thumb start position in subcells.
    pub const fn thumb_start(&self) -> usize {
        self.thumb_start
    }

    /// Returns the maximum scrollable offset in subcells.
    pub const fn max_offset(&self) -> usize {
        self.content_len.saturating_sub(self.viewport_len)
    }

    /// Returns the maximum thumb travel in subcells.
    pub const fn thumb_travel(&self) -> usize {
        self.track_len.saturating_sub(self.thumb_len)
    }

    /// Returns the thumb range in subcell coordinates.
    pub const fn thumb_range(&self) -> Range<usize> {
        self.thumb_start..self.thumb_start.saturating_add(self.thumb_len)
    }

    /// Returns whether a subcell position hits the thumb or the track.
    pub const fn hit_test(&self, position: usize) -> HitTest {
        if position >= self.thumb_start
            && position < self.thumb_start.saturating_add(self.thumb_len)
        {
            HitTest::Thumb
        } else {
            HitTest::Track
        }
    }

    /// Converts an offset (in subcells) to a thumb start position (in subcells).
    ///
    /// Larger offsets move the thumb toward the end of the track, clamped to the maximum travel.
    pub fn thumb_start_for_offset(&self, offset: usize) -> usize {
        let max_offset = self.max_offset();
        let offset = offset.min(max_offset);
        self.thumb_travel()
            .saturating_mul(offset)
            .checked_div(max_offset)
            .unwrap_or(0)
    }

    /// Converts a thumb start position (in subcells) to an offset (in subcells).
    ///
    /// Thumb positions beyond the end of travel are clamped to the maximum offset.
    pub fn offset_for_thumb_start(&self, thumb_start: usize) -> usize {
        let max_offset = self.max_offset();
        let thumb_start = thumb_start.min(self.thumb_travel());
        max_offset
            .saturating_mul(thumb_start)
            .checked_div(self.thumb_travel())
            .unwrap_or(0)
    }

    /// Returns how much of a cell is filled by the thumb.
    ///
    /// The `cell_index` is in terminal cells, not subcells. Use this to select the correct glyph
    /// for the track or thumb.
    pub fn cell_fill(&self, cell_index: usize) -> CellFill {
        if self.thumb_len == 0 {
            return CellFill::Empty;
        }

        let cell_start = cell_index.saturating_mul(SUBCELL);
        let cell_end = cell_start.saturating_add(SUBCELL);

        let thumb_end = self.thumb_start.saturating_add(self.thumb_len);
        let start = self.thumb_start.max(cell_start);
        let end = thumb_end.min(cell_end);

        if end <= start {
            return CellFill::Empty;
        }

        let len = end.saturating_sub(start).min(SUBCELL) as u8;
        let start = start.saturating_sub(cell_start).min(SUBCELL) as u8;

        if len as usize >= SUBCELL {
            CellFill::Full
        } else {
            CellFill::Partial { start, len }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fills_track_when_no_scroll() {
        let metrics = ScrollMetrics::new(
            crate::ScrollLengths {
                content_len: 10,
                viewport_len: 10,
            },
            0,
            4,
        );
        assert_eq!(metrics.thumb_len(), 32);
        assert_eq!(metrics.thumb_start(), 0);
    }

    #[test]
    fn clamps_offset_to_max() {
        let metrics = ScrollMetrics::new(
            crate::ScrollLengths {
                content_len: 100,
                viewport_len: 10,
            },
            200,
            4,
        );
        assert_eq!(metrics.offset(), 90);
        assert_eq!(metrics.thumb_start(), metrics.thumb_travel());
    }

    #[test]
    fn reports_partial_cell_fills() {
        let metrics = ScrollMetrics::new(
            crate::ScrollLengths {
                content_len: 10,
                viewport_len: 3,
            },
            1,
            4,
        );
        assert_eq!(metrics.cell_fill(0), CellFill::Partial { start: 3, len: 5 });
        assert_eq!(metrics.cell_fill(1), CellFill::Partial { start: 0, len: 4 });
        assert_eq!(metrics.cell_fill(2), CellFill::Empty);
    }

    #[test]
    fn distinguishes_thumb_vs_track_hits() {
        let metrics = ScrollMetrics::new(
            crate::ScrollLengths {
                content_len: 10,
                viewport_len: 3,
            },
            1,
            4,
        );
        assert_eq!(metrics.hit_test(0), HitTest::Track);
        assert_eq!(metrics.hit_test(4), HitTest::Thumb);
        assert_eq!(metrics.hit_test(12), HitTest::Track);
    }

    #[test]
    fn stays_scale_invariant_for_logical_units() {
        let track_cells = 10;
        let base = ScrollMetrics::new(
            crate::ScrollLengths {
                content_len: 200,
                viewport_len: 20,
            },
            10,
            track_cells,
        );
        let scaled = ScrollMetrics::new(
            crate::ScrollLengths {
                content_len: 200 * SUBCELL,
                viewport_len: 20 * SUBCELL,
            },
            10 * SUBCELL,
            track_cells,
        );
        assert_eq!(base.thumb_len(), scaled.thumb_len());
        assert_eq!(base.thumb_start(), scaled.thumb_start());
    }

    #[test]
    fn yields_empty_thumb_when_track_len_zero() {
        let lengths = crate::ScrollLengths {
            content_len: 10,
            viewport_len: 4,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 0);
        assert_eq!(metrics.track_len(), 0);
        assert_eq!(metrics.thumb_len(), 0);
        assert_eq!(metrics.cell_fill(0), CellFill::Empty);
    }

    #[test]
    fn reports_full_cell_when_thumb_covers_track() {
        let lengths = crate::ScrollLengths {
            content_len: 8,
            viewport_len: 8,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 1);
        assert_eq!(metrics.thumb_len(), SUBCELL);
        assert_eq!(metrics.cell_fill(0), CellFill::Full);
    }

    #[test]
    fn treats_zero_lengths_as_one() {
        let lengths = crate::ScrollLengths {
            content_len: 0,
            viewport_len: 0,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 1);
        assert_eq!(metrics.content_len(), 1);
        assert_eq!(metrics.viewport_len(), 1);
        assert_eq!(metrics.thumb_len(), SUBCELL);
    }

    #[test]
    fn thumb_start_for_offset_returns_zero_when_no_scroll() {
        let lengths = crate::ScrollLengths {
            content_len: 10,
            viewport_len: 10,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 4);
        assert_eq!(metrics.thumb_start_for_offset(5), 0);
    }

    #[test]
    fn offset_for_thumb_start_returns_zero_when_no_scroll() {
        let lengths = crate::ScrollLengths {
            content_len: 10,
            viewport_len: 10,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 4);
        assert_eq!(metrics.offset_for_thumb_start(5), 0);
    }

    #[test]
    fn conversions_return_zero_when_thumb_cannot_travel() {
        let lengths = crate::ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 1);
        assert_eq!(metrics.thumb_travel(), 0);
        assert_eq!(metrics.thumb_start_for_offset(5), 0);
        assert_eq!(metrics.offset_for_thumb_start(5), 0);
    }

    #[test]
    fn hit_test_returns_track_before_thumb_start() {
        let lengths = crate::ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        };
        let metrics = ScrollMetrics::new(lengths, 1, 4);
        assert_eq!(
            metrics.hit_test(metrics.thumb_start().saturating_sub(1)),
            HitTest::Track
        );
    }

    #[test]
    fn hit_test_returns_track_at_thumb_end() {
        let lengths = crate::ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        };
        let metrics = ScrollMetrics::new(lengths, 1, 4);
        let thumb_end = metrics.thumb_start().saturating_add(metrics.thumb_len());
        assert_eq!(metrics.hit_test(thumb_end), HitTest::Track);
    }

    #[test]
    fn reports_empty_cell_fill_when_thumb_len_zero() {
        let lengths = crate::ScrollLengths {
            content_len: 10,
            viewport_len: 4,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 0);
        assert_eq!(metrics.cell_fill(0), CellFill::Empty);
    }

    #[test]
    fn thumb_range_matches_start_and_len() {
        let lengths = crate::ScrollLengths {
            content_len: 10,
            viewport_len: 3,
        };
        let metrics = ScrollMetrics::new(lengths, 1, 4);
        assert_eq!(
            metrics.thumb_range(),
            metrics.thumb_start()..metrics.thumb_start().saturating_add(metrics.thumb_len())
        );
    }

    #[test]
    fn clamps_thumb_start_for_offset() {
        let lengths = crate::ScrollLengths {
            content_len: 100,
            viewport_len: 10,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 4);
        let max_offset = metrics.max_offset();
        assert_eq!(
            metrics.thumb_start_for_offset(max_offset.saturating_add(10)),
            metrics.thumb_travel()
        );
    }

    #[test]
    fn clamps_offset_for_thumb_start() {
        let lengths = crate::ScrollLengths {
            content_len: 100,
            viewport_len: 10,
        };
        let metrics = ScrollMetrics::new(lengths, 0, 4);
        let max_offset = metrics.max_offset();
        assert_eq!(
            metrics.offset_for_thumb_start(metrics.thumb_travel().saturating_add(10)),
            max_offset
        );
    }
}
