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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectOption<'a> {
    pub value: &'a str,
}
impl<'a> SelectPrompt<'a> {
    pub const fn new(label: &'a str, options: Vec<SelectOption<'a>>) -> Self {
        SelectPrompt {
            label: Some(label),
            options,
            block: None,
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
    }
}

impl<'a> StatefulWidget for SelectPrompt<'a> {
    type State = SelectState;

    fn render(mut self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let Some(block) = self.block.take() {
            let inner_area = block.inner(area);
            block.render(area, buf);
            area = inner_area;
        }

        let mut lines = Vec::new();
        if let Some(label) = self.label {
            lines.push(Line::from(vec![
                state.status().symbol(),
                " ".into(),
                label.bold(),
            ]));
        }
        for (i, option) in self.options.iter().enumerate() {
            let line = if i == state.focused_index() {
                Line::from(Span::styled(
                    format!("> {}", option.value),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::raw(format!("  {}", option.value)))
            };
            lines.push(line);
        }
        Paragraph::new(lines)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::widgets::Borders;
    #[test]
    fn new() {
        let options = vec![
            SelectOption { value: "Option 1" },
            SelectOption { value: "Option 2" },
            SelectOption { value: "Option 3" },
        ];

        let prompt = SelectPrompt::new("label", options.clone());
        assert_eq!(prompt.options, options);
        assert!(prompt.block.is_none());
    }

    #[test]
    fn default() {
        let prompt = SelectPrompt::default();
        assert_eq!(prompt.options, Vec::new());
        assert_eq!(prompt.block, None);
    }

    #[test]
    fn render_with_max_options() {
        let options = vec![
            SelectOption { value: "Option 1" },
            SelectOption { value: "Option 2" },
            SelectOption { value: "Option 3" },
        ];

        let prompt = SelectPrompt::new("label", options.clone())
            .with_block(Block::default().borders(Borders::ALL).title("Select"));

        let mut state = SelectState::default();
        state.with_focused_index(1);

        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = f.area();
                prompt.clone().draw(f, area, &mut state);
            })
            .unwrap();

        let mut expected = Buffer::with_lines(vec![
            "┌Select────────────┐",
            "│? label           │",
            "│  Option 1        │",
            "│> Option 2        │",
            "│  Option 3        │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]);

        expected.set_style(Rect::new(1, 1, 1, 1), Color::Cyan);

        expected.set_style(Rect::new(3, 1, 5, 1), Modifier::BOLD);

        expected.set_style(Rect::new(1, 3, 10, 1), (Color::Yellow, Modifier::BOLD));

        terminal.backend().assert_buffer(&expected);


    }
}
