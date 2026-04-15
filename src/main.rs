use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    error::Error,
};

use clap::{
    Parser,
    ValueEnum,
};

mod interpreter;

use interpreter::InterpreterMode;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Path of Chip-8 program to run.
    rom_path: PathBuf,

    /// Scale of the display window. (1 = 64x32)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..), default_value_t = 10)]
    scale: u8,

    /// Determines the behavior of some instructions.
    #[arg(short = 'C', value_enum, default_value_t = InterpreterMode::Octo)]
    chip_behavior: InterpreterMode,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Path: {:?}\nScreen Scale: {}\nChip Behavior: {:?}",
             args.rom_path, args.scale, args.chip_behavior);

    let mut file = File::open(args.rom_path)?;
    let mut rom_buf = Vec::new();
    file.read_to_end(&mut rom_buf)?;
    
    interpreter::run(rom_buf, args.scale, args.chip_behavior)?;

    Ok(())
}
