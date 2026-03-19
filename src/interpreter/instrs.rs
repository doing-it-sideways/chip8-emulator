use super::{ 
    Chip8,
    Address,
    error::InterpreterErr
};

#[derive(Debug, Copy, Clone)]
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
/// List of instructions the Chip-8 interpreter can process.
/// Lowercase names are what I find to be similar to other instructions in either assembly or
/// similar to instructions I noticed on the Game Boy SOC.
/// Uppercase names denote instructions that seem more Chip-8 specific in my view.
// TODO: xo-chip extension
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    nop,                    // 0x0000
    call_mchn(Address),     // 0NNN
    ClearScreen,            // 0x00E0
    ret,                    // 0x00EE
    jmp(Address),           // 1NNN
    call(Address),          // 2NNN
    SkipEqNum(Reg, u8),     // 3XNN
    SkipNotEqNum(Reg, u8),  // 4XNN
    SkipEqReg(Reg, Reg),    // 5XY0
    ld_nn(Reg, u8),         // 6XNN
    add_nn(Reg, u8),        // 7XNN
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
    ld_i(Address),         // ANNN
    jr(Address),            // BNNN
    GenRandom(Reg, u8),     // CXNN
    Draw(Reg, Reg, u8),     // DXYN
    SkipKeyPressed(Reg),    // EX9E
    SkipKeyNotPressed(Reg), // EXA1
    ld_reg_delay(Reg),      // FX07
    WaitKey(Reg),          // FX0A
    ld_delay_reg(Reg),      // FX15
    ld_sound_reg(Reg),      // FX18
    add_pc(Reg),            // FX1E
    LoadSpritePC(Reg),      // FX29
    ld_i_bcd(Reg),         // FX33
    ld_i_regs(Reg),        // FX55
    ld_regs_i(Reg)         // FX65
}

pub fn decode(instr: u16) -> Result<Instruction, InterpreterErr> {
    use Instruction::*;
    let op_code = OpCode(instr);

    // todo: cleanup later?
    let instr = match op_code.into() {
        (0, x, y, z) => match (x, y, z) {
            (0, 0, 0) => nop,
            (0, 0xE, 0) => ClearScreen,
            (0, 0xE, 0xE) => ret,
            _ => call_mchn(op_code.into()),
        },
        (1, _, _, _) => jmp(op_code.into()),
        (2, _, _, _) => call(op_code.into()),
        (3, x, _, _) => SkipEqNum(x, op_code.nn()),
        (4, x, _, _) => SkipNotEqNum(x, op_code.nn()),
        (5, x, y, 0) => SkipEqReg(x, y),
        (6, x, _, _) => ld_nn(x, op_code.nn()),
        (7, x, _, _) => add_nn(x, op_code.nn()),
        (8, x, y, instr) => match instr {
            0 => ld_reg(x, y),
            1 => or(x, y),
            2 => and(x, y),
            3 => xor(x, y),
            4 => add_reg(x, y),
            5 => sub_reg(x, y),
            6 => lsr(x, y),
            7 => sub_reg_rev(x, y),
            0xE => lsl(x, y),
            _ => return Err(InterpreterErr::InvalidInstr),
        },
        (9, x, y, 0) => SkipNotEqReg(x, y),
        (0xA, _, _, _) => ld_i(op_code.into()),
        (0xB, _, _, _) => jr(op_code.into()),
        (0xC, x, _, _) => GenRandom(x, op_code.nn()),
        (0xD, x, y, n) => Draw(x, y, n),
        (0xE, x, 9, 0xE) => SkipKeyPressed(x),
        (0xE, x, 0xA, 1) => SkipKeyNotPressed(x),
        (0xF, x, _, _) => match op_code.nn() {
            0x07 => ld_reg_delay(x),
            0x0A => WaitKey(x),
            0x15 => ld_delay_reg(x),
            0x18 => ld_sound_reg(x),
            0x1E => add_pc(x),
            0x29 => LoadSpritePC(x),
            0x33 => ld_i_bcd(x),
            0x55 => ld_i_regs(x),
            0x65 => ld_regs_i(x),
            _ => return Err(InterpreterErr::InvalidInstr),
        },
        _ => return Err(InterpreterErr::InvalidInstr),
    };

    Ok(instr)
}

/// Instruction related functions
impl Chip8 {
    fn skip(&mut self) {
        todo!()
    }

    fn rand_mask(mask: u8) -> u8 {
        todo!()
    }

    fn store_bcd(&mut self, reg: usize) {
        todo!()
    }
}

pub fn exec(state: &mut Chip8, instr: Instruction) -> Result<(), InterpreterErr> {
    use Instruction::*;
    let regs = &mut state.reg;
    let v = &mut regs.v;
    
    match instr {
        nop => (),
        call_mchn(_) => (), // only used on real hardware
        jmp(addr) => regs.pc = addr.into(),
        jr(addr) => regs.pc = v[0] as u16 + Into::<u16>::into(addr),
        call(addr) => {
            let old_addr = state.reg.pc.into();
            state.push_addr(old_addr);
            state.reg.pc = addr.into();
        },
        ret => state.reg.pc = state.pop_addr()?.into(),
        ld_reg(x, y) => v[x as usize] = v[y as usize],
        ld_nn(reg, num) => v[reg as usize] = num,
        ld_i(addr) => regs.i = addr,
        ld_reg_delay(reg) => v[reg as usize] = state.timer_delay,
        ld_delay_reg(reg) => state.timer_delay = v[reg as usize],
        ld_sound_reg(reg) => state.timer_sound = v[reg as usize],
        ld_i_bcd(reg) => state.store_bcd(reg as usize),
        ld_i_regs(reg) => {
            let reg = reg as usize;
            let i = regs.i.0 as usize;
            state.ram[i..=(i + reg)].copy_from_slice(&v[0..=reg]);
        },
        ld_regs_i(reg) => {
            let reg = reg as usize;
            let i = regs.i.0 as usize;
            v[0..=reg].copy_from_slice(&state.ram[i..=(i + reg)])
        },
        SkipEqNum(reg, num) => if v[reg as usize] == num { state.skip() },
        SkipNotEqNum(reg, num) => if v[reg as usize] != num { state.skip() },
        SkipEqReg(x, y) => if v[x as usize] == v[y as usize] { state.skip() },
        SkipNotEqReg(x, y) => if v[x as usize] == v[y as usize] { state.skip() },
        SkipKeyPressed(reg) => todo!(),
        SkipKeyNotPressed(reg) => todo!(),
        add_reg(x, y) => todo!(),
        add_nn(reg, num) => v[reg as usize] = v[reg as usize].wrapping_add(num),
        add_pc(reg) => regs.i = regs.i.0.wrapping_add(v[reg as usize] as u16).into(),
        sub_reg(x, y) => todo!(),
        sub_reg_rev(x, y) => todo!(),
        or(x, y) => v[x as usize] |= v[y as usize],
        and(x, y) => v[x as usize] &= v[y as usize],
        xor(x, y) => v[x as usize] ^= v[y as usize],
        lsl(x, y) => {
            v[0xF] = v[y as usize] >> 7;
            v[x as usize] = v[y as usize] << 1;
        },
        lsr(x, y) => {
            v[0xF] = v[y as usize] & 1;
            v[x as usize] = v[y as usize] >> 1;
        },
        ClearScreen => todo!(),
        GenRandom(reg, num) => v[reg as usize] = Chip8::rand_mask(num),
        Draw(x, y, num) => todo!(),
        WaitKey(reg) => todo!(),
        LoadSpritePC(reg) => todo!(),
    }

    Ok(())
}
