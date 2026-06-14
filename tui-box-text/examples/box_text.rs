//! Renders printable ASCII characters as box-drawing text.
//!
//! Run with `cargo run -p tui-box-text --example box_text`.
//!
//! Each `BoxChar` is rendered into a 4x3 cell so the example can show the full glyph shape for
//! every printable character. Smaller areas clip the generated box glyphs.
//!
//! Press `q` or `Esc` to quit.

use std::iter::zip;

use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Line;
use ratatui::{DefaultTerminal, Frame};
use tui_box_text::BoxChar;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(draw)?;
        if let Some(key) = event::read()?.as_key_press_event()
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    let [header, body] = layout.areas(frame.area());
    frame.render_widget(
        Line::from("Tui-box-text. Press Esc to exit").centered(),
        header,
    );
    // BoxChar glyphs are at most four columns wide and three rows tall.
    let mut areas = Vec::new();
    for y in (body.top() + 3..body.bottom()).step_by(3) {
        for x in (body.left() + 4..body.right()).step_by(4) {
            areas.push(Rect::new(x - 4, y - 3, 4, 3));
        }
    }
    for (c, area) in zip(' '..='~', areas) {
        let box_char = BoxChar::new(c);
        frame.render_widget(&box_char, area);
    }
}
