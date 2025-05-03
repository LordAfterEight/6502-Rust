use crate::opcodes::*;
use colored::Colorize;
type Byte = u8;
type Word = u16;
static MAX_MEM: usize = 1024 * 64;

const CYCLES_WARNING: &str = "No cycles left, aborting execution...";

pub fn error_loop() {
    println!("Press CTRL + C to exit");
    loop {}
}


pub struct Memory {
    pub data: [Word; MAX_MEM]
}

impl Memory {
    pub fn initialise(&mut self) {
        for i in 0..MAX_MEM {
            self.data[i] = 0;
        }
    }

    pub fn write_word(&mut self, value: Word, cycles: &mut u32, address: Byte) {
        self.data[address as usize]     = value & 0xFF;
        self.data[address as usize + 1] = value >> 8;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop();
        }
        *cycles -= 1;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop();
        }
        *cycles -= 1;
    }
}

pub struct CPU {
    pub program_counter: Word,
    pub stack_pointer: Byte,

    // Registers
    pub accumulator: Byte,
    pub idx_reg_x: Byte,
    pub idx_reg_y: Byte,

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
        self.stack_pointer = 0x010;

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


    pub fn lda_set_status(&mut self) {
        self.zero_flag = self.accumulator == 0;
        self.negative_flag = (self.accumulator & 0b10000000) > 0;
    }

    // Fetches a word from the PC address and returns it
    pub fn fetch_word(&mut self, cycles: &mut u32, memory: &Memory) -> Word {
        // First Byte
        let mut data = memory.data[self.program_counter as usize];
        if self.program_counter == u16::MAX {
            println!("Program counter would be out of bounds, stopping execution...");
            error_loop();
        }
        self.program_counter += 1;

        // Second Byte
        data |= memory.data[(self.program_counter << 8) as usize];
        if self.program_counter == u16::MAX {
            println!("Program counter would be out of bounds, aborting");
            error_loop();
        }
        self.program_counter += 1;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop();
        }
        *cycles -= 1;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop();
        }
        *cycles -= 1;
        return data
    }

    // Fetches a byte from the PC address and returns it
    pub fn fetch_byte(&mut self, cycles: &mut u32, memory: &Memory) -> Byte {
        let data = memory.data[self.program_counter as usize];
        if self.program_counter == u16::MAX {
            println!("Program counter would be out of bounds, aborting");
            error_loop();
        }
        self.program_counter += 1;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop();
        }
        *cycles -= 1;
        return data.try_into().unwrap()
    }

    // Reads a byte from the PC address and returns it without increasing the PC
    pub fn read_byte(&mut self, cycles: &mut u32, address: u8, memory: &Memory) -> Byte  {
        let data = memory.data[address as usize];

        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop();
        }

        *cycles -= 1;
        return data.try_into().unwrap()
    }

    // Executes an instruction
    pub fn execute(&mut self, mut cycles: u32, mut memory: &mut Memory) {
        while cycles > 0 {
            println!("Current address: {:#06X}", self.program_counter);
            println!("Value at current address: {} | {:#06X}",
                memory.data[self.program_counter as usize],
                memory.data[self.program_counter as usize],
            );


            let data = self.fetch_byte(&mut cycles, &memory);
            println!("Fetched instruction: {:#06X}", &data);
            match data {
                INS_LOADACCUMULATOR_IMMEDIATE => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.accumulator = value;
                    self.lda_set_status();
                },

                INS_LOADACCUMULATOR_ZERO_PAGE => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    self.accumulator = self.read_byte(&mut cycles, zero_page_address, memory);
                    self.lda_set_status();
                },

                INS_LOADACCUMULATOR_ZERO_PAGE_X => {
                    let mut zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    zero_page_address += self.idx_reg_x;
                    if cycles == u32::MIN {
                        println!("{}", CYCLES_WARNING.truecolor(200,100,0));
                        error_loop();
                    }
                    cycles -= 1;
                    self.accumulator = self.read_byte(&mut cycles, zero_page_address, memory);
                    self.lda_set_status();
                },

                INS_JUMP_TO_SUBROUTINE => {
                    let sub_address: Word = self.fetch_word(&mut cycles, &mut memory);
                    memory.write_word(self.program_counter-1, &mut cycles, self.stack_pointer);
                    self.program_counter = sub_address;
                    self.stack_pointer += 1;
                    if cycles == u32::MIN {
                        println!("{}", CYCLES_WARNING.truecolor(200,100,0));
                        error_loop();
                    }
                    cycles -= 1;

                },

                _ => println!("Invalid opcode: {:#06X}", &data)
            };
        }
        println!("Finished executing all instructions");
    }
}
