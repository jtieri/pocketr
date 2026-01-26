
// The Zero Flag (Z)
// This bit is set if and only if the result of an operation is zero. Used by conditional jumps.
const ZERO_FLAG_BIT_MASK: u8 = 0b1000_0000;

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
const SUBTRACTION_FLAG_BIT_MASK: u8 = 0b0100_0000;
const HALF_CARRY_FLAG_BIT_MASK: u8 = 0b0010_0000;

// The Carry Flag (C or Cy)
// Is set in these cases:
//  - When the result of an 8-bit addition is higher than $FF.
//  - When the result of a 16-bit addition is higher than $FFFF.
//  - When the result of a subtraction or comparison is lower than zero (like in Z80 and x86 CPUs, but unlike in 65XX and ARM CPUs).
//  - When a rotate/shift operation shifts out a “1” bit.
// Used by conditional jumps and instructions such as ADC, SBC, RL, RLA, etc.
const CARRY_FLAG_BIT_MASK: u8 = 0b0001_0000;

#[derive(Debug, Clone, Copy)]
pub struct Flags(pub u8);

impl Flags {
    #[inline]
    pub fn get_zero_flag(&self) -> bool {
        self.0 & ZERO_FLAG_BIT_MASK != 0
    }

    #[inline]
    pub fn get_subtract_flag(&self) -> bool {
        self.0 & SUBTRACTION_FLAG_BIT_MASK != 0
    }

    #[inline]
    pub fn get_half_carry_flag(&self) -> bool {
        self.0 & HALF_CARRY_FLAG_BIT_MASK != 0
    }

    #[inline]
    pub fn get_carry_flag(&self) -> bool {
        self.0 & CARRY_FLAG_BIT_MASK != 0
    }

    #[inline]
    pub fn set_zero_flag(&mut self, value: bool) {
        self.0 = (self.0 & !ZERO_FLAG_BIT_MASK) | ((value as u8) << 7);
    }

    #[inline]
    pub fn set_subtract_flag(&mut self, value: bool) {
        self.0 = (self.0 & !SUBTRACTION_FLAG_BIT_MASK) | ((value as u8) << 6);
    }

    #[inline]
    pub fn set_half_carry_flag(&mut self, value: bool) {
        self.0 = (self.0 & !HALF_CARRY_FLAG_BIT_MASK) | ((value as u8) << 5);
    }

    #[inline]
    pub fn set_carry_flag(&mut self, value: bool) {
        self.0 = (self.0 & !CARRY_FLAG_BIT_MASK) | ((value as u8) << 4);
    }

    // sanitize is meant to be called after multi-flag writes/updates or register loads.
    // this maintains the invariant of the LR35902 chip's flag register behavior,
    // which says that the first 4 bits of the flag register must always be 0s. 
    pub fn sanitize(&mut self) {
        // 0xF0 = 1111 0000
        self.0 &= 0xF0;
    }
}
