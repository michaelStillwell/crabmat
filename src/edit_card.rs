use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Clear},
    Frame,
};
use tui_textarea::{Input, Key};

use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    kanban::Card,
    ui::centered_rect,
};

pub fn render_card_screen(f: &mut Frame, app: &mut App, is_new: bool) {
    let popup_block = Block::default()
        .title(format!("Editing {}card", if is_new { "new " } else { "" }))
        .borders(Borders::ALL)
        .style(Style::default());

    let area = centered_rect(60, 25, f.size());
    f.render_widget(Clear, area);
    f.render_widget(popup_block, area);

    let card_editor = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let style = Style::default().fg(Color::White);

    let left_block = Block::default().borders(Borders::RIGHT).style(style);
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Min(1)])
        .split(card_editor[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(card_editor[1]);

    let title_block = Block::default().borders(Borders::BOTTOM).style(style);
    let description_block = Block::default().borders(Borders::NONE).style(style);

    app.set_title_block(title_block);
    app.set_description_block(description_block);

    f.render_widget(app.title_input.widget(), left_layout[0]);
    f.render_widget(app.description_input.widget(), left_layout[1]);
    f.render_widget(Text::from("Checklist TBD"), right_layout[0]);

    f.render_widget(left_block, card_editor[0]);
}

pub fn events(key: KeyEvent, is_new: bool, app: &mut App) {
    if let Some(editing) = &app.currently_editing {
        match Input::from(key) {
            Input {
                key: Key::Char('o') | Key::Char('O'),
                ..
            } if app.vim.is_normal()
                && !matches!(app.currently_editing, Some(CurrentlyEditing::Description)) => {}
            Input {
                key: Key::Enter, ..
            } if app.vim.is_insert()
                && !matches!(app.currently_editing, Some(CurrentlyEditing::Description)) => {}
            Input {
                key: Key::Char('s') | Key::Enter,
                ..
            } if app.vim.is_normal() => {
                if is_new {
                    app.kanban.add_card(
                        app.selected_column,
                        Card::new(
                            &app.title_input.lines().concat(),
                            &app.description_input.lines().join("\t\t\t\n"),
                        ),
                    );
                } else {
                    app.kanban.set_card_title(
                        app.selected_column,
                        app.selected_card,
                        &app.title_input.lines().join("\n"),
                    );
                    app.kanban.set_card_description(
                        app.selected_column,
                        app.selected_card,
                        &app.description_input.lines().join("\n"),
                    );
                }
                app.save_kanban();
                app.current_screen = CurrentScreen::Main;
            }
            Input {
                key: Key::Char('q') | Key::Esc,
                ..
            } if app.vim.is_normal() => app.stop_edit(),
            Input {
                key: Key::Char('j'),
                ctrl: true,
                ..
            } if app.vim.is_normal() && matches!(editing, CurrentlyEditing::Title) => {
                app.update_vim(Input::from(key));
                app.edit_description();
            }
            Input {
                key: Key::Char('k'),
                ctrl: true,
                ..
            } if app.vim.is_normal() && matches!(editing, CurrentlyEditing::Description) => {
                app.edit_title();
            }
            _ => {
                app.update_vim(Input::from(key));
            }
        }
    }
}
