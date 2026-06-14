use color_eyre::Result;
use lipsum::lipsum;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::{Constraint, Frame, Layout, Rect, Style, Stylize, Text};
use ratatui::widgets::{Paragraph, Wrap};
use tui_popup::{Popup, PopupState};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut state = PopupState::default();
    let mut exit = false;
    while !exit {
        terminal.draw(|frame| draw(frame, &mut state))?;
        handle_events(&mut state, &mut exit)?;
    }
    Ok(())
}

fn draw(frame: &mut Frame, state: &mut PopupState) {
    let vertical = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]);
    let [background_area, status_area] = vertical.areas(frame.area());

    render_background(frame, background_area);
    render_popup(frame, background_area, state);
    render_status_bar(frame, status_area, state);
}

fn render_background(frame: &mut Frame, area: Rect) {
    let lorem_ipsum = lipsum(area.area() as usize / 5);
    let background = Paragraph::new(lorem_ipsum)
        .wrap(Wrap { trim: false })
        .dark_gray();
    frame.render_widget(background, area);
}

fn render_popup(frame: &mut Frame, area: Rect, state: &mut PopupState) {
    let body = Text::from_iter([
        "q: exit",
        "r: reset",
        "j: move down",
        "k: move up",
        "h: move left",
        "l: move right",
    ]);
    let popup = Popup::new(body)
        .title("Popup")
        .style(Style::new().white().on_blue());
    frame.render_stateful_widget(popup, area, state);
}

/// Status bar at the bottom of the screen
///
/// Must be called after rendering the popup widget as it relies on the popup area being set
fn render_status_bar(frame: &mut Frame, area: Rect, state: &PopupState) {
    let popup_area = state.area().unwrap_or_default();
    let text = format!("Popup area: {popup_area:?}");
    let paragraph = Paragraph::new(text).style(Style::new().white().on_black());
    frame.render_widget(paragraph, area);
}

fn handle_events(popup: &mut PopupState, exit: &mut bool) -> Result<()> {
    match event::read()? {
        Event::Key(event) if event.kind == KeyEventKind::Press => {
            handle_key_event(event, popup, exit);
        }
        Event::Mouse(event) => popup.handle_mouse_event(event),
        _ => (),
    }
    Ok(())
}

fn handle_key_event(event: KeyEvent, popup: &mut PopupState, exit: &mut bool) {
    match event.code {
        KeyCode::Char('q') | KeyCode::Esc => *exit = true,
        KeyCode::Char('r') => *popup = PopupState::default(),
        KeyCode::Char('j') | KeyCode::Down => popup.move_down(1),
        KeyCode::Char('k') | KeyCode::Up => popup.move_up(1),
        KeyCode::Char('h') | KeyCode::Left => popup.move_left(1),
        KeyCode::Char('l') | KeyCode::Right => popup.move_right(1),
        _ => {}
    }
}
