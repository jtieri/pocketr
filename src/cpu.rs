use registers::Registers;
use instructions::INSTRUCTIONS;

use crate::cpu::{flags::Flags, instructions::PREFIXED_INSTRUCTIONS, registers::{Register8Bit, Register16Bit}};

mod flags;
mod registers;
mod instructions;

// PREFIX_BYTE is used as a prefix for 16-bit opcodes 
const PREFIX_BYTE: u8 = 0xCB;

#[derive(Debug)]
pub struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

#[derive(Debug)]
pub struct MemoryBus {
    memory: [u8; 0x10000], // Memory contains 65536 bytes (Addressed 0x0000-0xFFFF inclusive)   
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers {
                a: 0,
                f: Flags(0),
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
                sp: 0,
            },
            pc: 0,
            bus: MemoryBus { 
                memory: [0; 0x10000]
            },
        }
    }
    
    // step executes single machine instructions through a fetch, decode, and execute loop that processes program memory.
    pub fn step(&mut self) {
        let opcode = self.fetch();
        
        if opcode == PREFIX_BYTE {
            let opcode = self.fetch();
            self.execute_prefixed(opcode);
        } else {
            self.execute(opcode);
        }
    }
    
    // fetch reads a single byte from memory and increments the program counter by one.
    fn fetch(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
    
    // execute handles the "decode and execute" stages of the "fetch, decode, and execute" hot loop in the CPU.
    fn execute(&mut self, opcode: u8) {
        if let Some(&instruction_fn) = INSTRUCTIONS.get(opcode as usize) {
                instruction_fn(self);
        } else {
                panic!("Unknown instruction found for: 0x{:x}", opcode)
        }
    }
    
    // execute_prefixed handles the "decode and execute" stages of the "fetch, decode, and execute" hot loop in the CPU for prefixed opcodes.
    fn execute_prefixed(&mut self, opcode: u8) {
        if let Some(&instruction_fn) = PREFIXED_INSTRUCTIONS.get(opcode as usize) {
                instruction_fn(self);
        } else {
                panic!("Unknown prefixed instruction found for: 0x{}{:x}", PREFIX_BYTE, opcode)
        }
    }
    
    // read_register_8bit reads the contents of an 8-bit register.
    fn read_register_8bit(&self, register: Register8Bit) -> u8 {
        match register {
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
    
    // read_register_16bit reads the contents of a 16-bit register pair.
    fn read_register_16bit(&self, register: Register16Bit) -> u16 {
        match register {
            Register16Bit::AF => self.registers.read_af(),
            Register16Bit::BC => self.registers.read_bc(),
            Register16Bit::DE => self.registers.read_de(),
            Register16Bit::HL => self.registers.read_hl()
        }
    }
    
    // unimplemented is a temporary placeholder for instruction handlers that are not implemented yet.
    fn unimplemented(&mut self) {
        panic!("Unimplemented opcode")
    } 
    
    // nop handles the NOP CPU instruction that does nothing (no-op) other than advance the program counter by 1.
    // Opcode: 0x00
    // Bytes: 1
    // Cycles: 1
    // Flags: ----
    fn nop(&mut self) {}
    
    // add handles the ADD CPU instruction, which adds the value found in an 8-bit source register to the value in register A.
    // The new value is then stored in register A.
    // Opcode: 0x80 - 0x87
    // Bytes: 1
    // Cycles: 1 (2 for 0x86 when dealing with indirect memory access through register pair HL)
    // Flags: ZHC-
    fn add_a(&mut self, source: Register8Bit) {
        // Read value currently in the source register
        let val = self.read_register_8bit(source);
        
        // Add the value in the source register to the value in register A and handle overflow
        let (new_val, overflow) = self.registers.a.overflowing_add(val);
        
        // Update flags
        self.registers.f.set_zero_flag(new_val == 0);
        self.registers.f.set_subtract_flag(false);
        self.registers.f.set_carry_flag(overflow);
        self.registers.f.set_half_carry_flag((self.registers.a & 0xF) + (val & 0xF) > 0xF);
        
        // Write updated value to register A
        self.registers.a = new_val;
    }
    
    // adc_a handles the ADC, A CPU instruction, which adds the contents of an 8-bit source register and the CY flag
    // to the contents of register A. The new value is then stored in register A.
    // Opcode: 0x88 - 0x8F
    // Bytes: 1
    // Cycles: 1
    // Flags: ZO
    fn adc_a(&mut self, target: Register8Bit) {
        // Read the values currently in the source register and the carry flag
        let val = self.read_register_8bit(target);
        let cy_val = self.registers.f.get_carry_flag() as u8;
        
        // Add the value in the source register plus the value in the carry register 
        // to the value in register A and handle overflow
        let (new_val, overflow) = self.registers.a.overflowing_add(val + cy_val);
        
        // Update flags
        self.registers.f.set_zero_flag(new_val == 0);
        self.registers.f.set_subtract_flag(false);
        self.registers.f.set_carry_flag(overflow);
        self.registers.f.set_half_carry_flag((self.registers.a & 0xF) + (val & 0xF) + (cy_val & 0xF) > 0xF);
        
        // Write updated value to register A
        self.registers.a = new_val;
    }
}
