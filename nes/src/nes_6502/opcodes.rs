use super::{Nes6502, Flags6502, addressing_modes::AddressMode};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
    Xxx,    
}

impl Opcode {
    pub fn execute(opcode: Opcode, nes: &mut Nes6502) -> u8 {
        match opcode {
            Opcode::Adc => nes.adc(),
            Opcode::And => nes.and(),
            Opcode::Asl => nes.asl(),
            Opcode::Bcc => nes.bcc(),
            Opcode::Bcs => nes.bcs(),
            Opcode::Beq => nes.beq(),
            Opcode::Bit => nes.bit(),
            Opcode::Bmi => nes.bmi(),
            Opcode::Bne => nes.bne(),
            Opcode::Bpl => nes.bpl(),
            Opcode::Brk => nes.brk(),
            Opcode::Bvc => nes.bvc(),
            Opcode::Bvs => nes.bvs(),
            Opcode::Clc => nes.clc(),
            Opcode::Cld => nes.cld(),
            Opcode::Cli => nes.cli(),
            Opcode::Clv => nes.clv(),
            Opcode::Cmp => nes.cmp(),
            Opcode::Cpx => nes.cpx(),
            Opcode::Cpy => nes.cpy(),
            Opcode::Dec => nes.dec(),
            Opcode::Dex => nes.dex(),
            Opcode::Dey => nes.dey(),
            Opcode::Eor => nes.eor(),
            Opcode::Inc => nes.inc(),
            Opcode::Inx => nes.inx(),
            Opcode::Iny => nes.iny(),
            Opcode::Jmp => nes.jmp(),
            Opcode::Jsr => nes.jsr(),
            Opcode::Lda => nes.lda(),
            Opcode::Ldx => nes.ldx(),
            Opcode::Ldy => nes.ldy(),
            Opcode::Lsr => nes.lsr(),
            Opcode::Nop => nes.nop(),
            Opcode::Ora => nes.ora(),
            Opcode::Pha => nes.pha(),
            Opcode::Php => nes.php(),
            Opcode::Pla => nes.pla(),
            Opcode::Plp => nes.plp(),
            Opcode::Rol => nes.rol(),
            Opcode::Ror => nes.ror(),
            Opcode::Rti => nes.rti(),
            Opcode::Rts => nes.rts(),
            Opcode::Sbc => nes.sbc(),
            Opcode::Sec => nes.sec(),
            Opcode::Sed => nes.sed(),
            Opcode::Sei => nes.sei(),
            Opcode::Sta => nes.sta(),
            Opcode::Stx => nes.stx(),
            Opcode::Sty => nes.sty(),
            Opcode::Tax => nes.tax(),
            Opcode::Tay => nes.tay(),
            Opcode::Tsx => nes.tsx(),
            Opcode::Txa => nes.txa(),
            Opcode::Txs => nes.txs(),
            Opcode::Tya => nes.tya(),
            Opcode::Xxx => nes.xxx(),
        }
    }
}

impl Nes6502 {
    // Opcodes
    // The 6502 has 56 legal opcodes, illegal opcodes are not modeled
    // in this emulator. Most of these opcodes return 0 but some are
    // capable of requiring more cycles when executing under certain
    // conditions combined with certain addressing modes.

    /// add with carry
    /// This instruction is used to add a value to the accumulator and a carry bit. If
    /// the result is > 255 there is an overflow setting the carry bit. This allows you
    /// to chain together ADC instructions to add numbers larger than 8 bits. This in 
    /// itself is simple, however the 6502 supports the concepts of negativity/positivity
    /// and signed overflow
    /// 
    /// 10000100 = 128 + 4 = 132 in normal circumstance we know this as unsigned and it
    /// allows us to represent numbers between 0 and 255. The 6502 can also interpret this
    /// as something else, if we assume those 8 bits represent the range -128 to +127 it
    /// has become signed
    /// 
    /// Since 132 > 127 it effectively wraps around, through -128 to -124. The wraparound
    /// is called overflow and this is a useful to know as it indicates that the calculation
    /// has gone outside of the permissable range, and therefore no longer makes numeric
    /// sense
    /// 
    ///  10000100 = 132 or -124
    /// +00010001 = +17 or +17
    ///  ========   ===    ====
    ///  10010101 = 149 or -107
    /// 
    /// In principle under the -128 to 127 range:
    /// 10000000 = -128, 11111111 = -1, 00000000 = 0, 00000001 = +1, 00000001 = +127
    /// therefore negative numbers have the most significant set, positive numbers do not
    /// 
    /// To assist us the 6502 can set the overflow flag, if the result of the addition has
    /// wrapped around. V <- ~(A^M) & A^(A+M+C)
    /// 
    /// A  M  R | V | A^R | A^M |~(A^M) | 
    /// 0  0  0 | 0 |  0  |  0  |   1   |
    /// 0  0  1 | 1 |  1  |  0  |   1   |
    /// 0  1  0 | 0 |  0  |  1  |   0   |
    /// 0  1  1 | 0 |  1  |  1  |   0   |  so V = ~(A^M) & (A^R)
    /// 1  0  0 | 0 |  1  |  1  |   0   |
    /// 1  0  1 | 0 |  0  |  1  |   0   |
    /// 1  1  0 | 1 |  1  |  0  |   1   |
    /// 1  1  1 | 0 |  0  |  0  |   1   |
    /// We can see how the above equation calculates V, based on A, M and R. V was chosen
    /// based on the following hypothesis:
    ///       Positive Number + Positive Number = Negative Result -> Overflow
    ///       Negative Number + Negative Number = Positive Result -> Overflow
    ///       Positive Number + Negative Number = Either Result -> Cannot Overflow
    ///       Positive Number + Positive Number = Positive Result -> OK! No Overflow
    ///       Negative Number + Negative Number = Negative Result -> OK! NO Overflow
    pub(super) fn adc(&mut self) -> u8 {
        // fetch the data we are adding to the accumulator
        self.fetch();
        
        let temp: u16 = self.a as u16 + self.fetched as u16 + self.get_flag(Flags6502::C) as u16;
        // set the carry bit if temp has overflowed
        self.set_flag(Flags6502::C, temp > 255);
        // set the zero flag if temp is empty
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0);
        // set the overflow flag
        self.set_flag(Flags6502::V, ((!(self.a as u16 ^ self.fetched as u16 & self.a as u16 ^ temp)) & 0x0080) == 0);
        // set the negative flag if bit 7 is turned on
        self.set_flag(Flags6502::N, (temp & 0x80) != 0);

        // load the result into the accumulator
        self.a = temp as u8;

        1
    }
    /// and (with accumulator)
    pub(super) fn and(&mut self) -> u8 {
        // fetch the data we are adding to the accumulator
        self.fetch();
        
        self.a &= self.fetched;
        // if the result of the AND resulted in all the bits 
        // being zero set the zero flag
        self.set_flag(Flags6502::Z,  self.a == 0x00);
        // set the negative flag if bit 7 is equal to one
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        // needs an additional clock cycle
        1
    }
    /// arithmetic shift left
    pub(super) fn asl(&mut self) -> u8 {
        self.fetch();

        let temp = (self.fetched as u16) << 1;
        self.set_flag(Flags6502::C, (temp & 0xFF00) > 0);
        self.set_flag(Flags6502::Z, (temp & 0xFF) == 0x00);
        self.set_flag(Flags6502::N, (temp & 0x80) != 0);

        // if implied addressing is being used
        if self.lookup[self.opcode as usize].addrmode == AddressMode::Imp {
            self.a = (temp & 0x00FF) as u8;
        } else {
            self.bus.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        0
    }
    /// bit test
    pub(super) fn bit(&mut self) -> u8 {
        self.fetch();

        let temp = self.a & self.fetched;
        self.set_flag(Flags6502::Z, temp == 0x00);
        self.set_flag(Flags6502::N, (self.fetched & (1 << 7)) != 0);
        self.set_flag(Flags6502::V, (self.fetched & (1 << 6)) != 0);

        0
    }
    /// break / interrupt
    pub(super) fn brk(&mut self) -> u8 {
        self.pc += 1;

        self.set_flag(Flags6502::I, true);
        self.bus.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.bus.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        self.set_flag(Flags6502::B, true);
        self.bus.write(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;
        self.set_flag(Flags6502::B, false);

        self.pc = self.bus.read(0xFFFE) as u16 | ((self.bus.read(0xFFFF) as u16) << 8);
        
        0
    }
    /// branch on carry clear
    pub(super) fn bcc(&mut self) -> u8 {
        // check if the carry bit is clear
        if self.get_flag(Flags6502::C) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            // give an additional cycle if the page has changed
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        0
    }
    /// branch on carry set
    pub(super) fn bcs(&mut self) -> u8 {
        // check if the carry bit set
        if self.get_flag(Flags6502::C) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }
    /// branch on equal (zero set)
    pub(super) fn beq(&mut self) -> u8 {
        if self.get_flag(Flags6502::Z) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        
        0
    }
    /// branch on minus (negative set)
    pub(super) fn bmi(&mut self) -> u8 {
        if self.get_flag(Flags6502::N) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        
        0
    }
    /// branch on not equal (zero clear)
    pub(super) fn bne(&mut self) -> u8 {
        if self.get_flag(Flags6502::Z) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        
        0
    }
    /// branch on plus (negative clear)
    pub(super) fn bpl(&mut self) -> u8 {
        if self.get_flag(Flags6502::N) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        
        0
    }
    /// branch on overflow clear
    pub(super) fn bvc(&mut self) -> u8 {
        if self.get_flag(Flags6502::V) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        
        0
    }
    /// branch on overflow set
    pub(super) fn bvs(&mut self) -> u8 {
        if self.get_flag(Flags6502::V) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        
        0
    }
    /// clear carry
    pub(super) fn clc(&mut self) -> u8 {
        self.set_flag(Flags6502::C, false);
        0
    }
    /// clear decimal
    pub(super) fn cld(&mut self) -> u8 {
        self.set_flag(Flags6502::D, false);
        0
    }
    /// clear interrupt disable
    pub(super) fn cli(&mut self) -> u8 {
        self.set_flag(Flags6502::I, false);
        0
    }
    /// clear overflow
    pub(super) fn clv(&mut self) -> u8 {
        self.set_flag(Flags6502::V, false);
        0
    }
    /// compare (with accumulator)
    pub(super) fn cmp(&mut self) -> u8 {
        self.fetch();

        let temp = self.a as u16 + self.fetched as u16;

        self.set_flag(Flags6502::C, self.a >= self.fetched);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        1
    }
    /// compare with X
    pub(super) fn cpx(&mut self) -> u8 {
        self.fetch();

        let temp = self.x as u16 + self.fetched as u16;

        self.set_flag(Flags6502::C, self.x >= self.fetched);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        1
    }
    /// compare with Y
    pub(super) fn cpy(&mut self) -> u8 {
        self.fetch();

        let temp = self.y as u16 + self.fetched as u16;

        self.set_flag(Flags6502::C, self.y >= self.fetched);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        1
    }
    // decrement
    pub(super) fn dec(&mut self) -> u8 {
        self.fetch();

        let temp = self.fetched - 1;
        self.bus.write(self.addr_abs, temp);
        self.set_flag(Flags6502::Z, temp == 0x00);
        self.set_flag(Flags6502::N, (temp & 0x80) != 0);

        0

    }
    /// decrement X
    pub(super) fn dex(&mut self) -> u8 {
        self.x -= 1;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);

        0
    }
    /// decrement Y
    pub(super) fn dey(&mut self) -> u8 {
        self.y -= 1;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);

        0
    }
    /// exclusive or (with accumulator)
    pub(super) fn eor(&mut self) -> u8 {
        self.fetch();

        self.a = self.a ^ self.fetched;

        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);

        1
    }
    /// increment
    pub(super) fn inc(&mut self) -> u8 {
        self.fetch();

        let temp = self.fetched as u16 + 1;
        self.bus.write(self.addr_abs, temp as u8);
        
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        0
    }
    /// increment X
    pub(super) fn inx(&mut self) -> u8 {
        self.x += 1;
        
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);

        0
    }
    /// increment Y
    pub(super) fn iny(&mut self) -> u8 {
        self.y += 1;
        
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);

        0
    }
    /// jump
    pub(super) fn jmp(&mut self) -> u8 {
        self.pc = self.addr_abs;
        0
    }
    /// jump subroutine
    pub(super) fn jsr(&mut self) -> u8 {
        self.pc -= 1;

        // write the program counter to the stack
        self.bus.write(0x0100 + self.stkp as u16, (self.pc >> 8) as u8);
        self.stkp -= 1;
        self.bus.write(0x0100 + self.stkp as u16, self.pc as u8);
        self.stkp -= 1;

        self.pc = self.addr_abs;
        0
    }
    /// load accumulator
    pub(super) fn lda(&mut self) -> u8 {
        self.fetch();
        self.a = self.fetched;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);

        1
    }
    /// load X
    pub(super) fn ldx(&mut self) -> u8 {
        self.fetch();
        self.x = self.fetched;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);

        1
    }
    /// load Y
    pub(super) fn ldy(&mut self) -> u8 {
        self.fetch();
        self.y = self.fetched;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);

        1
    }
    /// logical shift right
    pub(super) fn lsr(&mut self) -> u8 {
        self.fetch();

        self.set_flag(Flags6502::C, (self.fetched & 0x0001) != 0);
        let temp = self.fetch() as u16 >> 1;
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        if self.lookup[self.opcode as usize].addrmode == AddressMode::Imp {
            self.a = temp as u8;
        } else {
            self.bus.write(self.addr_abs, temp as u8);
        }

        0
    }
    /// no operation
    pub(super) fn nop(&self) -> u8 {
        match self.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
            _ => 0,
        }
    }
    /// or with accumulator
    pub(super) fn ora(&mut self) -> u8 {
        self.fetch();

        self.a = self.a | self.fetched;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);

        1
    }
    /// push accumulator
    pub(super) fn pha(&mut self) -> u8 {
        // the stack is hardcoded to start at location 0x0100 the stack pointer is an offset to it
        self.bus.write(0x0100 + self.stkp as u16, self.a);
        self.stkp -= 1;
        0
    }
    /// push processor status (SR)
    pub(super) fn php(&mut self) -> u8 {
        self.bus.write(0x0100 + self.stkp as u16, self.status | Flags6502::B as u8 | Flags6502::U as u8);

        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, false);

        self.stkp -= 1;

        0
    }
    /// pull accumulator
    pub(super) fn pla(&mut self) -> u8{
        self.stkp += 1;
        self.a = self.bus.read(0x0100 + self.stkp as u16);
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        0
    }
    /// pull processor status (SR)
    pub(super) fn plp(&mut self) -> u8 {
        self.stkp += 1;

        self.status = self.bus.read(0x0100 + self.stkp as u16);

        self.set_flag(Flags6502::U, true);

        0
    }
    /// rotate left
    pub(super) fn rol(&mut self) -> u8 {
        self.fetch();

        let temp = (self.fetched << 1) as u16 | self.get_flag(Flags6502::C) as u16;
        self.set_flag(Flags6502::C, (temp & 0xFF00) != 0);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        if self.lookup[self.opcode as usize].addrmode == AddressMode::Imp {
            self.a = temp as u8;
        } else {
            self.bus.write(self.addr_abs, temp as u8);
        }

        0
    }
    /// rotate right
    pub(super) fn ror(&mut self) -> u8 {
        self.fetch();

        let temp = (self.get_flag(Flags6502::C) << 7) as u16 | (self.fetched >> 1) as u16;
        self.set_flag(Flags6502::C, (self.fetched & 0x01) != 0);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x00);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        if self.lookup[self.opcode as usize].addrmode == AddressMode::Imp {
            self.a = temp as u8;
        } else {
            self.bus.write(self.addr_abs, temp as u8);
        }

        0
    }
    /// return from interrupt
    /// Restores the state of the processor before the interrupt occured
    pub(super) fn rti(&mut self) -> u8 {
        self.stkp += 1;
        self.status = self.bus.read(0x0100 + self.stkp as u16);
        self.status &= !(Flags6502::B as u8);
        self.status &= !(Flags6502::U as u8);

        self.stkp += 1;
        self.pc = self.bus.read(0x0100 + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.bus.read(0x0100 + self.stkp as u16) as u16) << 8;

        0
    }
    /// return from subroutine
    pub(super) fn rts(&mut self) -> u8 {
        self.stkp += 1;
        self.pc = self.bus.read(0x0100 + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.bus.read(0x0100 + self.stkp as u16) as u16) << 8;
        
        self.pc += 1;

        0
    }
    /// subtract with carry
    pub(super) fn sbc(&mut self) -> u8 {
        self.fetch();

        // invert the bits
        let value: u16 = self.fetched as u16 ^ 0x00FF;
        let temp: u16 = self.a as u16 + value + self.get_flag(Flags6502::C) as u16;

        // set the carry bit if temp has overflowed
        self.set_flag(Flags6502::C, (temp & 0xFF00) != 0);
        // set the zero flag if temp is empty
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0);
        // set the negative flag if bit 7 is turned on
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);
        // set the overflow flag
        self.set_flag(Flags6502::V, ((temp ^ self.a as u16) & (temp ^ value) & 0x0080) != 0);
        self.a = temp as u8;
        1
    }
    /// set carry
    pub(super) fn sec(&mut self) -> u8 {
        self.set_flag(Flags6502::C, true);
        0
    }
    /// set decimal
    pub(super) fn sed(&mut self) -> u8 {
        self.set_flag(Flags6502::D, true);
        0
    }
    /// set interrupt disable
    pub(super) fn sei(&mut self) -> u8 {
        self.set_flag(Flags6502::I, true);
        0
    }
    /// store accumulator
    pub(super) fn sta(&mut self) -> u8 {
        self.bus.write(self.addr_abs, self.a);
        
        0
    }
    /// store X
    pub(super) fn stx(&mut self) -> u8 {
        self.bus.write(self.addr_abs, self.x);
        
        0
    }
    /// store Y
    pub(super) fn sty(&mut self) -> u8 {
        self.bus.write(self.addr_abs, self.y);
        
        0
    }
    /// transfer accumulator to X
    pub(super) fn tax(&mut self) -> u8 {
        self.x = self.a;

        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);

        0
    }
    /// transfer accumulator to Y
    pub(super) fn tay(&mut self) -> u8 {
        self.y = self.a;

        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);

        0
    }
    /// transfer stack pointer to X
    pub(super) fn tsx(&mut self) -> u8 {
        self.x = self.stkp;

        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);

        0
    }
    /// transfer X to accumulator
    pub(super) fn txa(&mut self) -> u8 {
        self.a = self.x;

        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);

        0
    }
    /// transfer X to stack pointer
    pub(super) fn txs(&mut self) -> u8 {
        self.stkp = self.x;

        0
    }
    /// transfer Y to accumulator 
    pub(super) fn tya(&mut self) -> u8 {
        self.a = self.y;

        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);

        0
    }
    pub(super) fn xxx(&mut self) -> u8 {
        self.info.push(format!("{:02X}: Invalid Opcode at address {:04X}", self.opcode, self.addr_abs));

        0
    }
}