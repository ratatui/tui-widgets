//! Demonstrates a multi-line text prompt.
//!
//! Run with `cargo run -p tui-prompts --example multi_line`.
//!
//! The prompt uses a bottom and right border to make the editable area visible while text wraps
//! across multiple rows. The app exits only after the prompt state reports completion.
//!
//! Use the prompt's normal text-editing keys to enter text. Run with `--debug` to show prompt
//! state while editing.

use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::*;
use tui_prompts::prelude::*;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let mut app = App::new(cli);
    ratatui::run(|terminal| app.run(terminal))
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct App<'a> {
    debug: bool,
    state: TextState<'a>,
}

impl App<'_> {
    pub const fn new(cli: Cli) -> Self {
        Self {
            debug: cli.debug,
            state: TextState::new().with_focus(FocusState::Focused),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.is_finished() {
            self.handle_events()?;
            terminal.draw(|frame| self.draw_ui(frame))?;
        }
        terminal.hide_cursor()?;
        // wait two seconds before exiting so the user can see the final state of the UI.
        sleep(Duration::from_secs(2));
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))?
            && let Some(key_event) = event::read()?.as_key_press_event()
        {
            self.handle_key_event(key_event);
        }
        Ok(())
    }

    fn draw_ui(&mut self, frame: &mut Frame) {
        let (text_area, debug_area) = self.split_layout(frame.area());
        self.draw_text_prompt(frame, text_area);
        self.draw_debug(frame, debug_area);
    }

    /// split the frame into 2 areas:
    /// - prompt area
    /// - debug area
    ///
    /// The debug area is only visible if the `debug` flag is set.
    fn split_layout(&self, area: Rect) -> (Rect, Rect) {
        if self.debug {
            let areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, 2); 2])
                .split(area);
            let textbox_area = Rect {
                height: 4, // 3 lines of text + 1 line of border
                ..areas[0]
            };
            (textbox_area, areas[1])
        } else {
            (area, Rect::default())
        }
    }

    fn draw_text_prompt(&mut self, frame: &mut Frame, area: Rect) {
        TextPrompt::from("Multi-line")
            .with_block(Block::new().borders(Borders::RIGHT | Borders::BOTTOM))
            .draw(frame, area, &mut self.state);
    }

    /// draw a debug string in the top right corner of the screen that shows the current state of
    /// the app.
    fn draw_debug(&self, frame: &mut Frame, area: Rect) {
        if !self.debug {
            return;
        }
        let debug = format!("{self:#?}");
        frame.render_widget(Paragraph::new(debug).wrap(Wrap { trim: false }), area);
    }

    const fn is_finished(&self) -> bool {
        self.state.is_finished()
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.state.handle_key_event(key_event);
    }
}
