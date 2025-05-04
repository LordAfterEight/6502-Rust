#![allow(dead_code)]
use crate::Byte;


// Opcodes

// Load accumulator
pub const INS_LOAD_ACCUMULATOR_IMMEDIATE: Byte = 0xA9;
pub const INS_LOAD_ACCUMULATOR_ZERO_PAGE: Byte = 0xA5;
pub const INS_LOAD_ACCUMULATOR_ZERO_PAGE_X: Byte = 0xB5;

// Load X Register
pub const INS_LOAD_X_REGISTER_IMMEDIATE: Byte = 0xA2;
pub const INS_LOAD_X_REGISTER_ZERO_PAGE: Byte = 0xA6;
pub const INS_LOAD_X_REGISTER_ZERO_PAGE_Y: Byte = 0xB6;

// Load Y Register
pub const INS_LOAD_Y_REGISTER_IMMEDIATE: Byte = 0xA;
pub const INS_LOAD_Y_REGISTER_ZERO_PAGE: Byte = 0xA4;
pub const INS_LOAD_Y_REGISTER_ZERO_PAGE_X: Byte = 0xB4;

// Store accumulator
pub const INS_STORE_ACCUMULATOR_ZERO_PAGE: Byte = 0x85;
pub const INS_STORE_ACCUMULATOR_ZERO_PAGE_X: Byte = 0x95;

// Jump To Subroutine 
pub const INS_JUMP_TO_SUBROUTINE: Byte = 0x20;

// System calls
pub const INS_FORCE_INTERRUPT : Byte = 0x00;
