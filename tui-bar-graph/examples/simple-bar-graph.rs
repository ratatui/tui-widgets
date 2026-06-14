//! Renders a full-screen bar graph with random values and a vertical color gradient.
//!
//! Run with `cargo run -p tui-bar-graph --example simple-bar-graph`.
//!
//! This focused example uses `BarStyle::Braille`, which can draw two data points per terminal
//! column. The data length is therefore twice the frame width so each braille half-cell has a
//! value to render.
//!
//! Press any key to quit. Rerun the example to generate a different random graph.

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use rand::RngExt;
use ratatui::{DefaultTerminal, Frame};
use tui_bar_graph::{BarGraph, BarStyle, ColorMode};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(render)?;
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

fn render(frame: &mut Frame) {
    // Braille bars render two horizontal samples in each terminal cell.
    let data_count = frame.area().width as usize * 2;
    let mut rng = rand::rng();
    let data: Vec<f64> = std::iter::repeat_with(|| rng.random())
        .take(data_count)
        .collect();
    let gradient = colorgrad::preset::rainbow();
    let bar_graph = BarGraph::new(data)
        .with_gradient(gradient)
        .with_color_mode(ColorMode::VerticalGradient)
        .with_bar_style(BarStyle::Braille);
    frame.render_widget(bar_graph, frame.area());
}
