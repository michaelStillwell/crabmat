use std::fmt::Display;

use ratatui::text::Line;
use tui_textarea::TextArea;

#[derive(Debug, Clone)]
pub struct Check {
    pub title: String,
    pub done: bool,
}

impl Check {
    pub fn new(title: &str, done: bool) -> Self {
        Self {
            title: title.to_string(),
            done,
        }
    }

    pub fn empty() -> Self {
        Self {
            title: String::new(),
            done: false,
        }
    }
}

impl From<Line<'_>> for Check {
    fn from(line: Line) -> Self {
        Check::from(line.to_string())
    }
}

impl From<TextArea<'_>> for Check {
    fn from(input: TextArea<'_>) -> Self {
        Check::from(input.lines().join("\n"))
    }
}

impl From<&TextArea<'_>> for Check {
    fn from(input: &TextArea<'_>) -> Self {
        Check::from(input.lines().join("\n"))
    }
}

impl From<String> for Check {
    fn from(value: String) -> Self {
        let mut check = Check::empty();
        let (done, title) = value.trim_start().split_at(4);
        check.done = done == "[x] ";
        check.title = title.to_string();

        check
    }
}

impl From<&String> for Check {
    fn from(value: &String) -> Self {
        let mut check = Check::empty();
        let (done, title) = value.trim_start().split_at(4);
        check.done = done == "[x] ";
        check.title = title.to_string();

        check
    }
}

impl Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", if self.done { "x" } else { " " }, self.title)
    }
}
