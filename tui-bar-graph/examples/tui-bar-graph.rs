use clap::{Parser, ValueEnum};
use colorgrad::Gradient;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use rand::RngExt;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};
use ratatui::{DefaultTerminal, Frame, symbols};
use strum::Display;
use tui_bar_graph::{BarGraph, BarStyle, ColorMode};

#[derive(Debug, Parser)]
struct Args {
    /// The style of bar to render (solid, quadrant, octant, or braille)
    #[arg(default_value_t = BarStyle::Braille)]
    bar_style: BarStyle,

    /// The color gradient preset to use
    ///
    /// This is a small selection of the color gradients available in the `colorgrad` crate.
    #[arg(default_value_t = Preset::Rainbow)]
    preset: Preset,
}

#[derive(Debug, Clone, ValueEnum, Display)]
#[strum(serialize_all = "lowercase")]
enum Preset {
    Rainbow,
    Sinebow,
    Viridis,
    Inferno,
    Magma,
    Plasma,
}

impl Preset {
    pub fn to_gradient(&self) -> Box<dyn Gradient> {
        match self {
            Self::Rainbow => Box::new(colorgrad::preset::rainbow()),
            Self::Sinebow => Box::new(colorgrad::preset::sinebow()),
            Self::Viridis => Box::new(colorgrad::preset::viridis()),
            Self::Inferno => Box::new(colorgrad::preset::inferno()),
            Self::Magma => Box::new(colorgrad::preset::magma()),
            Self::Plasma => Box::new(colorgrad::preset::plasma()),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    ratatui::run(|terminal| run(terminal, &args))
}

fn run(terminal: &mut DefaultTerminal, args: &Args) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, args))?;
        if matches!(
            event::read()?,
            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                ..
            })
        ) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, args: &Args) {
    const TITLE_FG: Color = SLATE.c800;
    const TITLE_BG: Color = SLATE.c100;
    const BLOCK_FG: Color = SLATE.c100;
    const BLOCK_BG: Color = SLATE.c900;

    let title = format!(
        "tui-bar-graph - BarStyle: {}, Preset: {}",
        args.bar_style, args.preset
    );
    let block = Block::new()
        .borders(Borders::TOP)
        .border_set(symbols::border::FULL)
        .border_style((TITLE_BG, TITLE_FG))
        .style((BLOCK_FG, BLOCK_BG))
        .title(
            Line::from(title)
                .centered()
                .bg(TITLE_BG)
                .fg(TITLE_FG)
                .bold(),
        );
    frame.render_widget(&block, frame.area());

    let area = block.inner(frame.area());
    let width = match args.bar_style {
        BarStyle::Solid => area.width as usize,
        BarStyle::Octant | BarStyle::Braille => area.width as usize * 2,
        BarStyle::Quadrant => area.width as usize * 4,
    };
    let mut rng = rand::rng();
    let data: Vec<f64> = std::iter::repeat_with(|| rng.random())
        .take(width)
        .collect();
    let gradient = args.preset.to_gradient();
    let bar_graph = BarGraph::new(data)
        .with_gradient(gradient)
        .with_color_mode(ColorMode::VerticalGradient)
        .with_bar_style(args.bar_style);
    frame.render_widget(bar_graph, area);
}
