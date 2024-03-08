use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use tui_textarea::{Input, Key};

use crate::{
    app::{App, CurrentScreen},
    kanban::Column,
    ui::centered_rect,
};

pub fn render_delete_col(f: &mut Frame, _app: &mut App, col: Column) {
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let lines = vec![
        Line::from(Span::styled("Delete", Style::default())),
        Line::from(Span::styled(&col.title, Style::default())),
        Line::from(vec![
            Span::styled("y", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("es | ", Style::default()),
            Span::styled("n", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("o", Style::default()),
        ]),
    ];
    let text = Paragraph::new(Text::from(lines))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    let area = centered_rect(20, 20, f.size());
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

    f.render_widget(text, left_layout[0]);

    f.render_widget(left_block, col_editor[0]);
}

pub fn events(key: KeyEvent, app: &mut App) {
    match Input::from(key) {
        Input {
            key: Key::Char('s') | Key::Enter,
            ..
        } if app.vim.is_normal() => {
            app.save_kanban();
            app.current_screen = CurrentScreen::Main;
        }
        Input {
            key: Key::Char('y'),
            ..
        } => {
            app.kanban.delete_column(app.selected_column);
            app.save_kanban();
            app.decrement_selected_column();
            app.current_screen = CurrentScreen::Main;
        }
        Input {
            key: Key::Char('q') | Key::Char('n') | Key::Esc,
            ..
        } if app.vim.is_normal() => app.stop_edit(),
        _ => {
            app.update_vim(Input::from(key));
        }
    }
}
