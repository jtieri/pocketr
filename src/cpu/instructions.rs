use super::CPU;
use super::registers::Register8Bit;

// TODO: continue to add opcode handler functions here.
pub const INSTRUCTIONS: [fn(&mut CPU); 256] = {
    // TODO: once every instruction handler is implemented we can refactor this code to remove the unnecessary allocation of the unimplmented elements
    let mut table: [fn(&mut CPU); 256] = [CPU::unimplemented; 256];
    
    table[0x00] = CPU::nop;
    
    
    table[0x80] = |cpu| cpu.add_a(Register8Bit::B);
    table[0x81] = |cpu| cpu.add_a(Register8Bit::C);
    table[0x82] = |cpu| cpu.add_a(Register8Bit::D);
    table[0x83] = |cpu| cpu.add_a(Register8Bit::E);
    table[0x84] = |cpu| cpu.add_a(Register8Bit::H);
    table[0x85] = |cpu| cpu.add_a(Register8Bit::L);
    table[0x86] = |cpu| cpu.add_a(Register8Bit::HLIndirect);
    table[0x87] = |cpu| cpu.add_a(Register8Bit::A);
    
    table[0x88] = |cpu| cpu.adc_a(Register8Bit::B);
    table[0x89] = |cpu| cpu.adc_a(Register8Bit::C);
    table[0x8A] = |cpu| cpu.adc_a(Register8Bit::D);
    table[0x8B] = |cpu| cpu.adc_a(Register8Bit::E);
    table[0x8C] = |cpu| cpu.adc_a(Register8Bit::H);
    table[0x8D] = |cpu| cpu.adc_a(Register8Bit::L);
    table[0x8E] = |cpu| cpu.adc_a(Register8Bit::HLIndirect);
    table[0x8F] = |cpu| cpu.adc_a(Register8Bit::A);

    
    table
};

// TODO: add prefixed instruction handler functions to the prefixed table.
pub const PREFIXED_INSTRUCTIONS: [fn(&mut CPU); 256]  = {
    // TODO: once every instruction handler is implemented we can refactor this code to remove the unnecessary allocation of the unimplmented elements
    let mut table: [fn(&mut CPU); 256] = [CPU::unimplemented; 256];
    
    table
};