mod tui;

use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyEvent, KeyModifiers};
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
    let cli = Cli::parse();
    let mut app = App::new(cli);
    app.run()?;
    Ok(())
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct App<'a> {
    debug: bool,
    current_field: Field,
    username_state: TextState<'a>,
    password_state: TextState<'a>,
    invisible_state: TextState<'a>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    #[default]
    Username,
    Password,
    Invisible,
}

impl<'a> App<'a> {
    pub fn new(cli: Cli) -> Self {
        Self {
            debug: cli.debug,
            ..Default::default()
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        *self.current_state().focus_state_mut() = FocusState::Focused;
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
        let (username_area, password_area, invisible_area, value_area, debug_area) =
            self.split_layout(frame.area());
        self.draw_text_prompt(frame, username_area);
        self.draw_password_prompt(frame, password_area);
        self.draw_invisible_prompt(frame, invisible_area);
        self.draw_state_value(frame, value_area);
        self.draw_debug(frame, debug_area);
    }

    /// split the frame into 5 areas:
    /// - username prompt
    /// - password prompt
    /// - invisible prompt
    /// - state value
    /// - debug area
    ///
    /// The debug area is only visible if the `debug` flag is set.
    fn split_layout(&self, area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let (prompt_area, debug_area) = if self.debug {
            let areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, 2); 2])
                .split(area);
            (areas[0], areas[1])
        } else {
            (area, area)
        };
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); 4])
            .split(prompt_area);
        (areas[0], areas[1], areas[2], areas[3], debug_area)
    }

    fn draw_text_prompt(&mut self, frame: &mut Frame, username_area: Rect) {
        TextPrompt::from("Username").draw(frame, username_area, &mut self.username_state);
    }

    fn draw_password_prompt(&mut self, frame: &mut Frame, password_area: Rect) {
        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, password_area, &mut self.password_state);
    }

    fn draw_invisible_prompt(&mut self, frame: &mut Frame, invisible_area: Rect) {
        TextPrompt::from("Invisible")
            .with_render_style(TextRenderStyle::Invisible)
            .draw(frame, invisible_area, &mut self.invisible_state);
    }

    /// draw the value of the current state underneath the prompts.
    fn draw_state_value(&mut self, frame: &mut Frame, value_area: Rect) {
        let state = self.current_state();
        let state = format!("  Value: {}", state.value());
        frame.render_widget(
            Paragraph::new(state).style(Style::new().dark_gray()),
            value_area,
        );
    }

    /// draw a debug string in the top right corner of the screen that shows the current state of
    /// the app.
    fn draw_debug(&self, frame: &mut Frame, area: Rect) {
        if !self.debug {
            return;
        }
        let debug = format!("{self:#?}");
        frame.render_widget(
            Paragraph::new(debug)
                .wrap(Wrap { trim: false })
                .block(Block::new().borders(Borders::LEFT)),
            area,
        );
    }

    const fn is_finished(&self) -> bool {
        self.username_state.is_finished()
            && self.password_state.is_finished()
            && self.invisible_state.is_finished()
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (event::KeyCode::Enter, _) => self.submit(),
            (event::KeyCode::Tab, KeyModifiers::NONE) => self.focus_next(),
            (event::KeyCode::BackTab, KeyModifiers::SHIFT) => self.focus_prev(),
            _ => self.focus_handle_event(key_event),
        }
    }

    fn focus_handle_event(&mut self, key_event: KeyEvent) {
        let state = self.current_state();
        state.handle_key_event(key_event);
    }

    fn focus_next(&mut self) {
        self.current_state().blur();
        self.current_field = self.next_field();
        self.current_state().focus();
    }

    fn focus_prev(&mut self) {
        self.current_state().blur();
        self.current_field = self.prev_field();
        self.current_state().focus();
    }

    fn submit(&mut self) {
        self.current_state().complete();
        if self.current_state().is_finished() && !self.is_finished() {
            self.current_state().blur();
            self.current_field = self.next_field();
            self.current_state().focus();
        }
    }

    const fn next_field(&self) -> Field {
        match self.current_field {
            Field::Username => Field::Password,
            Field::Password => Field::Invisible,
            Field::Invisible => Field::Username,
        }
    }

    const fn prev_field(&self) -> Field {
        match self.current_field {
            Field::Username => Field::Invisible,
            Field::Password => Field::Username,
            Field::Invisible => Field::Password,
        }
    }

    const fn current_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Username => &mut self.username_state,
            Field::Password => &mut self.password_state,
            Field::Invisible => &mut self.invisible_state,
        }
    }
}
