use std::{
    error::Error,
};

mod error;
mod instrs;
mod font;

mod graphics;
mod input;

#[derive(PartialEq, PartialOrd, Default, Debug)]
struct Address(u16);

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        Address(addr & 0x0FFF)
    }
}

const PC_INIT: Address = Address(0x200);

#[derive(Default, Debug)]
struct Registers {
    v: [u8; 0x10], // 16 general purpose register V0-VF, VF often modified by instructions as a flag register
    i: Address, // program counter
    sp: Address,
}

const RAM_DEFAULT_SIZE: u16 = 0x1000;
const STACK_DEFAULT_SIZE: u16 = 0x100;

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
    fn new() -> Self {
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
        
        chip8
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

pub fn run(rom_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut chip8 = Chip8::new();

    'run: loop {
        break 'run; // TODO
    }

    Ok(())
}