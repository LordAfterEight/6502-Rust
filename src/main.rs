mod structs;
mod opcodes;
use structs::*;
use opcodes::*;

type Byte = u8;
type Word = u16;
static MAX_MEM: usize = 1024 * 64;


fn main() {
    let mut cpu = CPU {
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

    let mut memory = Memory {
        data: [0;MAX_MEM]
    };

    cpu.reset(&memory);
    memory.data[0xFFFC] = INS_LOADACCUMULATOR_ZERO_PAGE as u16;
    memory.data[0xFFFD] = 0x42;
    memory.data[0x0042] = 0x84;
    println!("{} | {}", &cpu.program_counter, & memory.data[0]);
    println!("{} | {}", &cpu.program_counter, & memory.data[1]);
    cpu.execute(3, &memory);
}
