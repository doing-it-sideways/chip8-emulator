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

struct Registers {
    v: [u8; 0x10], // 16 general purpose register V0-VF, VF often modified by instructions as a flag register
    i: Address, // program counter
}

struct Chip8 {
    reg: Registers,
    pixels: [u64; 0x20] // 64x32 on/off values
}

pub fn run(rom_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    Ok(())
}