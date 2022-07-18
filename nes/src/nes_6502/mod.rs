// Opcodes are better in capitals change my mind
#![allow(non_snake_case)]

use std::{collections::BTreeMap, ops::RangeInclusive};

use crate::bus::{Bus, MEM_SIZE};

use self::{addressing_modes::AddressMode, opcodes::Opcode};
mod opcodes;
mod addressing_modes;

/// Contains CPU flags
pub enum Flags6502 {
    /// Carry Bit
    /// Either set by the user to inform an operation we want
    /// to use a carry bit, or set by the operation itself
    C = (1 << 0),
    /// Zero
    /// Set when the result of an operation equals zero
    Z = (1 << 1),   
    /// Disable Interrupts
    /// When this flag is set interrupts will be disabled
    I = (1 << 2),
    /// Decimal Mode
    /// (unused, the NES's 6502 does not have decimal mode)
    D = (1 << 3),
    /// Break
    /// Indicates if the break operation has been called
    B = (1 << 4),
    /// Unused
    U = (1 << 5),
    /// Overflow
    /// Used if you want to use the 6502 with signed variables
    V = (1 << 6),
    /// Negative
    /// Used if you want to use the 6502 with signed variables
    N = (1 << 7),
}

/// Contains NES hardware
pub struct Nes6502 {
    pub bus: Bus,

    /// Accumulator Register
    pub a: u8,
    /// X Register
    pub x: u8,
    /// Y Register
    pub y: u8,
    /// Stack pointer (pointers to a location on the bus)
    pub stkp: u8,
    /// Program counter
    pub pc: u16,
    /// Status Register
    pub status: u8,

    // Variables bellow assiste to facilitate emulation

    /// Represents the working input value to the ALU
    fetched: u8,
    /// All used memory addresses end up in here
    pub addr_abs: u16,
    /// Represents absolute address following a branch
    addr_rel: u16,
    /// The instruction byte
    pub opcode: u8,
    /// Counts how many cycles the instruction has remaining
    cycles: u8,
    lookup: Vec<Instruction>,
}

struct Instruction {
    name: &'static str,
    operate: Opcode,
    addrmode: AddressMode,
    cycles: u8,
}

impl Nes6502 {
    pub fn new() -> Nes6502 {
        let mut ram= [0; MEM_SIZE]; 
        ram.iter_mut().for_each(|r| *r = 0x00);

        let mut nes = Nes6502 { 
            a: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            pc: 0x0000,
            status: 0x00,
            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0,
            lookup: Vec::new(),

            bus: Bus {
                ram
            }   
        };

        nes.lookup = vec![
            Instruction{ name: "BRK", operate: Opcode::BRK, addrmode: AddressMode::IMP, cycles: 7 },Instruction{ name: "ORA", operate: Opcode::ORA,  addrmode: AddressMode::IZX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 3 },Instruction{ name: "ORA", operate: Opcode::ORA,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "ASL", operate: Opcode::ASL,  addrmode: AddressMode::ZP0, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "PHP", operate: Opcode::PHP,   addrmode: AddressMode::IMP, cycles: 3 },Instruction{ name: "ORA",  operate: Opcode::ORA,  addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "ASL",  operate: Opcode::ASL,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "ORA",  operate: Opcode::ORA, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "ASL",  operate: Opcode::ASL, addrmode: AddressMode::ABS, cycles: 6 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },
            Instruction{ name: "BPL", operate: Opcode::BPL, addrmode: AddressMode::REL, cycles: 2 },Instruction{ name: "ORA", operate: Opcode::ORA,  addrmode: AddressMode::IZY, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "ORA", operate: Opcode::ORA,  addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "ASL", operate: Opcode::ASL,  addrmode: AddressMode::ZPX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "CLC", operate: Opcode::CLC,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "ORA",  operate: Opcode::ORA,  addrmode: AddressMode::ABY, cycles: 4 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "ORA",  operate: Opcode::ORA, addrmode: AddressMode::ABX, cycles: 4 },Instruction{ name: "ASL",  operate: Opcode::ASL, addrmode: AddressMode::ABX, cycles: 7 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },
            Instruction{ name: "JSR", operate: Opcode::JSR, addrmode: AddressMode::ABS, cycles: 6 },Instruction{ name: "AND", operate: Opcode::AND,  addrmode: AddressMode::IZX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "BIT", operate: Opcode::BIT,   addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "AND", operate: Opcode::AND,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "ROL", operate: Opcode::ROL,  addrmode: AddressMode::ZP0, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "PLP", operate: Opcode::PLP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "AND",  operate: Opcode::AND,  addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "ROL",  operate: Opcode::ROL,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "BIT",  operate: Opcode::BIT,   addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "AND",  operate: Opcode::AND, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "ROL",  operate: Opcode::ROL, addrmode: AddressMode::ABS, cycles: 6 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },
            Instruction{ name: "BMI", operate: Opcode::BMI, addrmode: AddressMode::REL, cycles: 2 },Instruction{ name: "AND", operate: Opcode::AND,  addrmode: AddressMode::IZY, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "AND", operate: Opcode::AND,  addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "ROL", operate: Opcode::ROL,  addrmode: AddressMode::ZPX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "SEC", operate: Opcode::SEC,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "AND",  operate: Opcode::AND,  addrmode: AddressMode::ABY, cycles: 4 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "AND",  operate: Opcode::AND, addrmode: AddressMode::ABX, cycles: 4 },Instruction{ name: "ROL",  operate: Opcode::ROL, addrmode: AddressMode::ABX, cycles: 7 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },
            Instruction{ name: "RTI", operate: Opcode::RTI, addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "EOR", operate: Opcode::EOR,  addrmode: AddressMode::IZX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 3 },Instruction{ name: "EOR", operate: Opcode::EOR,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "LSR", operate: Opcode::LSR,  addrmode: AddressMode::ZP0, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "PHA", operate: Opcode::PHA,   addrmode: AddressMode::IMP, cycles: 3 },Instruction{ name: "EOR",  operate: Opcode::EOR,  addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "LSR",  operate: Opcode::LSR,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "JMP",  operate: Opcode::JMP,   addrmode: AddressMode::ABS, cycles: 3 },Instruction{ name: "EOR",  operate: Opcode::EOR, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "LSR",  operate: Opcode::LSR, addrmode: AddressMode::ABS, cycles: 6 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },
            Instruction{ name: "BVC", operate: Opcode::BVC, addrmode: AddressMode::REL, cycles: 2 },Instruction{ name: "EOR", operate: Opcode::EOR,  addrmode: AddressMode::IZY, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "EOR", operate: Opcode::EOR,  addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "LSR", operate: Opcode::LSR,  addrmode: AddressMode::ZPX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "CLI", operate: Opcode::CLI,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "EOR",  operate: Opcode::EOR,  addrmode: AddressMode::ABY, cycles: 4 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "EOR",  operate: Opcode::EOR, addrmode: AddressMode::ABX, cycles: 4 },Instruction{ name: "LSR",  operate: Opcode::LSR, addrmode: AddressMode::ABX, cycles: 7 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },
            Instruction{ name: "RTS", operate: Opcode::RTS, addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "ADC", operate: Opcode::ADC,  addrmode: AddressMode::IZX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 3 },Instruction{ name: "ADC", operate: Opcode::ADC,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "ROR", operate: Opcode::ROR,  addrmode: AddressMode::ZP0, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "PLA", operate: Opcode::PLA,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "ADC",  operate: Opcode::ADC,  addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "ROR",  operate: Opcode::ROR,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "JMP",  operate: Opcode::JMP,   addrmode: AddressMode::IND, cycles: 5 },Instruction{ name: "ADC",  operate: Opcode::ADC, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "ROR",  operate: Opcode::ROR, addrmode: AddressMode::ABS, cycles: 6 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },
            Instruction{ name: "BVS", operate: Opcode::BVS, addrmode: AddressMode::REL, cycles: 2 },Instruction{ name: "ADC", operate: Opcode::ADC,  addrmode: AddressMode::IZY, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "ADC", operate: Opcode::ADC,  addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "ROR", operate: Opcode::ROR,  addrmode: AddressMode::ZPX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "SEI", operate: Opcode::SEI,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "ADC",  operate: Opcode::ADC,  addrmode: AddressMode::ABY, cycles: 4 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "ADC",  operate: Opcode::ADC, addrmode: AddressMode::ABX, cycles: 4 },Instruction{ name: "ROR",  operate: Opcode::ROR, addrmode: AddressMode::ABX, cycles: 7 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },
            Instruction{ name: "???", operate: Opcode::NOP, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "STA", operate: Opcode::STA,  addrmode: AddressMode::IZX, cycles: 6 },Instruction{ name: "???", operate: Opcode::NOP, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "STY", operate: Opcode::STY,   addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "STA", operate: Opcode::STA,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "STX", operate: Opcode::STX,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 3 },Instruction{ name: "DEY", operate: Opcode::DEY,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::NOP,  addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "TXA",  operate: Opcode::TXA,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "STY",  operate: Opcode::STY,   addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "STA",  operate: Opcode::STA, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "STX",  operate: Opcode::STX, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 4 },
            Instruction{ name: "BCC", operate: Opcode::BCC, addrmode: AddressMode::REL, cycles: 2 },Instruction{ name: "STA", operate: Opcode::STA,  addrmode: AddressMode::IZY, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "STY", operate: Opcode::STY,   addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "STA", operate: Opcode::STA,  addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "STX", operate: Opcode::STX,  addrmode: AddressMode::ZPY, cycles: 4 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "TYA", operate: Opcode::TYA,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "STA",  operate: Opcode::STA,  addrmode: AddressMode::ABY, cycles: 5 },Instruction{ name: "TXS",  operate: Opcode::TXS,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "STA",  operate: Opcode::STA, addrmode: AddressMode::ABX, cycles: 5 },Instruction{ name: "???",  operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },
            Instruction{ name: "LDY", operate: Opcode::LDY, addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "LDA", operate: Opcode::LDA,  addrmode: AddressMode::IZX, cycles: 6 },Instruction{ name: "LDX", operate: Opcode::LDX, addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "LDY", operate: Opcode::LDY,   addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "LDA", operate: Opcode::LDA,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "LDX", operate: Opcode::LDX,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 3 },Instruction{ name: "TAY", operate: Opcode::TAY,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "LDA",  operate: Opcode::LDA,  addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "TAX",  operate: Opcode::TAX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "LDY",  operate: Opcode::LDY,   addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "LDA",  operate: Opcode::LDA, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "LDX",  operate: Opcode::LDX, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 4 },
            Instruction{ name: "BCS", operate: Opcode::BCS, addrmode: AddressMode::REL, cycles: 2 },Instruction{ name: "LDA", operate: Opcode::LDA,  addrmode: AddressMode::IZY, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "LDY", operate: Opcode::LDY,   addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "LDA", operate: Opcode::LDA,  addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "LDX", operate: Opcode::LDX,  addrmode: AddressMode::ZPY, cycles: 4 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "CLV", operate: Opcode::CLV,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "LDA",  operate: Opcode::LDA,  addrmode: AddressMode::ABY, cycles: 4 },Instruction{ name: "TSX",  operate: Opcode::TSX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "LDY",  operate: Opcode::LDY,   addrmode: AddressMode::ABX, cycles: 4 },Instruction{ name: "LDA",  operate: Opcode::LDA, addrmode: AddressMode::ABX, cycles: 4 },Instruction{ name: "LDX",  operate: Opcode::LDX, addrmode: AddressMode::ABY, cycles: 4 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 4 },
            Instruction{ name: "CPY", operate: Opcode::CPY, addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "CMP", operate: Opcode::CMP,  addrmode: AddressMode::IZX, cycles: 6 },Instruction{ name: "???", operate: Opcode::NOP, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "CPY", operate: Opcode::CPY,   addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "CMP", operate: Opcode::CMP,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "DEC", operate: Opcode::DEC,  addrmode: AddressMode::ZP0, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "INY", operate: Opcode::INY,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "CMP",  operate: Opcode::CMP,  addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "DEX",  operate: Opcode::DEX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "CPY",  operate: Opcode::CPY,   addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "CMP",  operate: Opcode::CMP, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "DEC",  operate: Opcode::DEC, addrmode: AddressMode::ABS, cycles: 6 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },
            Instruction{ name: "BNE", operate: Opcode::BNE, addrmode: AddressMode::REL, cycles: 2 },Instruction{ name: "CMP", operate: Opcode::CMP,  addrmode: AddressMode::IZY, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "CMP", operate: Opcode::CMP,  addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "DEC", operate: Opcode::DEC,  addrmode: AddressMode::ZPX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "CLD", operate: Opcode::CLD,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "CMP",  operate: Opcode::CMP,  addrmode: AddressMode::ABY, cycles: 4 },Instruction{ name: "NOP",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "CMP",  operate: Opcode::CMP, addrmode: AddressMode::ABX, cycles: 4 },Instruction{ name: "DEC",  operate: Opcode::DEC, addrmode: AddressMode::ABX, cycles: 7 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },
            Instruction{ name: "CPX", operate: Opcode::CPX, addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "SBC", operate: Opcode::SBC,  addrmode: AddressMode::IZX, cycles: 6 },Instruction{ name: "???", operate: Opcode::NOP, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "CPX", operate: Opcode::CPX,   addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "SBC", operate: Opcode::SBC,  addrmode: AddressMode::ZP0, cycles: 3 },Instruction{ name: "INC", operate: Opcode::INC,  addrmode: AddressMode::ZP0, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 5 },Instruction{ name: "INX", operate: Opcode::INX,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "SBC",  operate: Opcode::SBC,  addrmode: AddressMode::IMM, cycles: 2 },Instruction{ name: "NOP",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::SBC,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "CPX",  operate: Opcode::CPX,   addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "SBC",  operate: Opcode::SBC, addrmode: AddressMode::ABS, cycles: 4 },Instruction{ name: "INC",  operate: Opcode::INC, addrmode: AddressMode::ABS, cycles: 6 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },
            Instruction{ name: "BEQ", operate: Opcode::BEQ, addrmode: AddressMode::REL, cycles: 2 },Instruction{ name: "SBC", operate: Opcode::SBC,  addrmode: AddressMode::IZY, cycles: 5 },Instruction{ name: "???", operate: Opcode::XXX, addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 8 },Instruction{ name: "???", operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "SBC", operate: Opcode::SBC,  addrmode: AddressMode::ZPX, cycles: 4 },Instruction{ name: "INC", operate: Opcode::INC,  addrmode: AddressMode::ZPX, cycles: 6 },Instruction{ name: "???", operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 6 },Instruction{ name: "SED", operate: Opcode::SED,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "SBC",  operate: Opcode::SBC,  addrmode: AddressMode::ABY, cycles: 4 },Instruction{ name: "NOP",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 2 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },Instruction{ name: "???",  operate: Opcode::NOP,   addrmode: AddressMode::IMP, cycles: 4 },Instruction{ name: "SBC",  operate: Opcode::SBC, addrmode: AddressMode::ABX, cycles: 4 },Instruction{ name: "INC",  operate: Opcode::INC, addrmode: AddressMode::ABX, cycles: 7 },Instruction{ name: "???",  operate: Opcode::XXX,   addrmode: AddressMode::IMP, cycles: 7 },
        ];

        nes
    }

    /// Perform one clock cycle's worth of update
    pub fn clock(&mut self) {
        // Each instruction requires a variable number of clock cycles to execute,
        // in this emulator, the only thing that matters is the final result and
        // so the entire computation is performed in one hit.

        // To remain compliant with connected devices its important that the emulation
        // also takes time in order to execute instructions, so that delay is 
        // implemented by counting down the cycles required by the instruction. When it
        // reaches 0 the instruction is complete and the next is ready to be executed
        if self.cycles == 0 {
            // Read the next instruction byte, we use this to index the lookup table
            // to get the information needed to implement the instruction
            self.opcode = self.bus.read(self.pc);

            self.set_flag(Flags6502::U, true);
            // Increment program counter, we read the part we needed (the opcode byte)
            self.pc += 1;

            println!("Executing instruction: {} ({:02X})", self.lookup[self.opcode as usize].name, self.opcode);

            // get number of cycles needed for the instruction
            self.cycles = self.lookup[self.opcode as usize].cycles;

            // Perform a fetch of the intermmediate data using the required addressing mode
            let additional_cycle1 = AddressMode::execute(self.lookup[self.opcode as usize].addrmode, self);
            // Perform the operation
            let additional_cycle2 = Opcode::execute(self.lookup[self.opcode as usize].operate, self);

            // The addressing mode and opcode may have altered the number of cycles
            // required for this instruction to complete
            self.cycles += additional_cycle1 & additional_cycle2;

            self.set_flag(Flags6502::U, true);
        }
        // everytime we call the clock function one cycle has elapsed
        // so we decrement the number of cycles remaining for this instruction
        self.cycles -= 1;
    }
    pub fn complete(&self) -> bool {
        self.cycles == 0
    }

    // The functions below represent external event functions, In the NES hardware,
    // these represent pins that are asserted to produce a change in state.

    /// Reset Interrupt - forces the CPU into a known state
    pub fn reset(&mut self) {
        // this address contains the address we want to set our program counter to
        self.addr_abs = 0xFFFC;
        // get lo byte of the address
        let lo = self.bus.read(self.addr_abs + 0) as u16;
        // get hi byte of the address
        let hi = self.bus.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00 | Flags6502::U as u8;

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        self.cycles = 8;
    }
    /// Interrupt Request - Executes an instruction at a specific location
    pub fn irq(&mut self) {
        // if interrupts are enabled
        if self.get_flag(Flags6502::I) == 0 {
            // push the program counter to the stack
            self.bus.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.stkp -= 1;
            self.bus.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
            self.stkp -= 1;

            self.set_flag(Flags6502::B, false);
            self.set_flag(Flags6502::U, true);
            self.set_flag(Flags6502::I, true);
            // write the status to the stack
            self.bus.write(0x0100 + self.stkp as u16, self.status);
            self.stkp -= 1;

            // get the value of the new program counter; forces the program to jump to a 
            // known location set by the programmer to handle the interupt.
            self.addr_abs = 0xFFFE;
            let lo = self.bus.read(self.addr_abs + 0) as u16;
            let hi = self.bus.read(self.addr_abs + 1) as u16;
            self.pc = (hi << 8) | lo;

            self.cycles = 7;
        }
    }
    /// Non-Maskable Interrupt Request - Same as irq but cannot be disabled
    pub fn nmi(&mut self) {
        // push the program counter to the stack
        self.bus.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.bus.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, true);
        self.set_flag(Flags6502::I, true);
        // write the status to the stack
        self.bus.write(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;

        // get the value of the new program counter; forces the program to jump to a 
        // known location set by the programmer to handle the interupt.
        self.addr_abs = 0xFFFA;
        let lo = self.bus.read(self.addr_abs + 0) as u16;
        let hi = self.bus.read(self.addr_abs + 1) as u16;
        self.pc = (hi << 8) | lo;

        self.cycles = 7;
    }

    /// This function fetches the data used by the instruction into a convenient
    /// numeric variable. But some instructions dont have to fetch data as the
    /// source is implied by the instruction. For example INX increments the x
    /// register. There is no additional data required. For all other addressing 
    /// modes the data resides at the location held within addr_abs, so it is read
    /// from there.
    /// The read location of data can come from two sources, a memory address, or
    /// its immediately available as part of the instruction. This function decides
    /// depending on address mode of the instruction byte
    /// 
    /// This function stores the fetched data in the fetched variable, but also 
    /// returns it for convienience.
    fn fetch(&mut self) -> u8 {
        // If the addressing is implied ( no additional data; nothing to fetch )
        if self.lookup[self.opcode as usize].addrmode != AddressMode::IMP {
            // set fetched to the contents of the address
            self.fetched = self.bus.read(self.addr_abs);
        }
        // returned the fetched data
        self.fetched
    }

    // Convenience functions to access status register
    /// Returns the value of a specific bit of the status register
    pub fn get_flag(&self, f: Flags6502) -> u8 {
        return if (self.status & f as u8) > 0 {
            1
        } else {
            0
        }
    }
    /// Sets of clears a bit of the status register
    /// If v is true set a bit, other wise clear the bit
    fn set_flag(&mut self, f: Flags6502, v: bool) {
        if v {
            self.status |= f as u8;
        } else {
            self.status &= !(f as u8);
        }
    }    

    /// Produces a hash map of strings, with keys equivalent to instruction
    /// start locations in memory, for the specified address range
    pub fn disassemble(&self, range: RangeInclusive<u16>) -> BTreeMap<u16, String> {
        let mut addr = *range.start();
        let end = *range.end();

        let mut value: u8 = 0x00;
        let mut lo: u16 = 0x00;
        let mut hi: u16 = 0x00;
        let mut map_lines = BTreeMap::new();

        while addr <= end {
            let line_addr = addr;

            // prefix line with instruction address
            let mut s_inst = format!("${:X}: ", addr);

            // read instruction and get its readable name
            let opcode = self.bus.read(addr as u16);
            addr += 1;
            s_inst = format!("{}{} ", s_inst, self.lookup[opcode as usize].name); 

            if self.lookup[opcode as usize].addrmode == AddressMode::IMP {
                s_inst = format!("{} {{IMP}}", s_inst);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::IMM {
                value = self.bus.read(addr as u16);
                addr += 1;
                s_inst = format!("{}#${:02X} {{IMM}}", s_inst, value);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::ZP0 {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}${:02X} {{ZP0}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::ZPX {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}${:02X}, X {{ZPX}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::ZPY {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}${:02X}, Y {{ZPY}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::IZX {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}(${:02X}, X) {{IZX}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::IZY {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}(${:02X}), Y {{IZY}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::ABS {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:04X} {{ABS}}", s_inst, (hi << 8) | lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::ABX {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:04X}, X {{ABX}}", s_inst, (hi << 8) | lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::ABY {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:04X}, Y {{ABY}}", s_inst, (hi << 8) | lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::IND {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}(${:04X}) {{IND}}", s_inst, (hi << 8) | lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::REL {
                value = self.bus.read(addr as u16);
                addr += 1;
                s_inst = format!("{}${:X} [${:04X}] {{REL}}", s_inst, value, addr + value as u16);
            }

            map_lines.insert(line_addr, s_inst);
        }

        map_lines
    }
}