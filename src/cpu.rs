use registers::Registers;
use flags::Flags;

mod flags;
mod registers;

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
    
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
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
    
    pub fn with_rom(mut self, rom: &[u8]) -> Self {
        let len = rom.len();
        self.bus.memory[..len].copy_from_slice(rom);
        self
    }
    
    pub fn run(&mut self) {
        loop {
            self.step();
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
        // Decode bits 0-2 to get the 8-bit source register
        let r8_src = opcode & 0x07;
        
        // Decode bits 3-5 to get the 8-bit destination register
        let r8_dst = (opcode >> 3) & 0x07;
        
        // Decode bits 4-5 to get a 16-bit register pair
        let r16 = (opcode >> 4) & 0x03;
        
        match opcode {
            // ---- Misc, Loads, Inc/Dec, Rotates, Jumps: 0x00-0x3F ----
            
            // NOP
            0x00 => self.nop(),
            
            // LD BC, d16
            0x01 => {
              let value = self.fetch_imm16();
              self.registers.write_bc(value);
            },
            
            // LD (BC), A
            0x02 => {
                let value = self.registers.a;
                self.bus.write_byte(self.registers.read_bc(), value);
            },
            
            // ---- Register to Register Loads: 0x40-0x7F ----
            
            
            // ---- ALU Block: 0x80 - 0xBF ----
            0x80..=0xBF => {
                let value = self.read_r8(r8_src);
                
                // Decode bits 3-5 to get the ALU operation
                let operation = (opcode >> 3) & 0x07;
                
                match operation {
                    0 => self.add_a(value),
                    1 => self.adc_a(value),
                    2 => self.sub_a(value),
                    3 => self.sbc_a(value),
                    4 => self.and_a(value),
                    5 => self.xor_a(value),
                    6 => self.or_a(value),
                    7 => self.cp_a(value),
                    _ => unreachable!(),
                }
            },
            
            // ADD A,r 
            // 0x80..=0x87 => {
            //   let value = self.read_r8(r8_src);
            //   self.add_a(value);
            // },
            
            // // ADC A,r
            // 0x88..=0x8F => {
            //     let value = self.read_r8(r8_src);
            //     self.adc_a(value);
            // },
            
            // // Sub r
            // 0x90..=0x97 => {
                
            // },
            
            // // SBC A,r
            // 0x98..=0x9F => {
            // },
            
            // // AND r
            // 0xA0..=0xA7 => {
                
            // },
            
            // // XOR r
            // 0xA8..=0xAF => {
            // },
            
            // // OR r
            // 0xB0..=0xB7 => {
                
            // },
            
            // // CP r
            // 0xB8..=0xBF => {
                
            // },
            
            // ---- Control Flow, Stack, Etc: 0xC0-0xFF ---- 
            
           
            
            _ => panic!("Unknown instruction found for: 0x{:x}", opcode),
        }
    }
    
    // execute_prefixed handles the "decode and execute" stages of the "fetch, decode, and execute" hot loop in the CPU for prefixed opcodes.
    fn execute_prefixed(&mut self, opcode: u8) {
        let register = opcode & 0x07;
        let bit = (opcode >> 3) & 0x07;
        let category = opcode >> 6;
        
        match category {
            
            
            
            _ => panic!("Unknown prefixed instruction found for: 0x{}{:x}", PREFIX_BYTE, opcode)
        }
    }
    
    // read_r8 reads the contents of an 8-bit register.
    // Register index 6 is special, it reads from memory at the address in HL.       
    fn read_r8(&self, register: u8) -> u8 {
        match register {
            0 => self.registers.b,
            1 => self.registers.c,
            2 => self.registers.d,
            3 => self.registers.e,
            4 => self.registers.h,
            5 => self.registers.l,
            6 => self.bus.read_byte(self.registers.read_hl()),
            7 => self.registers.a,
            _ => unreachable!(),
        }
    }
    
    // write_r8 writes a value to an 8-bit register.
    // Register index 6 is special, it writes to memory at the address in HL.       
    fn write_r8(&mut self, register: u8, value: u8) {
        match register {
            0 => self.registers.b = value,
            1 => self.registers.c = value,
            2 => self.registers.d = value,
            3 => self.registers.e = value,
            4 => self.registers.h = value,
            5 => self.registers.l = value,
            6 => self.bus.write_byte(self.registers.read_hl(), value),
            7 => self.registers.a = value,
            _ => unreachable!(),
        }
    }
    
    fn read_r16(&self, register: u8) -> u16 {
        match register {
            0 => self.registers.read_bc(),
            1 => self.registers.read_de(),
            2 => self.registers.read_hl(),
            3 => self.registers.sp,
            _ => unreachable!()
        }
    }
    
    fn write_r16(&mut self, register: u8, value: u16) {
        match register {
            0 => self.registers.write_bc(value),
            1 => self.registers.write_de(value),
            2 => self.registers.write_hl(value),
            3 => self.registers.sp = value,
            _ => unreachable!()
        }
    }
    
    // fetch_imm8 reads one byte from memory and advances the program counter by 1.
    fn fetch_imm8(&mut self) -> u8 {
        self.fetch()
    }
    
    // fetch_imm16 reads two bytes from memory and returns them as one 16-bit little-endian value.
    // It also advances the program counter by 2.
    fn fetch_imm16(&mut self) -> u16 {
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        (hi << 8) | lo
    }
    
    // nop handles the NOP CPU instruction that does nothing (no-op) other than advance the program counter by 1.
    // Opcode: 0x00
    // Bytes: 1
    // Cycles: 1
    // Flags: ----
    fn nop(&mut self) {}
    
    // add handles the [ADD A,r] CPU instruction, which adds the contents of an 8-bit source register to the value in register A.
    // The new value is then stored in register A.
    // Opcode: 0x80 - 0x87
    // Bytes: 1
    // Cycles: 1 (2 for 0x86 when dealing with indirect memory access through register pair HL)
    // Flags: Z0HC
    fn add_a(&mut self, value: u8) {
        // Add the value from the source register to the value in register A and handle overflow
        let (result, overflow) = self.registers.a.overflowing_add(value);
        
        // Update flags
        self.registers.f.set_zero_flag(result == 0);
        self.registers.f.set_subtract_flag(false);
        self.registers.f.set_carry_flag(overflow);
        self.registers.f.set_half_carry_flag((self.registers.a & 0xF) + (value & 0xF) > 0xF);
        
        // Write updated value to register A
        self.registers.a = result;
    }
    
    // adc_a handles the [ADC A,r] CPU instruction, which adds the contents of an 8-bit source register and the Cy flag
    // to the value in register A. The new value is then stored in register A.
    // Opcode: 0x88 - 0x8F
    // Bytes: 1
    // Cycles: 1 (2 for 0x8E when dealing with indirect memory access through register pair HL)
    // Flags: Z0HC
    fn adc_a(&mut self, value: u8) {
        // Read the value currently in the carry flag
        let cy = self.registers.f.get_carry_flag() as u8;
        
        // Add the value from the source register, plus the value in the carry register,
        // to the value in register A and handle overflow
        // 
        // We perform this in two steps to detect overflow from either step
        let (result, overflow) = self.registers.a.overflowing_add(value);
        let (result, second_overflow) = result.overflowing_add(cy);
        
        // Update flags
        self.registers.f.set_zero_flag(result == 0);
        self.registers.f.set_subtract_flag(false);
        self.registers.f.set_carry_flag(overflow | second_overflow);
        self.registers.f.set_half_carry_flag((self.registers.a & 0xF) + (value & 0xF) + (cy & 0xF) > 0xF);
        
        // Write updated value to register A
        self.registers.a = result;
    }
    
    fn sub_a(&mut self, value: u8) {}
    
    fn sbc_a(&mut self, value: u8) {}
    
    fn and_a(&mut self, value: u8) {}
    
    fn xor_a(&mut self, value: u8) {}
    
    fn or_a(&mut self, value: u8) {}
    
    fn cp_a(&mut self, value: u8) {}
}
