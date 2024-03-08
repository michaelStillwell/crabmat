use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_textarea::{CursorMove, Input, TextArea};

use crate::{
    app::{App, CurrentScreen},
    delete_card, delete_col, edit_card, edit_col,
};

pub fn handle_events(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Release {
            return Ok(true);
        }
        match app.current_screen {
            CurrentScreen::Main => match key.code {
                KeyCode::Char('w') => {
                    println!("kanban: {}", app.kanban);
                }
                KeyCode::Char('q') => {
                    return Ok(true);
                }

                KeyCode::Char('h') | KeyCode::Right => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        if app.selected_column > 0 {
                            app.kanban
                                .swap_column(app.selected_column - 1, app.selected_column);
                            app.save_kanban();
                            app.decrement_selected_column();
                        }
                    } else {
                        app.decrement_selected_column();
                    }
                }
                KeyCode::Char('l') | KeyCode::Left => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        if app.kanban.columns().len() > 0
                            && app.selected_column < app.kanban.columns().len() - 1
                        {
                            app.kanban
                                .swap_column(app.selected_column, app.selected_column + 1);
                            app.save_kanban();
                            app.increment_selected_column();
                        }
                    } else {
                        app.increment_selected_column();
                    }
                }
                KeyCode::Char('j') | KeyCode::Up => {
                    app.decrement_selected_card();
                }
                KeyCode::Char('k') | KeyCode::Down => {
                    app.increment_selected_card();
                }

                // Move card
                KeyCode::Char('H') => {
                    if app
                        .kanban
                        .get_card(app.selected_column, app.selected_card)
                        .is_some()
                    {
                        app.kanban.move_card(
                            app.selected_column,
                            app.selected_column - 1,
                            app.selected_card,
                        );
                        app.save_kanban();
                    }
                }
                KeyCode::Char('L') => {
                    if app
                        .kanban
                        .get_card(app.selected_column, app.selected_card)
                        .is_some()
                    {
                        app.kanban.move_card(
                            app.selected_column,
                            app.selected_column + 1,
                            app.selected_card,
                        );
                        app.save_kanban();
                    }
                }
                KeyCode::Char('J') => {
                    if let Some(column) = app.kanban.get_column(app.selected_column) {
                        if app.selected_card < column.cards.len() - 1
                            && app
                                .kanban
                                .get_card(app.selected_column, app.selected_card + 1)
                                .is_some()
                        {
                            app.kanban.swap_card(
                                app.selected_column,
                                app.selected_card,
                                app.selected_card + 1,
                            );
                            app.decrement_selected_card();
                            app.save_kanban();
                        }
                    }
                }
                KeyCode::Char('K') => {
                    if app.selected_card > 0
                        && app
                            .kanban
                            .get_card(app.selected_column, app.selected_card)
                            .is_some()
                        && app
                            .kanban
                            .get_card(app.selected_column, app.selected_card - 1)
                            .is_some()
                    {
                        app.kanban.swap_card(
                            app.selected_column,
                            app.selected_card,
                            app.selected_card - 1,
                        );
                        app.increment_selected_card();
                        app.save_kanban();
                    }
                }

                // Update
                KeyCode::Char('e') | KeyCode::Enter => {
                    if let Some(card) = app.kanban.get_card(app.selected_column, app.selected_card)
                    {
                        app.title_input = TextArea::new(vec![card.title.to_string()]);
                        app.title_input.move_cursor(CursorMove::End);
                        app.description_input = TextArea::new(
                            card.description
                                .split('\n')
                                .map(|s| s.to_string())
                                .collect(),
                        );
                        app.description_input.move_cursor(CursorMove::End);

                        app.current_screen = CurrentScreen::Card(false);
                        app.edit_title();
                        app.update_vim(Input::from(KeyEvent::from(KeyCode::Char('A'))));
                    }
                }

                KeyCode::Char('E') => {
                    if let Some(col) = app.kanban.get_column(app.selected_column) {
                        app.title_input = TextArea::new(vec![col.title.to_string()]);
                        app.title_input.move_cursor(CursorMove::End);

                        app.current_screen = CurrentScreen::Col(false);
                        app.edit_title();
                    }
                }

                // Create
                KeyCode::Char('C') => {
                    app.title_input = TextArea::new(vec![]);
                    app.title_input.move_cursor(CursorMove::End);
                    app.description_input = TextArea::new(vec![]);
                    app.description_input.move_cursor(CursorMove::End);

                    app.current_screen = CurrentScreen::Col(true);
                    app.edit_title();
                    app.update_vim(Input::from(KeyEvent::from(KeyCode::Char('A'))));
                }

                KeyCode::Char('c') => {
                    if app.kanban.get_column(app.selected_column).is_some() {
                        app.title_input = TextArea::new(vec![]);
                        app.title_input.move_cursor(CursorMove::End);
                        app.description_input = TextArea::new(vec![]);
                        app.description_input.move_cursor(CursorMove::End);

                        app.current_screen = CurrentScreen::Card(true);
                        app.edit_title();
                        app.update_vim(Input::from(KeyEvent::from(KeyCode::Char('A'))));
                    }
                }

                // Delete
                KeyCode::Char('d') => {
                    if let Some(card) = app.kanban.get_card(app.selected_column, app.selected_card)
                    {
                        app.current_screen = CurrentScreen::DeleteCard(card.clone());
                    }
                }

                KeyCode::Char('D') => {
                    if let Some(col) = app.kanban.get_column(app.selected_column) {
                        app.current_screen = CurrentScreen::DeleteCol(col.clone());
                    }
                }

                _ => {}
            },
            CurrentScreen::Card(is_new) if key.kind == KeyEventKind::Press => {
                edit_card::events(key, is_new, app)
            }
            CurrentScreen::Col(is_new) if key.kind == KeyEventKind::Press => {
                edit_col::events(key, is_new, app)
            }
            CurrentScreen::DeleteCard(_) if key.kind == KeyEventKind::Press => {
                delete_card::events(key, app)
            }
            CurrentScreen::DeleteCol(_) if key.kind == KeyEventKind::Press => {
                delete_col::events(key, app)
            }

            _ => match key.code {
                _ => {}
            },
        }
    }

    Ok(false)
}
