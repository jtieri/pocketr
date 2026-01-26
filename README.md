# pocketr -> pocket rust


## Overview

A Gameboy is made up of a few primary components:

- CPU (Sharp LR35902 chip)
- RAM (used for data that no longer fits in a register, must be moved, etc.)
- ROM (read-only memory used during bootstrapping to set the machine up and enter the splash screen)
- I/O

CPU is composed of 8-bit registers, however, the instruction set permits 16-bit operations so hi/lo bits can be spread across register pairs. 

Some info taken from [gbdev.io](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)

| 16-bit |  Hi |  Lo | Name / Function           |
| -----: | :-: | :-: | ------------------------- |
|     AF |  A  |  –  | Accumulator & Flags       |
|     BC |  B  |  C  | BC                        |
|     DE |  D  |  E  | DE                        |
|     HL |  H  |  L  | HL                        |
|     SP |  –  |  –  | Stack Pointer             |
|     PC |  –  |  –  | Program Counter / Pointer |

Flags Register (Lower 8 bits of AF register)
| Bit | Name | Explanation            |
| --: | :--: | ---------------------- |
|   7 |   z  | Zero flag              |
|   6 |   n  | Subtraction flag (BCD) |
|   5 |   h  | Half Carry flag (BCD)  |
|   4 |   c  | Carry flag             |

> Note:
The flags register contains information about the result of the most recent instruction that affected flags.


## Game ROM

Memory can be thought of as a large array of 8-bit numbers. The beginning of this array contains
255 bytes, from index 0x0000 to 0x00FF, that serve as the instructions that tell the Game Boy how to bootstrap itself, which is the process that it goes through in order to be ready to execute a game. 
(This is also how the splash screen gets displayed)

The contents of a Game Boy cartridge become available to the CPU right after these 255 bytes. 
The contents of memory starting at index 0x100 until index 0x3FFF include the contents of the cartridge.

Memory contains 65,536 8-bit numbers in total.
Each of these numbers can be decoded as an instruction that the CPU knows how to run.

## Program Counter

The Program Counter (PC) is a 16-bit number that tells the Game Boy which instruction the Game Boy
is currently executing.

The main loop for processing instructions is:
- Use the PC to read the instruction byte from memory
- Translate the byte to one instance of the `Instruction` enum
- If the translation is successful call `execute` for that instruction or else panic, which should now return the next PC
- Set this next PC on the CPU

