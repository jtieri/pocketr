use std::convert::From;

// The Zero Flag (Z)
// This bit is set if and only if the result of an operation is zero. Used by conditional jumps.
const ZERO_FLAG_BIT_POSITION: u8 = 0b10000000;

// The BCD Flags (N, H)
// These flags are used by the DAA instruction only.
// N indicates whether the previous instruction has been a subtraction,
// and H indicates carry for the lower 4 bits of the result.
// DAA also uses the C flag, which must indicate carry for the upper 4 bits.
// After adding/subtracting two BCD numbers, DAA is used to convert the result to BCD format.
// BCD numbers range from $00 to $99 rather than $00 to $FF.
// Because only two flags (C and H) exist to indicate carry-outs of BCD digits,
// DAA is ineffective for 16-bit operations (which have 4 digits),
// and use for INC/DEC operations (which do not affect C-flag) has limits.
const SUBTRACTION_FLAG_BIT_POSITION: u8 = 0b01000000;
const HALF_CARRY_FLAG_BIT_POSITION: u8 = 0b00100000;

// The Carry Flag (C or Cy)
// Is set in these cases:
//  - When the result of an 8-bit addition is higher than $FF.
//  - When the result of a 16-bit addition is higher than $FFFF.
//  - When the result of a subtraction or comparison is lower than zero (like in Z80 and x86 CPUs, but unlike in 65XX and ARM CPUs).
//  - When a rotate/shift operation shifts out a “1” bit.
// Used by conditional jumps and instructions such as ADC, SBC, RL, RLA, etc.
const CARRY_FLAG_BIT_POSITION: u8 = 0b00010000;

#[derive(Debug, Default)]
pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl From<FlagsRegister> for u8 {
    fn from(flags: FlagsRegister) -> u8 {
        (flags.zero as u8) << 7
            | (flags.subtract as u8) << 6
            | (flags.half_carry as u8) << 5
            | (flags.carry as u8) << 4
    }
}

impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let z = (byte & ZERO_FLAG_BIT_POSITION) != 0;
        let n = (byte & SUBTRACTION_FLAG_BIT_POSITION) != 0;
        let h = (byte & HALF_CARRY_FLAG_BIT_POSITION) != 0;
        let c = (byte & CARRY_FLAG_BIT_POSITION) != 0;

        FlagsRegister {
            zero: z,
            subtract: n,
            half_carry: h,
            carry: c,
        }
    }
}
