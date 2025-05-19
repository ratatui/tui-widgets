use ratatui_core::layout::{Position, Size};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ScrollViewState {
    /// The offset is the number of rows and columns to shift the scroll view by.
    pub(crate) offset: Position,
    /// The size of the scroll view. Not set until the first render call.
    pub(crate) size: Option<Size>,
    /// The size of a page of the scroll view. Not set until the first render call.
    pub(crate) page_size: Option<Size>,
}

impl ScrollViewState {
    /// Create a new scroll view state with an offset of (0, 0)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new scroll view state with the given offset
    pub fn with_offset(offset: Position) -> Self {
        Self {
            offset,
            ..Default::default()
        }
    }

    /// Set the offset of the scroll view state
    pub const fn set_offset(&mut self, offset: Position) {
        self.offset = offset;
    }

    /// Get the offset of the scroll view state
    pub const fn offset(&self) -> Position {
        self.offset
    }

    /// Move the scroll view state up by one row
    pub const fn scroll_up(&mut self) {
        self.offset.y = self.offset.y.saturating_sub(1);
    }

    /// Move the scroll view state down by one row
    pub const fn scroll_down(&mut self) {
        self.offset.y = self.offset.y.saturating_add(1);
    }

    /// Move the scroll view state down by one page
    pub fn scroll_page_down(&mut self) {
        let page_size = self.page_size.map_or(1, |size| size.height);
        // we subtract 1 to ensure that there is a one row overlap between pages
        self.offset.y = self.offset.y.saturating_add(page_size).saturating_sub(1);
    }

    /// Move the scroll view state up by one page
    pub fn scroll_page_up(&mut self) {
        let page_size = self.page_size.map_or(1, |size| size.height);
        // we add 1 to ensure that there is a one row overlap between pages
        self.offset.y = self.offset.y.saturating_add(1).saturating_sub(page_size);
    }

    /// Move the scroll view state left by one column
    pub const fn scroll_left(&mut self) {
        self.offset.x = self.offset.x.saturating_sub(1);
    }

    /// Move the scroll view state right by one column
    pub const fn scroll_right(&mut self) {
        self.offset.x = self.offset.x.saturating_add(1);
    }

    /// Move the scroll view state to the top of the buffer
    pub const fn scroll_to_top(&mut self) {
        self.offset = Position::ORIGIN;
    }

    /// Move the scroll view state to the bottom of the buffer
    ///
    /// If the buffer size is not yet computed (done during the first rendering), it will not
    /// be taken into account and the scroll offset will be set to the maximum value: `u16::MAX`
    pub fn scroll_to_bottom(&mut self) {
        // the render call will adjust the offset to ensure that we don't scroll past the end of
        // the buffer, so we can set the offset to the maximum value here
        let bottom = self
            .size
            .map_or(u16::MAX, |size| size.height.saturating_sub(1));
        self.offset.y = bottom;
    }

    /// True if the scroll view state is at the bottom of the buffer
    ///
    /// This takes the page size into account. It returns true if the current scroll offset plus
    /// the page size matches or exceeds the buffer length.
    ///
    /// The buffer and the page size are unknown until computed during the first rendering. If the
    /// page size is not yet known, it won't be taken into account. If the buffer is not yet known,
    /// this function always returns true.
    pub fn is_at_bottom(&self) -> bool {
        let bottom = self.size.map_or(0, |size| size.height.saturating_sub(1));
        let page_size = self.page_size.map_or(0, |size| size.height);
        self.offset.y.saturating_add(page_size) >= bottom
    }
}
