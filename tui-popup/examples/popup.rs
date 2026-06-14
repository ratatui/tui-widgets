//! Renders a basic centered popup over wrapped background text.
//!
//! Run with `cargo run -p tui-popup --example popup --features crossterm`.
//!
//! This is the minimal popup example: `Popup::new` receives plain text, centers it in the frame,
//! and lets the popup size itself from the content.
//!
//! Press any key to quit.

use color_eyre::Result;
use lipsum::lipsum;
use ratatui::crossterm::event;
use ratatui::prelude::{Rect, Style, Stylize};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};
use tui_popup::Popup;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            render(frame);
        })?;
        if event::read()?.as_key_press_event().is_some() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let area = frame.area();
    let background = background(area);
    let popup = Popup::new("Press any key to exit")
        .title("tui-popup demo")
        .style(Style::new().white().on_blue());
    frame.render_widget(background, area);
    frame.render_widget(popup, area);
}

fn background(area: Rect) -> Paragraph<'static> {
    let lorem_ipsum = lipsum(area.area() as usize / 5);
    Paragraph::new(lorem_ipsum)
        .wrap(Wrap { trim: false })
        .dark_gray()
}
