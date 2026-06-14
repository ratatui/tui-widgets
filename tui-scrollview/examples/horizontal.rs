//! Demonstrates horizontal and vertical scrolling in the same `ScrollView`.
//!
//! Run with `cargo run -p tui-scrollview --example horizontal`.
//!
//! The content is twice as wide as the terminal and usually taller than the viewport, so this
//! example is useful for checking that both scrollbars update correctly.
//! `ScrollViewState` stores both offsets; the `ScrollView` clips the larger virtual buffer to the
//! current frame area on each draw.
//!
//! Controls:
//! - `h` / `Left`: scroll left
//! - `l` / `Right`: scroll right
//! - `j` / `Down`: scroll down
//! - `k` / `Up`: scroll up
//! - `q` / `Esc`: quit

use std::io;

use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::Size;
use ratatui::widgets::{Paragraph, Wrap};
use tui_scrollview::{ScrollView, ScrollViewState};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

#[derive(Debug, Default, Clone)]
struct App {
    text: String,
    scroll_view_state: ScrollViewState,
    state: AppState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    fn new() -> Self {
        Self {
            text: lipsum::lipsum(10_000),
            ..Default::default()
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.is_running() {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    const fn quit(&mut self) {
        self.state = AppState::Quit;
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();
            // Make the virtual buffer wider than the viewport so horizontal scrolling is visible.
            let size = Size::new(area.width * 2, area.width);
            let mut scroll_view = ScrollView::new(size);
            let paragraph = Paragraph::new(self.text.clone()).wrap(Wrap::default());
            scroll_view.render_widget(paragraph, scroll_view.area());
            frame.render_stateful_widget(scroll_view, area, &mut self.scroll_view_state);
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                Char('q') | Esc => self.quit(),
                Char('j') | Down => self.scroll_view_state.scroll_down(),
                Char('k') | Up => self.scroll_view_state.scroll_up(),
                Char('h') | Left => self.scroll_view_state.scroll_left(),
                Char('l') | Right => self.scroll_view_state.scroll_right(),
                _ => (),
            }
        }
        Ok(())
    }
}
