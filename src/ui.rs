use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the terminal backend
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let vibrant_border_style = Style::default().fg(Color::Rgb(50, 200, 50));

    let menu_items = [
        "Check balance",
        "Send transaction",
        "Create new address",
        "View transaction history",
        "Exit",
    ];

    let mut active_menu_item = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let menu_block = Block::default()
                .title(Spans::from(Span::styled(
                    "Rustchain MVP - Wallet",
                    vibrant_border_style,
                )))
                .borders(Borders::ALL)
                .border_style(vibrant_border_style);

            let menu_items: Vec<ListItem> = menu_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == active_menu_item {
                        Style::default().add_modifier(Modifier::REVERSED)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Spans::from(Span::styled(*item, style)))
                })
                .collect();

            let menu_widget = List::new(menu_items)
                .block(menu_block)
                .highlight_style(Style::default().bg(Color::DarkGray));
            f.render_widget(menu_widget, layout[0]);
        })?;

        // Handle input
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => {
                        if active_menu_item > 0 {
                            active_menu_item -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if active_menu_item < menu_items.len() - 1 {
                            active_menu_item += 1;
                        }
                    }
                    KeyCode::Enter => {
                        // Perform the action based on the selected menu item
                        match active_menu_item {
                            0 => println!("Checking balance..."),
                            1 => println!("Sending transaction..."),
                            2 => println!("Creating new address..."),
                            3 => println!("Viewing transaction history..."),
                            4 => break,
                            _ => {}
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Clean up and exit
    disable_raw_mode()?;
    Ok(())
}
