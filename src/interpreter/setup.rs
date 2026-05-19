use std::{
    path::PathBuf,
};

use clap::{
    Parser,
    ValueEnum,
};

use we_clap::WeParser;

#[derive(ValueEnum, Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InterpreterMode {
    /// Original Chip-8 behavior on the COSMAC-VIP
    COSMAC,
    /// Behavior of the Chip-8 on the CHIP-48 / SUPER-CHIP
    SUPERCHIP,
    /// Same as SUPER-CHIP and enables instructions from the Octo extensions
    #[default]
    Octo,
}

impl WeParser for Args {}

#[derive(Parser, Debug, Default)]
#[command(about, long_about = None)]
struct Args {
    /// Path of Chip-8 program to run.
    rom_path: PathBuf,

    /// Scale of the display window. (1 = 64x32)
    #[arg(short = 's', long, value_parser = clap::value_parser!(u8).range(1..), default_value_t = 10)]
    window_scale: u8,

    /// Determines the behavior of some instructions.
    #[arg(short = 'C', value_enum, default_value_t = InterpreterMode::Octo)]
    chip_behavior: InterpreterMode,
}

pub struct Settings {
    pub(super) rom_path: PathBuf,
    pub        window_scale: u8,
    pub(super) chip_behavior: InterpreterMode,
}

pub fn setup() -> Settings {
    let args: Args = Args::we_parse();

    println!("Path: {:?}\nScreen Scale: {}\nChip Behavior: {:?}",
             args.rom_path, args.window_scale, args.chip_behavior);

    Settings {
        rom_path: args.rom_path,
        window_scale: args.window_scale,
        chip_behavior: args.chip_behavior,
    }
}
