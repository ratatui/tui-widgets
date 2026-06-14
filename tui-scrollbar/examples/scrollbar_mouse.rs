//! Mouse + keyboard-driven scrollbar demo with smooth subcell movement.
//!
//! If you are new to this crate, this is the fastest way to see the full interaction model:
//! a horizontal scrollbar on the bottom edge and a vertical scrollbar on the right edge, both
//! wired to the same input flow your app would use. The example keeps the scroll offsets in app
//! state, renders the scrollbars each frame, and applies the commands returned by
//! [`ScrollBar::handle_mouse_event`].
//!
//! ## Why this example exists
//!
//! The demo uses subcell units so the thumb glides smoothly even when you step with the keyboard.
//! It also shows how to keep scroll offsets clamped when the terminal resizes.
//!
//! ## Implementation choices
//!
//! - The app owns the offsets and clamps them after each resize or input event.
//! - `ScrollMetrics` is used to derive the `ScrollLengths` and max offsets for each axis.
//! - `handle_mouse_event` stays thin by building scrollbars from the latest metrics, mirroring how
//!   an app would typically handle input per frame.
//!
//! ## Controls
//!
//! - Arrow keys: move the scrollbars in subcell steps.
//! - Mouse wheel: scroll the matching axis.
//! - Click + drag: grab the thumb and drag.
//! - `q` or `Esc`: exit the demo.

use std::io;

use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use tui_scrollbar::{
    SUBCELL, ScrollBar, ScrollBarArrows, ScrollBarInteraction, ScrollCommand, ScrollLengths,
    ScrollMetrics,
};

const KEY_STEP: usize = 1;
const TITLE_FG: Color = Color::Rgb(196, 206, 224);
const TITLE_BG: Color = Color::Rgb(32, 43, 64);
const BLOCK_FG: Color = Color::Rgb(196, 206, 224);
const BLOCK_BG: Color = Color::Rgb(13, 23, 38);
const SCROLLBAR_TRACK_BG: Color = Color::Rgb(40, 40, 40);
const SCROLLBAR_THUMB_BG: Color = SCROLLBAR_TRACK_BG;
const SCROLLBAR_THUMB_FG: Color = Color::Rgb(224, 224, 224);
const SCROLLBAR_ARROW_FG: Color = Color::Rgb(224, 224, 224);

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| {
        execute!(io::stdout(), event::EnableMouseCapture)?;
        let result = App::new().run(terminal);
        execute!(io::stdout(), event::DisableMouseCapture)?;
        result
    })
}

#[derive(Debug, Default)]
struct App {
    /// Current run state, toggled to `Quit` when the user exits.
    state: AppState,
    /// Latest layout rectangles used to place the scrollbars.
    layout: Option<LayoutState>,
    /// Vertical scroll offset in logical units (subcells for this demo).
    vertical_offset: usize,
    /// Horizontal scroll offset in logical units (subcells for this demo).
    horizontal_offset: usize,
    /// Drag state for the vertical scrollbar.
    vertical_interaction: ScrollBarInteraction,
    /// Drag state for the horizontal scrollbar.
    horizontal_interaction: ScrollBarInteraction,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    /// Keep running the event loop and rendering frames.
    Running,
    /// Exit the application on the next loop iteration.
    Quit,
}

#[derive(Debug, Clone, Copy)]
struct LayoutState {
    /// The content area used to compute scroll metrics.
    content: Rect,
    /// Rect for the vertical scrollbar (right edge).
    vertical_bar: Rect,
    /// Rect for the horizontal scrollbar (bottom edge).
    horizontal_bar: Rect,
}

impl App {
    /// Builds a fresh app state with zero offsets.
    fn new() -> Self {
        Self {
            state: AppState::Running,
            layout: None,
            vertical_offset: 0,
            horizontal_offset: 0,
            vertical_interaction: ScrollBarInteraction::new(),
            horizontal_interaction: ScrollBarInteraction::new(),
        }
    }

    /// Runs the event loop until the user exits.
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Renders the title block and the two scrollbars.
    fn render(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        if area.width < 2 || area.height < 2 {
            return;
        }

        let title = "tui-scrollbar - mouse scroll demo";
        let block = Block::new()
            .borders(Borders::TOP)
            .border_style(Style::new().fg(TITLE_FG).bg(TITLE_BG))
            .style(Style::new().fg(BLOCK_FG).bg(BLOCK_BG))
            .title(
                Line::from(title)
                    .centered()
                    .fg(TITLE_FG)
                    .bg(TITLE_BG)
                    .bold(),
            );
        frame.render_widget(&block, area);

        let content_area = Rect {
            y: area.y.saturating_add(1),
            height: area.height.saturating_sub(1),
            ..area
        };
        let help = "Arrows: move | Wheel: scroll | Drag: thumb | q/Esc: quit";
        let help_area = Rect {
            x: content_area.x.saturating_add(1),
            y: content_area.y,
            width: content_area.width.saturating_sub(1),
            height: 1,
        };
        if help_area.width > 0 {
            frame.render_widget(
                Paragraph::new(help).style(Style::new().fg(TITLE_FG)),
                help_area,
            );
        }
        let content_area = Rect {
            y: content_area.y.saturating_add(1),
            height: content_area.height.saturating_sub(1),
            ..content_area
        };

        // Split out the bottom row and right column for the scrollbars.
        let [content_row, bar_row] = content_area.layout(&Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));
        let [content, vertical_bar] = content_row.layout(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));
        let [horizontal_bar, _corner] = bar_row.layout(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));

        self.layout = Some(LayoutState {
            content,
            vertical_bar,
            horizontal_bar,
        });

        // Keep offsets valid when the terminal is resized.
        let (h_metrics, v_metrics) = self.metrics_for_layout(content);
        self.horizontal_offset = self.horizontal_offset.min(h_metrics.max_offset());
        self.vertical_offset = self.vertical_offset.min(v_metrics.max_offset());

        let horizontal_lengths = ScrollLengths {
            content_len: h_metrics.content_len(),
            viewport_len: h_metrics.viewport_len(),
        };
        let track_style = Style::new().bg(SCROLLBAR_TRACK_BG);
        let thumb_style = Style::new().fg(SCROLLBAR_THUMB_FG).bg(SCROLLBAR_THUMB_BG);
        let arrow_style = Style::new().fg(SCROLLBAR_ARROW_FG).bg(SCROLLBAR_TRACK_BG);
        let horizontal = ScrollBar::horizontal(horizontal_lengths)
            .arrows(ScrollBarArrows::Both)
            .offset(self.horizontal_offset)
            .scroll_step(SUBCELL)
            .track_style(track_style)
            .thumb_style(thumb_style)
            .arrow_style(arrow_style);
        let vertical_lengths = ScrollLengths {
            content_len: v_metrics.content_len(),
            viewport_len: v_metrics.viewport_len(),
        };
        let vertical = ScrollBar::vertical(vertical_lengths)
            .arrows(ScrollBarArrows::Both)
            .offset(self.vertical_offset)
            .scroll_step(SUBCELL)
            .track_style(track_style)
            .thumb_style(thumb_style)
            .arrow_style(arrow_style);

        frame.render_widget(&horizontal, horizontal_bar);
        frame.render_widget(&vertical, vertical_bar);
    }

    /// Handles keyboard and mouse events, updating offsets as needed.
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) => {
                if key.is_press() {
                    self.handle_key_event(key.code);
                }
            }
            Event::Mouse(event) => {
                self.handle_mouse_event(event);
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles keyboard input, updating offsets or exiting as needed.
    fn handle_key_event(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.state = AppState::Quit,
            KeyCode::Up | KeyCode::Char('k') => self.handle_key_scroll(0, -(KEY_STEP as isize)),
            KeyCode::Down | KeyCode::Char('j') => self.handle_key_scroll(0, KEY_STEP as isize),
            KeyCode::Left | KeyCode::Char('h') => self.handle_key_scroll(-(KEY_STEP as isize), 0),
            KeyCode::Right | KeyCode::Char('l') => self.handle_key_scroll(KEY_STEP as isize, 0),
            _ => {}
        }
    }

    /// Applies a keyboard delta to the scrollbar offsets.
    fn handle_key_scroll(&mut self, dx: isize, dy: isize) {
        let Some(layout) = self.layout else {
            return;
        };
        let (h_metrics, v_metrics) = self.metrics_for_layout(layout.content);
        self.horizontal_offset =
            Self::apply_delta(self.horizontal_offset, dx, h_metrics.max_offset());
        self.vertical_offset = Self::apply_delta(self.vertical_offset, dy, v_metrics.max_offset());
    }

    /// Handles crossterm mouse events using the scrollbar helpers.
    fn handle_mouse_event(&mut self, event: event::MouseEvent) {
        let Some(layout) = self.layout else {
            return;
        };
        let (h_metrics, v_metrics) = self.metrics_for_layout(layout.content);
        let horizontal = self.horizontal_scrollbar(h_metrics);
        let vertical = self.vertical_scrollbar(v_metrics);

        if let Some(command) = horizontal.handle_mouse_event(
            layout.horizontal_bar,
            event,
            &mut self.horizontal_interaction,
        ) {
            self.apply_command(command, true);
        }
        if let Some(command) =
            vertical.handle_mouse_event(layout.vertical_bar, event, &mut self.vertical_interaction)
        {
            self.apply_command(command, false);
        }
    }

    /// Applies a scroll command to the current axis offset.
    fn apply_command(&mut self, command: ScrollCommand, is_horizontal: bool) {
        let ScrollCommand::SetOffset(offset) = command;
        if is_horizontal {
            self.horizontal_offset = offset;
        } else {
            self.vertical_offset = offset;
        }
    }

    /// Builds a horizontal scrollbar from the current metrics.
    fn horizontal_scrollbar(&self, metrics: ScrollMetrics) -> ScrollBar {
        let lengths = ScrollLengths {
            content_len: metrics.content_len(),
            viewport_len: metrics.viewport_len(),
        };
        ScrollBar::horizontal(lengths)
            .arrows(ScrollBarArrows::Both)
            .offset(self.horizontal_offset)
            .scroll_step(SUBCELL)
    }

    /// Builds a vertical scrollbar from the current metrics.
    fn vertical_scrollbar(&self, metrics: ScrollMetrics) -> ScrollBar {
        let lengths = ScrollLengths {
            content_len: metrics.content_len(),
            viewport_len: metrics.viewport_len(),
        };
        ScrollBar::vertical(lengths)
            .arrows(ScrollBarArrows::Both)
            .offset(self.vertical_offset)
            .scroll_step(SUBCELL)
    }

    /// Derives metrics based on the current layout and desired scroll range.
    fn metrics_for_layout(&self, content: Rect) -> (ScrollMetrics, ScrollMetrics) {
        // Use subcell units so wheel/drag updates line up with the fractional renderer.
        let h_cells = content.width.max(1) as usize;
        let v_cells = content.height.max(1) as usize;
        let h_content = h_cells.saturating_mul(SUBCELL).max(1);
        let v_content = v_cells.saturating_mul(SUBCELL).max(1);
        let h_viewport = h_content.saturating_sub(100).max(1);
        let v_viewport = v_content.saturating_sub(100).max(1);
        (
            ScrollMetrics::new(
                ScrollLengths {
                    content_len: h_content,
                    viewport_len: h_viewport,
                },
                self.horizontal_offset,
                content.width,
            ),
            ScrollMetrics::new(
                ScrollLengths {
                    content_len: v_content,
                    viewport_len: v_viewport,
                },
                self.vertical_offset,
                content.height,
            ),
        )
    }

    /// Adds a signed delta while clamping to the provided maximum.
    fn apply_delta(current: usize, delta: isize, max: usize) -> usize {
        if delta < 0 {
            current.saturating_sub(delta.unsigned_abs())
        } else {
            current.saturating_add(delta as usize).min(max)
        }
    }
}
