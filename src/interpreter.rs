use std::{
    error::Error,
    fmt,
    time::Duration,
};

pub mod setup;
pub mod error;

pub mod graphics;
pub mod input;

mod instrs;
mod font;

use error::InterpreterErr;
use setup::InterpreterMode;

#[derive(PartialEq, PartialOrd, Default, Copy, Clone)]
struct Address(u16);

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        Address(addr & 0x0FFF)
    }
}

impl From<Address> for u16 {
    fn from(addr: Address) -> Self {
        addr.0 & 0x0FFF
    }
}

/// Literally just the regular debug print except hex value instead of decimal
impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Address").field(&format_args!("0x{:03X}", self.0)).finish()
    }
}

type Reg = u8;

#[derive(Default, Debug)]
struct Registers {
    v: [Reg; 0x10], // 16 general purpose register V0-VF, VF often modified by instructions as a flag register
    pc: u16,
    i: Address, // memory address register? related to io for memory
    sp: u16,
}

#[derive(Default, Debug)]
pub struct PixelBits([u64; 0x20]);

const ROM_START: usize = 0x200;
const PC_INIT: u16 = ROM_START as u16;
const PC_MAX: u16 = 0xE8F;
const RAM_DEFAULT_SIZE: usize = 0x1000;
const STACK_DEFAULT_SIZE: usize = 0x100;

#[derive(Default, Debug)]
// 60 t/s
pub struct Chip8 {
    ram: Vec<u8>,
    // normally, stack would grow down and be made of bytes,
    // but with vec we grow it up cause for chip-8 it doesn't matter cause stack is only for addresses
    stack: Vec<Address>,
    
    pixels: PixelBits,
    
    reg: Registers,
    sprite_to_draw: Option<(Reg, Reg, u8)>,
    input: u16, // bitwise 16 buttons 0 = msb, 0xF = lsb
    // on the COSMAC, WaitKey needs to wait for a key release, so we must store the key press
    cosmac_keypress: Option<u8>,

    timer_delay: u8,
    timer_sound: u8,

    chip_behavior: InterpreterMode,
}

#[derive(PartialEq)]
pub enum ProgramStatus {
    Ok,
    Quit,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run(settings: &setup::Settings,
           mut gctx: impl graphics::Graphics, mut ihandle: impl input::InputHandler)
           -> Result<ProgramStatus, Box<dyn Error>>
{
    let mut chip8 = Chip8::new(std::fs::read(settings.rom_path.as_path())?, 
                               settings.chip_behavior);

    'runloop: loop {
        match chip8.tick(&mut gctx, &mut ihandle)? {
            ProgramStatus::Quit => break 'runloop,
            _ => (),
        }

        std::thread::sleep(Duration::from_secs_f64(1.0 / 60.0));
    }

    Ok(ProgramStatus::Quit)
}

/// General functions
impl Chip8 {
    fn init(chip_behavior: InterpreterMode) -> Self {
        let mut chip8 = Self {
            ram: vec![0; RAM_DEFAULT_SIZE.into()],
            reg: Registers {
                pc: PC_INIT,
                ..Registers::default()
            },
            chip_behavior,
            ..Self::default()
        };

        chip8.stack.reserve(STACK_DEFAULT_SIZE.into());
        chip8.ram[..font::FONT_BYTES].copy_from_slice(&font::get_bytes());

        chip8
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(rom_data: Vec<u8>, chip_behavior: InterpreterMode) -> Self {
        assert!(rom_data.len() <= RAM_DEFAULT_SIZE - ROM_START);

        let mut chip8 = Chip8::init(chip_behavior);

        chip8.ram[ROM_START..ROM_START + rom_data.len()].copy_from_slice(&rom_data);
        
        chip8
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        Chip8::init(InterpreterMode::default())
    }

    pub fn reload(&mut self, mode: InterpreterMode, new_rom: Option<&[u8]>) {
        unsafe { self.ram.as_mut_ptr().write_bytes(0u8, RAM_DEFAULT_SIZE); }
        self.stack.clear();

        self.pixels = PixelBits::default();

        self.reg = Registers {
            pc: PC_INIT,
            ..Registers::default()
        };
        
        self.sprite_to_draw = None;
        self.input = 0;
        self.cosmac_keypress = None;

        self.timer_delay = 0;
        self.timer_sound = 0;

        self.chip_behavior = mode;

        if let Some(rom_data) = new_rom {
            self.ram[ROM_START..].fill(0);
            assert!(rom_data.len() <= RAM_DEFAULT_SIZE - ROM_START);
            self.ram[ROM_START..ROM_START + rom_data.len()].copy_from_slice(rom_data);
        }
    }

    pub fn tick(&mut self,
                gctx: &mut impl graphics::Graphics, ihandle: &mut impl input::InputHandler)
                -> Result<ProgramStatus, Box<dyn Error>>
    {
        for _ in 0..10 {
            let input_res = ihandle.handle(&mut self.input);
            if input_res == ProgramStatus::Quit {
                return Ok(ProgramStatus::Quit);
            }

            let instr = self.fetch();
            let cur_instr = instrs::decode(instr)?;
            println!("Cur instruction (0x{:04X}): {:?}", instr, cur_instr);

            instrs::exec(self, cur_instr)?;
        }

        if let Some((x, y, num)) = self.sprite_to_draw {
            self.set_pixels(x, y, num);
            self.reg.pc += 2;
            self.sprite_to_draw = None;
        }

        if let Some(val) = self.timer_delay.checked_sub(1) {
            self.timer_delay = val;
        }

        if self.timer_sound > 0 {
            // cosmac only played sounds on values > 1
            if self.chip_behavior >= InterpreterMode::SUPERCHIP || self.timer_sound > 1 {
                // TODO: play sound
            }
            
            self.timer_sound -= 1;
        }

        if let ProgramStatus::Quit = gctx.draw(&self.pixels)? {
            return Ok(ProgramStatus::Quit);
        }

        Ok(ProgramStatus::Ok)
    }

    fn fetch(&mut self) -> u16 {
        let pc = &mut self.reg.pc;
        let hi = self.ram[*pc as usize];
        let lo = self.ram[(*pc + 1) as usize];
        *pc += 2;

        ((hi as u16) << 8) | lo as u16
    }

    fn push_addr(&mut self, addr: Address) {
        self.stack.push(addr);
    }

    fn pop_addr(&mut self) -> Result<Address, InterpreterErr> {
        if let Some(addr) = self.stack.pop() {
            Ok(addr)
        }
        else {
            Err(InterpreterErr::StackErr)
        }
    }
}

impl PixelBits {
    pub fn get(&self, x: u8, y: u8) -> u8 {
        let val = self.0[y as usize] >> (x % graphics::WIDTH as u8) as u64;
        (val & 1) as u8
    }

    fn set(&mut self, x: u8, y: u8, val: u8) {
        if val == 0x1 {
            self.0[y as usize] |= 1 << (x % graphics::WIDTH as u8);
        }
        else {
            self.0[y as usize] &= !(1 << (x % graphics::WIDTH as u8));
        }
    }
}

impl fmt::Display for PixelBits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n")?;
        for y in 0..graphics::HEIGHT {
            write!(f, "[")?;
            for x in 0..graphics::WIDTH {
                write!(f, "{}, ", (self.0[y as usize] >> x) & 1)?;
            }
            write!(f, "]\n")?;
        }
        write!(f, "]")
    }
}
