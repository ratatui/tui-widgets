use itertools::Itertools;
use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::Block;
use strum::IntoEnumIterator;
use tui_cards::{Card, Rank, Suit};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    // fix problem with skipping the wrong number of characters when drawing cards
    // This is probably a bug in ratatui
    terminal.draw(|frame| frame.render_widget(Block::new().bg(Color::White), frame.area()))?;
    loop {
        if terminal.draw(draw).is_err() {
            break;
        }
        if matches!(
            event::read()?,
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }),
        ) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
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
