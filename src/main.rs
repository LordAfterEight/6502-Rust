#![allow(dead_code)]
mod cpu;
mod memory;
mod opcodes;
mod eventhandler;
mod gpu;
mod data;
use cpu::*;
use gpu::*;
use memory::*;
use opcodes::*;

use colored;
use clearscreen;
use crossterm;

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
        negative_flag: false,
    };

    let mut gpu = GPU {};

    let mut memory = Memory {
        data: [0;MAX_MEM]
    };

    cpu.reset();
    memory.init();
    gpu.init();
    memory.data[0xFFFC] = INS_JUMP_TO_SUBROUTINE;
    memory.data[0xFFFD] = 0xF000;




    memory.data[0xF0F0] = INS_RETURN_FROM_SUBROUTINE;
    memory.data[0xFFFE] = INS_FORCE_INTERRUPT as u16;
    cpu.execute(&mut memory, &mut gpu);
}
