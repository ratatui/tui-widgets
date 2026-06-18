use std::borrow::Cow;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, KeyCode, KeyEvent};
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

const FRUITS: [&str; 4] = ["Apple", "Banana", "Cherry", "Date"];
const LABEL: &str = "Select a fruit:";

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct App {
    debug: bool,
    quit: bool,
    state: SelectState,
}

impl App {
    pub const fn new(cli: Cli) -> Self {
        Self {
            debug: cli.debug,
            quit: false,
            state: SelectState::new().with_focus(FocusState::Focused),
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
        let (prompt_area, selected_area, debug_area) = self.split_layout(frame.area());
        self.draw_select_prompt(frame, prompt_area);
        self.draw_selected_value(frame, selected_area);
        self.draw_debug(frame, debug_area);
    }

    /// split the frame into 3 areas:
    /// - prompt area
    /// - selected value area
    /// - debug area
    ///
    /// The debug area is only visible if the `debug` flag is set.
    fn split_layout(&self, area: Rect) -> (Rect, Rect, Rect) {
        let (prompt_area, debug_area) = if self.debug {
            let areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, 2); 2])
                .split(area);
            (areas[0], areas[1])
        } else {
            (area, Rect::default())
        };

        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(7), Constraint::Length(1)])
            .split(prompt_area);
        (areas[0], areas[1], debug_area)
    }

    fn draw_select_prompt(&mut self, frame: &mut Frame, area: Rect) {
        SelectPrompt::new(Cow::Borrowed(LABEL), FRUITS.into())
            .with_block(Block::new().borders(Borders::ALL).title("Fruits"))
            .draw(frame, area, &mut self.state);
    }

    fn draw_selected_value(&self, frame: &mut Frame, area: Rect) {
        if !self.state.status().is_done() {
            return;
        }

        let selected = FRUITS[self.state.focused_index()];
        frame.render_widget(
            Paragraph::new(format!("  Selected: {selected}")).style(Style::new().dark_gray()),
            area,
        );
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.state.is_finished() && key_event.code == KeyCode::Esc {
            self.quit = true;
            return;
        }

        self.state.handle_key_event(key_event);
    }

    const fn is_finished(&self) -> bool {
        self.quit
    }
}
