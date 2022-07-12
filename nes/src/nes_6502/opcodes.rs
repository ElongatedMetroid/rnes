use super::{Nes6502, Flags6502};

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
        self.set_flag(Flags6502::V, ((!(self.a as u16 ^ self.fetched as u16 & self.a as u16 ^ temp)) & 0x0080) != 0);
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
    pub(super) fn ASL(&self) -> u8 {
        todo!()
    }
    /// bit test
    pub(super) fn BIT(&self) -> u8 {
        todo!()
    }
    /// break / interrupt
    pub(super) fn BRK(&self) -> u8 {
        todo!()
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
            self.addr_abs = self.pc + self.addr_rel;

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
    pub(super) fn CMP(&self) -> u8 {
        todo!()
    }
    /// compare with X
    pub(super) fn CPX(&self) -> u8 {
        todo!()
    }
    /// compare with Y
    pub(super) fn CPY(&self) -> u8 {
        todo!()
    }
    // decrement
    pub(super) fn DEC(&self) -> u8 {
        todo!()
    }
    /// decrement X
    pub(super) fn DEX(&self) -> u8 {
        todo!()
    }
    /// decrement Y
    pub(super) fn DEY(&self) -> u8 {
        todo!()
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
    pub(super) fn LDA(&self) -> u8 {
        todo!()
    }
    /// load X
    pub(super) fn LDX(&self) -> u8 {
        todo!()
    }
    /// load Y
    pub(super) fn LDY(&self) -> u8 {
        todo!()
    }
    /// logical shift right
    pub(super) fn LSR(&self) -> u8 {
        todo!()
    }
    /// no operation
    pub(super) fn NOP(&self) -> u8 {
        todo!()
    }
    /// or with accumulator
    pub(super) fn ORA(&self) -> u8 {
        todo!()
    }
    /// push accumulator
    pub(super) fn PHA(&mut self) -> u8 {
        // the stack is hardcoded to start at location 0x0100 the stack pointer is an offset to it
        self.write(0x0100 + self.stkp as u16, self.a);
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
        self.a = self.read(0x0100 + self.stkp as u16);
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
        self.status = self.read(0x0100 + self.stkp as u16);
        self.status &= !(Flags6502::B as u8);
        self.status &= !(Flags6502::U as u8);

        self.stkp += 1;
        self.pc = self.read(0x0100 + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.read(0x0100 + self.stkp as u16) as u16) << 8;

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
        self.a = temp as u8 & 0x00FF;
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
    pub(super) fn STA(&self) -> u8 {
        todo!()
    }
    /// store X
    pub(super) fn STX(&self) -> u8 {
        todo!()
    }
    /// store Y
    pub(super) fn STY(&self) -> u8 {
        todo!()
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
    pub(super) fn XXX(&self) -> u8 {
        todo!()
    }
}