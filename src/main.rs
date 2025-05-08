#![allow(dead_code)]
mod cpu;
mod memory;
mod opcodes;
mod eventhandler;
mod gpu;
use cpu::*;
use memory::*;
use opcodes::*;
use eventhandler::*;
use gpu::*;

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
    memory.initialise();
    memory.data[0xFFFC] = INS_JUMP_TO_SUBROUTINE;
    memory.data[0xFFFD] = 0xF000;


    memory.data[0xF000] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF001] = 0xF21E;
    memory.data[0xF002] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF003] = 0xF202;
    memory.data[0xF004] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF005] = 0xF20F;
    memory.data[0xF006] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF007] = 0xF214;
    memory.data[0xF008] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF009] = 0xF21F;
    memory.data[0xF00A] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF00B] = 0xF1FF;
    memory.data[0xF00C] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF00D] = 0xF23B;
    memory.data[0xF00E] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF00F] = 0xF23D;
    memory.data[0xF010] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF011] = 0xF1FF;
    memory.data[0xF012] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF013] = 0xF21D;
    memory.data[0xF014] = INS_GPU_DRAW_AT_CURSOR_POSITION;
    memory.data[0xF015] = 0xF1FF;


    memory.data[0xF0F0] = INS_RETURN_FROM_SUBROUTINE;
    memory.data[0xFFFE] = INS_FORCE_INTERRUPT as u16;
    cpu.execute(250, &mut memory, &mut gpu);
}
