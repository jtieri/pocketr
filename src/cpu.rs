use registers::Registers;
use instructions::INSTRUCTIONS;

use crate::cpu::registers::{Register8Bit, Register16Bit};

mod flags;
mod registers;
mod instructions;

// PREFIX_BYTE is used as a prefix for 16bit opcodes 
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
    
    fn execute_opcode(&mut self, opcode: u8) -> u16 {
        if let Some(&instruction_fn) = INSTRUCTIONS.get(opcode as usize) {
                instruction_fn(self);
                self.pc.wrapping_add(1)
        } else {
                panic!("Unknown instruction found for: 0x{:x}", opcode)
        }
    }
    
    fn read_register_8bit(&self, target: Register8Bit) -> u8 {
        match target {
            Register8Bit::A => self.registers.a,
            Register8Bit::B => self.registers.b,
            Register8Bit::C => self.registers.c,
            Register8Bit::D => self.registers.d,
            Register8Bit::E => self.registers.e,
            Register8Bit::H => self.registers.h,
            Register8Bit::L => self.registers.l,
            Register8Bit::HLIndirect => self.bus.read_byte(self.registers.read_hl())
        }
    }
    
    fn read_register_16bit(&mut self, target: Register16Bit) -> u16 {
        match target {
            Register16Bit::AF => self.registers.read_af(),
            Register16Bit::BC => self.registers.read_bc(),
            Register16Bit::DE => self.registers.read_de(),
            Register16Bit::HL => self.registers.read_hl()
        }
    }
    
    // nop is the NOP CPU instruction that does nothing (no-op) other than advance the program counter by 1.
    // Opcode: 0x00
    // Bytes: 1
    // Cycles: 1
    // Flags: ----
    fn nop(&mut self) {}
    
    // add is the ADD CPU instruction for adding the value found in an 8bit target register to the value in the A register.
    // The new value is then stored in the A register.
    // Opcode: 0x80 - 0x87
    // Bytes: 1
    // Cycles: 1
    // Flags: ZHC-
    fn add_a(&mut self, target: Register8Bit) {
        // Read value currently in the target register
        let val = self.read_register_8bit(target);
        
        // Add the target register value to the value in the A register and handle overflow
        let (new_val, overflow) = self.registers.a.overflowing_add(val);

        // Write updated value to the A register
        self.registers.a = new_val;
        
        // Update flags
        self.registers.f.set_zero_flag(new_val == 0);
        self.registers.f.set_subtract_flag(false);
        self.registers.f.set_carry_flag(overflow);
        self.registers.f.set_half_carry_flag((self.registers.a & 0xF) + (val & 0xF) > 0xF);
    }
    
    fn did_half_carry() {
        
    }
}
