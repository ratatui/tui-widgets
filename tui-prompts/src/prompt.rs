use std::iter::once;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use itertools::chain;
use ratatui_core::layout::Rect;
use ratatui_core::terminal::Frame;
use ratatui_core::widgets::StatefulWidget;
use unicode_width::UnicodeWidthChar;

use crate::Status;

/// A prompt that can be drawn to a terminal.
pub trait Prompt: StatefulWidget {
    /// Draws the prompt widget.
    ///
    /// This is in addition to the [`StatefulWidget`] trait implementation as we need the [`Frame`]
    /// to set the cursor position.
    ///
    /// [`StatefulWidget`]: ratatui_core::widgets::StatefulWidget
    /// [`Frame`]: ratatui_core::terminal::Frame
    fn draw(self, frame: &mut Frame, area: Rect, state: &mut Self::State);
}

/// The focus state of a prompt.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum FocusState {
    #[default]
    Unfocused,
    Focused,
}

/// The state of a prompt.
///
/// Keybindings:
/// - Enter: Complete
/// - Esc | Ctrl+C: Abort
/// - Left | Ctrl+B: Move cursor left
/// - Right | Ctrl+F: Move cursor right
/// - Home | Ctrl+A: Move cursor to start of line
/// - End | Ctrl+E: Move cursor to end of line
/// - Backspace | Ctrl+H: Delete character before cursor
/// - Delete | Ctrl+D: Delete character after cursor
/// - Ctrl+K: Delete from cursor to end of line
/// - Ctrl+U: Delete from cursor to start of line
pub trait State {
    /// The status of the prompt.
    fn status(&self) -> Status;

    /// A mutable reference to the status of the prompt.
    fn status_mut(&mut self) -> &mut Status;

    /// A mutable reference to the focus state of the prompt.
    fn focus_state_mut(&mut self) -> &mut FocusState;

    /// The focus state of the prompt.
    fn focus_state(&self) -> FocusState;

    /// Sets the focus state of the prompt to [`FocusState::Focused`].
    fn focus(&mut self) {
        *self.focus_state_mut() = FocusState::Focused;
    }

    /// Sets the focus state of the prompt to [`FocusState::Unfocused`].
    fn blur(&mut self) {
        *self.focus_state_mut() = FocusState::Unfocused;
    }

    /// Whether the prompt is focused.
    fn is_focused(&self) -> bool {
        self.focus_state() == FocusState::Focused
    }

    /// The position of the cursor in the prompt.
    fn position(&self) -> usize;

    /// A mutable reference to the position of the cursor in the prompt.
    fn position_mut(&mut self) -> &mut usize;

    /// The cursor position of the prompt.
    fn cursor(&self) -> (u16, u16);

    /// A mutable reference to the cursor position of the prompt.
    fn cursor_mut(&mut self) -> &mut (u16, u16);

    /// The value of the prompt.
    fn value(&self) -> &str;

    /// A mutable reference to the value of the prompt.
    fn value_mut(&mut self) -> &mut String;

    fn len(&self) -> usize {
        self.value().chars().count()
    }

    fn is_empty(&self) -> bool {
        self.value().len() == 0
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Release {
            return;
        }

        match (key_event.code, key_event.modifiers) {
            (KeyCode::Enter, _) => self.complete(),
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.abort(),
            (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => self.move_left(),
            (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => self.move_right(),
            (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => self.move_start(),
            (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => self.move_end(),
            (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.backspace();
            }
            (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => self.delete(),
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => self.kill(),
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => self.truncate(),
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => self.push(c),
            _ => {}
        }
    }

    fn complete(&mut self) {
        *self.status_mut() = Status::Done;
    }

    fn abort(&mut self) {
        *self.status_mut() = Status::Aborted;
    }

    fn delete(&mut self) {
        let position = self.position();
        if position == self.len() {
            return;
        }
        *self.value_mut() = chain!(
            self.value().chars().take(position),
            self.value().chars().skip(position + 1)
        )
        .collect();
    }

    fn backspace(&mut self) {
        let position = self.position();
        if position == 0 {
            return;
        }
        *self.value_mut() = chain!(
            self.value().chars().take(position.saturating_sub(1)),
            self.value().chars().skip(position)
        )
        .collect();
        *self.position_mut() = position.saturating_sub(1);
    }

    fn move_right(&mut self) {
        if self.position() == self.len() {
            return;
        }
        *self.position_mut() = self.position().saturating_add(1);
    }

    fn move_left(&mut self) {
        *self.position_mut() = self.position().saturating_sub(1);
    }

    fn move_end(&mut self) {
        *self.position_mut() = self.len();
    }

    fn move_start(&mut self) {
        *self.position_mut() = 0;
    }

    fn kill(&mut self) {
        let position = self.position();
        self.value_mut().truncate(position);
    }

    fn truncate(&mut self) {
        self.value_mut().clear();
        *self.position_mut() = 0;
    }

    fn push(&mut self, c: char) {
        if self.position() == self.len() {
            self.value_mut().push(c);
        } else {
            // We cannot use String::insert() as it operates on bytes, which can lead to incorrect
            // modifications with multibyte characters. Instead, we handle text
            // manipulation at the character level using Rust's char type for Unicode
            // correctness. Check docs of String::insert() and String::chars() for further info.
            *self.value_mut() = chain![
                self.value().chars().take(self.position()),
                once(c),
                self.value().chars().skip(self.position())
            ]
            .collect();
        }
        *self.position_mut() = self.position().saturating_add(1);
    }

    /// The character width of the stored value up to the position pos.
    fn width_to_pos(&self, pos: usize) -> usize {
        self.value()
            .chars()
            .take(pos)
            // assign a width of zero to control characters
            .map(|x| x.width().unwrap_or(0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::style::Stylize;

    use super::*;

    #[test]
    fn status_symbols() {
        assert_eq!(Status::Pending.symbol(), "?".cyan());
        assert_eq!(Status::Aborted.symbol(), "✘".red());
        assert_eq!(Status::Done.symbol(), "✔".green());
    }
}
