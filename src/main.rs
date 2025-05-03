mod structs;
use structs::*;

type Byte = u8;
type Word = u16;
static MAX_MEM: usize = 1024 * 64;


fn main() {
    let mut cpu = CPU{
        program_counter: 0,
        stack_pointer: 0,

        // Registers
        accumulator: 0,
        idx_reg_x: 0,
        idx_reg_y: 0,

        processor_status: 0,

        // Flags
        carry_flag: false,
        zero_flag:  false,
        interrupt_disable: false,
        decimal_mode: false,
        break_command: false,
        overflow_flag: false,
        negative_flag: false
    };

    let memory = Memory {
        data: [0;MAX_MEM]
    };

    cpu.reset(&memory);
    cpu.execute(2, &memory)
}
