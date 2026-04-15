use std::error::Error;

#[derive(Debug)]
pub enum InterpreterErr {
    StackErr,
    MemErr,
    InvalidInstr
}

impl std::fmt::Display for InterpreterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chip-8 interpreter error: {}", match *self {
            Self::InvalidInstr => "Invalid instruction",
            _ => "Unhandled error"
        })
    }
}

impl Error for InterpreterErr {}