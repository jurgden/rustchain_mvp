use std::io::{self};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::CrosstermBackend;
// use tui::layout::{Constraint, Direction, Layout};
// use tui::style::Modifier;
// use tui::text;
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the terminal backend
    let stdout = io::stdout().into_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
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
    // terminal.show_cursor()?;
    // Ok(())
}
