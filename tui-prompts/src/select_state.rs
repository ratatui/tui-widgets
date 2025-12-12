use std::borrow::Cow;

use crate::prelude::*;
use crate::select_prompt::SelectOption;
use crate::State;
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SelectState<'a> {
    status: Status,
    focus: FocusState,
    focused_index: usize,
    value: Cow<'a, String>,
    options: Cow<'a, [SelectOption<'a>]>,
    cursor: (u16, u16),
}

impl<'a> SelectState<'a> {
    pub fn new(options: impl Into<Cow<'a, [SelectOption<'a>]>>) -> Self {
        Self {
            options: options.into(),
            ..Default::default()
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

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.status.is_finished()
    }

    pub fn with_focused_index(&mut self, index: usize) -> &mut Self {
        self.focused_index = index;
        self
    }
}

impl State for SelectState<'_> {
    fn status(&self) -> Status {
        self.status
    }

    fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }

    fn focus_state(&self) -> FocusState {
        self.focus
    }

    fn position(&self) -> usize {
        self.focused_index
    }

    fn position_mut(&mut self) -> &mut usize {
        &mut self.focused_index
    }

    fn cursor(&self) -> (u16, u16) {
        self.cursor
    }

    fn cursor_mut(&mut self) -> &mut (u16, u16) {
        &mut self.cursor
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn value_mut(&mut self) -> &mut String {
        self.value.to_mut()
    }
}
