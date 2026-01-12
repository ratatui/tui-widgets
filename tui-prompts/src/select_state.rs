use crate::prelude::*;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SelectState {
    status: Status,
    focus: FocusState,
    focused_index: usize,
    //TODO: remove options_len and use options.len() directly
    options_len: usize,
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
        if self.focused_index < self.options_len.saturating_sub(1) {
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
        if self.options_len > 0 && index >= self.options_len {
            panic!(
                "Focused index {} out of bounds (len: {})",
                index, self.options_len
            );
        }
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

    pub fn complete(&mut self) {
        self.status = Status::Done;
    }

    pub fn abort(&mut self) {
        self.status = Status::Aborted;
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Enter => self.complete(),
            KeyCode::Esc => self.abort(),
            _ => {}
        }
    }
}
