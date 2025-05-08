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
    }

    pub fn clear_at_cursor() {
        execute!(
            std::io::stdout(),
            MoveLeft(1)
        );
        print!(" ");
    }
}
