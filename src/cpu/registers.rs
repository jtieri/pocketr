use super::flags::Flags;

// The Gameboy uses 8-bit registers but has instructions that allow games to read and write 16 bits of data.
#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16
}

// We treat the "Hi" register as a u16 which effectively just adds a byte of all 0s to the significant position.
// Then we shift the b register 8 positions so that it's occupying the most significant byte position.
// We then bitwise OR the c register so that the result is a two byte number with the contents of b in the most significant byte postion
// and the contents of c in the least significant byte position.
impl Registers {
    pub fn read_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f.0 as u16
    }
    
    pub fn write_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = Flags((value & 0xFF) as u8);
        self.f.sanitize(); // Important! Lower nibble must be 0
    }
    
    pub fn read_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn write_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn read_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn write_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    
    pub fn read_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    
    pub fn write_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
