//! Animates two overlaid equalizer widgets with interpolated random bands.
//!
//! Run with `cargo run -p tui-equalizer --example demo`.
//!
//! One low-brightness equalizer keeps the previous band values visible while a second equalizer
//! renders the interpolated values at full brightness. The overlay makes the transition between
//! random targets visible without adding state to the widget itself.
//!
//! Press `q` to quit.

use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use rand::{RngExt, rng};
use ratatui::{DefaultTerminal, Frame};
use tui_equalizer::{Band, Equalizer};

const FRAMES_PER_SECOND: f64 = 60.0;

// How often to randomly update the equalizer bands
const UPDATE_INTERVAL: Duration = Duration::from_millis(500);

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let width = terminal.size()?.width;
    let mut current_bands = random_bands(width);
    let mut next_bands = random_bands(width);
    let mut last_time = Instant::now();
    loop {
        let percent = last_time.elapsed().as_secs_f64() / UPDATE_INTERVAL.as_secs_f64();
        let interpolated = interpolate(&current_bands, &next_bands, percent);
        if percent >= 1.0 {
            // Update the bands every 500ms
            last_time = Instant::now();
            current_bands = interpolated.clone();
            next_bands = random_bands(width);
        }
        terminal.draw(|frame| render(frame, &current_bands, &interpolated))?;
        if handle_input()? == Command::Quit {
            break Ok(());
        }
    }
}

fn interpolate(current: &[Band], next: &[Band], percent: f64) -> Vec<Band> {
    current
        .iter()
        .zip(next.iter())
        .map(|(current, next)| Band {
            value: current.value + (next.value - current.value) * percent.clamp(0.0, 1.0),
        })
        .collect()
}

fn random_bands(count: u16) -> Vec<Band> {
    (0..count / 2)
        .map(|_| Band::from(rng().random_range(0.1..1.0)))
        .collect()
}

fn render(frame: &mut Frame, current: &[Band], bands: &[Band]) {
    let size = frame.area();
    // Draw the previous target first as a dim trail, then the interpolated values at full
    // brightness.
    frame.render_widget(
        Equalizer {
            bands: current.to_vec(),
            brightness: 0.15,
        },
        size,
    );
    frame.render_widget(
        Equalizer {
            bands: bands.to_vec(),
            brightness: 1.0,
        },
        size,
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Noop,
    Quit,
}

fn handle_input() -> Result<Command> {
    if !event::poll(Duration::from_secs_f64(1.0 / FRAMES_PER_SECOND))? {
        return Ok(Command::Noop);
    }
    if let Some(key) = event::read()?.as_key_press_event() {
        return match key.code {
            KeyCode::Char('q') => Ok(Command::Quit),
            _ => Ok(Command::Noop),
        };
    }
    Ok(Command::Noop)
}
