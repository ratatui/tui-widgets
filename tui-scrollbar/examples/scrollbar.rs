//! Fractional scrollbar step showcase.
//!
//! This example renders every 1/8th thumb step for both horizontal and vertical scrollbars so
//! you can visually compare partial glyphs and track alignment.
//!
//! Press `q` or `Esc` to exit.
//!
//! ## Structure
//!
//! - Calculation helpers (`build_metrics`, `step_entry`) derive `ScrollMetrics` and offsets so the
//!   demo can cover all 1/8th positions. These functions are not required for normal usage.
//! - Rendering helpers (`render_horizontal_steps`, `render_vertical_steps`) instantiate
//!   [`ScrollBar`] widgets and render them into each cell.
//!
//! ## Why this example exists
//!
//! Fractional glyphs can be hard to reason about without a full sweep. This example deliberately
//! draws every 1/8th step from both ends so you can verify the glyph ordering and thumb symmetry.

use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Paragraph;
use tui_scrollbar::{SUBCELL, ScrollBar, ScrollBarArrows, ScrollLengths, ScrollMetrics};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

#[derive(Debug, Default)]
struct App {
    /// Current run state for the render loop.
    state: AppState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    /// Continue drawing frames and handling input.
    Running,
    /// Exit the demo on the next tick.
    Quit,
}

impl App {
    /// Creates the demo app in its initial running state.
    const fn new() -> Self {
        Self {
            state: AppState::Running,
        }
    }

    /// Runs the draw loop until a quit key is pressed.
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| {
                render_scrollbars(frame.area(), frame);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Handles a single input event and updates the run state.
    fn handle_events(&mut self) -> Result<()> {
        if let Some(key) = event::read()?.as_key_press_event()
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            self.state = AppState::Quit;
        }
        Ok(())
    }
}

/// Splits the area into horizontal and vertical showcases and renders all 1/8th steps.
fn render_scrollbars(area: Rect, frame: &mut ratatui::Frame) {
    if area.height < 8 {
        return;
    }

    let title = "Fractional scrollbar steps (q/Esc to quit)";
    frame.render_widget(Paragraph::new(title), area);

    let content_area = Rect {
        y: area.y.saturating_add(1),
        height: area.height.saturating_sub(1),
        ..area
    };
    if content_area.height == 0 {
        return;
    }

    // Reserve enough width for 34 vertical bars (2 columns each) on the right.
    let min_left_width = 12;
    let max_right_width = 68;
    let right_width = max_right_width.min(content_area.width.saturating_sub(min_left_width));
    let [left_column, right_column] = content_area.layout(&Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(right_width),
    ]));

    // Stack up to 34 horizontal bars top-to-bottom in the left column.
    let max_rows = left_column.height as usize;
    let row_count = 34.min(max_rows);
    let left_cells =
        left_column.layout_vec(&Layout::vertical(vec![Constraint::Length(1); row_count]));

    // Arrange up to 34 vertical bars left-to-right in the right column.
    let bar_width = if right_column.width >= 68 { 2 } else { 1 };
    let max_cols = (right_column.width / bar_width) as usize;
    let col_count = 34.min(max_cols);
    let right_cells =
        right_column.layout_vec(&Layout::horizontal(vec![
            Constraint::Length(bar_width);
            col_count
        ]));

    render_horizontal_steps(frame, left_cells);
    render_vertical_steps(frame, right_cells);
}

/// Draws horizontal scrollbars that sweep every 1/8th thumb position, top to bottom.
fn render_horizontal_steps(frame: &mut ratatui::Frame, cells: Vec<Rect>) {
    for (index, area) in cells.iter().enumerate() {
        let [label_area, bar_area] = area.layout(&Layout::horizontal([
            Constraint::Length(2),
            Constraint::Fill(1),
        ]));
        if bar_area.width == 0 {
            continue;
        }
        let metrics = build_metrics(bar_area.width as usize, 6);
        let (label, thumb_start) = step_entry(&metrics, index);
        let label = (label % 8).to_string();
        let offset = metrics.offset_for_thumb_start(thumb_start);
        let lengths = ScrollLengths {
            content_len: metrics.content_len(),
            viewport_len: metrics.viewport_len(),
        };
        let scrollbar = ScrollBar::horizontal(lengths)
            .arrows(ScrollBarArrows::Both)
            .offset(offset);
        render_label(frame, label_area, &label);
        frame.render_widget(&scrollbar, bar_area);
    }
}

/// Draws vertical scrollbars that sweep every 1/8th thumb position, left to right.
fn render_vertical_steps(frame: &mut ratatui::Frame, cells: Vec<Rect>) {
    for (index, area) in cells.iter().enumerate() {
        let [label_area, bar_area] = area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
        ]));
        if bar_area.height == 0 {
            continue;
        }
        let metrics = build_metrics(bar_area.height as usize, 3);
        let (label, thumb_start) = step_entry(&metrics, index);
        let label = (label % 8).to_string();
        let offset = metrics.offset_for_thumb_start(thumb_start);
        let lengths = ScrollLengths {
            content_len: metrics.content_len(),
            viewport_len: metrics.viewport_len(),
        };
        let scrollbar = ScrollBar::vertical(lengths)
            .arrows(ScrollBarArrows::Both)
            .offset(offset);
        render_label(frame, label_area, &label);
        frame.render_widget(&scrollbar, bar_area);
    }
}

/// Renders the small modulo-8 label that marks the fractional step.
fn render_label(frame: &mut ratatui::Frame, area: Rect, label: &str) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    frame.render_widget(Paragraph::new(label), area);
}

/// Builds metrics where the thumb occupies a fixed number of cells on the track.
fn build_metrics(track_cells: usize, desired_thumb_cells: usize) -> ScrollMetrics {
    let track_len = track_cells.saturating_mul(SUBCELL);
    let viewport_len = track_len.max(1);
    let desired_thumb_len = desired_thumb_cells.saturating_mul(SUBCELL).max(1);
    let content_len =
        ((track_len as u128) * (viewport_len as u128) / (desired_thumb_len as u128)) as usize;
    let content_len = content_len.max(viewport_len.saturating_add(1));
    ScrollMetrics::new(
        ScrollLengths {
            content_len,
            viewport_len,
        },
        0,
        track_cells as u16,
    )
}

/// Returns the label index and thumb start for the 0..=16 or trailing sweep.
fn step_entry(metrics: &ScrollMetrics, index: usize) -> (usize, usize) {
    let max_start = metrics.thumb_travel();
    let local = index % 17;
    if index < 17 {
        (local, local.min(max_start))
    } else {
        let base = max_start.saturating_sub(16);
        (local, base.saturating_add(local).min(max_start))
    }
}
