use std::error::Error;

#[derive(Debug)]
pub enum InterpreterErr {
    StackErr,
    MemErr,
    InvalidInstr,
    APIError(String),
}

impl std::fmt::Display for InterpreterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InterpreterErr::*;
        write!(f, "Chip-8 interpreter error: {}", match *self {
            InvalidInstr => "Invalid instruction",
            _ => "Unhandled error"
        })
    }
}

impl Error for InterpreterErr {}