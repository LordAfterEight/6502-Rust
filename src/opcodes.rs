use crate::Byte;


// Opcodes

// Load accumulator
pub const INS_LOADACCUMULATOR_IMMEDIATE: Byte = 0xA9;
pub const INS_LOADACCUMULATOR_ZERO_PAGE: Byte = 0xA5;
pub const INS_LOADACCUMULATOR_ZERO_PAGE_X: Byte = 0xB5;

// Store accumulator
pub const INS_STOREACCUMULATOR_ZERO_PAGE: Byte = 0x85;
pub const INS_STOREACCUMULATOR_ZERO_PAGE_X: Byte = 0x95;

// Jump To Subroutine 
pub const INS_JUMP_TO_SUBROUTINE: Byte = 0x20;

// System calls
pub const INS_FORCE_INTERRUPT : Byte = 0x00;
