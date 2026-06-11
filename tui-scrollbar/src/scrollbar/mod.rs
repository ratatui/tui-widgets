//! Rendering and interaction for proportional scrollbars.
//!
//! This module provides the widget and interaction helpers. Glyph configuration lives in
//! [`crate::GlyphSet`], and the pure scrollbar geometry lives in [`crate::ScrollMetrics`].
//!
//! # Local model
//!
//! 1. [`ScrollBar`] stores orientation, logical lengths, offset, styles, glyphs, and interaction
//!    behavior.
//! 2. [`ScrollMetrics`] converts the current lengths and offset into thumb geometry.
//! 3. Rendering chooses track, thumb, and arrow glyphs from [`GlyphSet`].
//! 4. Input helpers return [`ScrollCommand`] values for the application to apply.
//!
//! The scrollbar renders only a single row or column. If you provide a larger [`Rect`], it will
//! still render into the first row/column of that area.
//!
//! ## Layout choices
//!
//! The widget treats the provided area as the track container. When arrows are enabled, one cell
//! at each end is reserved for the endcaps and the remaining inner area is used for the thumb.
//!
//! Arrow endcaps are optional. When enabled, they consume one cell at the start/end of the track,
//! and the thumb renders inside the remaining inner area.
//!
//! ## Interaction choices
//!
//! - The widget is stateless: it renders from inputs and returns commands instead of mutating
//!   scroll offsets. This keeps control with the application.
//! - Dragging stores a grab offset in subcells so the thumb does not jump under the pointer.
//! - Arrow endcaps consume track space; the inner track is used for metrics and hit testing so
//!   thumb math stays consistent regardless of arrows.
//!
//! Drag operations store a "grab offset" in subcells (1/8 of a cell; see [`crate::SUBCELL`]) so the
//! thumb does not jump when the pointer starts dragging; subsequent drag events subtract that
//! offset to keep the grab point stable.
//!
//! Wheel events are ignored unless their axis matches the scrollbar orientation. Positive deltas
//! scroll down/right.
//!
//! The example below renders a vertical scrollbar into a buffer. It demonstrates how the widget
//! uses `content_len`, `viewport_len`, and `offset` to decide the thumb size and position.
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::Rect;
//! use ratatui_core::widgets::Widget;
//! use tui_scrollbar::{ScrollBar, ScrollLengths};
//!
//! let area = Rect::new(0, 0, 1, 4);
//! let lengths = ScrollLengths {
//!     content_len: 120,
//!     viewport_len: 40,
//! };
//! let scrollbar = ScrollBar::vertical(lengths).offset(20);
//!
//! let mut buffer = Buffer::empty(area);
//! scrollbar.render(area, &mut buffer);
//! ```
//!
//! [`Rect`]: ratatui_core::layout::Rect

use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Style};

use crate::glyphs::GlyphSet;

mod interaction;
mod render;

/// Axis the scrollbar is laid out on.
///
/// Orientation determines whether the track length is derived from height or width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBarOrientation {
    /// A vertical scrollbar that fills a single column.
    Vertical,
    /// A horizontal scrollbar that fills a single row.
    Horizontal,
}

/// Behavior when the user clicks on the track outside the thumb.
///
/// Page clicks move by `viewport_len`. Jump-to-click centers the thumb near the click.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackClickBehavior {
    /// Move by one viewport length toward the click position.
    Page,
    /// Jump the thumb toward the click position.
    JumpToClick,
}

/// Which arrow endcaps to render on the track.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBarArrows {
    /// Do not render arrow endcaps.
    #[default]
    None,
    /// Render the arrow at the start of the track (top/left).
    Start,
    /// Render the arrow at the end of the track (bottom/right).
    End,
    /// Render arrows at both ends of the track.
    Both,
}

impl ScrollBarArrows {
    const fn has_start(self) -> bool {
        matches!(self, Self::Start | Self::Both)
    }

    const fn has_end(self) -> bool {
        matches!(self, Self::End | Self::Both)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrowHit {
    Start,
    End,
}

#[derive(Debug, Clone, Copy)]
struct ArrowLayout {
    track_area: Rect,
    start: Option<(u16, u16)>,
    end: Option<(u16, u16)>,
}

/// A proportional scrollbar widget with fractional thumb rendering.
///
/// # Method map
///
/// ## Construction
///
/// - [`Self::new`]
/// - [`Self::orientation`]
/// - [`Self::vertical`]
/// - [`Self::horizontal`]
///
/// ## Position and lengths
///
/// - [`Self::content_len`]
/// - [`Self::viewport_len`]
/// - [`Self::offset`]
///
/// ## Appearance
///
/// - [`Self::track_style`]
/// - [`Self::thumb_style`]
/// - [`Self::arrow_style`]
/// - [`Self::glyph_set`]
/// - [`Self::arrows`]
///
/// ## Interaction
///
/// - [`Self::handle_event`]
/// - [`Self::handle_mouse_event`], when a crossterm feature is enabled
/// - [`Self::track_click_behavior`]
/// - [`Self::scroll_step`]
///
/// # Important
///
/// - `content_len` and `viewport_len` are in logical units.
/// - Zero values are treated as 1.
/// - The scrollbar renders into a single row or column.
///
/// # Behavior
///
/// The thumb length is proportional to `viewport_len / content_len` and clamped to at least one
/// full cell for usability. When `content_len <= viewport_len`, the thumb fills the track. Areas
/// with zero width or height render nothing.
///
/// Arrow endcaps, when enabled, consume one cell at the start/end of the track. The thumb and
/// track render in the remaining inner area. Clicking an arrow steps the offset by `scroll_step`.
///
/// # Styling
///
/// Track glyphs use `track_style`. Thumb glyphs use `thumb_style`. Arrow endcaps use
/// `arrow_style`, which defaults to white on dark gray.
///
/// Scrollbar glyphs are terminal characters. For visible track glyphs, thumb blocks, and arrow
/// symbols, `Style::fg` colors the glyph itself and `Style::bg` colors the cell behind it. The
/// default [`GlyphSet::minimal`] track renders spaces, so only the track background is visible in
/// empty track cells. Visible track glyph sets, such as [`GlyphSet::box_drawing`] and
/// [`GlyphSet::unicode`], can use foreground color for the track line. Thumb glyphs are block
/// characters, so `Style::fg` is usually the useful knob for thumb color; `Style::bg` still colors
/// the rest of the cell. With partial thumb glyphs, especially on a visible line track such as
/// [`GlyphSet::box_drawing`], that background can show at the ends of the thumb. Match the thumb
/// background to the track background unless that contrast is intentional.
///
/// ```rust
/// use ratatui_core::style::{Color, Style};
/// use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
///
/// let lengths = ScrollLengths {
///     content_len: 120,
///     viewport_len: 30,
/// };
/// let scrollbar = ScrollBar::vertical(lengths)
///     .arrows(ScrollBarArrows::Both)
///     .track_style(Style::new().bg(Color::Black))
///     .thumb_style(Style::new().fg(Color::Rgb(255, 158, 100)))
///     .arrow_style(Style::new().fg(Color::Yellow).bg(Color::Black));
/// ```
///
/// # State
///
/// This widget is stateless. Pointer drag state lives in [`crate::ScrollBarInteraction`].
///
/// # Examples
///
/// Minimal rendering only needs an area, lengths, an offset, and a buffer.
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::widgets::Widget;
/// use tui_scrollbar::{ScrollBar, ScrollLengths};
///
/// let area = Rect::new(0, 0, 1, 5);
/// let lengths = ScrollLengths {
///     content_len: 200,
///     viewport_len: 40,
/// };
/// let scrollbar = ScrollBar::vertical(lengths).offset(60);
///
/// let mut buffer = Buffer::empty(area);
/// scrollbar.render(area, &mut buffer);
/// ```
///
/// ## Updating offsets on input
///
/// This is the typical pattern for pointer handling: feed events to the scrollbar and apply the
/// returned command to your stored offset.
///
/// ```rust,no_run
/// use ratatui_core::layout::Rect;
/// use tui_scrollbar::{
///     PointerButton, PointerEvent, PointerEventKind, ScrollBar, ScrollBarInteraction,
///     ScrollCommand, ScrollEvent, ScrollLengths,
/// };
///
/// let area = Rect::new(0, 0, 1, 10);
/// let lengths = ScrollLengths {
///     content_len: 400,
///     viewport_len: 80,
/// };
/// let scrollbar = ScrollBar::vertical(lengths).offset(0);
/// let mut interaction = ScrollBarInteraction::new();
/// let mut offset = 0;
///
/// let event = ScrollEvent::Pointer(PointerEvent {
///     column: 0,
///     row: 3,
///     kind: PointerEventKind::Down,
///     button: PointerButton::Primary,
/// });
///
/// if let Some(ScrollCommand::SetOffset(next)) =
///     scrollbar.handle_event(area, event, &mut interaction)
/// {
///     offset = next;
/// }
/// # let _ = offset;
/// ```
///
/// ## Track click behavior
///
/// Choose between classic page jumps or jump-to-click behavior.
///
/// ```rust
/// use tui_scrollbar::{ScrollBar, ScrollLengths, TrackClickBehavior};
///
/// let lengths = ScrollLengths {
///     content_len: 10,
///     viewport_len: 5,
/// };
/// let scrollbar =
///     ScrollBar::vertical(lengths).track_click_behavior(TrackClickBehavior::JumpToClick);
/// ```
///
/// ## Arrow endcaps
///
/// Arrow endcaps are optional. When enabled, they reserve one cell at each end of the track.
///
/// ```rust
/// use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
///
/// let lengths = ScrollLengths {
///     content_len: 120,
///     viewport_len: 24,
/// };
/// let scrollbar = ScrollBar::vertical(lengths).arrows(ScrollBarArrows::Both);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollBar {
    orientation: ScrollBarOrientation,
    content_len: usize,
    viewport_len: usize,
    offset: usize,
    track_style: Style,
    thumb_style: Style,
    arrow_style: Option<Style>,
    glyph_set: GlyphSet,
    arrows: ScrollBarArrows,
    track_click_behavior: TrackClickBehavior,
    scroll_step: usize,
}

impl ScrollBar {
    /// Creates a scrollbar with the given orientation and lengths.
    ///
    /// Use [`Self::vertical`] or [`Self::horizontal`] when the orientation is known at the call
    /// site.
    ///
    /// Zero lengths are treated as 1.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollBarOrientation, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::new(ScrollBarOrientation::Vertical, lengths);
    /// ```
    pub fn new(orientation: ScrollBarOrientation, lengths: crate::ScrollLengths) -> Self {
        Self {
            orientation,
            content_len: lengths.content_len,
            viewport_len: lengths.viewport_len,
            offset: 0,
            track_style: Style::new().bg(Color::DarkGray),
            thumb_style: Style::new().fg(Color::White).bg(Color::DarkGray),
            arrow_style: Some(Style::new().fg(Color::White).bg(Color::DarkGray)),
            glyph_set: GlyphSet::default(),
            arrows: ScrollBarArrows::default(),
            track_click_behavior: TrackClickBehavior::Page,
            scroll_step: 1,
        }
    }

    /// Creates a vertical scrollbar with the given content and viewport lengths.
    ///
    /// The track length is derived from the render area's height.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths);
    /// ```
    pub fn vertical(lengths: crate::ScrollLengths) -> Self {
        Self::new(ScrollBarOrientation::Vertical, lengths)
    }

    /// Creates a horizontal scrollbar with the given content and viewport lengths.
    ///
    /// The track length is derived from the render area's width.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::horizontal(lengths);
    /// ```
    pub fn horizontal(lengths: crate::ScrollLengths) -> Self {
        Self::new(ScrollBarOrientation::Horizontal, lengths)
    }

    /// Sets the scrollbar orientation.
    ///
    /// This is mostly useful when sharing a builder chain and choosing the orientation later.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollBarOrientation, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).orientation(ScrollBarOrientation::Horizontal);
    /// ```
    pub const fn orientation(mut self, orientation: ScrollBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the total scrollable content length in logical units.
    ///
    /// Larger values shrink the thumb, while smaller values enlarge it.
    ///
    /// Zero values are treated as 1.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).content_len(240);
    /// ```
    pub const fn content_len(mut self, content_len: usize) -> Self {
        self.content_len = content_len;
        self
    }

    /// Sets the visible viewport length in logical units.
    ///
    /// When `viewport_len >= content_len`, the thumb fills the track.
    ///
    /// Zero values are treated as 1.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).viewport_len(60);
    /// ```
    pub const fn viewport_len(mut self, viewport_len: usize) -> Self {
        self.viewport_len = viewport_len;
        self
    }

    /// Sets the current scroll offset in logical units.
    ///
    /// Offsets are clamped to `content_len - viewport_len` during rendering and input handling,
    /// not when this builder is called.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).offset(30);
    /// ```
    pub const fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the style applied to track glyphs.
    ///
    /// Track styling applies only to cells where the thumb is not rendered.
    ///
    /// ```rust
    /// use ratatui_core::style::{Color, Style};
    /// use tui_scrollbar::{ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).track_style(Style::new().bg(Color::Black));
    /// ```
    pub const fn track_style(mut self, style: Style) -> Self {
        self.track_style = style;
        self
    }

    /// Sets the style applied to thumb glyphs.
    ///
    /// Thumb styling applies to full and partial thumb cells. Thumb glyphs are block characters,
    /// so `Style::fg` usually controls the visible thumb color. Use `Style::bg` only when the
    /// cell behind the glyph should differ from the track. On partial thumb cells, the background
    /// can show at the thumb ends.
    ///
    /// ```rust
    /// use ratatui_core::style::{Color, Style};
    /// use tui_scrollbar::{ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar =
    ///     ScrollBar::vertical(lengths).thumb_style(Style::new().fg(Color::Rgb(255, 158, 100)));
    /// ```
    pub const fn thumb_style(mut self, style: Style) -> Self {
        self.thumb_style = style;
        self
    }

    /// Sets the style applied to arrow glyphs.
    ///
    /// Arrow endcaps render only when enabled with [`Self::arrows`]. If no arrow style is
    /// configured internally, arrows fall back to the track style.
    ///
    /// ```rust
    /// use ratatui_core::style::{Color, Style};
    /// use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths)
    ///     .arrows(ScrollBarArrows::Both)
    ///     .arrow_style(Style::new().fg(Color::Yellow).bg(Color::Black));
    /// ```
    pub const fn arrow_style(mut self, style: Style) -> Self {
        self.arrow_style = Some(style);
        self
    }

    /// Selects the glyph set used to render the track and thumb.
    ///
    /// [`GlyphSet::symbols_for_legacy_computing`] uses [Symbols for Legacy Computing] for 1/8th
    /// upper/right fills. Use [`GlyphSet::unicode`] if you want to avoid the legacy supplement, or
    /// [`GlyphSet::box_drawing`] when you want a visible line track.
    ///
    /// [Symbols for Legacy Computing]: https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing
    ///
    /// ```rust
    /// use tui_scrollbar::{GlyphSet, ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).glyph_set(GlyphSet::unicode());
    /// ```
    pub const fn glyph_set(mut self, glyph_set: GlyphSet) -> Self {
        self.glyph_set = glyph_set;
        self
    }

    /// Sets which arrow endcaps are rendered.
    ///
    /// Each enabled arrow reserves one cell at the start or end of the track.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).arrows(ScrollBarArrows::Both);
    /// ```
    pub const fn arrows(mut self, arrows: ScrollBarArrows) -> Self {
        self.arrows = arrows;
        self
    }

    /// Sets behavior for clicks on the track outside the thumb.
    ///
    /// Use [`TrackClickBehavior::Page`] for classic page-up/down behavior, or
    /// [`TrackClickBehavior::JumpToClick`] to move the thumb toward the click.
    ///
    /// This does not affect clicks on the thumb or arrow endcaps.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollLengths, TrackClickBehavior};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar =
    ///     ScrollBar::vertical(lengths).track_click_behavior(TrackClickBehavior::JumpToClick);
    /// ```
    pub const fn track_click_behavior(mut self, behavior: TrackClickBehavior) -> Self {
        self.track_click_behavior = behavior;
        self
    }

    /// Sets the scroll step used for wheel events.
    ///
    /// The wheel delta is multiplied by this value (in your logical units) and then clamped. A
    /// step of 0 is normalized to 1.
    ///
    /// ```rust
    /// use tui_scrollbar::{ScrollBar, ScrollLengths};
    ///
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 40,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).scroll_step(8);
    /// ```
    pub fn scroll_step(mut self, step: usize) -> Self {
        self.scroll_step = step.max(1);
        self
    }

    /// Computes the inner track area and arrow cell positions for this orientation.
    fn arrow_layout(&self, area: Rect) -> ArrowLayout {
        let mut track_area = area;
        let (start, end) = match self.orientation {
            ScrollBarOrientation::Vertical => {
                let start_enabled = self.arrows.has_start() && area.height > 0;
                let end_enabled = self.arrows.has_end() && area.height > start_enabled as u16;
                let start = start_enabled.then_some((area.x, area.y));
                let end = end_enabled
                    .then_some((area.x, area.y.saturating_add(area.height).saturating_sub(1)));
                if start_enabled {
                    track_area.y = track_area.y.saturating_add(1);
                    track_area.height = track_area.height.saturating_sub(1);
                }
                if end_enabled {
                    track_area.height = track_area.height.saturating_sub(1);
                }
                (start, end)
            }
            ScrollBarOrientation::Horizontal => {
                let start_enabled = self.arrows.has_start() && area.width > 0;
                let end_enabled = self.arrows.has_end() && area.width > start_enabled as u16;
                let start = start_enabled.then_some((area.x, area.y));
                let end = end_enabled
                    .then_some((area.x.saturating_add(area.width).saturating_sub(1), area.y));
                if start_enabled {
                    track_area.x = track_area.x.saturating_add(1);
                    track_area.width = track_area.width.saturating_sub(1);
                }
                if end_enabled {
                    track_area.width = track_area.width.saturating_sub(1);
                }
                (start, end)
            }
        };

        ArrowLayout {
            track_area,
            start,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::style::{Color, Style};

    use super::*;
    use crate::glyphs::GlyphSet;
    use crate::ScrollLengths;

    #[test]
    fn builder_methods_update_fields() {
        let lengths = ScrollLengths {
            content_len: 10,
            viewport_len: 4,
        };
        let track_style = Style::new().fg(Color::Red);
        let thumb_style = Style::new().bg(Color::Blue);
        let arrow_style = Style::new().fg(Color::Green);
        let glyphs = GlyphSet::unicode();

        let scrollbar = ScrollBar::new(ScrollBarOrientation::Vertical, lengths)
            .orientation(ScrollBarOrientation::Horizontal)
            .content_len(20)
            .viewport_len(5)
            .offset(3)
            .track_style(track_style)
            .thumb_style(thumb_style)
            .arrow_style(arrow_style)
            .glyph_set(glyphs.clone())
            .arrows(ScrollBarArrows::End)
            .track_click_behavior(TrackClickBehavior::JumpToClick)
            .scroll_step(0);

        assert_eq!(scrollbar.orientation, ScrollBarOrientation::Horizontal);
        assert_eq!(scrollbar.content_len, 20);
        assert_eq!(scrollbar.viewport_len, 5);
        assert_eq!(scrollbar.offset, 3);
        assert_eq!(scrollbar.track_style, track_style);
        assert_eq!(scrollbar.thumb_style, thumb_style);
        assert_eq!(scrollbar.arrow_style, Some(arrow_style));
        assert_eq!(scrollbar.glyph_set, glyphs);
        assert_eq!(scrollbar.arrows, ScrollBarArrows::End);
        assert_eq!(
            scrollbar.track_click_behavior,
            TrackClickBehavior::JumpToClick
        );
        assert_eq!(scrollbar.scroll_step, 1);
    }

    #[test]
    fn constructors_set_orientation() {
        let lengths = ScrollLengths {
            content_len: 10,
            viewport_len: 4,
        };
        let vertical = ScrollBar::vertical(lengths);
        let horizontal = ScrollBar::horizontal(lengths);

        assert_eq!(vertical.orientation, ScrollBarOrientation::Vertical);
        assert_eq!(horizontal.orientation, ScrollBarOrientation::Horizontal);
    }

    #[test]
    fn reserves_track_cells_for_arrows() {
        let lengths = ScrollLengths {
            content_len: 10,
            viewport_len: 4,
        };
        let scrollbar = ScrollBar::vertical(lengths).arrows(ScrollBarArrows::Both);
        let area = Rect::new(0, 0, 1, 5);
        let layout = scrollbar.arrow_layout(area);

        assert_eq!(layout.track_area.height, 3);
        assert_eq!(layout.start, Some((area.x, area.y)));
        assert_eq!(
            layout.end,
            Some((area.x, area.y.saturating_add(area.height).saturating_sub(1)))
        );
    }

    #[test]
    fn reserves_track_cells_for_horizontal_arrows() {
        let lengths = ScrollLengths {
            content_len: 10,
            viewport_len: 4,
        };
        let scrollbar = ScrollBar::horizontal(lengths).arrows(ScrollBarArrows::Both);
        let area = Rect::new(0, 0, 5, 1);
        let layout = scrollbar.arrow_layout(area);

        assert_eq!(layout.track_area.width, 3);
        assert_eq!(layout.start, Some((area.x, area.y)));
        assert_eq!(
            layout.end,
            Some((area.x.saturating_add(area.width).saturating_sub(1), area.y))
        );
    }
}
