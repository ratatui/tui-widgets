use crate::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SelectState {
    status: Status,
    focus: FocusState,
    focused_index: usize,
    options_len: usize,
    cursor: (u16, u16),
}

impl SelectState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn move_up(&mut self) {
        if self.focused_index > 0 {
            self.focused_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.focused_index < self.options_len - 1 {
            self.focused_index += 1;
        }
    }
    #[must_use]
    pub const fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub const fn with_focus(mut self, focus: FocusState) -> Self {
        self.focus = focus;
        self
    }

    pub fn focused_index(&self) -> usize {
        self.focused_index
    }

    pub fn set_focused_index(&mut self, index: usize) {
        self.focused_index = index;
    }

    pub fn set_options_len(&mut self, len: usize) {
        self.options_len = len;
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.status.is_finished()
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn with_focused_index(&mut self, index: usize) -> &mut Self {
        self.focused_index = index;
        self
    }
}
