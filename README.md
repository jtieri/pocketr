# pocketr -> pocket rust


## Overview

A Gameboy is made up of a few primary components:

- CPU (Sharp LR35902 chip)
- RAM (used for data that no longer fits in a register, must be moved, etc.)
- ROM (read-only memory used during bootstrapping to set the machine up and enter the splash screen)
- I/O

CPU is composed of 8-bit registers, however, the instruction set permits 16-bit operations so hi/lo bits can be spread across register pairs. 

Some info taken from [gbdev.io](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)

// ===================================================
// 16bit  |  Hi  |  Lo  |   Name/Function
// AF       A       -       Accumulator & Flags
// BC       B       C       BC
// DE       D       E       DE
// HL       H       L       HL
// SP       -       -       Stack Pointer
// PC       -       -       Program Counter/Pointer
// =================================================== 

// Flags Register (lower 8 bits of AF register)
//--------------------------------------------------
// Bit      Name        Explanation
// 7        z           Zero flag
// 6        n           Subtraction flag (BCD)
// 5        h           Half Carry flag (BCD)
// 4        c           Carry flag
//--------------------------------------------------
// * contains info about the result of the most recent instruction that has affected flags
