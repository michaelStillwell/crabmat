use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
    Frame,
};
use tui_textarea::{Input, Key};

use crate::{
    app::{App, CurrentScreen},
    kanban::Column,
    ui::centered_rect,
};

pub fn render_edit_col(f: &mut Frame, app: &mut App, is_new: bool) {
    let popup_block = Block::default()
        .title(if is_new {
            "New column"
        } else {
            "Editing column"
        })
        .borders(Borders::ALL)
        .style(Style::default());

    let area = centered_rect(60, 25, f.size());
    f.render_widget(Clear, area);
    f.render_widget(popup_block, area);

    let col_editor = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(100)])
        .split(area);

    let style = Style::default().fg(Color::White);

    let left_block = Block::default().borders(Borders::NONE).style(style);
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(col_editor[0]);

    let title_block = Block::default().borders(Borders::NONE).style(style);

    app.set_title_block(title_block);

    f.render_widget(app.title_input.widget(), left_layout[0]);

    f.render_widget(left_block, col_editor[0]);
}

// TODO: remove repeated code for editors
pub fn events(key: KeyEvent, is_new: bool, app: &mut App) {
    if let Some(_editing) = &app.currently_editing {
        match Input::from(key) {
            Input {
                key: Key::Char('o') | Key::Char('O'),
                ..
            } if app.vim.is_normal() => {}
            Input {
                key: Key::Enter, ..
            } if app.vim.is_insert() => {}
            Input {
                key: Key::Char('s') | Key::Enter,
                ..
            } if app.vim.is_normal() => {
                if is_new {
                    app.kanban
                        .add_column(Column::new(&app.title_input.lines().concat(), Vec::new()));
                } else {
                    app.kanban
                        .set_col_title(app.selected_column, &app.title_input.lines().join("\n"));
                }
                app.save_kanban();
                app.current_screen = CurrentScreen::Main;
            }
            Input {
                key: Key::Char('q') | Key::Esc,
                ..
            } if app.vim.is_normal() => app.stop_edit(),
            _ => {
                app.update_vim(Input::from(key));
            }
        }
    }
}
