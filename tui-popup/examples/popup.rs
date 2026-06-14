use color_eyre::Result;
use lipsum::lipsum;
use ratatui::Frame;
use ratatui::crossterm::event::{self, Event};
use ratatui::prelude::{Rect, Style, Stylize};
use ratatui::widgets::{Paragraph, Wrap};
use tui_popup::Popup;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            render(frame);
        })?;
        if matches!(event::read()?, Event::Key(_)) {
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
