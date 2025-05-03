type Byte = u8;
type Word = u16;
static MAX_MEM: usize = 1024 * 64;

pub struct Memory {
    data: [Byte; MAX_MEM]
}

impl Memory {
    fn initialise(&mut self) {
        for i in 0..MAX_MEM {
            self.data[i] = 0;
        }
    }
}

pub struct CPU {
    program_counter: Word,
    stack_pointer: Byte,

    // Registers
    accumulator: Byte,
    idx_reg_x: Byte,
    idx_reg_y: Byte,

    processor_status: Byte,

    // Flags
    carry_flag: bool,
    zero_flag:  bool,
    interrupt_disable: bool,
    decimal_mode: bool,
    break_command: bool,
    overflow_flag: bool,
    negative_flag: bool
}

impl CPU {
    fn reset(&mut self, memory: &Memory) {
        // Set adresses
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

    fn execute(&mut self, ticks: u32, memory: &Memory) {
    }

