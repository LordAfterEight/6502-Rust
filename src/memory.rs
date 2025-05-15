#![allow(unused_assignments)]
use crate::{
    MAX_MEM, Word,
    opcodes::*,
    data::*
};
use crate::colored::Colorize;
use std::io::Write;

pub struct Memory {
    pub data: [Word; MAX_MEM as usize]
}

impl Memory {
    pub fn init(&mut self) {

        let _ascii = ASCII::init();

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


        // Make CPU always jump to command line routine
        self.data[0xFFFC] = INS_JUMP_ABSOLUTE;
        self.data[0xFFFD] = 0xF000;


        // Draw chars for command line - COMMAND LINE ROUTINE STARTS HERE
        for mut i in 0xF000..0xF015 {
            self.data[i as usize] = INS_GPU_DRAW_AT_CURSOR_POSITION;
            i+=2;
        }
        self.data[0xF001] = 0xF21E; // [
        self.data[0xF003] = 0xF202; // C
        self.data[0xF005] = 0xF20F; // P
        self.data[0xF007] = 0xF214; // U
        self.data[0xF009] = 0xF21F; // ]
        self.data[0xF00B] = 0xF1FF; //
        self.data[0xF00D] = 0xF23B; // =
        self.data[0xF00F] = 0xF23D; // >
        self.data[0xF011] = 0xF1FF; //
        self.data[0xF013] = 0xF21C; // #
        self.data[0xF015] = 0xF1FF; //

        // Command line routine
        self.data[0xF016] = INS_WAIT_FOR_INPUT;
        self.data[0xF017] = INS_JUMP_ABSOLUTE;
        self.data[0xF018] = 0xF000;

        // Command line help display routine
        for mut i in 0xEF21..0xEF80 {
            self.data[i as usize] = INS_GPU_DRAW_AT_CURSOR_POSITION;
            i+=2;
        }
        for mut i in 0xEFA0..0xEFCF {
            self.data[i as usize] = INS_GPU_DRAW_AT_CURSOR_POSITION;
            i+=2;
        }

        self.data[0xEF20] = INS_GPU_MOVE_TO_NEXT_LINE;
        self.data[0xEF22] = 0xF230;
        self.data[0xEF24] = 0xF1FF;
        self.data[0xEF26] = 0xF23D;
        self.data[0xEF28] = 0xF23C;
        self.data[0xEF2A] = 0xF1FF;
        self.data[0xEF2C] = 0xF224;
        self.data[0xEF2E] = 0xF237;
        self.data[0xEF30] = 0xF228;
        self.data[0xEF32] = 0xF233;
        self.data[0xEF33] = INS_GPU_MOVE_TO_NEXT_LINE;

        self.data[0xEF35] = 0xF231;
        self.data[0xEF37] = 0xF1FF;
        self.data[0xEF39] = 0xF23D;
        self.data[0xEF3B] = 0xF23C;
        self.data[0xEF3D] = 0xF1FF;
        self.data[0xEF3F] = 0xF231;
        self.data[0xEF41] = 0xF224;
        self.data[0xEF43] = 0xF232;
        self.data[0xEF45] = 0xF224;
        self.data[0xEF47] = 0xF233;
        self.data[0xEF48] = INS_GPU_MOVE_TO_NEXT_LINE;

        self.data[0xEF4A] = 0xF223;
        self.data[0xEF4C] = 0xF1FF;
        self.data[0xEF4E] = 0xF23D;
        self.data[0xEF50] = 0xF23C;
        self.data[0xEF52] = 0xF1FF;
        self.data[0xEF54] = 0xF223;
        self.data[0xEF56] = 0xF234;
        self.data[0xEF58] = 0xF22C;
        self.data[0xEF5A] = 0xF22F;
        self.data[0xEF5B] = INS_GPU_MOVE_TO_NEXT_LINE;

        self.data[0xEF5D] = 0xF227;
        self.data[0xEF5F] = 0xF1FF;
        self.data[0xEF61] = 0xF23D;
        self.data[0xEF63] = 0xF23C;
        self.data[0xEF65] = 0xF1FF;
        self.data[0xEF67] = 0xF227;
        self.data[0xEF69] = 0xF224;
        self.data[0xEF6B] = 0xF22B;
        self.data[0xEF6D] = 0xF22F;
        self.data[0xEF6E] = INS_GPU_MOVE_TO_NEXT_LINE;

        self.data[0xEF70] = 0xF228;
        self.data[0xEF72] = 0xF1FF;
        self.data[0xEF74] = 0xF23D;
        self.data[0xEF76] = 0xF23C;
        self.data[0xEF78] = 0xF1FF;
        self.data[0xEF7A] = 0xF228;
        self.data[0xEF7C] = 0xF22D;
        self.data[0xEF7E] = 0xF225;
        self.data[0xEF80] = 0xF22E;
        self.data[0xEF81] = INS_GPU_MOVE_TO_NEXT_LINE;
        self.data[0xEF82] = INS_GPU_MOVE_TO_NEXT_LINE;
        self.data[0xEF83] = INS_JUMP_ABSOLUTE;
        self.data[0xEF84] = 0xF000;

        // Command line invalid command
        self.data[0xEFA1] = 0xF208;
        self.data[0xEFA3] = 0xF22D;
        self.data[0xEFA5] = 0xF235;
        self.data[0xEFA7] = 0xF220;
        self.data[0xEFA9] = 0xF22B;
        self.data[0xEFAB] = 0xF228;
        self.data[0xEFAD] = 0xF223;
        self.data[0xEFAF] = 0xF1FF;
        self.data[0xEFB1] = 0xF222;
        self.data[0xEFB3] = 0xF22E;
        self.data[0xEFB5] = 0xF22C;
        self.data[0xEFB7] = 0xF22C;
        self.data[0xEFB9] = 0xF220;
        self.data[0xEFBB] = 0xF22D;
        self.data[0xEFBD] = 0xF223;
        self.data[0xEFBF] = 0xF1FF;
        self.data[0xEFC1] = 0xF24C;
        self.data[0xEFC3] = 0xF1FF;
        self.data[0xEFC5] = 0xF233;
        self.data[0xEFC7] = 0xF238;
        self.data[0xEFC9] = 0xF22F;
        self.data[0xEFCB] = 0xF224;
        self.data[0xEFCD] = 0xF1FF;
        self.data[0xEFCF] = 0xF227;
        self.data[0xEFD0] = INS_GPU_MOVE_TO_NEXT_LINE;
        self.data[0xEFD1] = INS_GPU_MOVE_TO_NEXT_LINE;
        self.data[0xEFD2] = INS_JUMP_ABSOLUTE;
        self.data[0xEFD3] = 0xF000;
    }

    pub fn dump(&mut self) {
        let mut out = 0;
        for _y in 0..0xFFFF/8+1 {
            for x in 0..8 {
                let string = String::from(format!(
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
            write!(&mut file, "{:#06X} ", value).expect("Unable to write to file");
        }
        println!("{}","[i] Dumped memory".yellow());
    }
}
