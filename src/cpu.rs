use registers::Registers;
use instructions::INSTRUCTIONS;

mod flags;
mod registers;
mod instructions;

const PREFIX_BYTE: u8 = 0xCB;

#[derive(Debug)]
pub struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

#[derive(Debug)]
pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {
    pub fn step(&mut self) {
        let mut opcode = self.bus.read_byte(self.pc);
        
        let prefixed = opcode == PREFIX_BYTE;
        if prefixed { 
            opcode = self.bus.read_byte(self.pc);
        }
        // TODO: prefixed byte handling needs to be properly implemented
        
        self.pc = self.execute_opcode(opcode)
    }
    
    // nop is the NOP CPU instruction that results in a no operation.
    // Opcode: 0x00
    // Cycles: 1
    // Bytes: 1
    // Flags: None affected
    fn nop(&mut self) {
    
    }
        
    fn execute_opcode(&mut self, opcode: u8) -> u16 {
        if let Some(&instruction_fn) = INSTRUCTIONS.get(opcode as usize) {
                instruction_fn(self);
                self.pc.wrapping_add(1)
        } else {
                panic!("Unknown instruction found for: 0x{:x}", opcode)
        }
    }

    // pub fn execute(&mut self, instruction: Instruction) -> u16 {
    //     match instruction {
    //         Instruction::ADD(target) => {
    //             match target {
    //                 ArithmeticTarget::C => {
    //                     let value = self.registers.c;
    //                     let new_value = self.add(value);
    //                     self.registers.a = new_value;
    //                     self.pc.wrapping_add(1)
    //                 }
    //                 _ => { /* TODO: support more targets */ self.pc }
    //             }
    //         }
    //         _ => { /* TODO: support more instructions */ self.pc}
    //     }
    // }

    // fn add(&mut self, value: u8) -> u8 {
    //     let (result, carry) = self.registers.a.overflowing_add(value);

    //     // The Half Carry flag is set if adding the lower nibbles of the value and register A
    //     // together result in a value bigger than 0xF. If the result is larger than 0xF
    //     // then the addition caused a carry from the lower nibble to the upper nibble.
    //     let half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

    //     self.registers.f.set_zero_flag(result == 0);
    //     self.registers.f.set_subtract_flag(false);
    //     self.registers.f.set_carry_flag(carry);
    //     self.registers.f.set_half_carry_flag(half_carry);

    //     result
    // }
}

// enum Instruction {
//     // ADD adds a target register's contents to the A register's contents.
//     ADD(ArithmeticTarget),
//     INC(IncDecTarget)
// }

// // ArithmeticTarget represents the target register to be used in the ADD instruction.
// enum ArithmeticTarget {
//     A,
//     B,
//     C,
//     D,
//     E,
//     H,
//     L,
// }

// enum IncDecTarget {
//     BC,
//     DE
// }

// impl Instruction {
//     fn from_byte(byte: u8) -> Option<Instruction> {
//         match byte {
//             0x02 => Some(Instruction::INC(IncDecTarget::BC)),
//             0x13 => Some(Instruction::INC(IncDecTarget::DE)),
//             _ =>
//             /* TODO: Add mapping for rest of instructions */
//             {
//                 None
//             }
//         }
//     }
// }
