use super::CPU;
use super::registers::Register8Bit;

// TODO: continue to add opcode handler functions here.
pub const INSTRUCTIONS: [fn(&mut CPU); 256] = {
    let mut table: [fn(&mut CPU); 256] = [CPU::unimplemented; 256];
    
    table[0x00] = CPU::nop; // 0x00
    
    
    table[0x80] = |cpu| cpu.add_a(Register8Bit::B); // 0x80
    table[0x81] = |cpu| cpu.add_a(Register8Bit::C); // 0x81
    table[0x82] = |cpu| cpu.add_a(Register8Bit::D); // 0x82
    table[0x83] = |cpu| cpu.add_a(Register8Bit::E); // 0x83
    table[0x84] = |cpu| cpu.add_a(Register8Bit::H); // 0x84
    table[0x85] = |cpu| cpu.add_a(Register8Bit::L); // 0x85
    table[0x86] = |cpu| cpu.add_a(Register8Bit::HLIndirect); // 0x86
    table[0x87] = |cpu| cpu.add_a(Register8Bit::A); // 0x87
    
    table
};

// TODO: add prefixed instruction handler functions to the prefixed table.
pub const PREFIXED_INSTRUCTIONS: [fn(&mut CPU); 256]  = {
    let mut table: [fn(&mut CPU); 256] = [CPU::unimplemented; 256];
    table
};