#![allow(dead_code)]
mod structs;
mod opcodes;
use structs::*;
use opcodes::*;

use clearscreen;


type Byte = u8;
type Word = u16;
static MAX_MEM: usize = 1024 * 64;


fn main() {
    _ = clearscreen::clear();

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

    cpu.reset();
    memory.initialise();
    memory.data[0xFFFC] = INS_JUMP_ABSOLUTE as u16;
    memory.data[0xFFFD] = 0xF000;
    memory.data[0xF000] = INS_LOAD_ACCUMULATOR_IMMEDIATE as u16;
    memory.data[0xF001] = 0x82;
    memory.data[0xF002] = INS_STORE_ACCUMULATOR_ZERO_PAGE as u16;
    memory.data[0xF003] = 0xFEAA;
    memory.data[0xF004] = INS_JUMP_ABSOLUTE as u16;
    memory.data[0xF005] = 0xFFFF;
    memory.data[0xFFFF] = INS_FORCE_INTERRUPT as u16;
    cpu.execute(1, &mut memory);
}
