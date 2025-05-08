//use screen_13_window::{Window, WindowError};
use crate::{
    opcodes::*,
    memory::*
};

struct GPU {
}

impl GPU {
    /*pub fn init() {
        pretty_env_logger::init();

        Window::new()?.run(|frame| {
            frame
                .render_graph
        })
    }
    */
    pub fn write_letter(address: u16, payload: u16, memory: &mut Memory) {
        let letter = memory.data[address as usize]
        print!("{}", )
    }
}
