use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app::{App, CurrentScreen},
    delete_card::render_delete_card,
    delete_col::render_delete_col,
    edit_card::render_card_screen,
    edit_col::render_edit_col,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_layout = Layout::horizontal([Constraint::Min(1), Constraint::Max(7)])
        .flex(Flex::SpaceBetween)
        .split(chunks[0]);
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        app.kanban.title(),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, title_layout[0]);
    let cols_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let cols = Paragraph::new(Text::styled(
        format!(
            "{}/{}",
            if app.kanban.columns().len() == 0 {
                0
            } else {
                app.selected_column + 1
            },
            app.kanban.columns().len()
        ),
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Right)
    .block(cols_block);

    f.render_widget(cols, title_layout[1]);

    let column_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
        .split(chunks[1]);

    for (i, column) in app
        .kanban
        .columns()
        .iter()
        .skip(app.columns_start)
        .take(app.columns_offset)
        .enumerate()
    {
        let title = if column.title.is_empty() {
            "Column"
        } else {
            &column.title
        };
        let is_column_selected = i + app.columns_start == app.selected_column;
        let style = if is_column_selected {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let column_block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(style);
        let mut items = Vec::<ListItem>::new();

        if column.cards.len() == 0 {
            items.push(ListItem::new(
                Line::from("c to create card").alignment(Alignment::Center),
            ));
        }

        for (j, card) in column.cards.iter().enumerate() {
            // NOTE: ew, change colors
            let style = if is_column_selected && app.selected_card == j {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            items.push(ListItem::new(Line::from(Span::styled(
                card.title.clone(),
                style,
            ))));
        }

        let list = List::new(items).block(column_block);

        f.render_widget(list, column_layout[i]);
    }

    let current_navigation_text = vec![
        match app.vim.mode {
            crate::vim::Mode::Normal => {
                Span::styled("NORMAL", Style::default().fg(Color::DarkGray))
            }
            crate::vim::Mode::Insert => {
                Span::styled("INSERT", Style::default().fg(Color::LightBlue))
            }
            crate::vim::Mode::Visual => {
                Span::styled("VISUAL", Style::default().fg(Color::LightRed))
            }
            crate::vim::Mode::Operator(_) => {
                Span::styled("OPERATOR", Style::default().fg(Color::LightGreen))
            }
        },
        Span::styled(" | ", Style::default().fg(Color::White)),
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("Viewing Board", Style::default().fg(Color::DarkGray))
            }
            CurrentScreen::Card(is_new) => {
                if is_new {
                    Span::styled("Editing new card", Style::default().fg(Color::Yellow))
                } else {
                    Span::styled("Editing card", Style::default().fg(Color::Yellow))
                }
            }
            CurrentScreen::Col(is_new) => {
                if is_new {
                    Span::styled("Editing new column", Style::default().fg(Color::Yellow))
                } else {
                    Span::styled("Editing column", Style::default().fg(Color::Yellow))
                }
            }
            CurrentScreen::DeleteCol(_) => {
                Span::styled("Deleting column", Style::default().fg(Color::Red))
            }
            CurrentScreen::DeleteCard(_) => {
                Span::styled("Deleting card", Style::default().fg(Color::Red))
            }
        }
        .to_owned(),
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);

    match &app.current_screen {
        CurrentScreen::Main => {}
        CurrentScreen::Card(is_new) => render_card_screen(f, app, *is_new),
        CurrentScreen::Col(is_new) => render_edit_col(f, app, *is_new),
        CurrentScreen::DeleteCard(card_title) => render_delete_card(f, app, card_title.clone()),
        CurrentScreen::DeleteCol(column) => render_delete_col(f, app, column.clone()),
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
