//! Demonstrates rendering non-ASCII fonts with Hiragana and Greek text.
//!
//! Run with `cargo run -p tui-big-text --example non_ascii`.
//!
//! Press `q` or `Esc` to quit.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::Offset;
use ratatui::prelude::{Frame, Stylize};
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
    let title = Line::from("tui-big-text non-ASCII demo. <q> quit").cyan();

    let big_text = BigText::builder()
        .centered()
        .pixel_size(tui_big_text::PixelSize::HalfHeight)
        .lines(vec![
            "ひらがな".white().on_red().into(),
            "ελληνικά".white().on_blue().into(),
        ])
        .build();

    let area = frame.area();
    frame.render_widget(title, area);

    let area = area.offset(Offset { x: 0, y: 2 }).intersection(area);
    frame.render_widget(big_text, area);
}
