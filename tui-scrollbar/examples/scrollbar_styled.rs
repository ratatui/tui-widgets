//! Styled scrollbar showcase.
//!
//! This example renders a static layout with independently styled track, thumb, and arrow glyphs.

use std::time::Duration;

use color_eyre::Result;
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Color, Modifier};
use ratatui::widgets::{Block, Borders, Paragraph};
use tui_scrollbar::{GlyphSet, ScrollBar, ScrollBarArrows, ScrollLengths};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| {
        terminal.draw(|frame| render(frame.area(), frame))?;
        std::thread::sleep(Duration::from_secs(3));
        Ok(())
    })
}

fn render(area: Rect, frame: &mut ratatui::Frame) {
    if area.width < 2 || area.height < 2 {
        return;
    }

    let horizontal_bar = area
        .rows()
        .next_back()
        .unwrap_or(area)
        .inner(Margin::new(1, 0));
    let vertical_bar = area
        .columns()
        .next_back()
        .unwrap_or(area)
        .inner(Margin::new(0, 1));

    let block = Block::new()
        .borders(Borders::ALL)
        .title("styled scrollbars")
        .border_style((Color::LightBlue, Color::Black))
        .style((Color::Gray, Color::Black));
    let content = block.inner(area).inner(Margin::new(2, 1));
    frame.render_widget(block, area);

    frame.render_widget(
        Paragraph::new("track_style, thumb_style, and arrow_style can each use distinct colors")
            .style((Color::Gray, Color::Black)),
        content,
    );

    let lengths = ScrollLengths {
        content_len: 160,
        viewport_len: 40,
    };
    let horizontal = ScrollBar::horizontal(lengths)
        .offset(48)
        .arrows(ScrollBarArrows::Both)
        .glyph_set(GlyphSet::box_drawing())
        .track_style((Color::Blue, Color::Black).into())
        .thumb_style((Color::Yellow, Modifier::BOLD).into())
        .arrow_style((Color::LightGreen, Color::Black, Modifier::BOLD).into());
    let vertical = ScrollBar::vertical(lengths)
        .offset(80)
        .arrows(ScrollBarArrows::Both)
        .glyph_set(GlyphSet::box_drawing())
        .track_style((Color::Magenta, Color::Black).into())
        .thumb_style((Color::Cyan, Modifier::BOLD).into())
        .arrow_style((Color::LightRed, Color::Black, Modifier::BOLD).into());

    frame.render_widget(&horizontal, horizontal_bar);
    frame.render_widget(&vertical, vertical_bar);
}
