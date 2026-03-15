use std::{
    error::Error,
};

mod error;
mod instrs;
mod font;

mod graphics;
mod input;

use error::InterpreterErr;

#[derive(PartialEq, PartialOrd, Default, Debug, Copy, Clone)]
struct Address(u16);

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        Address(addr & 0x0FFF)
    }
}

impl From<Address> for u16 {
    fn from(addr: Address) -> Self {
        addr.0
    }
}

const ROM_START: usize = 0x200;
const PC_INIT: Address = Address(ROM_START as u16);

#[derive(Default, Debug)]
struct Registers {
    v: [u8; 0x10], // 16 general purpose register V0-VF, VF often modified by instructions as a flag register
    i: Address, // program counter
    sp: u16,
}

const RAM_DEFAULT_SIZE: usize = 0x1000;
const STACK_DEFAULT_SIZE: usize = 0x100;

#[derive(Default, Debug)]
// 60 t/s
struct Chip8 {
    ram: Vec<u8>,
    // normally, stack would grow down and be made of bytes,
    // but with vec we grow it up cause for chip-8 it doesn't matter cause stack is only for addresses
    stack: Vec<Address>,
    
    pixels: [u64; 0x20], // 64x32 on/off values
    
    reg: Registers,
    input: u16, // bitwise 16 buttons

    timer_delay: u8,
    timer_sound: u8,
}

impl Chip8 {
    fn new(rom_data: Vec<u8>) -> Self {
        assert!(rom_data.len() <= RAM_DEFAULT_SIZE - ROM_START);

        let mut chip8 = Chip8 {
            ram: vec![0; RAM_DEFAULT_SIZE.into()],
            reg: Registers {
                i: PC_INIT,
                ..Registers::default()
            },
            ..Chip8::default()
        };

        chip8.stack.reserve(STACK_DEFAULT_SIZE.into());

        chip8.ram[..font::FONT_BYTES].copy_from_slice(&font::get_bytes());
        chip8.ram[ROM_START..ROM_START + rom_data.len()].copy_from_slice(&rom_data);
        
        chip8
    }

    fn instr(&mut self) -> u16 {
        let mut i: u16 = self.reg.i.into();
        let hi = self.ram[i as usize];
        let lo = self.ram[(i + 1) as usize];
        i += 2;

        self.reg.i = Address::from(i);

        ((hi as u16) << 8) | lo as u16
    }

    fn push_addr(&mut self, addr: Address) {
        self.stack.push(addr);
    }

    fn pop_addr(&mut self) -> Result<Address, error::InterpreterErr> {
        if let Some(addr) = self.stack.pop() {
            Ok(addr)
        }
        else {
            Err(error::InterpreterErr::StackErr)
        }
    }
}

pub fn run(rom_data: Vec<u8>) -> Result<(), InterpreterErr> {
    let mut chip8 = Chip8::new(rom_data);

    'run: loop {
        let cur_instr = instrs::fetch(chip8.instr())?;
        println!("Cur instruction ({}): {:?}", chip8.instr(), cur_instr);

        instrs::exec(&mut chip8, cur_instr)?;
    }

    Ok(())
}