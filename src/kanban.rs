use std::fmt::Display;

use crate::io::{read_kanban, save_kanban};

#[derive(Debug, Clone)]
pub struct Card {
    pub title: String,
    pub description: String,
    // pub checklist: Vec<Check>,
}

impl Card {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            // checklist: Vec::new(),
        }
    }

    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            // checklist: Vec::new(),
        }
    }

    pub fn from(title: &str, description: &str) -> Self {
        let card = Card::new(title, description);

        // for check in checklist {
        //     card.add_check(&check.title, check.done);
        // }

        card
    }

    // pub fn add_check(&mut self, title: &str, checked: bool) {
    //     self.checklist.push(Check::new(title, checked));
    // }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub title: String,
    pub cards: Vec<Card>,
}

impl Column {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            cards: Vec::new(),
        }
    }

    pub fn new(title: &str, items: Vec<Card>) -> Self {
        Self {
            title: title.to_string(),
            cards: items,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

pub struct Kanban {
    title: String,
    columns: Vec<Column>,
}

impl Display for Kanban {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::from(self.title());

        for column in self.columns() {
            display.push_str(&format!("\n\t{}", column.title));
            for card in column.cards.iter() {
                display.push_str(&format!("\n\t\t{}", card.title));
                for line in card.description.lines() {
                    display.push_str(&format!("\n\t\t\t{}", line));
                }
                // for check in &card.checklist {
                //     display.push_str(&format!("\n\t\t\t\t{}", check));
                // }
            }
        }

        write!(f, "{}\n", display)
    }
}

impl Kanban {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            columns: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let lines = read_kanban(path)?;

        // TODO: do i want this or would it better to then prompt for a title?
        let title = if let Some(title) = lines.first() {
            title
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Empty file given.",
            ));
        };
        let mut kanban = Kanban::new(title);
        let mut col = Column::empty();
        let mut card = Card::empty();
        for line in lines.iter().skip(1) {
            let tabs = line.chars().filter(|c| c == &'\t').count();

            match tabs {
                1 => {
                    if !card.title.is_empty() {
                        col.add_card(card);
                        card = Card::empty();
                    }

                    if !col.title.is_empty() {
                        kanban.add_column(col);
                        col = Column::empty();
                    }

                    col.title = line.trim_start().to_string();
                    col.cards = Vec::new();
                }
                2 => {
                    if !card.title.is_empty() {
                        col.add_card(card);
                        card = Card::empty();
                    }

                    card.title = line.trim_start().to_string();
                }
                3 => {
                    if card.description.is_empty() {
                        card.description = format!("{}\n", line.trim_start());
                    } else {
                        card.description
                            .push_str(&format!("{}\n", line.trim_start()));
                    }
                }
                // 4 => {
                //     card.checklist.push(Check::from(line));
                // }
                _ => {}
            }
        }
        if !card.title.is_empty() {
            col.add_card(card);
        }
        if !col.title.is_empty() {
            kanban.add_column(col);
        }

        Ok(kanban)
    }

    pub fn _with_columns(title: &str, columns: Vec<Column>) -> Self {
        Self {
            title: title.to_string(),
            columns,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }

    pub fn get_column(&self, idx: usize) -> Option<&Column> {
        self.columns.get(idx)
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn swap_column(&mut self, first: usize, second: usize) {
        if self.columns().len() < first && self.columns().len() < second {
            return;
        }

        self.columns.swap(first, second);
    }

    pub fn delete_column(&mut self, idx: usize) {
        self.columns.remove(idx);
    }

    pub fn set_col_title(&mut self, column_idx: usize, title: &str) {
        if self.columns().len() < column_idx {
            return;
        }

        self.columns[column_idx].title = title.to_string();
    }

    pub fn get_card(&self, column_idx: usize, item_idx: usize) -> Option<&Card> {
        self.get_column(column_idx)?.cards.get(item_idx)
    }

    pub fn add_card(&mut self, column_idx: usize, card: Card) {
        if self.columns.len() < column_idx {
            return;
        }

        self.columns[column_idx].add_card(card);
    }

    pub fn swap_card(&mut self, column_idx: usize, first: usize, second: usize) {
        if self.columns()[column_idx].cards.len() < first
            && self.columns()[column_idx].cards.len() < second
        {
            return;
        }

        self.columns[column_idx].cards.swap(first, second);
    }

    pub fn move_card(&mut self, column_idx: usize, new_column_idx: usize, card_idx: usize) {
        if column_idx > self.columns.len() || new_column_idx > self.columns.len() {
            return;
        }

        if card_idx > self.columns[column_idx].cards.len()
            || card_idx > self.columns[new_column_idx].cards.len()
        {
            return;
        }

        let card = self.columns[column_idx].cards.swap_remove(card_idx);
        self.columns[new_column_idx].cards.push(card);
    }

    pub fn delete_card(&mut self, column_idx: usize, card_idx: usize) {
        if column_idx > self.columns.len() {
            return;
        }

        if card_idx > self.columns[column_idx].cards.len() {
            return;
        }

        let mut cards = Vec::new();
        for (col_idx, col) in self.columns.iter_mut().enumerate() {
            for (car_idx, card) in col.cards.iter_mut().enumerate() {
                if col_idx == column_idx && car_idx == card_idx {
                    continue;
                }
                cards.push(card.clone());
            }
        }
        self.columns[column_idx].cards = cards;
    }

    pub fn set_card_title(&mut self, column_idx: usize, item_idx: usize, title: &str) {
        if column_idx > self.columns.len() {
            return;
        }

        if item_idx > self.columns[column_idx].cards.len() {
            return;
        }

        self.columns[column_idx].cards[item_idx].title = title.to_string();
    }

    pub fn set_card_description(&mut self, column_idx: usize, item_idx: usize, description: &str) {
        if column_idx > self.columns.len() {
            return;
        }

        if item_idx > self.columns[column_idx].cards.len() {
            return;
        }

        self.columns[column_idx].cards[item_idx].description = description.to_string();
    }

    // TODO: save to file
    pub fn save(&self, path: &str) {
        let _ = save_kanban(self, path);
    }
}
