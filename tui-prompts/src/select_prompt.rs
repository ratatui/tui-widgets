use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph, StatefulWidget, Widget};

use crate::prelude::*;
use crate::select_state::SelectState;
use ratatui::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SelectPrompt<'a> {
    pub label: Option<&'a str>,
    pub options: Vec<SelectOption<'a>>,
    pub block: Option<Block<'a>>,
    pub focused_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectOption<'a> {
    pub value: &'a str,
    pub index: usize,
}
impl<'a> SelectPrompt<'a> {
    pub const fn new(label: &'a str, options: Vec<SelectOption<'a>>) -> Self {
        SelectPrompt {
            label: Some(label),
            options,
            block: None,
            focused_index: 0,
        }
    }

    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Prompt for SelectPrompt<'_> {
    fn draw(self, frame: &mut Frame, area: Rect, state: &mut Self::State) {
        frame.render_stateful_widget(self, area, state);
        if state.is_focused() {
            frame.set_cursor_position(state.cursor());
        }
    }
}

impl<'a> StatefulWidget for SelectPrompt<'a> {
    type State = SelectState<'a>;

    fn render(mut self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::widgets::Borders;
    use ratatui_macros::line;
    use rstest::{fixture, rstest};
    #[test]
    fn new() {
        let options = vec![
            SelectOption {
                value: "Option 1",
                index: 0,
            },
            SelectOption {
                value: "Option 2",
                index: 1,
            },
            SelectOption {
                value: "Option 3",
                index: 2,
            },
        ];

        let prompt = SelectPrompt::new("label", options.clone());
        assert_eq!(prompt.options, options);
        assert_eq!(prompt.focused_index, 0);
        assert!(prompt.block.is_none());
    }

    #[test]
    fn default() {
        let prompt = SelectPrompt::default();
        assert_eq!(prompt.options, Vec::new());
        assert_eq!(prompt.focused_index, 0);
        assert_eq!(prompt.block, None);
    }
}
