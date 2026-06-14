use std::borrow::Cow;

use crate::State;
use crate::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TextState<'a> {
    status: Status,
    focus: FocusState,
    position: usize,
    cursor: (u16, u16),
    value: Cow<'a, str>,
}

impl<'a> TextState<'a> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            status: Status::Pending,
            focus: FocusState::Unfocused,
            position: 0,
            cursor: (0, 0),
            value: Cow::Borrowed(""),
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

    #[must_use]
    pub fn with_value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.value = value.into();
        self
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.status.is_finished()
    }
}

impl State for TextState<'_> {
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
        self.position
    }

    fn position_mut(&mut self) -> &mut usize {
        &mut self.position
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

#[cfg(test)]
mod tests {
    use crate::{State, TextState};

    #[test]
    fn insert_multibyte_start() {
        let mut test = TextState::new().with_value("äë");
        test.move_start();
        test.push('Ï');
        assert_eq!(test.value(), "Ïäë");
        assert_eq!(test.position(), 1);
    }
    #[test]
    fn insert_multibyte_middle() {
        let mut test = TextState::new().with_value("äë");
        test.move_right();
        test.push('Ï');
        assert_eq!(test.value(), "äÏë");
        assert_eq!(test.position(), 2);
    }
    #[test]
    fn insert_multibyte_end() {
        let mut test = TextState::new().with_value("äë");
        test.move_end();
        test.push('Ï');
        assert_eq!(test.value(), "äëÏ");
        assert_eq!(test.position(), 3);
    }

    #[test]
    fn delete_multibyte_start() {
        let mut test = TextState::new().with_value("äë");
        test.move_start();
        test.delete();
        assert_eq!(test.value(), "ë");
        assert_eq!(test.position(), 0);
    }

    #[test]
    fn delete_multibyte_middle() {
        let mut test = TextState::new().with_value("äë");
        test.move_right();
        test.delete();
        assert_eq!(test.value(), "ä");
        assert_eq!(test.position(), 1);
    }

    #[test]
    fn delete_multibyte_end() {
        let mut test = TextState::new().with_value("äë");
        test.move_end();
        test.delete();
        assert_eq!(test.value(), "äë");
        assert_eq!(test.position(), 2);
    }

    #[test]
    fn backspace_multibyte_start() {
        let mut test = TextState::new().with_value("äë");
        test.move_start();
        test.backspace();
        assert_eq!(test.value(), "äë");
        assert_eq!(test.position(), 0);
    }

    #[test]
    fn backspace_multibyte_middle() {
        let mut test = TextState::new().with_value("äë");
        test.move_right();
        test.backspace();
        assert_eq!(test.value(), "ë");
        assert_eq!(test.position(), 0);
    }

    #[test]
    fn backspace_multibyte_end() {
        let mut test = TextState::new().with_value("äë");
        test.move_end();
        test.backspace();
        assert_eq!(test.value(), "ä");
        assert_eq!(test.position(), 1);
    }
}
