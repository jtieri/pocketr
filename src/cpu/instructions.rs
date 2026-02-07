use super::CPU;
use super::registers::Register8Bit;

// TODO: continue to add opcode handler functions here and increase the array size as needed.
pub const INSTRUCTIONS: [fn(&mut CPU); 9] = [
    CPU::nop, // 0x00
    |cpu| cpu.add_a(Register8Bit::B), // 0x80
    |cpu| cpu.add_a(Register8Bit::C), // 0x81
    |cpu| cpu.add_a(Register8Bit::D), // 0x82
    |cpu| cpu.add_a(Register8Bit::E), // 0x83
    |cpu| cpu.add_a(Register8Bit::H), // 0x84
    |cpu| cpu.add_a(Register8Bit::L), // 0x85
    |cpu| cpu.add_a(Register8Bit::HLIndirect), // 0x86
    |cpu| cpu.add_a(Register8Bit::A) // 0x87
];