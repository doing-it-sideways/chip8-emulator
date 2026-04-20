use std::{
    error::Error,
    fmt,
    time::Duration,
};

mod error;
mod instrs;
mod font;

mod graphics;
mod input;

use error::InterpreterErr;

#[derive(clap::ValueEnum, Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InterpreterMode {
    /// Original Chip-8 behavior on the COSMAC-VIP
    COSMAC,
    /// Behavior of the Chip-8 on the CHIP-48 / SUPER-CHIP
    SUPERCHIP,
    /// Same as SUPER-CHIP and enables instructions from the Octo extensions
    #[default]
    Octo,
}

#[derive(PartialEq, PartialOrd, Default, Copy, Clone)]
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

/// Literally just the regular debug print except hex value instead of decimal
impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Address").field(&format_args!("0x{:03X}", self.0)).finish()
    }
}

#[derive(Default, Debug)]
struct Registers {
    v: [u8; 0x10], // 16 general purpose register V0-VF, VF often modified by instructions as a flag register
    pc: u16,
    i: Address, // memory address register? related to io for memory
    sp: u16,
}

#[derive(Default, Debug)]
struct PixelBits([u64; 0x20]);

const ROM_START: usize = 0x200;
const PC_INIT: u16 = ROM_START as u16;
const PC_MAX: u16 = 0xE8F;
const RAM_DEFAULT_SIZE: usize = 0x1000;
const STACK_DEFAULT_SIZE: usize = 0x100;

#[derive(Default, Debug)]
// 60 t/s
struct Chip8 {
    ram: Vec<u8>,
    // normally, stack would grow down and be made of bytes,
    // but with vec we grow it up cause for chip-8 it doesn't matter cause stack is only for addresses
    stack: Vec<Address>,
    
    pixels: PixelBits,
    
    reg: Registers,
    input: u16, // bitwise 16 buttons 0 = msb, 0xF = lsb

    timer_delay: u8,
    timer_sound: u8,

    chip_behavior: InterpreterMode,
}

pub enum ProgramStatus {
    Ok,
    Quit,
}

pub fn run(rom_data: Vec<u8>, window_scale: u8, chip_behavior: InterpreterMode) -> 
    Result<ProgramStatus, Box<dyn Error>>
{
    let mut chip8 = Chip8::new(rom_data, chip_behavior);
    
    let sdl_ctx = sdl3::init()?;
    let mut event_pump = sdl_ctx.event_pump()?;
    let mut gctx = graphics::GraphicsCtx::init(&sdl_ctx, window_scale)?;

    'runloop: loop {
        match input::update(&mut event_pump) {
            (new_input, ProgramStatus::Ok) => chip8.input = new_input,
            (_, ProgramStatus::Quit) => break 'runloop,
        };
        
        for _ in 0..10 {
            let instr = chip8.fetch();
            let cur_instr = instrs::decode(instr)?;
            println!("Cur instruction (0x{:04X}): {:?}", instr, cur_instr);

            instrs::exec(&mut chip8, cur_instr)?;
        }

        if let Some(val) = chip8.timer_delay.checked_sub(1) {
            chip8.timer_delay = val;
        }

        if chip8.timer_sound > 0 {
            // cosmac only played sounds on values > 1
            if chip8.chip_behavior >= InterpreterMode::SUPERCHIP || chip8.timer_sound > 1 {
                // TODO: play sound
            }
            
            chip8.timer_sound -= 1;
        }

        if let ProgramStatus::Quit = gctx.draw(&chip8.pixels)? {
            break 'runloop;
        }

        std::thread::sleep(Duration::from_secs_f64(1.0 / 60.0));
    }

    Ok(ProgramStatus::Quit)
}

/// General functions
impl Chip8 {
    fn new(rom_data: Vec<u8>, chip_behavior: InterpreterMode) -> Self {
        assert!(rom_data.len() <= RAM_DEFAULT_SIZE - ROM_START);

        let mut chip8 = Chip8 {
            ram: vec![0; RAM_DEFAULT_SIZE.into()],
            reg: Registers {
                pc: PC_INIT,
                ..Registers::default()
            },
            chip_behavior,
            ..Chip8::default()
        };

        chip8.stack.reserve(STACK_DEFAULT_SIZE.into());

        chip8.ram[..font::FONT_BYTES].copy_from_slice(&font::get_bytes());
        chip8.ram[ROM_START..ROM_START + rom_data.len()].copy_from_slice(&rom_data);
        
        chip8
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

    fn is_key_pressed(&self, key: u8) -> bool {
        assert!(key <= 0xF);

        (self.input >> key) & 1 == 1
    }
}

impl PixelBits {
    fn get(&self, x: u8, y: u8) -> u8 {
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
