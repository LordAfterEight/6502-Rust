#![allow(dead_code)]
use crate::{Word};


// Opcodes

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
