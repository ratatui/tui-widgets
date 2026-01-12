mod tui;

use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyEvent};
use ratatui::crossterm::{self};
use ratatui::prelude::*;
use ratatui::widgets::*;
use tui::Tui;
use tui_prompts::prelude::*;
use tui_prompts::SelectState;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut app = App::new(cli);
    app.run()?;
    Ok(())
}

#[derive(Debug, Default, Clone)]
struct App {
    debug: bool,
    select_number_state: SelectState,
    select_fruit_state: SelectState,
}

impl App {
    fn new(cli: Cli) -> Self {
        Self {
            debug: cli.debug,
            select_number_state: SelectState::default(),
            select_fruit_state: SelectState::default(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;

        let numbers = vec!["One", "Two", "Three", "Four", "Five"];
        let fruits = vec!["Apple", "Banana", "Orange", "Grapes", "Mango"];
        while !self.is_finished() {
            self.select_number_state
                .render(&mut tui, "Select a number:", &numbers)?;
            self.select_fruit_state
                .render(&mut tui, "Select a fruit:", &fruits)?;
        }
        todo!()
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key_event) = event::read()? {
                self.handle_key_event(key_event)?;
            }
        }
        Ok(())
    }

    fn draw_ui(&mut self, frame: &mut Frame) {
        let (text_area, debug_area) = if self.debug {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(frame.size());
            (chunks[0], Some(chunks[1]))
        } else {
            (frame.size(), None)
        };
    }

    fn split_layout(&self, area: Rect) -> (Rect, Rect) {
        if self.debug {
            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Ratio(1, 2); 2])
                .split(area);
            let text_area = Rect {
                height: 4,
                ..areas[0]
            };
            (text_area, areas[1])
        } else {
            (area, Rect::default())
        }
    }

    fn draw_select_prompt(&mut self, frame: &mut Frame, area: Rect) {}

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        todo!()
    }

    const fn is_finished(&self) -> bool {
        self.select_number_state.is_finished() && self.select_fruit_state.is_finished()
    }
}
