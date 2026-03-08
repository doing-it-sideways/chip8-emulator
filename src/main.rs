use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    error::Error,
};

use clap::Parser;

mod interpreter;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Path of Chip-8 program to run.
    rom_path: PathBuf,

    /// Scale of the display window. (1 = 64x32)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..), default_value_t = 1)]
    scale: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Path: {:?}\nScreen Scale: {}", args.rom_path, args.scale);

    let mut file = File::open(args.rom_path)?;
    let mut rom_buf = Vec::new();
    file.read_to_end(&mut rom_buf)?;

    interpreter::run(rom_buf)
}
