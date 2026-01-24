use flags::FlagsRegister;

mod flags;

// The Gameboy uses 8-bit registers but has instructions  that allow games to read and write 16 bits of data.
#[derive(Debug, Default)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

// We treat the "Hi" register as a u16 which effectively just adds a byte of all 0s to the significant position.
// Then we shift the b register 8 positions so that it's occupying the most significant byte position.
// We then bitwise OR the c register so that the result is a two byte number with the contents of b in the most significant byte postion
// and the contents of c in the least significant byte position.
impl Registers {
    fn read_bc(&self) -> u16 {
        (self.b as u16) << 8 | 
        self.c as u16
    }
    
    fn write_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    
    fn read_de(&self) -> u16 {
        (self.d as u16) << 8 |
        self.e as u16
    }
    
    fn write_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
}
