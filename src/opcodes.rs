#![allow(dead_code)]
use crate::{Word};


// CPU opcodes

// Load accumulator
pub const INS_LOAD_ACCUMULATOR_IMMEDIATE: Word = 0xA9;
pub const INS_LOAD_ACCUMULATOR_ZERO_PAGE: Word = 0xA5;
pub const INS_LOAD_ACCUMULATOR_ZERO_PAGE_X: Word = 0xB5;

// Load X Register
pub const INS_LOAD_X_REGISTER_IMMEDIATE: Word = 0xA2;
pub const INS_LOAD_X_REGISTER_ZERO_PAGE: Word = 0xA6;
pub const INS_LOAD_X_REGISTER_ZERO_PAGE_Y: Word = 0xB6;

// Load Y Register
pub const INS_LOAD_Y_REGISTER_IMMEDIATE: Word = 0xA;
pub const INS_LOAD_Y_REGISTER_ZERO_PAGE: Word = 0xA4;
pub const INS_LOAD_Y_REGISTER_ZERO_PAGE_X: Word = 0xB4;

// Store accumulator
pub const INS_STORE_ACCUMULATOR_ZERO_PAGE: Word = 0x85;
pub const INS_STORE_ACCUMULATOR_ZERO_PAGE_X: Word = 0x95;

// Jump to / Return from subroutine
pub const INS_JUMP_TO_SUBROUTINE: Word = 0x20;
pub const INS_RETURN_FROM_SUBROUTINE: Word = 0x60;

// Jump
pub const INS_JUMP_ABSOLUTE: Word = 0x4C;
pub const INS_JUMP_INDIRECT: Word = 0x6C;

// System calls
pub const INS_FORCE_INTERRUPT: Word = 0x00;
pub const INS_NO_OPERATION: Word = 0xEA;

// IO
pub const INS_WAIT_FOR_INPUT: Word = 0x01;




// GPU opcodes
pub const INS_GPU_DRAW_AT_CURSOR_POSITION: Word = 0xA000;
pub const INS_GPU_CLEAR_AT_CURSOR_POSITION: Word = 0xA100;
pub const INS_GPU_MOVE_CURSOR_RIGHT: Word = 0xA200;
pub const INS_GPU_MOVE_CURSOR_LEFT: Word = 0xA201;
pub const INS_GPU_MOVE_CURSOR_UP: Word = 0xA202;
pub const INS_GPU_MOVE_CURSOR_DOWN: Word = 0xA203;
