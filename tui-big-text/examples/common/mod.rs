//! common module for examples

use std::io;

use color_eyre::Result;
use crossterm::ExecutableCommand;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::CrosstermBackend;

/// A type alias for the terminal
type Terminal = ratatui::Terminal<CrosstermBackend<io::Stdout>>;

/// A generic main loop for a TUI example that just renders the given function and exits when the
/// user presses 'q' or 'Esc'
pub fn run<F>(render: F) -> Result<()>
where
    F: Fn(&mut ratatui::Frame),
{
    install_panic_handler();

    with_terminal(|mut terminal| {
        loop {
            terminal.draw(|frame| render(frame))?;
            if let crossterm::event::Event::Key(event) = crossterm::event::read()?
                && matches!(
                    event.code,
                    crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc
                )
            {
                break Ok(());
            }
        }
    })
}

/// Install a panic handler that restores the terminal before panicking
fn install_panic_handler() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        if let Err(err) = restore() {
            eprintln!("failed to restore terminal: {err}");
        }
        hook(panic_info);
    }));
}

/// Run a function with a terminal ensuring that the terminal is restored afterwards
fn with_terminal<F>(f: F) -> color_eyre::Result<()>
where
    F: FnOnce(Terminal) -> color_eyre::Result<()>,
{
    let terminal = init()?;
    let result = f(terminal);
    restore()?;
    result?;
    Ok(())
}

/// Initialize a terminal
fn init() -> io::Result<Terminal> {
    io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

/// Restore the terminal to its original state
fn restore() -> io::Result<()> {
    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
