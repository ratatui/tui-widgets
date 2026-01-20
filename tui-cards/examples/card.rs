use itertools::Itertools;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::Block;
use ratatui::Frame;
use strum::IntoEnumIterator;
use tui_cards::{Card, CardSize, Rank, Suit};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut card_size = CardSize::Normal;
    // fix problem with skipping the wrong number of characters when drawing cards
    // This is probably a bug in ratatui
    terminal.draw(|frame| frame.render_widget(Block::new().bg(Color::White), frame.area()))?;
    loop {
        if terminal.draw(|frame| draw(frame, card_size)).is_err() {
            break;
        }
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => break,
                KeyCode::Char('s') => card_size = CardSize::Small,
                KeyCode::Char('n') => card_size = CardSize::Normal,
                _ => {}
            }
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame, card_size: CardSize) {
    let (card_width, card_height) = card_size.dimensions();
    let step_x = (card_width + 1) as usize;
    let step_y = (card_height + 1) as usize;
    let width = frame.area().width / step_x as u16 * step_x as u16;
    let height = frame.area().height / step_y as u16 * step_y as u16;
    let cards = Suit::iter()
        .cartesian_product(Rank::iter())
        .map(|(suit, rank)| Card::new(rank, suit, card_size));
    let x_iter = (0..width).step_by(step_x);
    let y_iter = (0..height).step_by(step_y);
    for (card, (y, x)) in cards.zip(y_iter.cartesian_product(x_iter)) {
        let area = Rect::new(x, y, card_width, card_height);
        frame.render_widget(&card, area);
    }
}
