use crate::Byte;
// Opcodes
pub const INS_LOADACCUMULATOR_IMMEDIATE: Byte = 0xA9;
pub const INS_LOADACCUMULATOR_ZERO_PAGE: Byte = 0xA5;
pub const INS_LOADACCUMULATOR_ZERO_PAGE_X: Byte = 0xB5;
pub const INS_JUMP_TO_SUBROUTINE: Byte = 0x20;
