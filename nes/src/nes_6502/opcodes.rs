use super::{Nes6502, Flags6502, addressing_modes::AddressMode};

#[derive(PartialEq, Clone, Copy)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC, 
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    XXX,
}

impl Opcode {
    pub fn execute(opcode: Opcode, nes: &mut Nes6502) -> u8 {
        match opcode {
            Opcode::ADC => nes.ADC(),
            Opcode::AND => nes.AND(),
            Opcode::ASL => nes.ASL(),
            Opcode::BCC => nes.BCC(),
            Opcode::BCS => nes.BCS(),
            Opcode::BEQ => nes.BEQ(),
            Opcode::BIT => nes.BIT(),
            Opcode::BMI => nes.BMI(),
            Opcode::BNE => nes.BNE(),
            Opcode::BPL => nes.BPL(),
            Opcode::BRK => nes.BRK(),
            Opcode::BVC => nes.BVC(),
            Opcode::BVS => nes.BVS(),
            Opcode::CLC => nes.CLC(),
            Opcode::CLD => nes.CLD(),
            Opcode::CLI => nes.CLI(),
            Opcode::CLV => nes.CLV(),
            Opcode::CMP => nes.CMP(),
            Opcode::CPX => nes.CPX(),
            Opcode::CPY => nes.CPY(),
            Opcode::DEC => nes.DEC(),
            Opcode::DEX => nes.DEX(),
            Opcode::DEY => nes.DEY(),
            Opcode::EOR => nes.EOR(),
            Opcode::INC => nes.INC(),
            Opcode::INX => nes.INX(),
            Opcode::INY => nes.INY(),
            Opcode::JMP => nes.JMP(),
            Opcode::JSR => nes.JSR(),
            Opcode::LDA => nes.LDA(),
            Opcode::LDX => nes.LDX(),
            Opcode::LDY => nes.LDY(),
            Opcode::LSR => nes.LSR(),
            Opcode::NOP => nes.NOP(),
            Opcode::ORA => nes.ORA(),
            Opcode::PHA => nes.PHA(),
            Opcode::PHP => nes.PHP(),
            Opcode::PLA => nes.PLA(),
            Opcode::PLP => nes.PLP(),
            Opcode::ROL => nes.ROL(),
            Opcode::ROR => nes.ROR(),
            Opcode::RTI => nes.RTI(),
            Opcode::RTS => nes.RTS(),
            Opcode::SBC => nes.SBC(),
            Opcode::SEC => nes.SEC(),
            Opcode::SED => nes.SED(),
            Opcode::SEI => nes.SEI(),
            Opcode::STA => nes.STA(),
            Opcode::STX => nes.STX(),
            Opcode::STY => nes.STY(),
            Opcode::TAX => nes.TAX(),
            Opcode::TAY => nes.TAY(),
            Opcode::TSX => nes.TSX(),
            Opcode::TXA => nes.TXA(),
            Opcode::TXS => nes.TXS(),
            Opcode::TYA => nes.TYA(),
            Opcode::XXX => nes.XXX(),
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
    pub(super) fn ADC(&mut self) -> u8 {
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
        self.a = temp as u8 & 0x00FF;

        1
    }
    /// and (with accumulator)
    pub(super) fn AND(&mut self) -> u8 {
        // fetch the data we are adding to the accumulator
        self.fetch();
        
        self.a = self.a & self.fetched;
        // if the result of the AND resulted in all the bits 
        // being zero set the zero flag
        self.set_flag(Flags6502::Z,  self.a == 0x00);
        // set the negative flag if bit 7 is equal to one
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        // needs an additional clock cycle
        1
    }
    /// arithmetic shift left
    pub(super) fn ASL(&mut self) -> u8 {
        self.fetch();

        let temp = (self.fetched as u16) << 1;
        self.set_flag(Flags6502::C, (temp & 0xFF00) > 0);
        self.set_flag(Flags6502::Z, (temp & 0xFF) == 0x00);
        self.set_flag(Flags6502::N, (temp & 0x80) != 0);

        // if implied addressing is being used
        if self.lookup[self.opcode as usize].addrmode == AddressMode::IMP {
            self.a = (temp & 0x00FF) as u8;
        } else {
            self.bus.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        0
    }
    /// bit test
    pub(super) fn BIT(&mut self) -> u8 {
        self.fetch();

        let temp = self.a & self.fetched;
        self.set_flag(Flags6502::Z, temp == 0x00);
        self.set_flag(Flags6502::N, (self.fetched & (1 << 7)) != 0);
        self.set_flag(Flags6502::V, (self.fetched & (1 << 6)) != 0);

        0
    }
    /// break / interrupt
    pub(super) fn BRK(&mut self) -> u8 {
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
    pub(super) fn BCC(&mut self) -> u8 {
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
    pub(super) fn BCS(&mut self) -> u8 {
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
    pub(super) fn BEQ(&mut self) -> u8 {
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
    pub(super) fn BMI(&mut self) -> u8 {
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
    pub(super) fn BNE(&mut self) -> u8 {
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
    pub(super) fn BPL(&mut self) -> u8 {
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
    pub(super) fn BVC(&mut self) -> u8 {
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
    pub(super) fn BVS(&mut self) -> u8 {
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
    pub(super) fn CLC(&mut self) -> u8 {
        self.set_flag(Flags6502::C, false);
        0
    }
    /// clear decimal
    pub(super) fn CLD(&mut self) -> u8 {
        self.set_flag(Flags6502::D, false);
        0
    }
    /// clear interrupt disable
    pub(super) fn CLI(&mut self) -> u8 {
        self.set_flag(Flags6502::I, false);
        0
    }
    /// clear overflow
    pub(super) fn CLV(&mut self) -> u8 {
        self.set_flag(Flags6502::V, false);
        0
    }
    /// compare (with accumulator)
    pub(super) fn CMP(&mut self) -> u8 {
        self.fetch();

        let temp = self.a as u16 + self.fetched as u16;

        self.set_flag(Flags6502::C, self.a >= self.fetched);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        1
    }
    /// compare with X
    pub(super) fn CPX(&mut self) -> u8 {
        self.fetch();

        let temp = self.x as u16 + self.fetched as u16;

        self.set_flag(Flags6502::C, self.x >= self.fetched);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        1
    }
    /// compare with Y
    pub(super) fn CPY(&mut self) -> u8 {
        self.fetch();

        let temp = self.y as u16 + self.fetched as u16;

        self.set_flag(Flags6502::C, self.y >= self.fetched);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) != 0);

        1
    }
    // decrement
    pub(super) fn DEC(&mut self) -> u8 {
        self.fetch();

        let temp = self.fetched - 1;
        self.bus.write(self.addr_abs, temp);
        self.set_flag(Flags6502::Z, temp == 0x00);
        self.set_flag(Flags6502::N, (temp & 0x80) != 0);

        0

    }
    /// decrement X
    pub(super) fn DEX(&mut self) -> u8 {
        self.x -= 1;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);

        0
    }
    /// decrement Y
    pub(super) fn DEY(&mut self) -> u8 {
        self.y -= 1;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);

        0
    }
    /// exclusive or (with accumulator)
    pub(super) fn EOR(&self) -> u8 {
        todo!()
    }
    /// increment
    pub(super) fn INC(&self) -> u8 {
        todo!()
    }
    /// increment X
    pub(super) fn INX(&self) -> u8 {
        todo!()
    }
    /// increment Y
    pub(super) fn INY(&self) -> u8 {
        todo!()
    }
    /// jump
    pub(super) fn JMP(&self) -> u8 {
        todo!()
    }
    /// jump subroutine
    pub(super) fn JSR(&self) -> u8 {
        todo!()
    }
    /// load accumulator
    pub(super) fn LDA(&mut self) -> u8 {
        self.fetch();
        self.a = self.fetched;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);

        1
    }
    /// load X
    pub(super) fn LDX(&mut self) -> u8 {
        self.fetch();
        self.x = self.fetched;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);

        1
    }
    /// load Y
    pub(super) fn LDY(&mut self) -> u8 {
        self.fetch();
        self.y = self.fetched;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);

        1
    }
    /// logical shift right
    pub(super) fn LSR(&self) -> u8 {
        todo!()
    }
    /// no operation
    pub(super) fn NOP(&self) -> u8 {
        0
    }
    /// or with accumulator
    pub(super) fn ORA(&self) -> u8 {
        todo!()
    }
    /// push accumulator
    pub(super) fn PHA(&mut self) -> u8 {
        // the stack is hardcoded to start at location 0x0100 the stack pointer is an offset to it
        self.bus.write(0x0100 + self.stkp as u16, self.a);
        self.stkp -= 1;
        0
    }
    /// push processor status (SR)
    pub(super) fn PHP(&self) -> u8 {
        todo!()
    }
    /// pull accumulator
    pub(super) fn PLA(&mut self) -> u8{
        self.stkp += 1;
        self.a = self.bus.read(0x0100 + self.stkp as u16);
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        0
    }
    /// pull processor status (SR)
    pub(super) fn PLP(&self) -> u8 {
        todo!()
    }
    /// rotate left
    pub(super) fn ROL(&self) -> u8 {
        todo!()
    }
    /// rotate right
    pub(super) fn ROR(&self) -> u8 {
        todo!()
    }
    /// return from interrupt
    /// Restores the state of the processor before the interrupt occured
    pub(super) fn RTI(&mut self) -> u8 {
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
    pub(super) fn RTS(&self) -> u8 {
        todo!()
    }
    /// subtract with carry
    pub(super) fn SBC(&mut self) -> u8 {
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
    pub(super) fn SEC(&self) -> u8 {
        todo!()
    }
    /// set decimal
    pub(super) fn SED(&self) -> u8 {
        todo!()
    }
    /// set interrupt disable
    pub(super) fn SEI(&self) -> u8 {
        todo!()
    }
    /// store accumulator
    pub(super) fn STA(&mut self) -> u8 {
        self.bus.write(self.addr_abs, self.a);
        
        0
    }
    /// store X
    pub(super) fn STX(&mut self) -> u8 {
        self.bus.write(self.addr_abs, self.x);
        
        0
    }
    /// store Y
    pub(super) fn STY(&mut self) -> u8 {
        self.bus.write(self.addr_abs, self.y);
        
        0
    }
    /// transfer accumulator to X
    pub(super) fn TAX(&self) -> u8 {
        todo!()
    }
    /// transfer accumulator to Y
    pub(super) fn TAY(&self) -> u8 {
        todo!()
    }
    /// transfer stack pointer to X
    pub(super) fn TSX(&self) -> u8 {
        todo!()
    }
    /// transfer X to accumulator
    pub(super) fn TXA(&self) -> u8 {
        todo!()
    }
    /// transfer X to stack pointer
    pub(super) fn TXS(&self) -> u8 {
        todo!()
    }
    /// transfer Y to accumulator 
    pub(super) fn TYA(&self) -> u8 {
        todo!()
    }
    pub(super) fn XXX(&mut self) -> u8 {
        self.info.push(format!("{:02X}: Invalid Opcode at address {:04X}", self.opcode, self.addr_abs));

        0
    }
}