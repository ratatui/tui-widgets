//! An example of using multiple tabs each with a scrollable view, as well as how state can be
//! managed across multiple tabs using Stateful Widgets.
//!
//! This example uses the `unstable-widget-ref` feature in Ratatui to allow the tab widgets to
//! be created once and then reused across multiple frames. Each tab has some static lorem ipsum
//! text, and we store the scroll state for each tab separately.
//!
//! Controls:
//! - `Tab`: move to the next tab
//! - `Shift+Tab`: move to the previous tab
//! - `j` / `Down`: scroll down
//! - `k` / `Up`: scroll up
//! - `f` / `PageDown`: scroll one page down
//! - `b` / `PageUp`: scroll one page up
//! - `g` / `Home`: scroll to the top
//! - `G` / `End`: scroll to the bottom
//! - `q` / `Esc`: quit

use std::collections::HashMap;
use std::fmt::Debug;
use std::io;

use color_eyre::Result;
use lipsum::lipsum;
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect, Size};
use ratatui::style::Stylize;
use ratatui::style::palette::tailwind;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, StatefulWidget, StatefulWidgetRef, Tabs, Widget, Wrap};
use tui_scrollview::{ScrollView, ScrollViewState};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

#[derive(Default)]
struct App {
    state: AppState,
    tabs: HashMap<
        VisibleTab,
        (
            Box<dyn StatefulWidgetRef<State = ScrollViewState>>,
            ScrollViewState,
        ),
    >,
    visible_tab: VisibleTab,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum VisibleTab {
    #[default]
    Red,
    Green,
    Blue,
}

#[derive(Debug, Default)]
struct RedTab {
    text: String,
}

impl RedTab {
    fn new() -> Self {
        Self { text: lipsum(500) }
    }
}

#[derive(Debug, Default, Clone)]
struct GreenTab {
    text: String,
}

impl GreenTab {
    fn new() -> Self {
        Self {
            text: lipsum(1_000),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct BlueTab {
    text: String,
}

impl BlueTab {
    fn new() -> Self {
        Self {
            text: lipsum(10_000),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    fn new() -> Self {
        let mut tabs: HashMap<
            VisibleTab,
            (
                Box<dyn StatefulWidgetRef<State = ScrollViewState>>,
                ScrollViewState,
            ),
        > = HashMap::new();
        tabs.insert(
            VisibleTab::Red,
            (Box::new(RedTab::new()), ScrollViewState::default()),
        );
        tabs.insert(
            VisibleTab::Green,
            (Box::new(GreenTab::new()), ScrollViewState::default()),
        );
        tabs.insert(
            VisibleTab::Blue,
            (Box::new(BlueTab::new()), ScrollViewState::default()),
        );
        Self {
            tabs,
            ..Default::default()
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.is_running() {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        if let Some(key) = event::read()?.as_key_press_event() {
            let (_widget, scroll_view_state) = self
                .tabs
                .get_mut(&self.visible_tab)
                .expect("visible tab should exist");
            match key.code {
                Char('q') | Esc => self.quit(),
                Tab => {
                    self.visible_tab = match self.visible_tab {
                        VisibleTab::Red => VisibleTab::Green,
                        VisibleTab::Green => VisibleTab::Blue,
                        VisibleTab::Blue => VisibleTab::Red,
                    }
                }
                BackTab => {
                    self.visible_tab = match self.visible_tab {
                        VisibleTab::Red => VisibleTab::Blue,
                        VisibleTab::Green => VisibleTab::Red,
                        VisibleTab::Blue => VisibleTab::Green,
                    }
                }
                Char('j') | Down => scroll_view_state.scroll_down(),
                Char('k') | Up => scroll_view_state.scroll_up(),
                Char('f') | PageDown => scroll_view_state.scroll_page_down(),
                Char('b') | PageUp => scroll_view_state.scroll_page_up(),
                Char('g') | Home => scroll_view_state.scroll_to_top(),
                Char('G') | End => scroll_view_state.scroll_to_bottom(),
                _ => (),
            }
        }
        Ok(())
    }

    const fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]);
        let [title, tabs, body] = layout.areas(area);

        self.title().render(title, buf);
        self.tabs().render(tabs, buf);
        let (tab, state) = self.tabs.get_mut(&self.visible_tab).unwrap();
        tab.render_ref(body, buf, state);
    }
}

impl App {
    fn title(&self) -> impl Widget {
        let palette = tailwind::SLATE;
        let fg = palette.c900;
        let bg = palette.c300;
        let keys_fg = palette.c50;
        let keys_bg = palette.c600;
        Line::from(vec![
            "Tui-scrollview  ".into(),
            "  ↓ | ↑ | PageDown | PageUp | Home | End | Tab  "
                .fg(keys_fg)
                .bg(keys_bg),
            "  Quit: ".into(),
            " Esc ".fg(keys_fg).bg(keys_bg),
        ])
        .style((fg, bg))
    }

    fn tabs(&self) -> impl Widget {
        let selected = self.visible_tab as usize;
        Tabs::new([
            " Red ".fg(tailwind::RED.c900),
            " Green ".fg(tailwind::GREEN.c900),
            " Blue ".fg(tailwind::BLUE.c900),
        ])
        .padding("", "")
        .divider("")
        .select(selected)
        .style(tailwind::SLATE.c900)
    }
}

impl StatefulWidgetRef for RedTab {
    type State = ScrollViewState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollViewState) {
        const SCROLLVIEW_HEIGHT: u16 = 50;
        let mut scroll_view = ScrollView::new(Size::new(area.width - 1, SCROLLVIEW_HEIGHT));
        scroll_view.render_widget(
            Paragraph::new(self.text.clone())
                .white()
                .on_red()
                .wrap(Wrap::default()),
            Rect::new(0, 0, area.width - 1, SCROLLVIEW_HEIGHT),
        );
        scroll_view.render(area, buf, state);
    }
}

impl StatefulWidgetRef for GreenTab {
    type State = ScrollViewState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollViewState) {
        const SCROLLVIEW_HEIGHT: u16 = 100;
        let mut scroll_view = ScrollView::new(Size::new(area.width - 1, SCROLLVIEW_HEIGHT));
        scroll_view.render_widget(
            Paragraph::new(self.text.clone())
                .white()
                .on_green()
                .wrap(Wrap::default()),
            Rect::new(0, 0, area.width - 1, SCROLLVIEW_HEIGHT),
        );
        scroll_view.render(area, buf, state);
    }
}

impl StatefulWidgetRef for BlueTab {
    type State = ScrollViewState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollViewState) {
        const SCROLLVIEW_HEIGHT: u16 = 200;
        let mut scroll_view = ScrollView::new(Size::new(area.width - 1, SCROLLVIEW_HEIGHT));
        scroll_view.render_widget(
            Paragraph::new(self.text.clone())
                .white()
                .on_blue()
                .wrap(Wrap::default()),
            Rect::new(0, 0, area.width - 1, SCROLLVIEW_HEIGHT),
        );
        scroll_view.render(area, buf, state);
    }
}
