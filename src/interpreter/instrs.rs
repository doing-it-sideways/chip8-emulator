use super::{ 
    Chip8,
    Address,
    error::InterpreterErr
};

pub struct OpCode(u16);

type Reg = u8;

impl OpCode {
    fn lead_nibble(&self) -> u8 {
        ((self.0 & 0xF000) >> 12) as u8
    }

    /// Provides shorthand to get the VX register from some instructions.
    fn x(&self) -> Reg {
        ((self.0 & 0x0F00) >> 8) as Reg
    }

    /// Provides shorthand to get the VY register from some instructions.
    fn y(&self) -> Reg {
        ((self.0 & 0x00F0) >> 4) as Reg
    }

    /// Provides shorthand to get the byte value at the end of some instructions.
    fn nn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Provides shorthand to get the nibble value at the end of some instructions.
    fn n(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }
}

impl Into<Address> for OpCode {
    fn into(self) -> Address {
        Address::from(self.0)
    }
}

impl Into<(u8, u8, u8, u8)> for OpCode {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.lead_nibble(), self.x(), self.y(), self.n())
    }
}

// yes this is slightly inefficient, but makes intent clearer and match statements prettier,
// splitting code from fetch and exec
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Instruction {
    nop,                    // 0x0000
    call_mchn(Address),     // 0NNN what is this instruction?????
    ClearScreen,            // 0x00E0
    ret,                    // 0x00EE
    jmp(Address),           // 1NNN
    call(Address),          // 2NNN
    SkipEqNum(Reg, u8),     // 3XNN
    SkipNotEqNum(Reg, u8),  // 4XNN
    SkipEqReg(Reg, Reg),    // 5XY0
    ld_nn(Reg, Reg),        // 6XNN
    add_nn(Reg, Reg),       // 7XNN
    ld_reg(Reg, Reg),       // 8XY0
    or(Reg, Reg),           // 8XY1
    and(Reg, Reg),          // 8XY2
    xor(Reg, Reg),          // 8XY3
    add_reg(Reg, Reg),      // 8XY4
    sub_reg(Reg, Reg),      // 8XY5
    lsr(Reg, Reg),          // 8XY6
    sub_reg_rev(Reg, Reg),  // 8XY7
    lsl(Reg, Reg),          // 8XYE
    SkipNotEqReg(Reg, Reg), // 9XY0
    ld_pc(Address),         // ANNN
    jr(Address),            // BNNN
    GenRandom(Reg, u8),     // CXNN
    Draw(Reg, Reg, u8),     // DXYN
    SkipKeyPressed(Reg),    // EX9E
    SkipKeyNotPressed(Reg), // EXA1
    ld_reg_delay(Reg),      // FX07
    wait_key(Reg),          // FX0A
    ld_delay_reg(Reg),      // FX15
    ld_sound_reg(Reg),      // FX18
    add_pc(Reg),            // FX1E
    LoadSpritePC(Reg),      // FX29
    ld_pc_bcd(Reg),         // FX33
    ld_pc_regs(Reg),        // FX55
    ld_regs_pc(Reg),        // FX65
}

pub fn fetch(instr: u16) -> Result<Instruction, InterpreterErr> {
    use Instruction::*;
    let op_code = OpCode(instr);

    let instr = match op_code.lead_nibble() {
        0 => {
            nop
        },
        1 => jmp(op_code.into()),
        2 => call(op_code.into()),
        3 => SkipEqNum(op_code.x(), op_code.nn()),
        4 => SkipNotEqNum(op_code.x(), op_code.nn()),
        _ => return Err(InterpreterErr::InvalidInstr),
    };

    Ok(instr)
}

pub fn exec(state: &mut Chip8, instr: Instruction) {
    use Instruction::*;
    todo!()
}
