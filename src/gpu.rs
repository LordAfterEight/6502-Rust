#![allow(unused_imports)]
use crate::{
    opcodes::*,
    memory::*,
    crossterm::{
        cursor::{
            MoveLeft,
            MoveRight,
            MoveToPreviousLine,
            MoveToNextLine
        },
        ExecutableCommand,
        execute
    }
};
use std::io::Write;

pub struct GPU {
}

impl GPU {
    /*pub fn init() {
        pretty_env_logger::init();

        Window::new()?.run(|frame| {
            frame
                .render_graph
        })
    }*/

    pub fn write_letter(&self, address: u16, memory: &mut Memory) {
        let letter: u8 = memory.data[address as usize].try_into().unwrap();
        print!("{}", char::from(letter));
        _ = std::io::stdout().flush();
    }

    pub fn clear_at_cursor(&self) {
        execute!(
            std::io::stdout(),
            MoveLeft(1)
        );
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
}
