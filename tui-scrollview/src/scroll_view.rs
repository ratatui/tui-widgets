use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Rect, Size};
use ratatui_core::style::Style;
use ratatui_core::symbols::scrollbar::{
    DOUBLE_HORIZONTAL, DOUBLE_VERTICAL, Set as ScrollbarSymbolsSet,
};
use ratatui_core::widgets::{StatefulWidget, Widget};
use ratatui_widgets::scrollbar::{Scrollbar, ScrollbarOrientation, ScrollbarState};

use crate::ScrollViewState;

/// A widget that can scroll its contents
///
/// Allows you to render a widget into a buffer larger than the area it is rendered into, and then
/// scroll the contents of that buffer around.
///
/// Note that the origin of the buffer is always at (0, 0), and the buffer is always the size of the
/// size passed to `new`. The `ScrollView` widget itself is responsible for rendering the visible
/// area of the buffer into the main buffer.
///
/// # Examples
///
/// ```rust
/// use ratatui::{prelude::*, layout::Size, widgets::*, symbols::scrollbar::Set};
/// use tui_scrollview::{ScrollView, ScrollViewState};
///
/// # fn render(buf: &mut Buffer) {
/// let mut scroll_view = ScrollView::new(Size::new(20, 20));
///
/// // you can set custom scrollbars orientation, symbols and style
/// scroll_view = scroll_view
///     .vertical_scrollbar_orientation(ScrollbarOrientation::VerticalRight)
///     .vertical_scrollbar_symbols(Set { begin: "├", thumb: "■", track: "─", end: "┤" })
///     .vertical_scrollbar_style(Style::new().dark_gray())
///     .horizontal_scrollbar_orientation(ScrollbarOrientation::VerticalRight)
///     .horizontal_scrollbar_symbols(Set { begin: "├", thumb: "■", track: "─", end: "┤" })
///     .horizontal_scrollbar_style(Style::new().dark_gray());
///
/// // render a few widgets into the buffer at various positions
/// scroll_view.render_widget(Paragraph::new("Hello, world!"), Rect::new(0, 0, 20, 1));
/// scroll_view.render_widget(Paragraph::new("Hello, world!"), Rect::new(10, 10, 20, 1));
/// scroll_view.render_widget(Paragraph::new("Hello, world!"), Rect::new(15, 15, 20, 1));
///
/// // You can also render widgets into the buffer programmatically
/// Line::raw("Hello, world!").render(Rect::new(0, 0, 20, 1), scroll_view.buf_mut());
///
/// // usually you would store the state of the scroll view in a struct that implements
/// // StatefulWidget (or in your app state if you're using an `App` struct)
/// let mut state = ScrollViewState::default();
///
/// // you can also scroll the view programmatically
/// state.scroll_down();
///
/// // render the scroll view into the main buffer at the given position within a widget
/// let scroll_view_area = Rect::new(0, 0, 10, 10);
/// scroll_view.render(scroll_view_area, buf, &mut state);
/// # }
/// // or if you're rendering in a terminal draw closure instead of from within another widget:
/// # fn terminal_draw(frame: &mut Frame, scroll_view: ScrollView, state: &mut ScrollViewState) {
/// frame.render_stateful_widget(scroll_view, frame.size(), state);
/// # }
/// ```
///
/// If you store the `ScrollView`, render it by reference so the same prepared buffer can be reused
/// across frames.
///
/// ```rust
/// use ratatui::prelude::*;
/// use tui_scrollview::{ScrollView, ScrollViewState};
///
/// # fn terminal_draw(frame: &mut Frame, scroll_view: &ScrollView, state: &mut ScrollViewState) {
/// frame.render_stateful_widget(scroll_view, frame.area(), state);
/// # }
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ScrollView<'a> {
    buf: Buffer,
    size: Size,
    vertical_scrollbar_orientation: ScrollbarOrientation,
    vertical_scrollbar_symbols: ScrollbarSymbolsSet<'a>,
    vertical_scrollbar_style: Style,
    vertical_scrollbar_visibility: ScrollbarVisibility,
    horizontal_scrollbar_orientation: ScrollbarOrientation,
    horizontal_scrollbar_symbols: ScrollbarSymbolsSet<'a>,
    horizontal_scrollbar_style: Style,
    horizontal_scrollbar_visibility: ScrollbarVisibility,
}

/// The visibility of the vertical and horizontal scrollbars.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ScrollbarVisibility {
    /// Render the scrollbar only whenever needed.
    #[default]
    Automatic,
    /// Always render the scrollbar.
    Always,
    /// Never render the scrollbar (hide it).
    Never,
}

impl<'a> ScrollView<'a> {
    /// Create a new scroll view with a buffer of the given size
    ///
    /// The buffer will be empty, with coordinates ranging from (0, 0) to (size.width, size.height).
    pub fn new(size: Size) -> Self {
        // TODO: this is replaced with Rect::from(size) in the next version of ratatui
        let area = Rect::new(0, 0, size.width, size.height);
        Self {
            buf: Buffer::empty(area),
            size,
            horizontal_scrollbar_orientation: ScrollbarOrientation::HorizontalBottom,
            horizontal_scrollbar_symbols: DOUBLE_HORIZONTAL,
            horizontal_scrollbar_style: Style::default(),
            horizontal_scrollbar_visibility: ScrollbarVisibility::default(),
            vertical_scrollbar_orientation: ScrollbarOrientation::VerticalRight,
            vertical_scrollbar_symbols: DOUBLE_VERTICAL,
            vertical_scrollbar_style: Style::default(),
            vertical_scrollbar_visibility: ScrollbarVisibility::default(),
        }
    }

    /// The content size of the scroll view
    pub const fn size(&self) -> Size {
        self.size
    }

    /// The area of the buffer that is available to be scrolled
    pub const fn area(&self) -> Rect {
        self.buf.area
    }

    /// The buffer containing the contents of the scroll view
    pub const fn buf(&self) -> &Buffer {
        &self.buf
    }

    /// The mutable buffer containing the contents of the scroll view
    ///
    /// This can be used to render widgets into the buffer programmatically
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};
    /// # use tui_scrollview::ScrollView;
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20));
    /// Line::raw("Hello, world!").render(Rect::new(0, 0, 20, 1), scroll_view.buf_mut());
    /// ```
    pub const fn buf_mut(&mut self) -> &mut Buffer {
        &mut self.buf
    }

    /// Set the vertical scrollbar orientation
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};  
    /// # use tui_scrollview::ScrollView;
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20))
    ///     .vertical_scrollbar_orientation(ScrollbarOrientation::VerticalRight);
    /// ```
    pub const fn vertical_scrollbar_orientation(
        mut self,
        orientation: ScrollbarOrientation,
    ) -> Self {
        self.vertical_scrollbar_orientation = orientation;
        self
    }

    /// Set the vertical scrollbar symbols set
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*, symbols::scrollbar::Set};  
    /// # use tui_scrollview::ScrollView;
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20))
    ///     .vertical_scrollbar_symbols(Set { begin: "├", thumb: "■", track: "─", end: "┤" });
    /// ```
    pub const fn vertical_scrollbar_symbols(mut self, set: ScrollbarSymbolsSet<'a>) -> Self {
        self.vertical_scrollbar_symbols = set;
        self
    }

    /// Set the vertical scrollbar style
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};  
    /// # use tui_scrollview::ScrollView;
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20))
    ///     .vertical_scrollbar_style(Style::new().dark_gray());
    /// ```
    pub const fn vertical_scrollbar_style(mut self, style: Style) -> Self {
        self.vertical_scrollbar_style = style;
        self
    }

    /// Set the visibility of the vertical scrollbar
    ///
    /// See [`ScrollbarVisibility`] for all the options.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};
    /// # use tui_scrollview::{ScrollView, ScrollbarVisibility};
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20))
    ///     .vertical_scrollbar_visibility(ScrollbarVisibility::Always);
    /// ```
    pub const fn vertical_scrollbar_visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.vertical_scrollbar_visibility = visibility;
        self
    }

    /// Set the horizontal scrollbar orientation
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};  
    /// # use tui_scrollview::ScrollView;
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20))
    ///     .horizontal_scrollbar_orientation(ScrollbarOrientation::HorizontalBottom);
    /// ```
    pub const fn horizontal_scrollbar_orientation(
        mut self,
        orientation: ScrollbarOrientation,
    ) -> Self {
        self.horizontal_scrollbar_orientation = orientation;
        self
    }

    /// Set the horizontal scrollbar symbols set
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*, symbols::scrollbar::Set};  
    /// # use tui_scrollview::ScrollView;
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20))
    ///     .horizontal_scrollbar_symbols(Set { begin: "├", thumb: "■", track: "─", end: "┤" });
    /// ```
    pub const fn horizontal_scrollbar_symbols(mut self, set: ScrollbarSymbolsSet<'a>) -> Self {
        self.horizontal_scrollbar_symbols = set;
        self
    }

    /// Set the horizontal scrollbar style
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};  
    /// # use tui_scrollview::ScrollView;
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20))
    ///     .horizontal_scrollbar_style(Style::new().dark_gray());
    /// ```
    pub const fn horizontal_scrollbar_style(mut self, style: Style) -> Self {
        self.horizontal_scrollbar_style = style;
        self
    }

    /// Set the visibility of the horizontal scrollbar
    ///
    /// See [`ScrollbarVisibility`] for all the options.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};
    /// # use tui_scrollview::{ScrollView, ScrollbarVisibility};
    ///
    /// let mut scroll_view = ScrollView::new(Size::new(20, 20))
    ///     .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);
    /// ```
    pub const fn horizontal_scrollbar_visibility(
        mut self,
        visibility: ScrollbarVisibility,
    ) -> Self {
        self.horizontal_scrollbar_visibility = visibility;
        self
    }

    /// Set the visibility of both vertical and horizontal scrollbars
    ///
    /// See [`ScrollbarVisibility`] for all the options.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, layout::Size, widgets::*};
    /// # use tui_scrollview::{ScrollView, ScrollbarVisibility};
    ///
    /// let mut scroll_view =
    ///     ScrollView::new(Size::new(20, 20)).scrollbars_visibility(ScrollbarVisibility::Automatic);
    /// ```
    pub const fn scrollbars_visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.vertical_scrollbar_visibility = visibility;
        self.horizontal_scrollbar_visibility = visibility;
        self
    }

    /// Render a widget into the scroll buffer
    ///
    /// This is the equivalent of `Frame::render_widget`, but renders the widget into the scroll
    /// buffer rather than the main buffer. The widget will be rendered into the area of the buffer
    /// specified by the `area` parameter.
    ///
    /// This should not be confused with the `render` method, which renders the visible area of the
    /// ScrollView into the main buffer.
    pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
        widget.render(area, &mut self.buf);
    }

    /// Render a stateful widget into the scroll buffer
    ///
    /// This is the equivalent of `Frame::render_stateful_widget`, but renders the stateful widget
    /// into the scroll buffer rather than the main buffer. The stateful widget will be rendered
    /// into the area of the buffer specified by the `area` parameter.
    ///
    /// This should not be confused with the `render` method, which renders the visible area of the
    /// ScrollView into the main buffer.
    pub fn render_stateful_widget<W: StatefulWidget>(
        &mut self,
        widget: W,
        area: Rect,
        state: &mut W::State,
    ) {
        widget.render(area, &mut self.buf, state);
    }
}

impl<'a> StatefulWidget for ScrollView<'a> {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        (&self).render(area, buf, state);
    }
}

impl<'a> StatefulWidget for &ScrollView<'a> {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (mut x, mut y) = state.offset.into();
        let horizontal_space = area.width as i32 - self.size.width as i32;
        let vertical_space = area.height as i32 - self.size.height as i32;
        let (show_horizontal, show_vertical) =
            self.visible_scrollbars(horizontal_space, vertical_space);

        // Scrollbars steal space from the viewport. Clamp offsets against that final viewport,
        // not the raw render area, so the bottom position is the last full page of content.
        let viewport_width = area.width.saturating_sub(show_vertical as u16);
        let viewport_height = area.height.saturating_sub(show_horizontal as u16);

        // If the content fits in a direction, discard any stale offset for that direction.
        if horizontal_space > 0 {
            x = 0;
        }
        if vertical_space > 0 {
            y = 0;
        }

        // `saturating_sub` covers both "content smaller than viewport" and zero-sized viewport
        // cases. Zero-sized areas later panic while rendering scrollbars, matching existing
        // behavior, but this arithmetic still must not wrap before that boundary.
        let max_x_offset = self.buf.area.width.saturating_sub(viewport_width);
        let max_y_offset = self.buf.area.height.saturating_sub(viewport_height);

        x = x.min(max_x_offset);
        y = y.min(max_y_offset);
        state.offset = (x, y).into();
        state.size = Some(self.size);
        let (viewport_area, content_area) = self.render_scrollbars(area, buf, state);
        state.page_size = Some(viewport_area.as_size());
        let visible_area = viewport_area.intersection(self.buf.area);
        self.render_visible_area(content_area, buf, visible_area);
    }
}

impl<'a> ScrollView<'a> {
    /// Render needed scrollbars.
    ///
    /// Returns a tuple of the visible content area relative to the scrollview's buffer area and
    /// the destination area within `area` where the content should be drawn.
    ///
    /// Scrollbars positioned on the leading (top or left) edges occupy the first row or column,
    /// so the content is shifted past them. Scrollbars on the trailing (bottom or right) edges
    /// leave the origin unchanged.
    fn render_scrollbars(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut ScrollViewState,
    ) -> (Rect, Rect) {
        // fit value per direction
        //   > 0 => fits
        //  == 0 => exact fit
        //   < 0 => does not fit
        let horizontal_space = area.width as i32 - self.size.width as i32;
        let vertical_space = area.height as i32 - self.size.height as i32;

        // If the content fits in a direction, reset state to reflect it.
        if horizontal_space > 0 {
            state.offset.x = 0;
        }
        if vertical_space > 0 {
            state.offset.y = 0;
        }

        let (show_horizontal, show_vertical) =
            self.visible_scrollbars(horizontal_space, vertical_space);

        let new_height = if show_horizontal {
            // if both bars are rendered, avoid the corner
            let width = area.width.saturating_sub(show_vertical as u16);
            let render_area = Rect { width, ..area };
            // render scrollbar, update available space
            self.render_horizontal_scrollbar(render_area, buf, state);
            area.height.saturating_sub(1)
        } else {
            area.height
        };

        let new_width = if show_vertical {
            // if both bars are rendered, avoid the corner
            let height = area.height.saturating_sub(show_horizontal as u16);
            let render_area = Rect { height, ..area };
            // render scrollbar, update available space
            self.render_vertical_scrollbar(render_area, buf, state);
            area.width.saturating_sub(1)
        } else {
            area.width
        };

        // Scrollbars on the leading (top/left) edges push the content away from the origin.
        let offset_x = u16::from(show_vertical && self.is_vertical_scrollbar_on_left());
        let offset_y = u16::from(show_horizontal && self.is_horizontal_scrollbar_on_top());
        let content_area = Rect::new(
            area.x.saturating_add(offset_x),
            area.y.saturating_add(offset_y),
            new_width,
            new_height,
        );

        (
            Rect::new(state.offset.x, state.offset.y, new_width, new_height),
            content_area,
        )
    }

    /// Resolve whether to render each scrollbar.
    ///
    /// Considers the visibility options set by the user and whether the scrollview size fits into
    /// the the available area on each direction.
    ///
    /// The space arguments are the difference between the scrollview size and the available area.
    ///
    /// Returns a bool tuple with (horizontal, vertical) resolutions.
    const fn visible_scrollbars(&self, horizontal_space: i32, vertical_space: i32) -> (bool, bool) {
        type V = crate::scroll_view::ScrollbarVisibility;

        match (
            self.horizontal_scrollbar_visibility,
            self.vertical_scrollbar_visibility,
        ) {
            // straightforward, no need to check fit values
            (V::Always, V::Always) => (true, true),
            (V::Never, V::Never) => (false, false),
            (V::Always, V::Never) => (true, false),
            (V::Never, V::Always) => (false, true),

            // Auto => render scrollbar only if it doesn't fit
            (V::Automatic, V::Never) => (horizontal_space < 0, false),
            (V::Never, V::Automatic) => (false, vertical_space < 0),

            // Auto => render scrollbar if:
            //   it doesn't fit; or
            //   exact fit (other scrollbar steals a line and triggers it)
            (V::Always, V::Automatic) => (true, vertical_space <= 0),
            (V::Automatic, V::Always) => (horizontal_space <= 0, true),

            // depends solely on fit values
            (V::Automatic, V::Automatic) => {
                if horizontal_space >= 0 && vertical_space >= 0 {
                    // there is enough space for both dimensions
                    (false, false)
                } else if horizontal_space < 0 && vertical_space < 0 {
                    // there is not enough space for either dimension
                    (true, true)
                } else if horizontal_space > 0 && vertical_space < 0 {
                    // horizontal fits, vertical does not
                    (false, true)
                } else if horizontal_space < 0 && vertical_space > 0 {
                    // vertical fits, horizontal does not
                    (true, false)
                } else {
                    // one is an exact fit and other does not fit which triggers both scrollbars to
                    // be visible because the other scrollbar will steal a line from the buffer
                    (true, true)
                }
            }
        }
    }

    fn render_vertical_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &ScrollViewState) {
        let scrollbar_height = self.size.height.saturating_sub(area.height);
        let mut scrollbar_state =
            ScrollbarState::new(scrollbar_height as usize).position(state.offset.y as usize);
        let scrollbar = Scrollbar::new(self.vertical_scrollbar_orientation.clone())
            .symbols(self.vertical_scrollbar_symbols.clone())
            .style(self.vertical_scrollbar_style);
        scrollbar.render(area, buf, &mut scrollbar_state)
    }

    fn render_horizontal_scrollbar(&self, area: Rect, buf: &mut Buffer, state: &ScrollViewState) {
        let scrollbar_width = self.size.width.saturating_sub(area.width);
        let mut scrollbar_state =
            ScrollbarState::new(scrollbar_width as usize).position(state.offset.x as usize);
        let scrollbar = Scrollbar::new(self.horizontal_scrollbar_orientation.clone())
            .symbols(self.horizontal_scrollbar_symbols.clone())
            .style(self.horizontal_scrollbar_style);
        scrollbar.render(area, buf, &mut scrollbar_state);
    }

    fn render_visible_area(&self, area: Rect, buf: &mut Buffer, visible_area: Rect) {
        // TODO: there's probably a more efficient way to do this
        for (src_row, dst_row) in visible_area.rows().zip(area.rows()) {
            for (src_col, dst_col) in src_row.columns().zip(dst_row.columns()) {
                buf[dst_col] = self.buf[src_col].clone();
            }
        }
    }

    fn is_horizontal_scrollbar_on_top(&self) -> bool {
        matches!(
            self.horizontal_scrollbar_orientation,
            ScrollbarOrientation::HorizontalTop
        )
    }

    fn is_vertical_scrollbar_on_left(&self) -> bool {
        matches!(
            self.vertical_scrollbar_orientation,
            ScrollbarOrientation::VerticalLeft
        )
    }
}

#[cfg(test)]
mod tests {
    use ratatui::symbols::scrollbar::Set;
    use ratatui_core::text::Span;
    use rstest::{fixture, rstest};

    use super::*;

    /// Initialize a buffer and a scroll view with a buffer size of 10x10
    ///
    /// The buffer will be filled with characters from A to Z in a 10x10 grid
    ///
    /// ```plain
    /// ABCDEFGHIJ
    /// KLMNOPQRST
    /// UVWXYZABCD
    /// EFGHIJKLMN
    /// OPQRSTUVWX
    /// YZABCDEFGH
    /// IJKLMNOPQR
    /// STUVWXYZAB
    /// CDEFGHIJKL
    /// MNOPQRSTUV
    /// ```
    #[fixture]
    fn scroll_view<'a>() -> ScrollView<'a> {
        let mut scroll_view = ScrollView::new(Size::new(10, 10));
        for y in 0..10 {
            for x in 0..10 {
                let c = char::from_u32((x + y * 10) % 26 + 65).unwrap();
                let widget = Span::raw(format!("{c}"));
                let area = Rect::new(x as u16, y as u16, 1, 1);
                scroll_view.render_widget(widget, area);
            }
        }
        scroll_view
    }

    #[test]
    fn dimensions_follow_content_and_viewport_changes() {
        let mut state = ScrollViewState::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        ScrollView::new(Size::new(10, 10)).render(buf.area, &mut buf, &mut state);
        assert_eq!(state.size(), Some(Size::new(10, 10)));
        assert_eq!(state.page_size(), Some(Size::new(5, 5)));

        let mut buf = Buffer::empty(Rect::new(0, 0, 8, 7));
        ScrollView::new(Size::new(10, 10))
            .scrollbars_visibility(ScrollbarVisibility::Never)
            .render(buf.area, &mut buf, &mut state);
        assert_eq!(state.size(), Some(Size::new(10, 10)));
        assert_eq!(state.page_size(), Some(Size::new(8, 7)));

        ScrollView::new(Size::new(2, 3)).render(buf.area, &mut buf, &mut state);
        assert_eq!(state.size(), Some(Size::new(2, 3)));
        assert_eq!(state.page_size(), Some(Size::new(8, 7)));
    }

    #[rstest]
    fn zero_offset(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::default();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDE▲",
                "KLMNO█",
                "UVWXY█",
                "EFGHI║",
                "OPQRS▼",
                "◄██═► ",
            ])
        )
    }

    #[rstest]
    fn render_by_reference_matches_owned_render(scroll_view: ScrollView) {
        let mut owned_state = ScrollViewState::default();
        let mut borrowed_state = ScrollViewState::default();
        let mut owned_buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut borrowed_buf = Buffer::empty(Rect::new(0, 0, 6, 6));

        scroll_view
            .clone()
            .render(owned_buf.area, &mut owned_buf, &mut owned_state);
        (&scroll_view).render(borrowed_buf.area, &mut borrowed_buf, &mut borrowed_state);

        assert_eq!(borrowed_buf, owned_buf);
        assert_eq!(borrowed_state, owned_state);
    }

    #[rstest]
    fn move_right(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::with_offset((3, 0).into());
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "DEFGH▲",
                "NOPQR█",
                "XYZAB█",
                "HIJKL║",
                "RSTUV▼",
                "◄═██► ",
            ])
        )
    }

    #[rstest]
    fn move_down(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::with_offset((0, 3).into());
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "EFGHI▲",
                "OPQRS║",
                "YZABC█",
                "IJKLM█",
                "STUVW▼",
                "◄██═► ",
            ])
        )
    }

    #[rstest]
    fn is_not_at_bottom_until_the_last_row_is_visible(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::with_offset((0, 4).into());

        scroll_view.render(buf.area, &mut buf, &mut state);

        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "OPQRS▲",
                "YZABC║",
                "IJKLM█",
                "STUVW█",
                "CDEFG▼",
                "◄██═► ",
            ])
        );
        assert!(!state.is_at_bottom());
    }

    #[rstest]
    fn move_to_bottom(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::default();

        // Prior to rendering, page and buffer size are unknown. We default to `true`.
        assert!(state.is_at_bottom());

        scroll_view.clone().render(buf.area, &mut buf, &mut state);

        // The vertical view size is five which means the page size is five.
        // We have not scrolled yet, so the view is at the top and not at the bottom.
        // => We see the top five rows
        assert!(!state.is_at_bottom());

        // Since the content height is ten,
        assert_eq!(state.size.unwrap().height, 10);
        // if we scroll down one page (five rows),
        state.scroll_down();
        state.scroll_down();
        state.scroll_down();
        state.scroll_down();
        state.scroll_down();

        // we reach the bottom,
        assert!(state.is_at_bottom());
        assert_eq!(state.offset.y, 5);

        // and we see the last five rows of the content.
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "YZABC▲",
                "IJKLM║",
                "STUVW█",
                "CDEFG█",
                "MNOPQ▼",
                "◄██═► ",
            ])
        );

        // We could also jump directly to the bottom...
        state.scroll_to_bottom();
        assert!(state.is_at_bottom());

        // ...which sets the offset to the last row of content,
        // ensuring to be at the bottom regardless of the page size.
        assert_eq!(state.offset.y, state.size.unwrap().height - 1);
    }

    #[rstest]
    fn rendering_at_bottom_uses_the_last_full_page(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 11, 6));
        let mut state = ScrollViewState::default();

        state.scroll_to_bottom();
        scroll_view.render(buf.area, &mut buf, &mut state);

        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "OPQRSTUVWX▲",
                "YZABCDEFGH║",
                "IJKLMNOPQR█",
                "STUVWXYZAB█",
                "CDEFGHIJKL█",
                "MNOPQRSTUV▼",
            ])
        );
        assert_eq!(state.offset.y, 4);
        assert_eq!(state.page_size.unwrap().height, 6);
    }

    #[rstest]
    #[case::always_always(
        ScrollbarVisibility::Always,
        ScrollbarVisibility::Always,
        1,
        1,
        (true, true)
    )]
    #[case::never_never(
        ScrollbarVisibility::Never,
        ScrollbarVisibility::Never,
        -1,
        -1,
        (false, false)
    )]
    #[case::always_never(
        ScrollbarVisibility::Always,
        ScrollbarVisibility::Never,
        1,
        1,
        (true, false)
    )]
    #[case::never_always(
        ScrollbarVisibility::Never,
        ScrollbarVisibility::Always,
        1,
        1,
        (false, true)
    )]
    #[case::automatic_never_needs_horizontal(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Never,
        -1,
        1,
        (true, false)
    )]
    #[case::automatic_never_fits_horizontal(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Never,
        1,
        -1,
        (false, false)
    )]
    #[case::never_automatic_needs_vertical(
        ScrollbarVisibility::Never,
        ScrollbarVisibility::Automatic,
        1,
        -1,
        (false, true)
    )]
    #[case::never_automatic_fits_vertical(
        ScrollbarVisibility::Never,
        ScrollbarVisibility::Automatic,
        -1,
        1,
        (false, false)
    )]
    #[case::always_automatic_exact_fit(
        ScrollbarVisibility::Always,
        ScrollbarVisibility::Automatic,
        1,
        0,
        (true, true)
    )]
    #[case::always_automatic_vertical_fits(
        ScrollbarVisibility::Always,
        ScrollbarVisibility::Automatic,
        1,
        1,
        (true, false)
    )]
    #[case::automatic_always_exact_fit(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Always,
        0,
        1,
        (true, true)
    )]
    #[case::automatic_always_horizontal_fits(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Always,
        1,
        1,
        (false, true)
    )]
    #[case::automatic_automatic_both_fit(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Automatic,
        1,
        1,
        (false, false)
    )]
    #[case::automatic_automatic_both_overflow(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Automatic,
        -1,
        -1,
        (true, true)
    )]
    #[case::automatic_automatic_only_vertical_overflows(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Automatic,
        1,
        -1,
        (false, true)
    )]
    #[case::automatic_automatic_only_horizontal_overflows(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Automatic,
        -1,
        1,
        (true, false)
    )]
    #[case::automatic_automatic_exact_fit_with_other_overflow(
        ScrollbarVisibility::Automatic,
        ScrollbarVisibility::Automatic,
        0,
        -1,
        (true, true)
    )]
    fn visible_scrollbars_honors_visibility_policy(
        #[case] horizontal_visibility: ScrollbarVisibility,
        #[case] vertical_visibility: ScrollbarVisibility,
        #[case] horizontal_space: i32,
        #[case] vertical_space: i32,
        #[case] expected: (bool, bool),
    ) {
        let scroll_view = ScrollView::new(Size::new(1, 1))
            .horizontal_scrollbar_visibility(horizontal_visibility)
            .vertical_scrollbar_visibility(vertical_visibility);

        assert_eq!(
            scroll_view.visible_scrollbars(horizontal_space, vertical_space),
            expected
        );
    }

    #[rstest]
    fn hides_both_scrollbars(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHIJ",
                "KLMNOPQRST",
                "UVWXYZABCD",
                "EFGHIJKLMN",
                "OPQRSTUVWX",
                "YZABCDEFGH",
                "IJKLMNOPQR",
                "STUVWXYZAB",
                "CDEFGHIJKL",
                "MNOPQRSTUV",
            ])
        )
    }

    #[rstest]
    fn hides_horizontal_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 11, 9));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHIJ▲",
                "KLMNOPQRST█",
                "UVWXYZABCD█",
                "EFGHIJKLMN█",
                "OPQRSTUVWX█",
                "YZABCDEFGH█",
                "IJKLMNOPQR█",
                "STUVWXYZAB█",
                "CDEFGHIJKL▼",
            ])
        )
    }

    #[rstest]
    fn hides_vertical_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 11));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHI",
                "KLMNOPQRS",
                "UVWXYZABC",
                "EFGHIJKLM",
                "OPQRSTUVW",
                "YZABCDEFG",
                "IJKLMNOPQ",
                "STUVWXYZA",
                "CDEFGHIJK",
                "MNOPQRSTU",
                "◄███████►",
            ])
        )
    }

    #[rstest]
    fn renders_custom_horizontal_bottom_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 11));
        let mut state = ScrollViewState::new();
        scroll_view
            .horizontal_scrollbar_symbols(Set {
                begin: "├",
                thumb: "■",
                track: "─",
                end: "┤",
            })
            .render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHI",
                "KLMNOPQRS",
                "UVWXYZABC",
                "EFGHIJKLM",
                "OPQRSTUVW",
                "YZABCDEFG",
                "IJKLMNOPQ",
                "STUVWXYZA",
                "CDEFGHIJK",
                "MNOPQRSTU",
                "├■■■■■■■┤",
            ])
        )
    }

    #[rstest]
    fn renders_custom_horizontal_top_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 11));
        let mut state = ScrollViewState::new();
        scroll_view
            .horizontal_scrollbar_orientation(ScrollbarOrientation::HorizontalTop)
            .horizontal_scrollbar_symbols(Set {
                begin: "├",
                thumb: "■",
                track: "─",
                end: "┤",
            })
            .render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "├■■■■■■■┤",
                "ABCDEFGHI",
                "KLMNOPQRS",
                "UVWXYZABC",
                "EFGHIJKLM",
                "OPQRSTUVW",
                "YZABCDEFG",
                "IJKLMNOPQ",
                "STUVWXYZA",
                "CDEFGHIJK",
                "MNOPQRSTU",
            ])
        )
    }

    #[rstest]
    fn renders_custom_vertical_right_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 11, 9));
        let mut state = ScrollViewState::new();
        scroll_view
            .vertical_scrollbar_symbols(Set {
                begin: "┬",
                thumb: "█",
                track: "│",
                end: "┴",
            })
            .render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHIJ┬",
                "KLMNOPQRST█",
                "UVWXYZABCD█",
                "EFGHIJKLMN█",
                "OPQRSTUVWX█",
                "YZABCDEFGH█",
                "IJKLMNOPQR█",
                "STUVWXYZAB█",
                "CDEFGHIJKL┴",
            ])
        )
    }

    #[rstest]
    fn renders_custom_vertical_left_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 11, 9));
        let mut state = ScrollViewState::new();
        scroll_view
            .vertical_scrollbar_orientation(ScrollbarOrientation::VerticalLeft)
            .vertical_scrollbar_symbols(Set {
                begin: "┬",
                thumb: "█",
                track: "│",
                end: "┴",
            })
            .render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "┬ABCDEFGHIJ",
                "█KLMNOPQRST",
                "█UVWXYZABCD",
                "█EFGHIJKLMN",
                "█OPQRSTUVWX",
                "█YZABCDEFGH",
                "█IJKLMNOPQR",
                "█STUVWXYZAB",
                "┴CDEFGHIJKL",
            ])
        )
    }

    /// Tests the scenario where the vertical scrollbar steals a column from the right side of the
    /// buffer which causes the horizontal scrollbar to be shown.
    #[rstest]
    fn does_not_hide_horizontal_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 9));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHI▲",
                "KLMNOPQRS█",
                "UVWXYZABC█",
                "EFGHIJKLM█",
                "OPQRSTUVW█",
                "YZABCDEFG█",
                "IJKLMNOPQ║",
                "STUVWXYZA▼",
                "◄███████► ",
            ])
        )
    }

    /// Tests the scenario where the horizontal scrollbar steals a row from the bottom side of the
    /// buffer which causes the vertical scrollbar to be shown.
    #[rstest]
    fn does_not_hide_vertical_scrollbar(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 10));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGH▲",
                "KLMNOPQR█",
                "UVWXYZAB█",
                "EFGHIJKL█",
                "OPQRSTUV█",
                "YZABCDEF█",
                "IJKLMNOP█",
                "STUVWXYZ█",
                "CDEFGHIJ▼",
                "◄█████═► ",
            ])
        )
    }

    /// The purpose of this test is to ensure that the buffer offset is correctly calculated when
    /// rendering a scroll view into a buffer (i.e. the buffer offset is not always (0, 0)).
    #[rstest]
    fn ensure_buffer_offset_is_correct(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 20));
        let mut state = ScrollViewState::with_offset((2, 3).into());
        scroll_view.render(Rect::new(5, 6, 7, 8), &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "     GHIJKL▲        ",
                "     QRSTUV║        ",
                "     ABCDEF█        ",
                "     KLMNOP█        ",
                "     UVWXYZ█        ",
                "     EFGHIJ█        ",
                "     OPQRST▼        ",
                "     ◄═███►         ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
                "                    ",
            ])
        )
    }
    /// The purpose of this test is to ensure that the last elements are rendered.
    #[rstest]
    fn ensure_buffer_last_elements(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::with_offset((5, 5).into());
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "DEFGH▲",
                "NOPQR║",
                "XYZAB█",
                "HIJKL█",
                "RSTUV▼",
                "◄═██► ",
            ])
        )
    }
    #[rstest]
    fn zero_width(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 0, 10));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(buf, Buffer::empty(Rect::new(0, 0, 0, 10)));
    }

    #[rstest]
    fn zero_height(scroll_view: ScrollView) {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 0));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(buf, Buffer::empty(Rect::new(0, 0, 10, 0)));
    }

    #[rstest]
    fn never_vertical_scrollbar(mut scroll_view: ScrollView) {
        scroll_view = scroll_view.vertical_scrollbar_visibility(ScrollbarVisibility::Never);
        let mut buf = Buffer::empty(Rect::new(0, 0, 11, 9));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHIJ ",
                "KLMNOPQRST ",
                "UVWXYZABCD ",
                "EFGHIJKLMN ",
                "OPQRSTUVWX ",
                "YZABCDEFGH ",
                "IJKLMNOPQR ",
                "STUVWXYZAB ",
                "CDEFGHIJKL ",
            ])
        )
    }

    #[rstest]
    fn never_horizontal_scrollbar(mut scroll_view: ScrollView) {
        scroll_view = scroll_view.horizontal_scrollbar_visibility(ScrollbarVisibility::Never);
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 11));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHI",
                "KLMNOPQRS",
                "UVWXYZABC",
                "EFGHIJKLM",
                "OPQRSTUVW",
                "YZABCDEFG",
                "IJKLMNOPQ",
                "STUVWXYZA",
                "CDEFGHIJK",
                "MNOPQRSTU",
                "         ",
            ])
        )
    }

    #[rstest]
    fn does_not_trigger_horizontal_scrollbar(mut scroll_view: ScrollView) {
        scroll_view = scroll_view.vertical_scrollbar_visibility(ScrollbarVisibility::Never);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 9));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHIJ",
                "KLMNOPQRST",
                "UVWXYZABCD",
                "EFGHIJKLMN",
                "OPQRSTUVWX",
                "YZABCDEFGH",
                "IJKLMNOPQR",
                "STUVWXYZAB",
                "CDEFGHIJKL",
            ])
        )
    }

    #[rstest]
    fn does_not_trigger_vertical_scrollbar(mut scroll_view: ScrollView) {
        scroll_view = scroll_view.horizontal_scrollbar_visibility(ScrollbarVisibility::Never);
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 10));
        let mut state = ScrollViewState::new();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEFGHI",
                "KLMNOPQRS",
                "UVWXYZABC",
                "EFGHIJKLM",
                "OPQRSTUVW",
                "YZABCDEFG",
                "IJKLMNOPQ",
                "STUVWXYZA",
                "CDEFGHIJK",
                "MNOPQRSTU",
            ])
        )
    }

    #[rstest]
    fn does_not_render_vertical_scrollbar(mut scroll_view: ScrollView) {
        scroll_view = scroll_view.vertical_scrollbar_visibility(ScrollbarVisibility::Never);
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::default();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEF",
                "KLMNOP",
                "UVWXYZ",
                "EFGHIJ",
                "OPQRST",
                "◄███═►",
            ])
        )
    }

    #[rstest]
    fn does_not_render_horizontal_scrollbar(mut scroll_view: ScrollView) {
        scroll_view = scroll_view.horizontal_scrollbar_visibility(ScrollbarVisibility::Never);
        let mut buf = Buffer::empty(Rect::new(0, 0, 7, 6));
        let mut state = ScrollViewState::default();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEF▲",
                "KLMNOP█",
                "UVWXYZ█",
                "EFGHIJ█",
                "OPQRST║",
                "YZABCD▼",
            ])
        )
    }

    #[rstest]
    #[rustfmt::skip]
    fn does_not_render_both_scrollbars(mut scroll_view: ScrollView) {
        scroll_view = scroll_view.scrollbars_visibility(ScrollbarVisibility::Never);
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 6));
        let mut state = ScrollViewState::default();
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "ABCDEF",
                "KLMNOP",
                "UVWXYZ",
                "EFGHIJ",
                "OPQRST",
                "YZABCD",
            ])
        )
    }

    #[rstest]
    #[rustfmt::skip]
    fn render_stateful_widget(mut scroll_view: ScrollView) {
        use ratatui_widgets::list::{List, ListState};
        scroll_view = scroll_view.horizontal_scrollbar_visibility(ScrollbarVisibility::Never);
        let mut buf = Buffer::empty(Rect::new(0, 0, 7, 5));
        let mut state = ScrollViewState::default();
        let mut list_state = ListState::default();
        let items: Vec<String> = (1..=10).map(|i| format!("Item {i}")).collect();
        let list = List::new(items);
        scroll_view.render_stateful_widget(list, scroll_view.area(), &mut list_state);
        scroll_view.render(buf.area, &mut buf, &mut state);
        assert_eq!(
            buf,
            Buffer::with_lines(vec![
                "Item 1▲",
                "Item 2█",
                "Item 3█",
                "Item 4║",
                "Item 5▼",
            ])
        )
    }
}
