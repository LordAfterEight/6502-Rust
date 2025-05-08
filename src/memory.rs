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

        self.data[0xF1FF] = 0x0020;

        // Write letters A - Z (ALL CAPITAL) to addresses 0xF200 - 0xF219
        let mut increment = 0;
        for i in 0xF200..0xF21A {
            self.data[i as usize] = 0x41+increment;
            increment += 1;
        }
        // Write letters a - z (all lowercase) to addresses 0xF230 - 0xF249
        increment = 0;
        for i in 0xF220..0xF23A {
            self.data[i as usize] = 0x61+increment;
            increment += 1;
        }

        // Write numbers 0 - 9 to addresses 0xF260 - 0xF269
        increment = 0;
        for i in 0xF240..0xF24A {
            self.data[i as usize] = 0x0030+increment;
            increment += 1;
        }

        self.data[0xF21A] = 0x0021; // !
        self.data[0xF21B] = 0x0022; // "
        self.data[0xF21C] = 0x0023; // #
        self.data[0xF21D] = 0x0024; // $
        self.data[0xF21E] = 0x005B; // [
        self.data[0xF21F] = 0x005D; // ]

        self.data[0xF23A] = 0x005C; // /
        self.data[0xF23B] = 0x003C; // <
        self.data[0xF23C] = 0x003E; // >
        self.data[0xF23D] = 0x003D; // =
        self.data[0xF23E] = 0x002D; // -
        self.data[0xF23F] = 0x007E; // ~

        self.data[0xF24A] = 0x003A; // :
        self.data[0xF24B] = 0x005F; // _
        self.data[0xF24C] = 0x007C; // |
        self.data[0xF24D] = 0x0026; // &
        self.data[0xF24E] = 0x003F; // ?
        self.data[0xF24F] = 0x0040; // @
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
