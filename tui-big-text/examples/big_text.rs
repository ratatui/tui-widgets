//! Renders a basic `BigText` widget with per-line styling.
//!
//! Run with `cargo run -p tui-big-text --example big_text`.
//!
//! The builder-level style is blue, while individual `Line` styles override it. This shows the
//! same styling precedence callers get when mixing widget defaults with styled text spans.
//!
//! Press `q` or `Esc` to quit.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::Offset;
use ratatui::prelude::{Frame, Style, Stylize};
use ratatui::text::Line;
use tui_big_text::BigText;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if let Some(key) = event::read()?.as_key_press_event()
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let title = Line::from("tui-big-text demo. <q> quit").cyan();

    let big_text = BigText::builder()
        .style(Style::new().blue())
        .lines(vec![
            "Tui-".red().into(),
            "big-".white().into(),
            "text".into(),
        ])
        .build();

    let area = frame.area();
    frame.render_widget(title, area);

    let area = area.offset(Offset { x: 0, y: 2 }).intersection(area);
    frame.render_widget(big_text, area);
}
