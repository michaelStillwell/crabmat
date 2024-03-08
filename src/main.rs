#![allow(dead_code)]

use std::{error::Error, io::Write};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::handle_events;
use kanban::Kanban;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod check;
mod delete_card;
mod delete_col;
mod edit_card;
mod edit_col;
mod events;
mod io;
mod kanban;
mod ui;
mod vim;

use crate::{app::App, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).unwrap_or("kanban".to_string());
    let kanban = match Kanban::from_file(&path) {
        Ok(kanban) => kanban,
        Err(_) => {
            print!("Please enter title for board: ");
            let _ = std::io::stdout().flush();
            let mut buffer = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut buffer)?;
            Kanban::new(&buffer)
        },
    };

    enable_raw_mode()?;
    let mut stderr = std::io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(path, kanban);
    app.save_kanban();
    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> std::io::Result<()> {
    let mut stop = false;
    while !stop {
        terminal.draw(|f| ui(f, app))?;
        stop = handle_events(app)?;
    }

    Ok(())
}
