use crate::{
    MAX_MEM, Byte, Word,
    opcodes::*,
};
use crate::colored::Colorize;
use std::io::Write;

pub struct Memory {
    pub data: [Word; MAX_MEM as usize]
}

impl Memory {
    pub fn initialise(&mut self) {
        for i in 0..MAX_MEM {
            self.data[i as usize] = INS_NO_OPERATION;
        }
    }

    pub fn dump(&mut self) {
        let mut out = 0;
        for mut y in 0..0xFFFF/8+1 {
            for mut x in 0..8 {
                let mut string = String::from(format!(
                    "{:#06X}",
                    self.data[out+x as usize]
                ));
                let mut out_string = string.red();
                if self.data[out+x as usize] != 0xEA {
                    out_string = string.green();
                }
                print!("{:#06X} : {} | ",
                    out+x,
                    out_string,
                );
            }
            print!("\n");
            out+=8
        }
        println!("{}","\n[i] Dumped memory".yellow());
    }

    pub fn dump_to_file(&mut self) {
        let mut file = std::fs::File::options().write(true).open("hex.bin").expect("Unable to open file");
        for value in self.data {
            write!(&mut file, "{:#06X}  ", value).expect("Unable to write to file");
        }
        println!("{}","[i] Dumped memory".yellow());
    }
}
