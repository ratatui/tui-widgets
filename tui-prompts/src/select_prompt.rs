use std::borrow::Cow;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Alignment, Rect};
use ratatui_core::style::{Color, Modifier, Style, Stylize};
use ratatui_core::terminal::Frame;
use ratatui_core::text::{Line, Span};
use ratatui_core::widgets::{StatefulWidget, Widget};
use ratatui_widgets::block::Block;
use ratatui_widgets::paragraph::Paragraph;

use crate::prelude::*;
use crate::select_state::SelectState;

/// A prompt widget for choosing one option from a list.
///
/// `SelectPrompt` owns the label and options to render, while [`SelectState`] tracks the focused
/// option, focus state, and completion status. Render the prompt before routing key events so the
/// state can learn how many options are currently selectable.
///
/// When the render area is shorter than the option list, the prompt renders a window around the
/// focused option so the highlighted row remains visible.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SelectPrompt<'a> {
    label: Option<Cow<'a, str>>,
    options: SelectOptionList<'a>,
    block: Option<Block<'a>>,
}

/// An ordered list of selectable values for a [`SelectPrompt`].
///
/// Arrays and vectors of values that convert into [`SelectOption`] can be converted into a
/// `SelectOptionList`, which keeps simple string lists concise:
///
/// ```
/// use std::borrow::Cow;
///
/// use tui_prompts::{SelectOptionList, SelectPrompt};
///
/// let options: SelectOptionList = ["Rust", "Zig", "Go"].into();
/// let prompt = SelectPrompt::new(Cow::Borrowed("Language"), options);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SelectOptionList<'a> {
    options: Vec<SelectOption<'a>>,
}

/// A selectable value rendered by a [`SelectPrompt`].
///
/// String slices and owned strings can be converted into options directly.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectOption<'a> {
    value: Cow<'a, str>,
}

impl<'a> SelectPrompt<'a> {
    /// Creates a select prompt from a label and an ordered option list.
    #[must_use]
    pub const fn new(label: Cow<'a, str>, options: SelectOptionList<'a>) -> Self {
        Self {
            label: Some(label),
            options,
            block: None,
        }
    }

    /// Wraps the prompt in a [`Block`].
    #[must_use]
    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> SelectOptionList<'a> {
    /// Creates an option list from explicit option values.
    #[must_use]
    pub const fn new(options: Vec<SelectOption<'a>>) -> Self {
        Self { options }
    }

    /// Returns the number of options in the list.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.options.len()
    }

    /// Returns whether the list contains no options.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    /// Returns an iterator over the options.
    pub fn iter(&self) -> impl Iterator<Item = &SelectOption<'a>> {
        self.options.iter()
    }
}

impl<'a> SelectOption<'a> {
    /// Creates a selectable option.
    #[must_use]
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the text rendered for this option.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl<'a> From<&'a str> for SelectOption<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SelectOption<'_> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<'a, T> From<Vec<T>> for SelectOptionList<'a>
where
    T: Into<SelectOption<'a>>,
{
    fn from(options: Vec<T>) -> Self {
        Self::new(options.into_iter().map(Into::into).collect())
    }
}

impl<'a, T, const N: usize> From<[T; N]> for SelectOptionList<'a>
where
    T: Into<SelectOption<'a>>,
{
    fn from(options: [T; N]) -> Self {
        Self::new(options.into_iter().map(Into::into).collect())
    }
}

impl<'a> IntoIterator for &'a SelectOptionList<'a> {
    type IntoIter = std::slice::Iter<'a, SelectOption<'a>>;
    type Item = &'a SelectOption<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.options.iter()
    }
}

impl Prompt for SelectPrompt<'_> {
    fn draw(self, frame: &mut Frame, area: Rect, state: &mut Self::State) {
        frame.render_stateful_widget(self, area, state);
    }
}

impl<'a> StatefulWidget for SelectPrompt<'a> {
    type State = SelectState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = self.render_block(area, buf);
        let visible_option_count = self.visible_option_count(area);
        self.sync_state(state, visible_option_count);

        let lines = self.lines(state, visible_option_count);
        Paragraph::new(lines)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}

impl SelectPrompt<'_> {
    fn render_block(&mut self, area: Rect, buf: &mut Buffer) -> Rect {
        if let Some(block) = self.block.take() {
            let inner_area = block.inner(area);
            block.render(area, buf);
            inner_area
        } else {
            area
        }
    }

    fn visible_option_count(&self, area: Rect) -> usize {
        let label_height = usize::from(self.label.is_some());
        (area.height as usize).saturating_sub(label_height)
    }

    fn sync_state(&self, state: &mut SelectState, visible_option_count: usize) {
        state.option_count = if visible_option_count == 0 {
            0
        } else {
            self.options.len()
        };
        state.focused_index = state.clamp_focused_index(state.focused_index);
    }

    fn lines(mut self, state: &SelectState, visible_option_count: usize) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        if let Some(label) = self.label.take() {
            lines.push(Line::from(vec![
                state.status().symbol(),
                " ".into(),
                label.into_owned().bold(),
            ]));
        }

        let option_start = visible_window_start(
            state.focused_index(),
            self.options.len(),
            visible_option_count,
        );
        lines.extend(
            self.options
                .iter()
                .enumerate()
                .skip(option_start)
                .take(visible_option_count)
                .map(|(i, option)| option_line(option, i == state.focused_index())),
        );
        lines
    }
}

fn option_line(option: &SelectOption<'_>, focused: bool) -> Line<'static> {
    if focused {
        Line::from(Span::styled(
            format!("> {}", option.value()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
    } else {
        Line::from(Span::raw(format!("  {}", option.value())))
    }
}

const fn visible_window_start(
    focused_index: usize,
    option_count: usize,
    visible_count: usize,
) -> usize {
    if visible_count == 0 || option_count <= visible_count {
        0
    } else if focused_index >= visible_count {
        focused_index + 1 - visible_count
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use ratatui::backend::TestBackend;
    use ratatui::widgets::Borders;
    use ratatui::Terminal;
    use rstest::{fixture, rstest};

    use super::*;

    #[test]
    fn new() {
        let options = vec![
            SelectOption::from("Option 1"),
            SelectOption::from("Option 2"),
            SelectOption::from("Option 3"),
        ];

        let prompt = SelectPrompt::new(Cow::Borrowed("label"), options.clone().into());
        assert_eq!(prompt.options, SelectOptionList::new(options));
        assert!(prompt.block.is_none());
    }

    #[test]
    fn default() {
        let prompt = SelectPrompt::default();
        assert_eq!(prompt.options, SelectOptionList::default());
        assert_eq!(prompt.block, None);
    }

    #[test]
    fn option_list_from_strings() {
        let options = SelectOptionList::from(["Option 1", "Option 2"]);

        assert_eq!(options.len(), 2);
        assert!(!options.is_empty());
        assert_eq!(
            (&options)
                .into_iter()
                .map(SelectOption::value)
                .collect::<Vec<_>>(),
            ["Option 1", "Option 2"],
        );
    }

    #[test]
    fn render_with_max_options() {
        let options = vec![
            SelectOption::from("Option 1"),
            SelectOption::from("Option 2"),
            SelectOption::from("Option 3"),
        ];

        let prompt = prompt_with_block(options.clone());
        let mut state = SelectState::default();
        state.set_focused_index(1);

        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        draw_prompt(&mut terminal, prompt, &mut state);

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
    #[fixture]
    fn terminal() -> Terminal<TestBackend> {
        Terminal::new(TestBackend::new(20, 10)).unwrap()
    }

    fn prompt_with_block(options: impl Into<SelectOptionList<'static>>) -> SelectPrompt<'static> {
        SelectPrompt::new(Cow::Borrowed("label"), options.into())
            .with_block(Block::default().borders(Borders::ALL).title("Select"))
    }

    fn draw_prompt(
        terminal: &mut Terminal<TestBackend>,
        prompt: SelectPrompt<'_>,
        state: &mut SelectState,
    ) {
        terminal
            .draw(|frame| {
                let area = frame.area();
                prompt.clone().draw(frame, area, state);
            })
            .unwrap();
    }

    #[rstest]
    fn render_selected(mut terminal: Terminal<TestBackend>) {
        let options = vec![
            SelectOption::from("Option 1"),
            SelectOption::from("Option 2"),
            SelectOption::from("Option 3"),
        ];

        let prompt = prompt_with_block(options.clone());
        let mut state = SelectState::default().with_status(Status::Done);
        state.set_focused_index(2);

        draw_prompt(&mut terminal, prompt, &mut state);

        let mut expected = Buffer::with_lines(vec![
            "┌Select────────────┐",
            "│✔ label           │",
            "│  Option 1        │",
            "│  Option 2        │",
            "│> Option 3        │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]);

        expected.set_style(Rect::new(1, 1, 1, 1), Color::Green);

        expected.set_style(Rect::new(3, 1, 5, 1), Modifier::BOLD);

        expected.set_style(Rect::new(1, 4, 10, 1), (Color::Yellow, Modifier::BOLD));

        terminal.backend().assert_buffer(&expected);
    }

    #[test]
    fn render_scrolls_focused_option_into_view() {
        let options = ["Option 1", "Option 2", "Option 3", "Option 4"].into();
        let prompt = SelectPrompt::new(Cow::Borrowed("label"), options);
        let mut state = SelectState::new();
        state.set_focused_index(3);

        let backend = TestBackend::new(20, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        draw_prompt(&mut terminal, prompt, &mut state);

        let mut expected = Buffer::with_lines(vec![
            "? label             ",
            "  Option 3          ",
            "> Option 4          ",
        ]);

        expected.set_style(Rect::new(0, 0, 1, 1), Color::Cyan);
        expected.set_style(Rect::new(2, 0, 5, 1), Modifier::BOLD);
        expected.set_style(Rect::new(0, 2, 10, 1), (Color::Yellow, Modifier::BOLD));

        assert_eq!(state.option_count, 4);
        terminal.backend().assert_buffer(&expected);
    }

    #[test]
    fn render_disables_option_navigation_when_no_options_are_visible() {
        let options = ["Option 1", "Option 2"].into();
        let prompt = SelectPrompt::new(Cow::Borrowed("label"), options);
        let mut state = SelectState::new();
        state.set_focused_index(1);

        let backend = TestBackend::new(20, 1);
        let mut terminal = Terminal::new(backend).unwrap();

        draw_prompt(&mut terminal, prompt, &mut state);

        let mut expected = Buffer::with_lines(vec!["? label             "]);

        expected.set_style(Rect::new(0, 0, 1, 1), Color::Cyan);
        expected.set_style(Rect::new(2, 0, 5, 1), Modifier::BOLD);

        assert_eq!(state.option_count, 0);
        terminal.backend().assert_buffer(&expected);
    }
}
