//! Renders a QR code for <https://ratatui.rs>.
//!
//! Run with `cargo run -p tui-qrcode --example qrcode`.
//!
//! The example uses inverted colors to render a white-on-black QR code.
//!
//! Press any key to quit.

use qrcode::QrCode;
use ratatui::crossterm::event;
use ratatui::{DefaultTerminal, Frame};
use tui_qrcode::{Colors, QrCodeWidget};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(render)?;
        if event::read()?.as_key_press_event().is_some() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
    let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted);
    frame.render_widget(widget, frame.area());
}
