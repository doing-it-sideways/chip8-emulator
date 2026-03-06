use std::error::Error;

#[derive(Debug)]
pub enum InterpreterErr {

}

impl std::fmt::Display for InterpreterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl Error for InterpreterErr {}

mod font;

pub fn run(rom_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    Ok(())
}