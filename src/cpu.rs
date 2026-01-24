use registers::Registers;

mod flags;
mod registers;

#[derive(Debug, Default)]
pub struct CPU {
    registers: Registers,
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
