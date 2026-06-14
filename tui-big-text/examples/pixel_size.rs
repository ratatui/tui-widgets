use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::prelude::*;
use tui_big_text::{BigText, PixelSize};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if let Some(key) = event::read()?.as_key_press_event()
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let title = Line::from("tui-big-text pixel size demo. <q> quit")
        .centered()
        .cyan();

    let full_size_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .lines(vec!["FullSize".white().into()])
        .build();

    let half_height_text = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines(vec!["1/2 high".green().into()])
        .build();

    let half_wide_text = BigText::builder()
        .pixel_size(PixelSize::HalfWidth)
        .lines(vec!["1/2 wide".red().into()])
        .build();

    let quadrant_text = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec!["Quadrant".blue().into(), " 1/2*1/2".blue().into()])
        .build();

    let third_text = BigText::builder()
        .pixel_size(PixelSize::ThirdHeight)
        .lines(vec!["1/3".yellow().into(), "high".yellow().into()])
        .build();

    let sextant_text = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .lines(vec!["Sextant".cyan().into(), " 1/2*1/3".cyan().into()])
        .build();

    let quarter_text = BigText::builder()
        .pixel_size(PixelSize::QuarterHeight)
        .lines(vec!["1/4".dark_gray().into(), "high".dark_gray().into()])
        .build();

    let octant_text = BigText::builder()
        .pixel_size(PixelSize::Octant)
        .lines(vec!["Octant".magenta().into(), " 1/2*1/4".magenta().into()])
        .build();

    // Setup layout for the title and 8 blocks
    use Constraint::*;
    let [top, full, half_height, upper_middle, lower_middle, bottom] = Layout::vertical([
        Length(2),
        Length(8),
        Length(4),
        Length(8),
        Length(6),
        Length(4),
    ])
    .areas(frame.area());
    let [half_wide, quadrant] = Layout::horizontal([Length(32), Length(32)]).areas(upper_middle);
    let [third_height, sextant] = Layout::horizontal([Length(32), Length(32)]).areas(lower_middle);
    let [quarter_height, octant] = Layout::horizontal([Length(32), Length(32)]).areas(bottom);

    frame.render_widget(title, top);
    frame.render_widget(full_size_text, full);
    frame.render_widget(half_height_text, half_height);
    frame.render_widget(half_wide_text, half_wide);
    frame.render_widget(quadrant_text, quadrant);
    frame.render_widget(third_text, third_height);
    frame.render_widget(sextant_text, sextant);
    frame.render_widget(quarter_text, quarter_height);
    frame.render_widget(octant_text, octant);
}
