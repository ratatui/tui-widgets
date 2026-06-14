//! Shows a popup containing a fixed-size scrollable paragraph.
//!
//! Run with `cargo run -p tui-popup --example paragraph --features crossterm`.
//!
//! `Paragraph` does not report its desired popup size, so the example wraps it in
//! `KnownSizeWrapper`. The wrapper gives `Popup` a stable width and height while the paragraph
//! keeps its own scroll offset.
//!
//! Controls:
//! - `j` / `Down`: scroll down
//! - `k` / `Up`: scroll up
//! - `q` / `Esc`: quit

use color_eyre::Result;
use lipsum::lipsum;
use ratatui::Frame;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::prelude::{Rect, Span, Style, Stylize, Text};
use ratatui::widgets::{Paragraph, Wrap};
use tui_popup::{KnownSizeWrapper, Popup};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Default)]
struct App {
    should_exit: bool,
    lorem_ipsum: String,
    scroll: u16,
}

impl App {
    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        self.lorem_ipsum = lipsum(2000);
        while !self.should_exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        self.render_background(frame, area);
        self.render_popup(frame);
    }

    fn render_background(&self, frame: &mut Frame, area: Rect) {
        let text = Text::raw(&self.lorem_ipsum);
        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false }).dark_gray();
        frame.render_widget(paragraph, area);
    }

    fn render_popup(&self, frame: &mut Frame) {
        let lines: Text = (0..10).map(|i| Span::raw(format!("Line {i}"))).collect();
        let paragraph = Paragraph::new(lines).scroll((self.scroll, 0));
        // Popup needs KnownSize for placement; Paragraph owns the scroll offset but not a desired
        // size.
        let wrapper = KnownSizeWrapper {
            inner: paragraph,
            width: 21,
            height: 5,
        };
        let popup = Popup::new(wrapper)
            .title("scroll: ↑/↓ quit: Esc")
            .style(Style::new().white().on_blue());
        frame.render_widget(popup, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
                KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
                KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
                _ => {}
            }
        }
        Ok(())
    }

    const fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    const fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }
}
