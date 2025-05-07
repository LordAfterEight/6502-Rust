use crate::opcodes::*;
use colored::{Colorize, ColoredString};
use std::io::Write;
type Byte = u8;
type Word = u16;
static MAX_MEM: u32 = 1024 * 64;

const CLOCK_SPEED: u64 = 1 / 1; // Second number is desired frequency in Hz


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

    pub fn dumps(&mut self) {
        let mut file = std::fs::File::options().write(true).open("hex.bin").expect("Unable to open file");
        for value in self.data {
            write!(&mut file, "{:#06X}  ", value).expect("Unable to write to file");
        }
        println!("{}","[i] Dumped memory".yellow());
    }
}

pub struct CPU {
    pub program_counter: Word,
    pub stack_pointer: Word,

    // Registers
    pub accumulator: Word,
    pub idx_reg_x: Word,
    pub idx_reg_y: Word,

    pub processor_status: Byte,

    // Flags
    pub carry_flag: bool,
    pub zero_flag:  bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow_flag: bool,
    pub negative_flag: bool
}

impl CPU {

    pub fn reset(&mut self) {
        // Set addresses
        self.program_counter = 0xFFFC;
        self.stack_pointer = 0x0100; // stack location: 0x0100 - 0x01FF

        // Set values
        self.accumulator = 0;
        self.idx_reg_x = 0;
        self.idx_reg_y = 0;

        // Set all flags to 0
        self.carry_flag = false;
        self.zero_flag = false;
        self.interrupt_disable = false;
        self.decimal_mode = false;
        self.break_command = false;
        self.overflow_flag = false;
        self.negative_flag = false;
    }

    pub fn error_loop(
        &mut self,
        error: &str,
        line: u32,
        cycles: u32,
        memory: &mut Memory
    ) {
        //memory.dump();
        println!("\n{} Line:{} | Cycle:{}\n{} {}\n\n{}",
            "[!] Entered safe loop at:".truecolor(200,100,0),
            line,
            cycles,
            "[i] Reason:".yellow(),
            error,
            "Press CTRL + C to exit".cyan()
        );
        println!("PC: {:#06X}", self.program_counter);
        loop {
        }
    }

    pub fn write_byte(&mut self, value: Word, cycles: &mut u32, address: Word, memory: &mut Memory) {
        memory.data[address as usize] = value;
        *cycles += 1;
    }

    pub fn write_word(&mut self, value: Word, cycles: &mut u32, address: Word, memory: &mut Memory) {
        memory.data[address as usize]     = value & 0xFF;
        memory.data[address as usize + 1] = value >> 8;
        *cycles += 2;
    }

    pub fn set_zero_and_negative_flags(&mut self, register: Word) {
        self.zero_flag = register == 0;
        self.negative_flag = (register & 0b10000000) > 0;
    }

    // Fetches a word from the PC address and returns it
    pub fn fetch_word(&mut self, cycles: &mut u32, memory: &mut Memory) -> Word {
        // First Byte
        let mut data = memory.data[(self.program_counter << 8) as usize];
        if self.program_counter == u16::MAX {
            self.program_counter = 0x0000;
        }
        self.program_counter += 1;

        // Second Byte
        data |= memory.data[self.program_counter as usize];
        if self.program_counter == u16::MAX {
            self.program_counter = 0x0000;
        }
        self.program_counter += 1;
        *cycles += 2;
        return data
    }

    // Fetches a byte from the PC address and returns it
    pub fn fetch_byte(&mut self, cycles: &mut u32, memory: &Memory) -> Word {
        let data = memory.data[self.program_counter as usize];
        println!("Fetched instruction {:#06X} at {:#06X}", &data, self.program_counter);
        std::thread::sleep(std::time::Duration::from_secs(CLOCK_SPEED));
        if self.program_counter == u16::MAX {
            self.program_counter = 0x0000;
        }
        self.program_counter += 1;
        *cycles += 1;
        return data
    }

    // Reads a byte from the PC address and returns it without increasing the PC
    pub fn read_byte(&mut self, cycles: &mut u32, address: Word, memory: &Memory) -> Word  {
        let data = memory.data[address as usize];
        *cycles += 1;
        return data.try_into().unwrap()
    }

    pub fn read_word(&mut self, cycles: &mut u32, address: Word, memory: &Memory) -> Word  {
        let lo_byte: Word = self.read_byte(cycles, address, memory) as u16;
        let hi_byte: Word = self.read_byte(cycles, address+1, memory) as u16;
        *cycles += 1;
        let return_value: Word = lo_byte | (hi_byte << 8);
        return return_value
    }

    pub fn stack_pointer_as_address(&mut self) -> Word {
        let return_value: Word = 0x100 | self.stack_pointer as u16;
        return return_value
    }

    pub fn push_word_to_stack(&mut self, cycles: &mut u32, memory: &mut Memory, value: Word) {
        let sp_as_addr: Word = self.stack_pointer_as_address();
        self.write_byte(value >> 8, cycles, sp_as_addr, memory);
        self.write_byte(value & 0xFF, cycles, sp_as_addr, memory);
    }

    // Executes an instruction
    pub fn execute(&mut self, mut cycles: u32, mut memory: &mut Memory) {
        while cycles > 0 {

            println!("\nFetching instruction...");
            std::thread::sleep(std::time::Duration::from_secs(CLOCK_SPEED));
            let data = self.fetch_byte(&mut cycles, memory);

            match data {
                INS_FORCE_INTERRUPT => {
                    self.push_word_to_stack(
                        &mut cycles,
                        memory,
                        self.processor_status.try_into().unwrap()
                    );
                    self.program_counter = self.read_word(&mut cycles, 0xFFFE, &memory);
                    self.interrupt_disable = true;
                    self.break_command = true;
                    println!("Forced interrupt");
                    self.error_loop("Forced interrupt", 205, cycles, memory);
                },

                INS_NO_OPERATION => {
                    println!("Doing nothing...\n");
                    if self.program_counter == u16::MAX {
                        self.program_counter = 0x0000;
                    }
                    cycles += 1;
                    continue;
                },

                INS_JUMP_ABSOLUTE => {
                    let address = self.fetch_byte(&mut cycles, &memory);
                    self.program_counter = address;
                    println!("Jumped to address {:#06X}\n", self.program_counter);
                    cycles += 1;
                },

                INS_JUMP_TO_SUBROUTINE => {
                    let sub_address: Word = self.fetch_byte(&mut cycles, &mut memory);
                    self.write_byte(self.program_counter-1, &mut cycles, self.stack_pointer, &mut memory);
                    println!("Jumping from {:#06X} to subroutine at {:#06X}",
                        self.program_counter,
                        sub_address
                    );
                    self.program_counter = sub_address;
                    if self.stack_pointer == u16::MAX {
                        println!("Stack pointer would be out of bounds, stopping...\n");
                        self.error_loop("Stack pointer out of bounds", 229, cycles, memory);
                    }
                    self.stack_pointer += 1;
                    cycles += 1;
                },

                INS_RETURN_FROM_SUBROUTINE => {
                    self.program_counter = self.read_byte(&mut cycles, self.stack_pointer - 1, &mut memory) + 1;
                    println!("Returned to {:#06X} from subroutine", self.program_counter);
                    if self.stack_pointer == u16::MAX {
                        println!("Stack pointer would be out of bounds, stopping...\n");
                        self.error_loop("Stack pointer out of bounds", 303, cycles, memory);
                    }
                    self.stack_pointer += 1;
                    cycles += 1;

                },

                INS_LOAD_ACCUMULATOR_IMMEDIATE => {
                    let value: Word = self.fetch_byte(&mut cycles, memory);
                    self.accumulator = value;
                    self.set_zero_and_negative_flags(self.accumulator);
                    println!("Loaded value {:#06X} into register A\n",
                        value
                    );
                },

                INS_LOAD_ACCUMULATOR_ZERO_PAGE => {
                    let zero_page_address: Word = self.fetch_byte(&mut cycles, memory);
                    self.accumulator = self.read_byte(&mut cycles, zero_page_address.into(), memory);
                    self.set_zero_and_negative_flags(self.accumulator);
                    println!("Loaded value {:#06X} into register A\n",
                        zero_page_address
                    );
                },

                INS_LOAD_ACCUMULATOR_ZERO_PAGE_X => {
                    let mut zero_page_address: Word = self.fetch_byte(&mut cycles, memory);
                    zero_page_address += self.idx_reg_x;
                    cycles += 1;
                    self.accumulator = self.read_byte(&mut cycles, zero_page_address.into(), memory);
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                INS_LOAD_X_REGISTER_IMMEDIATE => {
                    let value: Word = self.fetch_byte(&mut cycles, memory);
                    self.idx_reg_x = value;
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                INS_LOAD_Y_REGISTER_IMMEDIATE => {
                    let value: Word = self.fetch_byte(&mut cycles, memory);
                    self.idx_reg_y = value;
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                INS_STORE_ACCUMULATOR_ZERO_PAGE => {
                    let zero_page_address: Word = self.fetch_byte(&mut cycles, memory) as u16;
                    memory.data[zero_page_address as usize] = self.accumulator as u16;
                    cycles += 1;
                    self.set_zero_and_negative_flags(self.accumulator);
                    println!("Stored register A content to address {:#06X}\n",
                        zero_page_address
                    );
                },

                INS_STORE_X_REGISTER_ZERO_PAGE => {
                    let zero_page_address: Word = self.fetch_byte(&mut cycles, memory) as u16;
                    memory.data[zero_page_address as usize] = self.idx_reg_x as u16;
                    cycles += 1;
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                INS_STORE_Y_REGISTER_ZERO_PAGE => {
                    let zero_page_address: Word = self.fetch_byte(&mut cycles, memory) as u16;
                    memory.data[zero_page_address as usize] = self.idx_reg_y as u16;
                    cycles += 1;
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                _ => println!("Invalid opcode: {:#06X}", &data)
            };
            std::thread::sleep(std::time::Duration::from_secs(CLOCK_SPEED));
        }
        println!("Finished executing all instructions in {} cycles", cycles);
    }
}
