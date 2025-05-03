use crate::opcodes::*;
type Byte = u8;
type Word = u16;
static MAX_MEM: usize = 1024 * 64;



pub struct Memory {
    pub data: [Word; MAX_MEM]
}

impl Memory {
    pub fn initialise(&mut self) {
        for i in 0..MAX_MEM {
            self.data[i] = 0;
        }
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
    pub fn reset(&mut self, memory: &Memory) {
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


    pub fn LDASetStatus(&mut self) {
        self.zero_flag = self.accumulator == 0;
        self.negative_flag = (self.accumulator & 0b10000000) > 0;
    }

    // Fetches a byte from the PC address and returns it
    pub fn fetch_byte(&mut self, cycles: &mut u32, memory: &Memory) -> Byte {
        let data = memory.data[self.program_counter as usize];
        self.program_counter += 1;
        *cycles -= 1;
        return data.try_into().unwrap()
    }

    // Reads a byte from the PC address and returns it without increasing the PC
    pub fn read_byte(&mut self, cycles: &mut u32, address: u8, memory: &Memory) -> Byte  {
        let data = memory.data[address as usize];
        *cycles -= 1;
        return data.try_into().unwrap()
    }

    // Executes an instruction
    pub fn execute(&mut self, mut cycles: u32, memory: &Memory) {
        while cycles > 0 {
            println!("Cycles left: {}", &cycles);
            let data = self.fetch_byte(&mut cycles, &memory);
            match data {
                INS_LOADACCUMULATOR_IMMEDIATE => {
                    println!("Instruction: {:#06x}", &data);
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.accumulator = value;
                    self.LDASetStatus();
                },
                INS_LOADACCUMULATOR_ZERO_PAGE => {
                    println!("Instruction: {:#06x}", &data);
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    self.accumulator = self.read_byte(&mut cycles, zero_page_address, memory);
                    self.LDASetStatus();
                },
                _ => println!("Invalid opcode: {:#06x}", &data)
            };
        }
        println!("Finished executing all instructions");
    }
}
