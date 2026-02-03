use itertools::Itertools;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::Block;
use ratatui::Frame;
use strum::IntoEnumIterator;
use tui_cards::{Card, CardSize, Rank, Suit};

#[derive(Debug, Clone, Copy, Default)]
enum CardStyle {
    #[default]
    Transparent,
    Classic,
    Dark,
    Colorful,
}

impl CardStyle {
    fn next(self) -> Self {
        match self {
            Self::Transparent => Self::Classic,
            Self::Classic => Self::Dark,
            Self::Dark => Self::Colorful,
            Self::Colorful => Self::Transparent,
        }
    }

    fn style(self) -> Style {
        match self {
            Self::Transparent => Style::new(),
            Self::Classic => Style::new().bg(Color::White),
            Self::Dark => Style::new().bg(Color::DarkGray),
            Self::Colorful => Style::new().bg(Color::Rgb(255, 250, 205)),
        }
    }

    fn background(self) -> Color {
        match self {
            Self::Transparent => Color::DarkGray,
            Self::Classic => Color::White,
            Self::Dark => Color::Black,
            Self::Colorful => Color::Rgb(70, 130, 180),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut card_size = CardSize::Normal;
    let mut card_style = CardStyle::default();
    loop {
        if terminal
            .draw(|frame| draw(frame, card_size, card_style))
            .is_err()
        {
            break;
        }
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => break,
                KeyCode::Char('s') => card_size = CardSize::Small,
                KeyCode::Char('n') => card_size = CardSize::Normal,
                KeyCode::Char('t') => card_style = card_style.next(),
                _ => {}
            }
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame, card_size: CardSize, card_style: CardStyle) {
    frame.render_widget(Block::new().bg(card_style.background()), frame.area());

    let (card_width, card_height) = card_size.dimensions();
    let step_x = (card_width + 1) as usize;
    let step_y = (card_height + 1) as usize;
    let width = frame.area().width / step_x as u16 * step_x as u16;
    let height = frame.area().height / step_y as u16 * step_y as u16;
    let cards = Suit::iter()
        .cartesian_product(Rank::iter())
        .map(|(suit, rank)| Card::new(rank, suit, card_size).style(card_style.style()));
    let x_iter = (0..width).step_by(step_x);
    let y_iter = (0..height).step_by(step_y);
    for (card, (y, x)) in cards.zip(y_iter.cartesian_product(x_iter)) {
        let area = Rect::new(x, y, card_width, card_height);
        frame.render_widget(&card, area);
    }
}
