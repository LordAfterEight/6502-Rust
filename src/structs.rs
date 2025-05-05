use crate::opcodes::*;
use colored::Colorize;
type Byte = u8;
type Word = u16;
static MAX_MEM: usize = 1024 * 64;

const CYCLES_WARNING: &str = "No cycles left, stopping...";

pub fn error_loop(error: &str) {
    println!("{}", "\nFalling back to safe loop...\n\n".truecolor(200,100,0));
    println!("{}\n{} {}\n\n{}",
        "[!] Entered safe loop".truecolor(200,100,0),
        "[i] Reason:".yellow(),
        error,
        "Press CRL + C to exit".cyan()
    );
    loop {
    }
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

    pub fn dump(&mut self) {
        let mut out = 0;
        for mut y in 0..0xFFFF/4+1 {
            for mut x in 0..4 {
                print!("{:#06X} : {:#06X} | ",
                    out+x,
                    self.data[out+x as usize],
                );
            }
            print!("\n");
            out+=4
        }
        println!("{}","\n[i] Dumped memory".yellow());
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
        self.stack_pointer = 0x010; // stack location: 0x0100 - 0x01FF

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

    pub fn write_byte(&mut self, value: Word, cycles: &mut u32, address: Word, memory: &mut Memory) {
        memory.data[address as usize] = value;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop("No cycles left");
        }
        *cycles -= 1;
    }

    pub fn write_word(&mut self, value: Word, cycles: &mut u32, address: Byte, memory: &mut Memory) {
        memory.data[address as usize]     = value & 0xFF;
        memory.data[address as usize + 1] = value >> 8;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop("No cycles left");
        }
        *cycles -= 1;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop("No cycles left");
        }
        *cycles -= 1;
    }


    pub fn set_zero_and_negative_flags(&mut self, register: Byte) {
        self.zero_flag = register == 0;
        self.negative_flag = (register & 0b10000000) > 0;
    }

    // Fetches a word from the PC address and returns it
    pub fn fetch_word(&mut self, cycles: &mut u32, memory: &Memory) -> Word {
        // First Byte
        let mut data = memory.data[self.program_counter as usize];
        if self.program_counter == u16::MAX {
            println!("Program counter would be out of bounds, stopping execution...");
            error_loop("Program counter out of bounds");
        }
        self.program_counter += 1;

        // Second Byte
        data |= memory.data[(self.program_counter << 8) as usize];
        if self.program_counter == u16::MAX {
            println!("Program counter would be out of bounds, aborting");
            error_loop("Program counter out of bounds");
        }
        self.program_counter += 1;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop("No cycles left");
        }
        *cycles -= 1;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop("No cycles left");
        }
        *cycles -= 1;
        return data
    }

    // Fetches a byte from the PC address and returns it
    pub fn fetch_byte(&mut self, cycles: &mut u32, memory: &Memory) -> Byte {
        let data = memory.data[self.program_counter as usize];
        if self.program_counter == u16::MAX {
            println!("Program counter would be out of bounds, aborting");
            error_loop("Program counter out of bounds");
        }
        self.program_counter += 1;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop("Program counter out of bounds");
        }
        *cycles -= 1;
        return data.try_into().unwrap()
    }

    // Reads a byte from the PC address and returns it without increasing the PC
    pub fn read_byte(&mut self, cycles: &mut u32, address: Word, memory: &Memory) -> Byte  {
        let data = memory.data[address as usize];
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop("No cycles left");
        }
        *cycles -= 1;
        return data.try_into().unwrap()
    }

    pub fn read_word(&mut self, cycles: &mut u32, address: Word, memory: &Memory) -> Word  {
        let lo_byte: Word = self.read_byte(cycles, address, memory) as u16;
        let hi_byte: Word = self.read_byte(cycles, address+1, memory) as u16;
        if *cycles == u32::MIN {
            println!("{}", CYCLES_WARNING.truecolor(200,100,0));
            error_loop("No cycles left");
        }
        *cycles -= 1;
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
            println!("Current address: {:#06X}", self.program_counter);
            println!("Value at current address: {} | {:#06X}",
                memory.data[self.program_counter as usize],
                memory.data[self.program_counter as usize],
            );


            let data = self.fetch_byte(&mut cycles, &memory);
            println!("Fetched instruction: {:#06X}", &data);
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
                    error_loop("Forced interrupt");
                },

                INS_LOAD_ACCUMULATOR_IMMEDIATE => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.accumulator = value;
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                INS_LOAD_ACCUMULATOR_ZERO_PAGE => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    self.accumulator = self.read_byte(&mut cycles, zero_page_address.into(), memory);
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                INS_LOAD_ACCUMULATOR_ZERO_PAGE_X => {
                    let mut zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    zero_page_address += self.idx_reg_x;
                    if cycles == u32::MIN {
                        println!("{}", CYCLES_WARNING.truecolor(200,100,0));
                        error_loop("No cycles left");
                    }
                    cycles -= 1;
                    self.accumulator = self.read_byte(&mut cycles, zero_page_address.into(), memory);
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                INS_LOAD_X_REGISTER_IMMEDIATE => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.idx_reg_x = value;
                    self.set_zero_and_negative_flags(self.accumulator);
                },

                INS_JUMP_TO_SUBROUTINE => {
                    let sub_address: Word = self.fetch_word(&mut cycles, &mut memory);
                    self.write_word(self.program_counter-1, &mut cycles, self.stack_pointer, &mut memory);
                    self.program_counter = sub_address;
                    if self.stack_pointer == u8::MAX {
                        println!("Stack pointer would be out of bounds, stopping...");
                        error_loop("Stack pointer out of bounds");
                    }
                    self.stack_pointer += 1;
                    if cycles == u32::MIN {
                        println!("{}", CYCLES_WARNING.truecolor(200,100,0));
                        error_loop("No cycles left");
                    }
                    cycles -= 1;

                },

                _ => println!("Invalid opcode: {:#06X}", &data)
            };
        }
        println!("Finished executing all instructions");
    }
}
