//! Renders a grid of playing-card widgets for every suit and rank.
//!
//! Run with `cargo run -p tui-cards --example card`.
//!
//! Cards are drawn into 15x10 cells, so the example truncates the terminal area to whole-card
//! multiples before placing the grid. Extra rows or columns are left unused instead of drawing
//! partial cards.
//!
//! Press `q` or `Esc` to quit.

use itertools::Itertools;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Frame};
use strum::IntoEnumIterator;
use tui_cards::{Card, Rank, Suit};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(draw)?;
        if let Some(key) = event::read()?.as_key_press_event()
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    // Cards are designed for a 15x10 cell; leave partial cells unused.
    let width = frame.area().width / 15 * 15;
    let height = frame.area().height / 10 * 10;
    let cards = Suit::iter()
        .cartesian_product(Rank::iter())
        .map(|(suit, rank)| Card::new(rank, suit));
    let x_iter = (0..width).step_by(15);
    let y_iter = (0..height).step_by(10);
    for (card, (y, x)) in cards.zip(y_iter.cartesian_product(x_iter)) {
        let area = Rect::new(x, y, 15, 10);
        frame.render_widget(&card, area);
    }
}
