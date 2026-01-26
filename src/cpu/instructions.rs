use super::CPU;

pub const INSTRUCTIONS: [fn(&mut CPU); 1] = [
    CPU::nop // 0x00
    
    // TODO: continue to add opcode handler functions here and increase the array size as needed.
];