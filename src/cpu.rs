use flags::Flags;

mod flags;

#[derive(Debug, Default)]
pub struct CPU {
    registers: Registers,
}

// The Gameboy uses 8-bit registers but has instructions that allow games to read and write 16 bits of data.
#[derive(Debug, Default)]
pub struct Registers {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    
    // TODO: need to implement the program counter (PC)
    // TODO: need to implement the stack pointer (SP) 
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /* TODO: support more targets */ }
                }
            }
            _ => { /* TODO: support more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        let half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        
        self.registers.f.set_zero_flag(new_value == 0);
        self.registers.f.set_subtract_flag(false);
        self.registers.f.set_carry_flag(did_overflow);
        self.registers.f.set_half_carry_flag(half_carry);
        
        new_value
    }
}

// We treat the "Hi" register as a u16 which effectively just adds a byte of all 0s to the significant position.
// Then we shift the b register 8 positions so that it's occupying the most significant byte position.
// We then bitwise OR the c register so that the result is a two byte number with the contents of b in the most significant byte postion
// and the contents of c in the least significant byte position.
impl Registers {
    fn read_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn write_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn read_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn write_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    
    fn read_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    
    fn write_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

enum Instruction {
    // ADD adds a target register's contents to the A register's contents.
    ADD(ArithmeticTarget),
}

// ArithmeticTarget represents the target register to be used in the ADD instruction.
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
