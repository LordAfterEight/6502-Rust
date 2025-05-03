type Byte = u8;
type Word = u16;

struct CPU {
    program_counter: Byte,
    stack_pointer: Byte,

    // Registers
    accumulator: Byte,
    idx_reg_x: Byte,
    idx_reg_y: Byte,

    processor_status: Byte,

    // Flags
    carry_flag: bool,
    zero_flag:  bool,
    interrupt_disable: bool,
    decimal_mode: bool,
    break_command: bool,
    overflow_flag: bool,
    negative_flag: bool
}

fn main() {
}
