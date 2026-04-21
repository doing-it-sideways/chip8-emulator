use super::{ 
    Chip8,
    Address,
    error::InterpreterErr,
    PC_MAX,
    graphics::{ WIDTH, HEIGHT },
    InterpreterMode,
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
#[derive(Debug)]
pub enum Instruction {
    nop,                    // 0000
    #[allow(unused)]
    call_mchn(Address),     // 0NNN -- this instruction is treated as a nop
    s_scd(u8),              // 00CN -- SUPER-CHIP+
    o_scu(u8),              // 00DN -- Octo+
    ClearScreen,            // 00E0
    ret,                    // 00EE
    s_scr,                  // 00FB -- SUPER-CHIP+
    s_scl,                  // 00FC -- SUPER-CHIP+
    s_exit,                 // 00FD -- SUPER-CHIP+
    s_lores,                // 00FE -- SUPER-CHIP+
    s_hires,                // 00FF -- SUPER-CHIP+
    jmp(Address),           // 1NNN
    call(Address),          // 2NNN
    SkipEqNum(Reg, u8),     // 3XNN
    SkipNotEqNum(Reg, u8),  // 4XNN
    SkipEqReg(Reg, Reg),    // 5XY0
    o_ld_regxy_i(Reg, Reg), // 5XY2 -- Octo+
    o_ld_i_regxy(Reg, Reg), // 5XY3 -- Octo+
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
    ld_i(Address),          // ANNN
    jr(Address),            // BNNN -- Different behavior on SUPER-CHIP
    GenRandom(Reg, u8),     // CXNN
    s_Draw16x16(Reg, Reg),  // DXY0 -- SUPER-CHIP+
    Draw(Reg, Reg, u8),     // DXYN
    SkipKeyPressed(Reg),    // EX9E
    SkipKeyNotPressed(Reg), // EXA1
    o_ld_ipc_nnnn,          // F000 -- Octo+
    o_Plane(Reg),           // FX01 -- Octo+
    o_ld_audio_i,           // F002 -- Octo+
    ld_reg_delay(Reg),      // FX07
    WaitKey(Reg),           // FX0A
    ld_delay_reg(Reg),      // FX15
    ld_sound_reg(Reg),      // FX18
    add_pc(Reg),            // FX1E
    ld_i_font(Reg),         // FX29
    s_ld_i_font10(Reg),     // FX30 -- SUPER-CHIP+
    ld_i_bcd(Reg),          // FX33
    o_SetPitch(Reg),        // FX3A -- Octo+
    ld_i_regs(Reg),         // FX55
    ld_regs_i(Reg),         // FX65
    s_ld_flags_regs(Reg),   // FX75 -- SUPER-CHIP+
    s_ld_regs_flags(Reg),   // FX85 -- SUPER-CHIP+
}

pub fn decode(instr: u16) -> Result<Instruction, InterpreterErr> {
    use Instruction::*;
    let op_code = OpCode(instr);

    // todo: cleanup later?
    let instr = match op_code.into() {
        (0, x, y, z) => match (x, y, z) {
            (0, 0, 0) => nop,
            (0, 0xC, n) => s_scd(n),
            (0, 0xD, n) => o_scu(n),
            (0, 0xE, 0) => ClearScreen,
            (0, 0xE, 0xE) => ret,
            (0, 0xF, 0xB) => s_scr,
            (0, 0xF, 0xC) => s_scl,
            (0, 0xF, 0xD) => s_exit,
            (0, 0xF, 0xE) => s_lores,
            (0, 0xF, 0xF) => s_hires,
            _ => call_mchn(op_code.into()),
        },
        (1, _, _, _) => jmp(op_code.into()),
        (2, _, _, _) => call(op_code.into()),
        (3, x, _, _) => SkipEqNum(x, op_code.nn()),
        (4, x, _, _) => SkipNotEqNum(x, op_code.nn()),
        (5, x, y, 0) => SkipEqReg(x, y),
        (5, x, y, 2) => o_ld_regxy_i(x, y),
        (5, x, y, 3) => o_ld_i_regxy(x, y),
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
        (0xD, x, y, 0) => s_Draw16x16(x, y),
        (0xD, x, y, n) => Draw(x, y, n),
        (0xE, x, 9, 0xE) => SkipKeyPressed(x),
        (0xE, x, 0xA, 1) => SkipKeyNotPressed(x),
        (0xF, 0, 0, 0) => o_ld_ipc_nnnn,
        (0xF, 0, 0, 2) => o_ld_audio_i,
        (0xF, x, _, _) => match op_code.nn() {
            0x01 => o_Plane(x),
            0x07 => ld_reg_delay(x),
            0x0A => WaitKey(x),
            0x15 => ld_delay_reg(x),
            0x18 => ld_sound_reg(x),
            0x1E => add_pc(x),
            0x29 => ld_i_font(x),
            0x30 => s_ld_i_font10(x),
            0x33 => ld_i_bcd(x),
            0x3A => o_SetPitch(x),
            0x55 => ld_i_regs(x),
            0x65 => ld_regs_i(x),
            0x75 => s_ld_flags_regs(x),
            0x85 => s_ld_regs_flags(x),
            _ => return Err(InterpreterErr::InvalidInstr),
        },
        _ => return Err(InterpreterErr::InvalidInstr),
    };

    Ok(instr)
}

/// Instruction related functions
impl Chip8 {
    fn skip(&mut self) -> Result<(), InterpreterErr> {
        self.reg.pc += 2;

        if self.reg.pc > PC_MAX {
            Err(InterpreterErr::MemErr)
        }
        else {
            Ok(())
        }
    }

    fn rand_mask(mask: u8) -> u8 {
        rand::random::<u8>() & mask
    }

    fn store_bcd(&mut self, reg: usize) {
        let mut val = self.reg.v[reg];
        let i = self.reg.i.0 as usize;

        self.ram[i + 2] = val % 10;
        val /= 10;
        self.ram[i + 1] = val % 10;
        val /= 10;
        self.ram[i] = val;
    }

    fn any_key_pressed(&self) -> bool {
        self.input != 0
    }

    fn is_key_reg_pressed(&self, reg: usize) -> bool {
        let key = self.reg.v[reg];
        assert!(key <= 0xF);

        self.is_key_pressed(key)
    }

    fn set_pixels(&mut self, x: u8, y: u8, bytes: u8) {
        // TODO: simulate waiting for vblank interrupt
        // if self.chip_behavior == InterpreterMode::COSMAC && true {
            
        // }

        // sprites always 8 pixels wide, height ranging from 1-15 pixels
        assert!(bytes <= 8 * 15);

        let x = self.reg.v[x as usize] % WIDTH as u8;
        let y = self.reg.v[y as usize] % HEIGHT as u8;
        let mut flipped = false;
        
        for y_line in 0..bytes {
            // thank you @aquova on github
            let addr: u16 = Into::<u16>::into(self.reg.i) + y_line as u16;
            let pixels = self.ram[addr as usize];

            // sprites clip
            if self.chip_behavior == InterpreterMode::COSMAC && y + y_line >= HEIGHT as u8 {
                break;
            }

            for x_line in 0..8 {
                // sprites clip
                if self.chip_behavior == InterpreterMode::COSMAC && x + x_line >= WIDTH as u8 {
                    break;
                }

                if (pixels & (0b1000_0000 >> x_line)) != 0 {
                    let x = (x + x_line) % WIDTH as u8;
                    let y = (y + y_line) % HEIGHT as u8;

                    let old_val = self.pixels.get(x, y);
                    flipped |= old_val == 1;
                    self.pixels.set(x, y, old_val ^ 1);
                }
            }

            // this code was broken
            // let x = x + i % SPRITE_WIDTH;
            // let y = y + i / SPRITE_WIDTH;

            // let old_val = self.pixels.get(x, y);
            // let new_val = old_val ^ self.ram[(start + i as u16) as usize];

            // self.pixels.set(x, y, new_val);
            // if old_val == 1 && new_val == 0 {
            //     unset = true;
            // }
        }

        if flipped {
            self.reg.v[0xF] = 0x1;
        }
        else {
            self.reg.v[0xF] = 0x0;
        }
    }
}

pub fn exec(state: &mut Chip8, instr: Instruction) -> Result<(), InterpreterErr> {
    use Instruction::*;
    use InterpreterMode::*;
    let regs = &mut state.reg;
    let v = &mut regs.v;
    
    match instr {
        nop => (),
        call_mchn(_) => (), // only used on real hardware
        jmp(addr) => regs.pc = addr.into(),
        jr(addr) => {
            // incorrect behavior but this is how SUPER-CHIP implemented it
            if state.chip_behavior == SUPERCHIP {
                let x: u16 = (addr.0 & 0x0F00) >> 8;
                regs.pc = addr.0 + v[x as usize] as u16;
            }
            // correct behavior
            else {
                regs.pc = addr.0 + v[0] as u16;
            }
        },
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

            // COSMAC would increment i reg not just store in temp var
            if state.chip_behavior == COSMAC {
                regs.i.0 += reg as u16 + 1;
            }
        },
        ld_regs_i(reg) => {
            let reg = reg as usize;
            let i = regs.i.0 as usize;
            v[0..=reg].copy_from_slice(&state.ram[i..=(i + reg)]);

            // COSMAC would increment i reg not just store in temp var
            if state.chip_behavior == COSMAC {
                regs.i.0 += reg as u16 + 1;
            }
        },
        ld_i_font(reg) => regs.i = ((v[reg as usize] as u16) * 5).into(), // from aquova tutorial -> stored font in beginning of ram
        SkipEqNum(reg, num) => if v[reg as usize] == num { state.skip()? },
        SkipNotEqNum(reg, num) => if v[reg as usize] != num { state.skip()? },
        SkipEqReg(x, y) => if v[x as usize] == v[y as usize] { state.skip()? },
        SkipNotEqReg(x, y) => if v[x as usize] != v[y as usize] { state.skip()? },
        SkipKeyPressed(reg) => if state.is_key_reg_pressed(reg as usize) { state.skip()? },
        SkipKeyNotPressed(reg) => if !state.is_key_reg_pressed(reg as usize) { state.skip()? },
        add_reg(x, y) => {
            let carry: bool;
            (v[x as usize], carry) = v[x as usize].overflowing_add(v[y as usize]);
            v[0xF] = if carry { 0x1 } else { 0x0 };
        },
        add_nn(reg, num) => v[reg as usize] = v[reg as usize].wrapping_add(num),
        add_pc(reg) => regs.i = regs.i.0.wrapping_add(v[reg as usize] as u16).into(),
        sub_reg(x, y) => {
            let carry: bool;
            (v[x as usize], carry) = v[x as usize].overflowing_sub(v[y as usize]);
            v[0xF] = if carry { 0x0 } else { 0x1 };
        },
        sub_reg_rev(x, y) => {
            let carry: bool;
            (v[x as usize], carry) = v[y as usize].overflowing_sub(v[x as usize]);
            v[0xF] = if carry { 0x0 } else { 0x1 };
        },
        or(x, y) => {
            v[x as usize] |= v[y as usize];

            if state.chip_behavior == COSMAC {
                v[0xF] = 0;
            }
        },
        and(x, y) => {
            v[x as usize] &= v[y as usize];

            if state.chip_behavior == COSMAC {
                v[0xF] = 0;
            }
        },
        xor(x, y) => {
            v[x as usize] ^= v[y as usize];

            if state.chip_behavior == COSMAC {
                v[0xF] = 0;
            }
        },
        lsl(x, y) => {
            if state.chip_behavior >= SUPERCHIP {
                v[0xF] = (v[x as usize] & 0b1000_0000) >> 7;
                v[x as usize] <<= 1;
            }
            else {
                v[0xF] = (v[y as usize] & 0b1000_0000) >> 7;
                v[x as usize] = v[y as usize] << 1;
            }
        },
        lsr(x, y) => {
            if state.chip_behavior >= SUPERCHIP {
                v[0xF] = v[x as usize] & 1;
                v[x as usize] >>= 1;
            }
            else {
                v[0xF] = v[y as usize] & 1;
                v[x as usize] = v[y as usize] >> 1;
            }
        },
        ClearScreen => state.pixels = Default::default(),
        GenRandom(reg, num) => v[reg as usize] = Chip8::rand_mask(num),
        Draw(x, y, num) => state.set_pixels(x, y, num),
        WaitKey(reg) => {
            if state.any_key_pressed() {
                for key in 0..0xF {
                    if state.is_key_pressed(key) {
                        let v = &mut state.reg.v; // what great borrowing rules, i hate this.
                        v[reg as usize] = key;
                        break;
                    }
                }
            }
            else {
                state.reg.pc -= 2
            }
        },
        _ => todo!()
    }

    Ok(())
}
