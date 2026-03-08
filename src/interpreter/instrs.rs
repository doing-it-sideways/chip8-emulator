use super::{ Chip8, Address, font };

pub struct OpCode(u16);

impl OpCode {
    /// Provides shorthand to get the VX register from an instruction.
    pub fn x(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    /// Provides shorthand to get the VY register from an instruction.
    pub fn y(&self) -> u8 {
        ((self.0 & 0x00F0) >> 4) as u8
    }
}

// yes this is slightly inefficient, but makes intent clearer and match statements prettier,
// splitting code from fetch and exec
#[allow(non_camel_case_types)]
pub enum Instruction {
    nop,                // 0x0000
    call_mchn(Address), // 0NNN what is this instruction?????
    ClearScreen,        // 0x00E0
    ret,                // 0x00EE
    jmp(Address),       // 1NNN
    call(Address),      // 2NNN
    SkipEqNum(u8),      // 3XNN
    SkipNotEqNum(u8),   // 4XNN
    SkipEqReg,          // 5XY0
    ld_nn(u8),          // 6XNN
    add_nn(u8),         // 7XNN
    ld_reg,             // 8XY0
    or,                 // 8XY1
    and,                // 8XY2
    xor,                // 8XY3
    add_reg,            // 8XY4
    sub_reg,            // 8XY5
    lsr,                // 8XY6
    sub_reg_rev,        // 8XY7
    lsl,                // 8XYE
    SkipNotEqReg,       // 9XY0
    ld_pc(Address),     // ANNN
    jr(Address),        // BNNN
    GenRandom(u8),      // CXNN
    Draw(u8),           // DXYN
    SkipKeyPressed,     // EX9E
    SkipKeyNotPressed,  // EXA1
    ld_reg_delay,       // FX07
    wait_key,           // FX0A
    ld_delay_reg,       // FX15
    ld_sound_reg,       // FX18
    add_pc,             // FX1E
    LoadSpritePC,       // FX29
    ld_pc_bcd,          // FX33
    ld_pc_regs,         // FX55
    ld_regs_pc,         // FX65
}

pub fn fetch(instr: OpCode) -> Instruction {
    todo!()
}

pub fn exec(state: &mut Chip8, instr: Instruction) {
    todo!()
}
