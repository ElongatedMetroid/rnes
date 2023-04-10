use std::fmt::Display;

use enum_display_derive::Display;

#[derive(Clone, Copy)]
pub struct Opcode {
    name: OpcodeName,
    addressing_mode: AddressingMode,
}

impl Opcode {
    pub fn name(&self) -> OpcodeName {
        self.name
    }
    pub fn addressing_mode(&self) -> AddressingMode {
        self.addressing_mode
    }
}

/// Names or acronyms of all the opcodes, copied from https://github.com/mre/mos6502/blob/master/src/instruction.rs
#[derive(Display, Clone, Copy)]
pub enum OpcodeName {
    ADC, // ADd with Carry................ | NV ...ZC A            = A + M + C
    AND, // logical AND (bitwise)......... | N. ...Z. A            = A && M
    ASL, // Arithmetic Shift Left......... | N. ...ZC A            = M << 1
    BCC, // Branch if Carry Clear......... | .. .....         PC   = !C
    BCS, // Branch if Carry Set........... | .. .....         PC   = C
    BEQ, // Branch if Equal (to zero?).... | .. .....         PC   = Z
    BIT, // BIT test...................... | NV ...Z.              = A & M
    BMI, // Branch if Minus............... | .. .....         PC   = N
    BNE, // Branch if Not Equal........... | .. .....         PC   = !Z
    BPL, // Branch if Positive............ | .. .....         PC   = Z
    BRK, // BReaK......................... | .. B....       S PC   =
    BVC, // Branch if oVerflow Clear...... | .. .....         PC   = !V
    BVS, // Branch if oVerflow Set........ | .. .....         PC   = V
    CLC, // CLear Carry flag.............. | .. ....C              = 0
    CLD, // Clear Decimal Mode............ | .. .D...              = 0
    CLI, // Clear Interrupt Disable....... | .. ..I..              = 0
    CLV, // Clear oVerflow flag........... | .V .....              = 0
    CMP, // Compare....................... | N. ...ZC              = A - M
    CPX, // Compare X register............ | N. ...ZC              = X - M
    CPY, // Compare Y register............ | N. ...ZC              = Y - M
    DEC, // DECrement memory.............. | N. ...Z.            M = M - 1
    DEX, // DEcrement X register.......... | N. ...Z.   X          = X - 1
    DEY, // DEcrement Y register.......... | N. ...Z.     Y        = Y - 1
    EOR, // Exclusive OR (bitwise)........ | N. ...Z. A            = A ^ M
    INC, // INCrement memory.............. | N. ...Z.            M = M + 1
    INX, // INcrement X register.......... | N. ...Z.   X          = X + 1
    INY, // INcrement Y register.......... | N. ...Z.     Y        = Y + 1
    JMP, // JuMP.......................... | .. .....       S PC   =
    JSR, // Jump to SubRoutine............ | .. .....       S PC   =
    LDA, // LoaD Accumulator.............. | N. ...Z. A            = M
    LDX, // LoaD X register............... | N. ...Z.   X          = M
    LDY, // LoaD Y register............... | N. ...Z.     Y        = M
    LSR, // Logical Shift Right........... | N. ...ZC A            = A/2
    //                               or N. ...ZC            M = M/2
    NOP, // No OPeration.................. | .. .....              =
    ORA, // inclusive OR (bitwise)........ | N. ...Z. A            = A | M
    PHA, // PusH Accumulator.............. | .. .....       S    M = A
    PHP, // PusH Processor status......... | .. .....       S    M = F
    PLA, // PuLl Accumulator.............. | N. ...Z. A     S      = M (stack)
    PLP, // PuLl Processor status......... | NV BDIZC       S      = M (stack)
    ROL, // ROtate Left................... | N. ...ZC A            = C A rotated
    //                               or N. ...ZC            M = C M rotated
    ROR, // ROtate Right.................. | N. ...ZC A            = C A rotated
    //                               or N. ...ZC            M = C M rotated
    RTI, // ReTurn from Interrupt......... | NV BDIZC         PC   = M (stack)
    RTS, // ReTurn from Subroutine........ | .. .....         PC   = M (stack)
    SBC, // SuBtract with Carry........... | NV ...ZC A            = A-M-(1-C)
    SEC, // SEt Carry flag................ | .. ....C              = 1
    SED, // SEt Decimal flag.............. | .. .D...              = 1
    SEI, // SEt Interrupt disable......... | .. ..I..              = 1
    STA, // STore Accumulator............. | .. .....            M = A
    STX, // STore X register.............. | .. .....            M = X
    STY, // STore Y register.............. | .. .....            M = Y
    TAX, // Transfer Accumulator to X..... | N. ...Z.   X          = A
    TAY, // Transfer Accumulator to Y..... | N. ...Z.     Y        = A
    TSX, // Transfer Stack pointer to X... | N. ...Z.   X          = S
    TXA, // Transfer X to Accumulator..... | N. ...Z. A            = X
    TXS, // Transfer X to Stack pointer... | .. .....       S      = X
    TYA, // Transfer Y to Accumulator..... | N. ...Z. A            = Y
}

/// addressing modes, copied from https://github.com/mre/mos6502/blob/master/src/instruction.rs
#[derive(Display, Clone, Copy)]
pub enum AddressingMode {
    Accumulator,      // 1    LSR A        work directly on accumulator
    Implied,          // 1    BRK
    Immediate,        // 2    LDA #10      8-bit constant in instruction
    ZeroPage,         // 2    LDA $00      zero-page address
    ZeroPageX,        // 2    LDA $80,X    address is X register + 8-bit constant
    ZeroPageY,        // 2    LDX $10,Y    address is Y register + 8-bit constant
    Relative,         // 2    BNE LABEL    branch target as signed relative offset
    Absolute,         // 3    JMP $1000    full 16-bit address
    AbsoluteX,        // 3    STA $1000,X  full 16-bit address plus X register
    AbsoluteY,        // 3    STA $1000,Y  full 16-bit address plus Y register
    Indirect,         // 3    JMP ($1000)  jump to address stored at address
    IndexedIndirectX, // 2    LDA ($10,X)  load from address stored at (constant
    //                   zero page address plus X register)
    IndirectIndexedY, // 2    LDA ($10),Y  load from (address stored at constant
                      //                   zero page address) plus Y register
}

/// Lookup table for all the Opcodes
pub static OPCODES: [Option<Opcode>; 256] = [
    // 0x00
    Some(Opcode { name: OpcodeName::BRK, addressing_mode: AddressingMode::Implied }),
    // 0x01
    Some(Opcode { name: OpcodeName::ORA, addressing_mode: AddressingMode::IndexedIndirectX}),
    // 0x02
    None,
    // 0x03
    None,
    // 0x04
    None,
    // 0x05
    Some(Opcode { name: OpcodeName::ORA, addressing_mode: AddressingMode::ZeroPage }),
    // 0x06
    Some(Opcode { name: OpcodeName::ASL, addressing_mode: AddressingMode::ZeroPage }),
    // 0x07
    None,
    // 0x08
    Some(Opcode { name: OpcodeName::PHP, addressing_mode: AddressingMode::Implied }),
    // 0x09
    Some(Opcode { name: OpcodeName::ORA, addressing_mode: AddressingMode::Immediate }),
    // 0x0A
    Some(Opcode { name: OpcodeName::ASL, addressing_mode: AddressingMode::Accumulator }),
    // 0x0B
    None,
    // 0x0C
    None,
    // 0x0D
    Some(Opcode { name: OpcodeName::ORA, addressing_mode: AddressingMode::Absolute }),
    // 0x0E
    Some(Opcode { name: OpcodeName::ASL, addressing_mode: AddressingMode::Absolute }),
    // 0x0F
    None,
    // 0x10
    Some(Opcode { name: OpcodeName::BPL, addressing_mode: AddressingMode::Relative }),
    // 0x11
    Some(Opcode { name: OpcodeName::ORA, addressing_mode: AddressingMode::IndirectIndexedY }),
    // 0x12
    None,
    // 0x13
    None,
    // 0x14
    None,
    // 0x15
    Some(Opcode { name: OpcodeName::ORA, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x16
    Some(Opcode { name: OpcodeName::ASL, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x17
    None,
    // 0x18
    Some(Opcode { name: OpcodeName::CLC, addressing_mode: AddressingMode::Implied }),
    // 0x19
    Some(Opcode { name: OpcodeName::ORA, addressing_mode: AddressingMode::AbsoluteY }),
    // 0x1A
    None,
    // 0x1B
    None,
    // 0x1C
    None,
    // 0x1D
    Some(Opcode { name: OpcodeName::ORA, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x1E
    Some(Opcode { name: OpcodeName::ASL, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x1F
    None,
    // 0x20
    Some(Opcode { name: OpcodeName::JSR, addressing_mode: AddressingMode::Absolute }),
    // 0x21
    Some(Opcode { name: OpcodeName::AND, addressing_mode: AddressingMode::IndexedIndirectX }),
    // 0x22
    None,
    // 0x23
    None,
    // 0x24
    Some(Opcode { name: OpcodeName::BIT, addressing_mode: AddressingMode::ZeroPage }),
    // 0x25
    Some(Opcode { name: OpcodeName::AND, addressing_mode: AddressingMode::ZeroPage }),
    // 0x26
    Some(Opcode { name: OpcodeName::ROL, addressing_mode: AddressingMode::ZeroPage }),
    // 0x27
    None,
    // 0x28
    Some(Opcode { name: OpcodeName::PLP, addressing_mode: AddressingMode::Implied }),
    // 0x29
    Some(Opcode { name: OpcodeName::AND, addressing_mode: AddressingMode::Immediate }),
    // 0x2A
    Some(Opcode { name: OpcodeName::ROL, addressing_mode: AddressingMode::Accumulator }),
    // 0x2B
    None,
    // 0x2C
    Some(Opcode { name: OpcodeName::BIT, addressing_mode: AddressingMode::Absolute }),
    // 0x2D
    Some(Opcode { name: OpcodeName::AND, addressing_mode: AddressingMode::Absolute }),
    // 0x2E
    Some(Opcode { name: OpcodeName::ROL, addressing_mode: AddressingMode::Absolute }),
    // 0x2F
    None,
    // 0x30
    Some(Opcode { name: OpcodeName::BMI, addressing_mode: AddressingMode::Relative }),
    // 0x31
    Some(Opcode { name: OpcodeName::AND, addressing_mode: AddressingMode::IndirectIndexedY }),
    // 0x32
    None,
    // 0x33
    None,
    // 0x34
    None,
    // 0x35
    Some(Opcode { name: OpcodeName::AND, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x36
    Some(Opcode { name: OpcodeName::ROL, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x37
    None,
    // 0x38
    Some(Opcode { name: OpcodeName::SEC, addressing_mode: AddressingMode::Implied }),
    // 0x39
    Some(Opcode { name: OpcodeName::AND, addressing_mode: AddressingMode::AbsoluteY }),
    // 0x3A
    None,
    // 0x3B
    None,
    // 0x3C
    None,
    // 0x3D
    Some(Opcode { name: OpcodeName::AND, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x3E
    Some(Opcode { name: OpcodeName::ROL, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x3F
    None,
    // 0x40
    Some(Opcode { name: OpcodeName::RTI, addressing_mode: AddressingMode::Implied }),
    // 0x41
    Some(Opcode { name: OpcodeName::EOR, addressing_mode: AddressingMode::IndexedIndirectX }),
    // 0x42
    None,
    // 0x43
    None,
    // 0x44
    None,
    // 0x45
    Some(Opcode { name: OpcodeName::EOR, addressing_mode: AddressingMode::ZeroPage }),
    // 0x46
    Some(Opcode { name: OpcodeName::LSR, addressing_mode: AddressingMode::ZeroPage }),
    // 0x47
    None,
    // 0x48
    Some(Opcode { name: OpcodeName::PHA, addressing_mode: AddressingMode::Implied }),
    // 0x49
    Some(Opcode { name: OpcodeName::EOR, addressing_mode: AddressingMode::Immediate }),
    // 0x4A
    Some(Opcode { name: OpcodeName::LSR, addressing_mode: AddressingMode::Accumulator }),
    // 0x4B
    None,
    // 0x4C
    Some(Opcode { name: OpcodeName::JMP, addressing_mode: AddressingMode::Absolute }),
    // 0x4D
    Some(Opcode { name: OpcodeName::EOR, addressing_mode: AddressingMode::Absolute }),
    // 0x4E
    Some(Opcode { name: OpcodeName::LSR, addressing_mode: AddressingMode::Absolute }),
    // 0x4F
    None,
    // 0x50
    Some(Opcode { name: OpcodeName::BVC, addressing_mode: AddressingMode::Relative }),
    // 0x51
    Some(Opcode { name: OpcodeName::EOR, addressing_mode: AddressingMode::IndirectIndexedY }),
    // 0x52
    None,
    // 0x53
    None,
    // 0x54
    None,
    // 0x55
    Some(Opcode { name: OpcodeName::EOR, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x56
    Some(Opcode { name: OpcodeName::LSR, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x57
    None,
    // 0x58
    Some(Opcode { name: OpcodeName::CLI, addressing_mode: AddressingMode::Implied }),
    // 0x59
    Some(Opcode { name: OpcodeName::EOR, addressing_mode: AddressingMode::AbsoluteY }),
    // 0x5A
    None,
    // 0x5B
    None,
    // 0x5C
    None,
    // 0x5D
    Some(Opcode { name: OpcodeName::EOR, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x5E
    Some(Opcode { name: OpcodeName::LSR, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x5F
    None,
    // 0x60
    Some(Opcode { name: OpcodeName::RTS, addressing_mode: AddressingMode::Implied }),
    // 0x61
    Some(Opcode { name: OpcodeName::ADC, addressing_mode: AddressingMode::IndexedIndirectX }),
    // 0x62
    None,
    // 0x63
    None,
    // 0x64
    None,
    // 0x65
    Some(Opcode { name: OpcodeName::ADC, addressing_mode: AddressingMode::ZeroPage }),
    // 0x66
    Some(Opcode { name: OpcodeName::ROR, addressing_mode: AddressingMode::ZeroPage }),
    // 0x67
    None,
    // 0x68
    Some(Opcode { name: OpcodeName::PLA, addressing_mode: AddressingMode::Implied }),
    // 0x69
    Some(Opcode { name: OpcodeName::ADC, addressing_mode: AddressingMode::Immediate }),
    // 0x6A
    Some(Opcode { name: OpcodeName::ROR, addressing_mode: AddressingMode::Accumulator }),
    // 0x6B
    None,
    // 0x6C
    Some(Opcode { name: OpcodeName::JMP, addressing_mode: AddressingMode::Indirect }),
    // 0x6D
    Some(Opcode { name: OpcodeName::ADC, addressing_mode: AddressingMode::Absolute }),
    // 0x6E
    Some(Opcode { name: OpcodeName::ROR, addressing_mode: AddressingMode::Absolute }),
    // 0x6F
    None,
    // 0x70
    Some(Opcode { name: OpcodeName::BVS, addressing_mode: AddressingMode::Relative }),
    // 0x71
    Some(Opcode { name: OpcodeName::ADC, addressing_mode: AddressingMode::IndirectIndexedY }),
    // 0x72
    None,
    // 0x73
    None,
    // 0x74
    None,
    // 0x75
    Some(Opcode { name: OpcodeName::ADC, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x76
    Some(Opcode { name: OpcodeName::ROR, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x77
    None,
    // 0x78
    Some(Opcode { name: OpcodeName::SEI, addressing_mode: AddressingMode::Implied }),
    // 0x79
    Some(Opcode { name: OpcodeName::ADC, addressing_mode: AddressingMode::AbsoluteY }),
    // 0x7A
    None,
    // 0x7B
    None,
    // 0x7C
    None,
    // 0x7D
    Some(Opcode { name: OpcodeName::ADC, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x7E
    Some(Opcode { name: OpcodeName::ROR, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x7F
    None,
    // 0x80
    None,
    // 0x81
    Some(Opcode { name: OpcodeName::STA, addressing_mode: AddressingMode::IndexedIndirectX }),
    // 0x82
    None,
    // 0x83
    None,
    // 0x84
    Some(Opcode { name: OpcodeName::STY, addressing_mode: AddressingMode::ZeroPage }),
    // 0x85
    Some(Opcode { name: OpcodeName::STA, addressing_mode: AddressingMode::ZeroPage }),
    // 0x86
    Some(Opcode { name: OpcodeName::STX, addressing_mode: AddressingMode::ZeroPage }),
    // 0x87
    None,
    // 0x88
    Some(Opcode { name: OpcodeName::DEY, addressing_mode: AddressingMode::Implied }),
    // 0x89
    None,
    // 0x8A
    Some(Opcode { name: OpcodeName::TXA, addressing_mode: AddressingMode::Implied }),
    // 0x8B
    None,
    // 0x8C
    Some(Opcode { name: OpcodeName::STY, addressing_mode: AddressingMode::Absolute }),
    // 0x8D
    Some(Opcode { name: OpcodeName::STA, addressing_mode: AddressingMode::Absolute }),
    // 0x8E
    Some(Opcode { name: OpcodeName::STX, addressing_mode: AddressingMode::Absolute }),
    // 0x8F
    None,
    // 0x90
    Some(Opcode { name: OpcodeName::BCC, addressing_mode: AddressingMode::Relative }),
    // 0x91
    Some(Opcode { name: OpcodeName::STA, addressing_mode: AddressingMode::IndirectIndexedY }),
    // 0x92
    None,
    // 0x93
    None,
    // 0x94
    Some(Opcode { name: OpcodeName::STY, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x95
    Some(Opcode { name: OpcodeName::STA, addressing_mode: AddressingMode::ZeroPageX }),
    // 0x96
    Some(Opcode { name: OpcodeName::STX, addressing_mode: AddressingMode::ZeroPageY }),
    // 0x97
    None,
    // 0x98
    Some(Opcode { name: OpcodeName::TYA, addressing_mode: AddressingMode::Implied }),
    // 0x99
    Some(Opcode { name: OpcodeName::STA, addressing_mode: AddressingMode::AbsoluteY }),
    // 0x9A
    Some(Opcode { name: OpcodeName::TXS, addressing_mode: AddressingMode::Implied }),
    // 0x9B
    None,
    // 0x9C
    None,
    // 0x9D
    Some(Opcode { name: OpcodeName::STA, addressing_mode: AddressingMode::AbsoluteX }),
    // 0x9E
    None,
    // 0x9F
    None,
    // 0xA0
    Some(Opcode { name: OpcodeName::LDY, addressing_mode: AddressingMode::Immediate }),
    // 0xA1
    Some(Opcode { name: OpcodeName::LDA, addressing_mode: AddressingMode::IndexedIndirectX }),
    // 0xA2
    Some(Opcode { name: OpcodeName::LDX, addressing_mode: AddressingMode::Immediate }),
    // 0xA3
    None,
    // 0xA4
    Some(Opcode { name: OpcodeName::LDY, addressing_mode: AddressingMode::ZeroPage }),
    // 0xA5
    Some(Opcode { name: OpcodeName::LDA, addressing_mode: AddressingMode::ZeroPage }),
    // 0xA6
    Some(Opcode { name: OpcodeName::LDX, addressing_mode: AddressingMode::ZeroPage }),
    // 0xA7
    None,
    // 0xA8
    Some(Opcode { name: OpcodeName::TAY, addressing_mode: AddressingMode::Implied }),
    // 0xA9
    Some(Opcode { name: OpcodeName::LDA, addressing_mode: AddressingMode::Immediate }),
    // 0xAA
    Some(Opcode { name: OpcodeName::TAX, addressing_mode: AddressingMode::Implied }),
    // 0xAB
    None,
    // 0xAC
    Some(Opcode { name: OpcodeName::LDY, addressing_mode: AddressingMode::Absolute }),
    // 0xAD
    Some(Opcode { name: OpcodeName::LDA, addressing_mode: AddressingMode::Absolute }),
    // 0xAE
    Some(Opcode { name: OpcodeName::LDX, addressing_mode: AddressingMode::Absolute }),
    // 0xAF
    None,
    // 0xB0
    Some(Opcode { name: OpcodeName::BCS, addressing_mode: AddressingMode::Relative }),
    // 0xB1
    Some(Opcode { name: OpcodeName::LDA, addressing_mode: AddressingMode::IndirectIndexedY }),
    // 0xB2
    None,
    // 0xB3
    None,
    // 0xB4
    Some(Opcode { name: OpcodeName::LDY, addressing_mode: AddressingMode::ZeroPageX }),
    // 0xB5
    Some(Opcode { name: OpcodeName::LDA, addressing_mode: AddressingMode::ZeroPageX }),
    // 0xB6
    Some(Opcode { name: OpcodeName::LDX, addressing_mode: AddressingMode::ZeroPageY }),
    // 0xB7
    None,
    // 0xB8
    Some(Opcode { name: OpcodeName::CLV, addressing_mode: AddressingMode::Implied }),
    // 0xB9
    Some(Opcode { name: OpcodeName::LDA, addressing_mode: AddressingMode::AbsoluteY }),
    // 0xBA
    Some(Opcode { name: OpcodeName::TSX, addressing_mode: AddressingMode::Implied }),
    // 0xBB
    None,
    // 0xBC
    Some(Opcode { name: OpcodeName::LDY, addressing_mode: AddressingMode::AbsoluteX }),
    // 0xBD
    Some(Opcode { name: OpcodeName::LDA, addressing_mode: AddressingMode::AbsoluteX }),
    // 0xBE
    Some(Opcode { name: OpcodeName::LDX, addressing_mode: AddressingMode::AbsoluteY }),
    // 0xBF
    None,
    // 0xC0
    Some(Opcode { name: OpcodeName::CPY, addressing_mode: AddressingMode::Immediate }),
    // 0xC1
    Some(Opcode { name: OpcodeName::CMP, addressing_mode: AddressingMode::IndexedIndirectX }),
    // 0xC2
    None,
    // 0xC3
    None,
    // 0xC4
    Some(Opcode { name: OpcodeName::CPY, addressing_mode: AddressingMode::ZeroPage }),
    // 0xC5
    Some(Opcode { name: OpcodeName::CMP, addressing_mode: AddressingMode::ZeroPage }),
    // 0xC6
    Some(Opcode { name: OpcodeName::DEC, addressing_mode: AddressingMode::ZeroPage }),
    // 0xC7
    None,
    // 0xC8
    Some(Opcode { name: OpcodeName::INY, addressing_mode: AddressingMode::Implied }),
    // 0xC9
    Some(Opcode { name: OpcodeName::CMP, addressing_mode: AddressingMode::Immediate }),
    // 0xCA
    Some(Opcode { name: OpcodeName::DEX, addressing_mode: AddressingMode::Implied }),
    // 0xCB
    None,
    // 0xCC
    Some(Opcode { name: OpcodeName::CPY, addressing_mode: AddressingMode::Absolute }),
    // 0xCD
    Some(Opcode { name: OpcodeName::CMP, addressing_mode: AddressingMode::Absolute }),
    // 0xCE
    Some(Opcode { name: OpcodeName::DEC, addressing_mode: AddressingMode::Absolute }),
    // 0xCF
    None,
    // 0xD0
    Some(Opcode { name: OpcodeName::BNE, addressing_mode: AddressingMode::Relative }),
    // 0xD1
    Some(Opcode { name: OpcodeName::CMP, addressing_mode: AddressingMode::IndirectIndexedY }),
    // 0xD2
    None,
    // 0xD3
    None,
    // 0xD4
    None,
    // 0xD5
    Some(Opcode { name: OpcodeName::CMP, addressing_mode: AddressingMode::ZeroPageX }),
    // 0xD6
    Some(Opcode { name: OpcodeName::DEC, addressing_mode: AddressingMode::ZeroPageX }),
    // 0xD7
    None,
    // 0xD8
    Some(Opcode { name: OpcodeName::CLD, addressing_mode: AddressingMode::Implied }),
    // 0xD9
    Some(Opcode { name: OpcodeName::CMP, addressing_mode: AddressingMode::AbsoluteY }),
    // 0xDA
    None,
    // 0xDB
    None,
    // 0xDC
    None,
    // 0xDD
    Some(Opcode { name: OpcodeName::CMP, addressing_mode: AddressingMode::AbsoluteX }),
    // 0xDE
    Some(Opcode { name: OpcodeName::DEC, addressing_mode: AddressingMode::AbsoluteX }),
    // 0xDF
    None,
    // 0xE0
    Some(Opcode { name: OpcodeName::CPX, addressing_mode: AddressingMode::Immediate }),
    // 0xE1
    Some(Opcode { name: OpcodeName::SBC, addressing_mode: AddressingMode::IndexedIndirectX }),
    // 0xE2
    None,
    // 0xE3
    None,
    // 0xE4
    Some(Opcode { name: OpcodeName::CPX, addressing_mode: AddressingMode::ZeroPage }),
    // 0xE5
    Some(Opcode { name: OpcodeName::SBC, addressing_mode: AddressingMode::ZeroPage }),
    // 0xE6
    Some(Opcode { name: OpcodeName::INC, addressing_mode: AddressingMode::ZeroPage }),
    // 0xE7
    None,
    // 0xE8
    Some(Opcode { name: OpcodeName::INX, addressing_mode: AddressingMode::Implied }),
    // 0xE9
    Some(Opcode { name: OpcodeName::SBC, addressing_mode: AddressingMode::Immediate }),
    // 0xEA
    Some(Opcode { name: OpcodeName::NOP, addressing_mode: AddressingMode::Implied }),
    // 0xEB
    None,
    // 0xEC
    Some(Opcode { name: OpcodeName::CPX, addressing_mode: AddressingMode::Absolute }),
    // 0xED
    Some(Opcode { name: OpcodeName::SBC, addressing_mode: AddressingMode::Absolute }),
    // 0xEE
    Some(Opcode { name: OpcodeName::INC, addressing_mode: AddressingMode::Absolute }),
    // 0xEF
    None,
    // 0xF0
    Some(Opcode { name: OpcodeName::BEQ, addressing_mode: AddressingMode::Relative }),
    // 0xF1
    Some(Opcode { name: OpcodeName::SBC, addressing_mode: AddressingMode::IndirectIndexedY }),
    // 0xF2
    None,
    // 0xF3
    None,
    // 0xF4
    None,
    // 0xF5
    Some(Opcode { name: OpcodeName::SBC, addressing_mode: AddressingMode::ZeroPageX }),
    // 0xF6
    Some(Opcode { name: OpcodeName::INC, addressing_mode: AddressingMode::ZeroPageX }),
    // 0xF7
    None,
    // 0xF8
    Some(Opcode { name: OpcodeName::SED, addressing_mode: AddressingMode::Implied }),
    // 0xF9
    Some(Opcode { name: OpcodeName::SBC, addressing_mode: AddressingMode::AbsoluteY }),
    // 0xFA
    None,
    // 0xFB
    None,
    // 0xFC
    None,
    // 0xFD
    Some(Opcode { name: OpcodeName::SBC, addressing_mode: AddressingMode::AbsoluteX }),
    // 0xFE
    Some(Opcode { name: OpcodeName::INC, addressing_mode: AddressingMode::AbsoluteX }),
    // 0xFF
    None,
];
