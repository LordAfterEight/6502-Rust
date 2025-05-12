#![allow(unused_imports)]
use crate::{
    opcodes::*,
    memory::*,
    crossterm::{
        cursor::{
            MoveTo,
            MoveLeft,
            MoveRight,
            MoveToPreviousLine,
            MoveToNextLine,
            position
        },
        terminal::{
            ScrollUp,
            ScrollDown
        },
        ExecutableCommand,
        execute
    }
};
use std::io::Write;

pub struct GPU {
}

impl GPU {
    pub fn init(&self) {
        execute!(
            std::io::stdout(),
            MoveTo(0,0)
        );
    }

    pub fn write_letter(&self, address: u16, memory: &mut Memory) {
        let letter: u8 = memory.data[address as usize].try_into().unwrap();
        print!("{}", char::from(letter));
        _ = std::io::stdout().flush();
    }

    pub fn clear_at_cursor(&self) {
        print!(" ");
        _ = std::io::stdout().flush();
    }

    pub fn move_down(&self, lines: u16) {
        execute!(
            std::io::stdout(),
            MoveToNextLine(lines)
        );
        _ = std::io::stdout().flush();
    }

    pub fn scroll_up(&self, lines: u16) {
        execute!(
            std::io::stdout(),
            ScrollUp(lines)
        );
        std::io::stdout().flush();
    }

    pub fn move_to_next_line(&self, lines: u16) {
        execute!(
            std::io::stdout(),
            MoveToNextLine(lines)
        );
        std::io::stdout().flush();
    }

    pub fn update(&self) {
        let (cursor_x, cursor_y) = position().unwrap();
        if cursor_y >= 30 {
            self.scroll_up(1);
            _ = std::io::stdout().flush();
        }
    }
}
