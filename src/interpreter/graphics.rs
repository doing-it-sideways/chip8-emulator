use std::{ 
    error::Error,
};

use super::{
    ProgramStatus,
    PixelBits,
};

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub trait Graphics {
    fn draw(&mut self, pixels: &PixelBits) -> Result<ProgramStatus, Box<dyn Error>>;
}
