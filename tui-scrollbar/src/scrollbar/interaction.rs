//! Input handling and hit-testing helpers for the scrollbar widget.
//!
//! This module groups pointer/wheel handling so the main widget definition stays focused on
//! configuration and rendering. The functions here are pure in/out helpers that return
//! [`ScrollCommand`] values for the application to apply.
//!
//! When a pointer presses inside the thumb, the handler stores a subcell grab offset so subsequent
//! drag events keep the pointer anchored to the same position within the thumb.

use ratatui_core::layout::Rect;

use super::{ArrowHit, ArrowLayout, ScrollBar, ScrollBarOrientation, TrackClickBehavior};
use crate::ScrollLengths;
#[cfg(any(feature = "crossterm_0_28", feature = "crossterm_0_29"))]
use crate::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use crate::input::{
    DragState, PointerButton, PointerEvent, PointerEventKind, ScrollAxis, ScrollBarInteraction,
    ScrollCommand, ScrollEvent, ScrollWheel,
};
use crate::metrics::{HitTest, SUBCELL, ScrollMetrics};

impl ScrollBar {
    /// Handles a backend-agnostic scrollbar event.
    ///
    /// Returns a [`ScrollCommand`] when the event should update the caller-owned offset. This
    /// method does not mutate your application state directly.
    ///
    /// Pointer events outside the track are ignored. Scroll wheel events are ignored unless the
    /// axis matches the scrollbar orientation.
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use tui_scrollbar::{
    ///     PointerButton, PointerEvent, PointerEventKind, ScrollBar, ScrollBarInteraction,
    ///     ScrollEvent, ScrollLengths,
    /// };
    ///
    /// let area = Rect::new(0, 0, 1, 6);
    /// let lengths = ScrollLengths {
    ///     content_len: 120,
    ///     viewport_len: 24,
    /// };
    /// let scrollbar = ScrollBar::vertical(lengths).offset(0);
    /// let mut interaction = ScrollBarInteraction::new();
    /// let event = ScrollEvent::Pointer(PointerEvent {
    ///     column: 0,
    ///     row: 2,
    ///     kind: PointerEventKind::Down,
    ///     button: PointerButton::Primary,
    /// });
    ///
    /// let _ = scrollbar.handle_event(area, event, &mut interaction);
    /// ```
    pub fn handle_event(
        &self,
        area: Rect,
        event: ScrollEvent,
        interaction: &mut ScrollBarInteraction,
    ) -> Option<ScrollCommand> {
        if area.width == 0 || area.height == 0 {
            return None;
        }

        let layout = self.arrow_layout(area);
        let lengths = ScrollLengths {
            content_len: self.content_len,
            viewport_len: self.viewport_len,
        };
        let track_cells = match self.orientation {
            ScrollBarOrientation::Vertical => layout.track_area.height,
            ScrollBarOrientation::Horizontal => layout.track_area.width,
        };
        let metrics = ScrollMetrics::new(lengths, self.offset, track_cells);

        match event {
            ScrollEvent::Pointer(event) => {
                if let Some(command) =
                    self.handle_arrow_pointer(&layout, metrics, event, interaction)
                {
                    return Some(command);
                }
                self.handle_pointer_event(layout.track_area, metrics, event, interaction)
            }
            ScrollEvent::ScrollWheel(event) => self.handle_scroll_wheel(area, metrics, event),
        }
    }

    #[cfg(any(feature = "crossterm_0_28", feature = "crossterm_0_29"))]
    /// Handles crossterm mouse events for this scrollbar.
    ///
    /// This helper converts crossterm events into [`ScrollEvent`] values before delegating to
    /// [`Self::handle_event`]. See the `scrollbar_mouse` example for a complete terminal event
    /// loop with mouse capture.
    pub fn handle_mouse_event(
        &self,
        area: Rect,
        event: MouseEvent,
        interaction: &mut ScrollBarInteraction,
    ) -> Option<ScrollCommand> {
        let event = match event.kind {
            MouseEventKind::Down(MouseButton::Left) => Some(ScrollEvent::Pointer(PointerEvent {
                column: event.column,
                row: event.row,
                kind: PointerEventKind::Down,
                button: PointerButton::Primary,
            })),
            MouseEventKind::Up(MouseButton::Left) => Some(ScrollEvent::Pointer(PointerEvent {
                column: event.column,
                row: event.row,
                kind: PointerEventKind::Up,
                button: PointerButton::Primary,
            })),
            MouseEventKind::Drag(MouseButton::Left) => Some(ScrollEvent::Pointer(PointerEvent {
                column: event.column,
                row: event.row,
                kind: PointerEventKind::Drag,
                button: PointerButton::Primary,
            })),
            MouseEventKind::ScrollUp => Some(ScrollEvent::ScrollWheel(ScrollWheel {
                axis: ScrollAxis::Vertical,
                delta: -1,
                column: event.column,
                row: event.row,
            })),
            MouseEventKind::ScrollDown => Some(ScrollEvent::ScrollWheel(ScrollWheel {
                axis: ScrollAxis::Vertical,
                delta: 1,
                column: event.column,
                row: event.row,
            })),
            MouseEventKind::ScrollLeft => Some(ScrollEvent::ScrollWheel(ScrollWheel {
                axis: ScrollAxis::Horizontal,
                delta: -1,
                column: event.column,
                row: event.row,
            })),
            MouseEventKind::ScrollRight => Some(ScrollEvent::ScrollWheel(ScrollWheel {
                axis: ScrollAxis::Horizontal,
                delta: 1,
                column: event.column,
                row: event.row,
            })),
            _ => None,
        };

        event.and_then(|event| self.handle_event(area, event, interaction))
    }

    /// Handles pointer down/drag/up events and converts them to scroll commands.
    fn handle_pointer_event(
        &self,
        area: Rect,
        metrics: ScrollMetrics,
        event: PointerEvent,
        interaction: &mut ScrollBarInteraction,
    ) -> Option<ScrollCommand> {
        if event.button != PointerButton::Primary {
            return None;
        }

        match event.kind {
            PointerEventKind::Down => {
                let cell_index = axis_cell_index(area, event.column, event.row, self.orientation)?;
                let position = cell_index
                    .saturating_mul(SUBCELL)
                    .saturating_add(SUBCELL / 2);
                if metrics.thumb_len() == 0 {
                    return None;
                }
                match metrics.hit_test(position) {
                    HitTest::Thumb => {
                        let grab_offset = position.saturating_sub(metrics.thumb_start());
                        interaction.start_drag(grab_offset);
                        None
                    }
                    HitTest::Track => {
                        interaction.stop_drag();
                        self.handle_track_click(metrics, position)
                    }
                }
            }
            PointerEventKind::Drag => match interaction.drag_state {
                DragState::Idle => None,
                DragState::Dragging { grab_offset } => {
                    let cell_index =
                        axis_cell_index_clamped(area, event.column, event.row, self.orientation)?;
                    let position = cell_index
                        .saturating_mul(SUBCELL)
                        .saturating_add(SUBCELL / 2);
                    let thumb_start = position.saturating_sub(grab_offset);
                    Some(ScrollCommand::SetOffset(
                        metrics.offset_for_thumb_start(thumb_start),
                    ))
                }
            },
            PointerEventKind::Up => {
                interaction.stop_drag();
                None
            }
        }
    }

    /// Converts a click on the track into a page or jump action.
    fn handle_track_click(&self, metrics: ScrollMetrics, position: usize) -> Option<ScrollCommand> {
        if metrics.max_offset() == 0 {
            return None;
        }

        match self.track_click_behavior {
            TrackClickBehavior::Page => {
                let thumb_end = metrics.thumb_start().saturating_add(metrics.thumb_len());
                if position < metrics.thumb_start() {
                    Some(ScrollCommand::SetOffset(
                        metrics.offset().saturating_sub(metrics.viewport_len()),
                    ))
                } else if position >= thumb_end {
                    Some(ScrollCommand::SetOffset(
                        (metrics.offset() + metrics.viewport_len()).min(metrics.max_offset()),
                    ))
                } else {
                    None
                }
            }
            TrackClickBehavior::JumpToClick => {
                let half_thumb = metrics.thumb_len() / 2;
                let thumb_start = position.saturating_sub(half_thumb);
                Some(ScrollCommand::SetOffset(
                    metrics.offset_for_thumb_start(thumb_start),
                ))
            }
        }
    }

    /// Handles scroll wheel input, respecting axis and clamping to bounds.
    fn handle_scroll_wheel(
        &self,
        _area: Rect,
        metrics: ScrollMetrics,
        event: ScrollWheel,
    ) -> Option<ScrollCommand> {
        let matches_axis = matches!(
            (self.orientation, event.axis),
            (ScrollBarOrientation::Vertical, ScrollAxis::Vertical)
                | (ScrollBarOrientation::Horizontal, ScrollAxis::Horizontal)
        );

        if !matches_axis {
            return None;
        }

        let step = self.scroll_step.max(1) as isize;
        let delta = event.delta.saturating_mul(step);
        let max_offset = metrics.max_offset() as isize;
        let next = (metrics.offset() as isize).saturating_add(delta);
        let next = next.clamp(0, max_offset);
        Some(ScrollCommand::SetOffset(next as usize))
    }

    /// Handles arrow clicks by stepping the offset in the requested direction.
    fn handle_arrow_pointer(
        &self,
        layout: &ArrowLayout,
        metrics: ScrollMetrics,
        event: PointerEvent,
        interaction: &mut ScrollBarInteraction,
    ) -> Option<ScrollCommand> {
        if event.button != PointerButton::Primary || event.kind != PointerEventKind::Down {
            return None;
        }

        let hit = self.arrow_hit(layout, event)?;
        if metrics.max_offset() == 0 {
            return None;
        }

        interaction.stop_drag();
        let step = self.scroll_step.max(1) as isize;
        let delta = match hit {
            ArrowHit::Start => -step,
            ArrowHit::End => step,
        };
        let max_offset = metrics.max_offset() as isize;
        let next = (metrics.offset() as isize).saturating_add(delta);
        let next = next.clamp(0, max_offset);
        Some(ScrollCommand::SetOffset(next as usize))
    }

    /// Returns which arrow (if any) a pointer event hit.
    fn arrow_hit(&self, layout: &ArrowLayout, event: PointerEvent) -> Option<ArrowHit> {
        if let Some((x, y)) = layout.start
            && event.column == x
            && event.row == y
        {
            return Some(ArrowHit::Start);
        }
        if let Some((x, y)) = layout.end
            && event.column == x
            && event.row == y
        {
            return Some(ArrowHit::End);
        }
        None
    }
}

/// Returns the cell index along the scroll axis for a pointer location.
fn axis_cell_index(
    area: Rect,
    column: u16,
    row: u16,
    orientation: ScrollBarOrientation,
) -> Option<usize> {
    match orientation {
        ScrollBarOrientation::Vertical => {
            if row < area.y || row >= area.y.saturating_add(area.height) {
                None
            } else {
                Some(row.saturating_sub(area.y) as usize)
            }
        }
        ScrollBarOrientation::Horizontal => {
            if column < area.x || column >= area.x.saturating_add(area.width) {
                None
            } else {
                Some(column.saturating_sub(area.x) as usize)
            }
        }
    }
}

/// Returns a clamped cell index along the scroll axis for drag updates.
fn axis_cell_index_clamped(
    area: Rect,
    column: u16,
    row: u16,
    orientation: ScrollBarOrientation,
) -> Option<usize> {
    match orientation {
        ScrollBarOrientation::Vertical => {
            if area.height == 0 {
                return None;
            }
            let end = area.y.saturating_add(area.height).saturating_sub(1);
            let row = row.clamp(area.y, end);
            Some(row.saturating_sub(area.y) as usize)
        }
        ScrollBarOrientation::Horizontal => {
            if area.width == 0 {
                return None;
            }
            let end = area.x.saturating_add(area.width).saturating_sub(1);
            let column = column.clamp(area.x, end);
            Some(column.saturating_sub(area.x) as usize)
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::layout::Rect;

    use super::*;
    use crate::{ScrollBarArrows, ScrollLengths};

    #[test]
    fn pages_when_clicking_track() {
        let lengths = ScrollLengths {
            content_len: 100,
            viewport_len: 20,
        };
        let scrollbar = ScrollBar::vertical(lengths)
            .arrows(ScrollBarArrows::None)
            .offset(40);
        let area = Rect::new(0, 0, 1, 10);
        let event = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 0,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        let expected = 20;
        let mut interaction = ScrollBarInteraction::default();
        assert_eq!(
            scrollbar.handle_event(area, event, &mut interaction),
            Some(ScrollCommand::SetOffset(expected))
        );
    }

    #[test]
    fn updates_offset_while_dragging() {
        let lengths = ScrollLengths {
            content_len: 16,
            viewport_len: 8,
        };
        let scrollbar = ScrollBar::vertical(lengths)
            .arrows(ScrollBarArrows::None)
            .offset(0);
        let area = Rect::new(0, 0, 1, 4);
        let mut interaction = ScrollBarInteraction::default();
        let down = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 0,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        assert_eq!(scrollbar.handle_event(area, down, &mut interaction), None);

        let drag = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 1,
            kind: PointerEventKind::Drag,
            button: PointerButton::Primary,
        });
        assert_eq!(
            scrollbar.handle_event(area, drag, &mut interaction),
            Some(ScrollCommand::SetOffset(4))
        );
    }

    #[test]
    fn applies_scroll_step_to_wheel() {
        let lengths = ScrollLengths {
            content_len: 100,
            viewport_len: 20,
        };
        let scrollbar = ScrollBar::vertical(lengths)
            .arrows(ScrollBarArrows::None)
            .offset(40)
            .scroll_step(3);
        let area = Rect::new(0, 0, 1, 10);
        let mut interaction = ScrollBarInteraction::default();
        let event = ScrollEvent::ScrollWheel(ScrollWheel {
            axis: ScrollAxis::Vertical,
            delta: 1,
            column: 0,
            row: 0,
        });
        assert_eq!(
            scrollbar.handle_event(area, event, &mut interaction),
            Some(ScrollCommand::SetOffset(43))
        );
    }

    #[test]
    fn steps_offset_when_clicking_arrows() {
        let lengths = ScrollLengths {
            content_len: 100,
            viewport_len: 20,
        };
        let scrollbar = ScrollBar::vertical(lengths)
            .arrows(ScrollBarArrows::Both)
            .offset(10)
            .scroll_step(5);
        let area = Rect::new(0, 0, 1, 5);
        let mut interaction = ScrollBarInteraction::default();
        let up = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 0,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        assert_eq!(
            scrollbar.handle_event(area, up, &mut interaction),
            Some(ScrollCommand::SetOffset(5))
        );

        let down = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 4,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        assert_eq!(
            scrollbar.handle_event(area, down, &mut interaction),
            Some(ScrollCommand::SetOffset(15))
        );
    }

    #[test]
    fn ignores_scroll_wheel_on_other_axis() {
        let lengths = ScrollLengths {
            content_len: 100,
            viewport_len: 20,
        };
        let scrollbar = ScrollBar::vertical(lengths);
        let area = Rect::new(0, 0, 1, 5);
        let mut interaction = ScrollBarInteraction::default();
        let event = ScrollEvent::ScrollWheel(ScrollWheel {
            axis: ScrollAxis::Horizontal,
            delta: 1,
            column: 0,
            row: 2,
        });
        assert_eq!(scrollbar.handle_event(area, event, &mut interaction), None);
    }

    #[test]
    fn applies_negative_scroll_wheel_delta() {
        let lengths = ScrollLengths {
            content_len: 100,
            viewport_len: 20,
        };
        let scrollbar = ScrollBar::vertical(lengths).offset(10).scroll_step(2);
        let area = Rect::new(0, 0, 1, 5);
        let event = ScrollEvent::ScrollWheel(ScrollWheel {
            axis: ScrollAxis::Vertical,
            delta: -1,
            column: 0,
            row: 2,
        });
        let mut interaction = ScrollBarInteraction::default();
        assert_eq!(
            scrollbar.handle_event(area, event, &mut interaction),
            Some(ScrollCommand::SetOffset(8))
        );
    }

    #[test]
    fn jumps_toward_track_click() {
        let lengths = ScrollLengths {
            content_len: 8,
            viewport_len: 4,
        };
        let scrollbar = ScrollBar::vertical(lengths)
            .arrows(ScrollBarArrows::None)
            .track_click_behavior(TrackClickBehavior::JumpToClick);
        let area = Rect::new(0, 0, 1, 4);
        let event = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 2,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        let expected = 3;
        let mut interaction = ScrollBarInteraction::default();
        assert_eq!(
            scrollbar.handle_event(area, event, &mut interaction),
            Some(ScrollCommand::SetOffset(expected))
        );
    }

    #[test]
    fn clears_drag_on_pointer_up() {
        let lengths = ScrollLengths {
            content_len: 100,
            viewport_len: 20,
        };
        let scrollbar = ScrollBar::vertical(lengths);
        let area = Rect::new(0, 0, 1, 5);
        let mut interaction = ScrollBarInteraction::default();
        interaction.start_drag(3);
        let event = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 1,
            kind: PointerEventKind::Up,
            button: PointerButton::Primary,
        });
        assert_eq!(scrollbar.handle_event(area, event, &mut interaction), None);
        assert_eq!(interaction.drag_state, DragState::Idle);
    }

    #[test]
    fn ignores_pointer_events_outside_track() {
        let lengths = ScrollLengths {
            content_len: 100,
            viewport_len: 20,
        };
        let scrollbar = ScrollBar::vertical(lengths);
        let area = Rect::new(0, 0, 1, 5);
        let event = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 6,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        let mut interaction = ScrollBarInteraction::default();
        assert_eq!(scrollbar.handle_event(area, event, &mut interaction), None);
    }

    #[test]
    fn ignores_arrow_clicks_when_max_offset_zero() {
        let lengths = ScrollLengths {
            content_len: 10,
            viewport_len: 10,
        };
        let scrollbar = ScrollBar::vertical(lengths).arrows(ScrollBarArrows::Both);
        let area = Rect::new(0, 0, 1, 5);
        let event = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 0,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        let mut interaction = ScrollBarInteraction::default();
        assert_eq!(scrollbar.handle_event(area, event, &mut interaction), None);
    }

    #[test]
    fn stops_drag_on_track_click() {
        let lengths = ScrollLengths {
            content_len: 10,
            viewport_len: 5,
        };
        let scrollbar = ScrollBar::vertical(lengths).arrows(ScrollBarArrows::None);
        let area = Rect::new(0, 0, 1, 4);
        let mut interaction = ScrollBarInteraction::default();
        interaction.start_drag(2);
        let event = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 3,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        assert_eq!(
            scrollbar.handle_event(area, event, &mut interaction),
            Some(ScrollCommand::SetOffset(5))
        );
        assert_eq!(interaction.drag_state, DragState::Idle);
    }

    #[test]
    fn returns_none_when_clicking_inside_thumb_in_page_mode() {
        let lengths = ScrollLengths {
            content_len: 100,
            viewport_len: 20,
        };
        let scrollbar = ScrollBar::vertical(lengths).arrows(ScrollBarArrows::None);
        let area = Rect::new(0, 0, 1, 10);
        let mut interaction = ScrollBarInteraction::default();
        let event = ScrollEvent::Pointer(PointerEvent {
            column: 0,
            row: 0,
            kind: PointerEventKind::Down,
            button: PointerButton::Primary,
        });
        assert_eq!(scrollbar.handle_event(area, event, &mut interaction), None);
    }
}
