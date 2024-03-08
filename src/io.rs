use core::panic;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use crate::kanban::Kanban;

pub fn save_kanban(kanban: &Kanban, path: &str) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    write!(file, "{}", kanban.to_string())?;
    Ok(())
}

pub fn read_kanban(path: &str) -> std::io::Result<Vec<String>> {
    let file = match OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&path)
    {
        Ok(file) => file,
        Err(e) => {
            panic!("hello {:?}", e);
        }
    };
    let buffer = BufReader::new(&file);
    let data = buffer
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    Ok(data)
}
