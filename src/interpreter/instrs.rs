use super::{ Address, font };

pub struct Instruction(u16);

impl Instruction {
    pub fn x(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    pub fn y(&self) -> u8 {
        ((self.0 & 0x00F0) >> 4) as u8
    }
}

pub enum Instructions {
    
}

pub fn exec() {
    
}
