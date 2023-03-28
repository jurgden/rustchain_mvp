use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::Terminal;

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the terminal backend
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Enter the main loop
    loop {
        // Draw the UI
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Rustchain MVP")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        // Handle input
        let stdin = std::io::stdin();
        for key in stdin.keys() {
            match key? {
                Key::Char('q') => break, // Quit on 'q'
                _ => continue,
            }
        }
    }

    // Clean up and exit
    terminal.show_cursor()?;
    Ok(())
}
