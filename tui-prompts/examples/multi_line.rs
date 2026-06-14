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

    pub fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;

        while !self.is_finished() {
            self.handle_events()?;
            tui.draw(|frame| self.draw_ui(frame))?;
        }
        tui.hide_cursor()?;
        // wait two seconds before exiting so the user can see the final state of the UI.
        sleep(Duration::from_secs(2));
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key_event) = event::read()?
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
