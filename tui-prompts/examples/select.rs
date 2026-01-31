mod tui;

use std::panic;
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
    panic::set_hook(Box::new(|info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)
            .expect("Failed to leave alternate screen");
        eprintln!("Panic: {info:?}");
    }));

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
        let mut select_number_state = SelectState::default();
        select_number_state.focus();

        Self {
            debug: cli.debug,
            select_number_state,
            select_fruit_state: SelectState::default(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let numbers = vec!["One", "Two", "Three", "Four", "Five"];
        let fruits = vec!["Apple", "Banana", "Cherry", "Date"];
        let number_label = "Select a number:";
        let fruit_label = "Select a fruit:";

        let mut tui = Tui::new()?;

        while !self.is_finished() {
            self.handle_events()?;
            tui.draw(|frame| self.draw_ui(frame, fruit_label, &fruits, number_label, &numbers))?;
        }
        tui.hide_cursor()?;
        sleep(Duration::from_secs(2));
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key_event) = event::read()? {
                self.handle_key_event(key_event);
            }
        }
        Ok(())
    }

    fn draw_ui(
        &mut self,
        frame: &mut Frame,
        fruit_label: &str,
        fruit_options: &[&str],
        number_label: &str,
        number_options: &[&str],
    ) {
        let (text_area, debug_area) = self.split_layout(frame.size());
        self.draw_select_prompt(
            frame,
            text_area,
            fruit_label,
            fruit_options,
            number_label,
            number_options,
        );
        self.draw_debug(frame, debug_area);
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

    fn draw_select_prompt(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        fruit_label: &str,
        fruit_options: &[&str],
        number_label: &str,
        number_options: &[&str],
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Min(5)])
            .split(area);

        SelectPrompt::from_strings(number_label, number_options.to_vec())
            .with_block(Block::new().borders(Borders::ALL).title("Numbers"))
            .draw(frame, chunks[0], &mut self.select_number_state);

        SelectPrompt::from_strings(fruit_label, fruit_options.to_vec())
            .with_block(Block::new().borders(Borders::ALL).title("Fruits"))
            .draw(frame, chunks[1], &mut self.select_fruit_state);
    }

    fn draw_debug(&self, frame: &mut Frame, area: Rect) {
        if !self.debug {
            return;
        }
        let debug = format!("{self:#?}");
        frame.render_widget(Paragraph::new(debug).wrap(Wrap { trim: false }), area);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if !self.select_number_state.is_finished() {
            self.select_number_state.handle_key_event(key_event);
            // If number selection just finished, focus fruit
            if self.select_number_state.is_finished() {
                self.select_number_state.blur();
                self.select_fruit_state.focus();
            }
        } else if !self.select_fruit_state.is_finished() {
            self.select_fruit_state.handle_key_event(key_event);
        }
    }

    const fn is_finished(&self) -> bool {
        self.select_number_state.is_finished() && self.select_fruit_state.is_finished()
    }
}
