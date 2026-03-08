use super::{ Address, font };

pub struct Instruction(u16);

impl Instruction {
    pub fn X(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    pub fn Y(&self) -> u8 {
        ((self.0 & 0x00F0) >> 4) as u8
    }
}

pub enum Instructions {
    
}

pub fn exec() {
    
}
