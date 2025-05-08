use crate::crossterm::event::{self};

pub fn read_event() -> event::KeyCode {
    loop{
        let mut key_code = event::KeyCode::Char(' ');
        if let Ok(event::Event::Key(key)) = event::read() {
            if key.kind == event::KeyEventKind::Press {
                key_code = key.code;
            }
        }
        if key_code == event::KeyCode::Enter {
            continue;
        }
        return key_code;
    }
}
