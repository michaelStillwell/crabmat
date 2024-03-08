use ratatui::{
    style::{Modifier, Style},
    widgets::Block,
};
use tui_textarea::{Input, TextArea};

use crate::{
    kanban::{Card, Column, Kanban},
    vim::{Mode, Transition, Vim},
};

pub enum CurrentScreen {
    Main,
    Card(bool),
    Col(bool),
    DeleteCard(Card),
    DeleteCol(Column),
}

pub enum CurrentlyEditing {
    Title,
    Description,
}

pub struct App {
    pub path: String,
    pub vim: Vim,
    pub title_input: TextArea<'static>,
    pub description_input: TextArea<'static>,
    pub kanban: Kanban,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub columns_start: usize,
    pub columns_offset: usize,
    pub selected_column: usize,
    pub selected_card: usize,
    pub selected_check: usize,
}

impl App {
    pub fn new(path: String, kanban: Kanban) -> App {
        let mut title_input = TextArea::default();
        title_input.set_style(Style::default());
        title_input.set_cursor_style(Style::default());
        title_input.set_cursor_line_style(Style::default());
        let mut description_input = TextArea::default();
        description_input.set_style(Style::default());
        description_input.set_cursor_style(Style::default());
        description_input.set_cursor_line_style(Style::default());

        App {
            kanban,
            path,
            vim: Vim::new(Mode::Normal),
            title_input,
            description_input,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            columns_start: 0,
            columns_offset: 3,
            selected_column: 0,
            selected_card: 0,
            selected_check: 0,
        }
    }

    pub fn save_kanban(&mut self) {
        self.kanban.save(&self.path);

        self.title_input = TextArea::default();
        self.description_input = TextArea::default();
        self.currently_editing = None;
    }

    pub fn stop_edit(&mut self) {
        self.title_input.select_all();
        self.title_input.cut();
        self.description_input.select_all();
        self.description_input.cut();
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
        self.title_input.set_cursor_line_style(Style::default());
        self.title_input.set_cursor_style(Style::default());
        self.description_input
            .set_cursor_line_style(Style::default());
        self.description_input.set_cursor_style(Style::default());
    }

    pub fn edit_title(&mut self) {
        self.currently_editing = Some(CurrentlyEditing::Title);
        self.title_input
            .set_style(Style::default().add_modifier(Modifier::BOLD));
        self.title_input.set_cursor_line_style(Style::default());
        self.title_input
            .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        self.description_input.set_style(Style::default());
        self.description_input
            .set_cursor_line_style(Style::default());
        self.description_input.set_cursor_style(Style::default());
    }

    pub fn edit_description(&mut self) {
        self.currently_editing = Some(CurrentlyEditing::Description);
        self.description_input
            .set_style(Style::default().add_modifier(Modifier::BOLD));
        self.description_input
            .set_cursor_line_style(Style::default());
        self.description_input
            .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        self.title_input.set_style(Style::default());
        self.title_input.set_cursor_style(Style::default());
    }

    pub fn title_value(&self) -> String {
        self.title_input.lines().join("\n")
    }

    pub fn _description_value(&self) -> String {
        self.description_input.lines().join("\n")
    }

    pub fn set_title_block(&mut self, block: Block<'static>) {
        self.title_input.set_block(block)
    }

    pub fn set_description_block(&mut self, block: Block<'static>) {
        self.description_input.set_block(block)
    }

    pub fn increment_selected_column(&mut self) {
        if self.kanban.columns().is_empty() {
            return;
        }

        let new = self.selected_column + 1;
        if new < self.kanban.columns().len() {
            self.selected_column = new;
            if self.selected_column > self.columns_start + self.columns_offset - 1 {
                self.columns_start += 1;
            }
        } else {
            self.selected_column = 0;
            self.columns_start = 0;
        }

        if let Some(column) = self.kanban.columns().get(self.selected_column) {
            if column.cards.is_empty() {
                return;
            }

            if self.selected_card > column.cards.len() - 1 {
                self.selected_card = column.cards.len() - 1;
            }
        }
    }

    pub fn decrement_selected_column(&mut self) {
        if self.kanban.columns().is_empty() {
            return;
        }

        if self.selected_column == 0 {
            self.selected_column = self.kanban.columns().len() - 1;
            self.columns_start = self.kanban.columns().len() - self.columns_offset;
        } else {
            self.selected_column -= 1;
            if self.columns_start > 0 && self.selected_column == self.columns_start - 1 {
                self.columns_start -= 1;
            }
        }

        if let Some(column) = self.kanban.columns().get(self.selected_column) {
            if column.cards.is_empty() {
                return;
            }

            if self.selected_card > column.cards.len() - 1 {
                self.selected_card = column.cards.len() - 1;
            }
        }
    }

    pub fn increment_selected_card(&mut self) {
        if self.kanban.columns().is_empty() {
            return;
        }

        if let Some(column) = self.kanban.columns().get(self.selected_column) {
            if column.cards.is_empty() {
                return;
            }

            if self.selected_card == 0 {
                self.selected_card = column.cards.len() - 1;
            } else {
                self.selected_card = self.selected_card - 1;
            }
        }
    }

    pub fn decrement_selected_card(&mut self) {
        if self.kanban.columns().is_empty() {
            return;
        }

        if let Some(column) = self.kanban.columns().get(self.selected_column) {
            if column.cards.is_empty() {
                return;
            }

            let new = self.selected_card + 1;
            if new < column.cards.len() {
                self.selected_card = new;
            } else {
                self.selected_card = 0;
            }
        }
    }

    pub fn update_vim(&mut self, key: Input) {
        if let Some(editing) = &self.currently_editing {
            let input = if matches!(editing, CurrentlyEditing::Title) {
                &mut self.title_input
            } else {
                &mut self.description_input
            };
            let vim = Vim::new(self.vim.mode);
            self.vim = match self.vim.transition(Input::from(key), input) {
                Transition::Mode(mode) if vim.mode != mode => Vim::new(mode),
                Transition::Nop | Transition::Mode(_) => vim,
                Transition::Pending(input) => vim.with_pending(input),
                Transition::Quit => vim,
            };
        }
    }
}
