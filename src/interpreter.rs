use std::{
    error::Error,
};

mod error;
mod instrs;
mod font;

#[derive(PartialEq, PartialOrd)]
struct Address(u16);

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        Address(addr & 0x0FFF)
    }
}

pub fn run(rom_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    Ok(())
}