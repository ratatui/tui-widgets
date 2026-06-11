use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::prelude::*;

/// The state for a [`SelectPrompt`](crate::SelectPrompt).
///
/// Rendering a [`SelectPrompt`](crate::SelectPrompt) keeps the focused index within the rendered
/// options. Until the prompt has been rendered at least once, explicit focused-index changes are
/// accepted and clamped on the next render. Once the state is completed or aborted,
/// [`SelectState::handle_key_event`] ignores later key events.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SelectState {
    status: Status,
    focus: FocusState,
    pub(crate) focused_index: usize,
    pub(crate) option_count: usize,
}

impl SelectState {
    /// Creates a pending, unfocused select state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            status: Status::Pending,
            focus: FocusState::Unfocused,
            focused_index: 0,
            option_count: 0,
        }
    }

    /// Moves focus to the previous option.
    ///
    /// Focus stays at index 0 when the first option is already focused.
    pub fn move_up(&mut self) {
        if self.focused_index > 0 {
            self.focused_index -= 1;
        }
    }

    /// Moves focus to the next option.
    ///
    /// Focus stays at the last rendered option. The prompt learns the option count during render,
    /// so this is a no-op until the state has been rendered with at least one visible option.
    pub fn move_down(&mut self) {
        if self.focused_index < self.option_count.saturating_sub(1) {
            self.focused_index += 1;
        }
    }

    /// Sets the prompt status.
    #[must_use]
    pub const fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Sets whether the prompt is focused.
    #[must_use]
    pub const fn with_focus(mut self, focus: FocusState) -> Self {
        self.focus = focus;
        self
    }

    /// Returns the currently focused option index.
    #[must_use]
    pub const fn focused_index(&self) -> usize {
        self.focused_index
    }

    /// Sets the focused option index.
    ///
    /// If the state has already been rendered with options, the index is clamped to the last
    /// available option. If the state has not been rendered yet, the value is accepted and clamped
    /// during the next render.
    pub fn set_focused_index(&mut self, index: usize) {
        self.focused_index = self.clamp_focused_index(index);
    }

    /// Returns whether the select prompt has completed or aborted.
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.status.is_finished()
    }

    /// Returns the current prompt status.
    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Sets focus to [`FocusState::Focused`].
    pub fn focus(&mut self) {
        self.focus = FocusState::Focused;
    }

    /// Sets focus to [`FocusState::Unfocused`].
    pub fn blur(&mut self) {
        self.focus = FocusState::Unfocused;
    }

    /// Returns whether the select prompt is focused.
    #[must_use]
    pub fn is_focused(&self) -> bool {
        self.focus == FocusState::Focused
    }

    /// Completes the prompt.
    pub fn complete(&mut self) {
        self.status = Status::Done;
    }

    /// Aborts the prompt.
    pub fn abort(&mut self) {
        self.status = Status::Aborted;
    }

    /// Handles a key event for select prompt navigation and completion.
    ///
    /// Pressing Up or Down moves the focused option, Enter completes the prompt when an option is
    /// visible, and Escape or Ctrl+C aborts the prompt. Key release events and events routed after
    /// completion or abort are ignored.
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Release || self.status.is_finished() {
            return;
        }

        match (key.code, key.modifiers) {
            (KeyCode::Up, _) => self.move_up(),
            (KeyCode::Down, _) => self.move_down(),
            (KeyCode::Enter, _) if self.option_count > 0 => self.complete(),
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.abort(),
            _ => {}
        }
    }

    pub(crate) const fn clamp_focused_index(&self, index: usize) -> usize {
        if self.option_count == 0 {
            index
        } else if index >= self.option_count {
            self.option_count - 1
        } else {
            index
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode, kind: KeyEventKind) -> KeyEvent {
        KeyEvent::new_with_kind(code, KeyModifiers::NONE, kind)
    }

    fn ctrl_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new_with_kind(code, KeyModifiers::CONTROL, KeyEventKind::Press)
    }

    #[test]
    fn render_option_count_clamps_focused_index() {
        let mut state = SelectState::new();

        state.set_focused_index(5);
        state.option_count = 3;
        state.focused_index = state.clamp_focused_index(state.focused_index);

        assert_eq!(state.focused_index(), 2);
    }

    #[test]
    fn set_focused_index_clamps_when_option_count_is_known() {
        let mut state = SelectState::new();

        state.option_count = 3;
        state.set_focused_index(5);

        assert_eq!(state.focused_index(), 2);
    }

    #[test]
    fn move_down_stops_at_last_option() {
        let mut state = SelectState::new();
        state.option_count = 2;

        state.move_down();
        state.move_down();

        assert_eq!(state.focused_index(), 1);
    }

    #[test]
    fn handle_key_event_accepts_repeated_navigation() {
        let mut state = SelectState::new();
        state.option_count = 2;

        state.handle_key_event(key(KeyCode::Down, KeyEventKind::Repeat));

        assert_eq!(state.focused_index(), 1);
    }

    #[test]
    fn handle_key_event_ignores_key_release() {
        let mut state = SelectState::new();
        state.option_count = 2;

        state.handle_key_event(key(KeyCode::Down, KeyEventKind::Release));

        assert_eq!(state.focused_index(), 0);
    }

    #[test]
    fn handle_key_event_aborts_on_ctrl_c() {
        let mut state = SelectState::new();

        state.handle_key_event(ctrl_key(KeyCode::Char('c')));

        assert_eq!(state.status(), Status::Aborted);
    }

    #[test]
    fn handle_key_event_ignores_events_after_completion() {
        let mut state = SelectState::new();
        state.option_count = 2;
        state.handle_key_event(key(KeyCode::Enter, KeyEventKind::Press));

        state.handle_key_event(key(KeyCode::Down, KeyEventKind::Press));
        state.handle_key_event(key(KeyCode::Esc, KeyEventKind::Press));

        assert_eq!(state.focused_index(), 0);
        assert_eq!(state.status(), Status::Done);
    }

    #[test]
    fn handle_key_event_ignores_events_after_abort() {
        let mut state = SelectState::new();
        state.option_count = 2;
        state.handle_key_event(key(KeyCode::Esc, KeyEventKind::Press));

        state.handle_key_event(key(KeyCode::Enter, KeyEventKind::Press));

        assert_eq!(state.status(), Status::Aborted);
    }

    #[test]
    fn handle_key_event_does_not_complete_without_visible_options() {
        let mut state = SelectState::new();

        state.handle_key_event(key(KeyCode::Enter, KeyEventKind::Press));

        assert_eq!(state.status(), Status::Pending);
    }
}
