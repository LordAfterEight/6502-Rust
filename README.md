# VM16B
What started as a 6502 emulator has now become my own virtual machine with some aspects inspired by the 6502

# Current Features
Currently this virtual machine's memory has a hardcoded nanokernel and a command line supporting currently five single-character commands:

- q for shutting the machine down
- r to reset the processor
- d to dump the memory (this wouldn't work if this machine was physical)
- h to display all possible commands
- i to display system information (not implemented yet)

## System
the CPU of this system was originally supposed to be a 6502 emulation, but I decided to just make up my own instead. Some of the OpCodes are from the 6502, but I added my own to work with the system's GPU. I still plan to make the machine create its own window, however currently it still runs in your terminal and thus can't display proper graphics and uses print!() and println!() to write text.

The memory has 65536 16-bit addresses (128KB of storage space) and functions as both ROM and RAM.

The CPU's registers are all 16-bit.

## How do you program the memory?
Currently the only way to program the memory is to assign values to individual addresses. In the code the memory is an array with 65536 spaces so programming it is as easy as doing this:
```
memory.data[usize] = u16
```
