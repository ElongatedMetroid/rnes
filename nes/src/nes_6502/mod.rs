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

    /// Put all things you would usually println into here
    pub info: Vec<String>,
}

struct Instruction {
    name: &'static str,
    operate: Opcode,
    addrmode: AddressMode,
    cycles: u8,
}

impl Default for Nes6502 {
    fn default() -> Self {
        Self::new()    
    }
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
            info: Vec::new(),

            bus: Bus {
                ram
            }   
        };

        nes.lookup = vec![
            Instruction{ name: "BRK", operate: Opcode::Brk, addrmode: AddressMode::Imp, cycles: 7 },Instruction{ name: "ORA", operate: Opcode::Ora,  addrmode: AddressMode::Izx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 3 },Instruction{ name: "ORA", operate: Opcode::Ora,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "ASL", operate: Opcode::Asl,  addrmode: AddressMode::Zp0, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "PHP", operate: Opcode::Php,   addrmode: AddressMode::Imp, cycles: 3 },Instruction{ name: "ORA",  operate: Opcode::Ora,  addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "ASL",  operate: Opcode::Asl,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "ORA",  operate: Opcode::Ora, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "ASL",  operate: Opcode::Asl, addrmode: AddressMode::Abs, cycles: 6 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },
            Instruction{ name: "BPL", operate: Opcode::Bpl, addrmode: AddressMode::Rel, cycles: 2 },Instruction{ name: "ORA", operate: Opcode::Ora,  addrmode: AddressMode::Izy, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "ORA", operate: Opcode::Ora,  addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "ASL", operate: Opcode::Asl,  addrmode: AddressMode::Zpx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "CLC", operate: Opcode::Clc,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "ORA",  operate: Opcode::Ora,  addrmode: AddressMode::Aby, cycles: 4 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "ORA",  operate: Opcode::Ora, addrmode: AddressMode::Abx, cycles: 4 },Instruction{ name: "ASL",  operate: Opcode::Asl, addrmode: AddressMode::Abx, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },
            Instruction{ name: "JSR", operate: Opcode::Jsr, addrmode: AddressMode::Abs, cycles: 6 },Instruction{ name: "AND", operate: Opcode::And,  addrmode: AddressMode::Izx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "BIT", operate: Opcode::Bit,   addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "AND", operate: Opcode::And,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "ROL", operate: Opcode::Rol,  addrmode: AddressMode::Zp0, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "PLP", operate: Opcode::Plp,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "AND",  operate: Opcode::And,  addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "ROL",  operate: Opcode::Rol,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "BIT",  operate: Opcode::Bit,   addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "AND",  operate: Opcode::And, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "ROL",  operate: Opcode::Rol, addrmode: AddressMode::Abs, cycles: 6 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },
            Instruction{ name: "BMI", operate: Opcode::Bmi, addrmode: AddressMode::Rel, cycles: 2 },Instruction{ name: "AND", operate: Opcode::And,  addrmode: AddressMode::Izy, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "AND", operate: Opcode::And,  addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "ROL", operate: Opcode::Rol,  addrmode: AddressMode::Zpx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "SEC", operate: Opcode::Sec,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "AND",  operate: Opcode::And,  addrmode: AddressMode::Aby, cycles: 4 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "AND",  operate: Opcode::And, addrmode: AddressMode::Abx, cycles: 4 },Instruction{ name: "ROL",  operate: Opcode::Rol, addrmode: AddressMode::Abx, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },
            Instruction{ name: "RTI", operate: Opcode::Rti, addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "EOR", operate: Opcode::Eor,  addrmode: AddressMode::Izx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 3 },Instruction{ name: "EOR", operate: Opcode::Eor,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "LSR", operate: Opcode::Lsr,  addrmode: AddressMode::Zp0, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "PHA", operate: Opcode::Pha,   addrmode: AddressMode::Imp, cycles: 3 },Instruction{ name: "EOR",  operate: Opcode::Eor,  addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "LSR",  operate: Opcode::Lsr,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "JMP",  operate: Opcode::Jmp,   addrmode: AddressMode::Abs, cycles: 3 },Instruction{ name: "EOR",  operate: Opcode::Eor, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "LSR",  operate: Opcode::Lsr, addrmode: AddressMode::Abs, cycles: 6 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },
            Instruction{ name: "BVC", operate: Opcode::Bvc, addrmode: AddressMode::Rel, cycles: 2 },Instruction{ name: "EOR", operate: Opcode::Eor,  addrmode: AddressMode::Izy, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "EOR", operate: Opcode::Eor,  addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "LSR", operate: Opcode::Lsr,  addrmode: AddressMode::Zpx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "CLI", operate: Opcode::Cli,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "EOR",  operate: Opcode::Eor,  addrmode: AddressMode::Aby, cycles: 4 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "EOR",  operate: Opcode::Eor, addrmode: AddressMode::Abx, cycles: 4 },Instruction{ name: "LSR",  operate: Opcode::Lsr, addrmode: AddressMode::Abx, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },
            Instruction{ name: "RTS", operate: Opcode::Rts, addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "ADC", operate: Opcode::Adc,  addrmode: AddressMode::Izx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 3 },Instruction{ name: "ADC", operate: Opcode::Adc,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "ROR", operate: Opcode::Ror,  addrmode: AddressMode::Zp0, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "PLA", operate: Opcode::Pla,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "ADC",  operate: Opcode::Adc,  addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "ROR",  operate: Opcode::Ror,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "JMP",  operate: Opcode::Jmp,   addrmode: AddressMode::Ind, cycles: 5 },Instruction{ name: "ADC",  operate: Opcode::Adc, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "ROR",  operate: Opcode::Ror, addrmode: AddressMode::Abs, cycles: 6 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },
            Instruction{ name: "BVS", operate: Opcode::Bvs, addrmode: AddressMode::Rel, cycles: 2 },Instruction{ name: "ADC", operate: Opcode::Adc,  addrmode: AddressMode::Izy, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "ADC", operate: Opcode::Adc,  addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "ROR", operate: Opcode::Ror,  addrmode: AddressMode::Zpx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "SEI", operate: Opcode::Sei,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "ADC",  operate: Opcode::Adc,  addrmode: AddressMode::Aby, cycles: 4 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "ADC",  operate: Opcode::Adc, addrmode: AddressMode::Abx, cycles: 4 },Instruction{ name: "ROR",  operate: Opcode::Ror, addrmode: AddressMode::Abx, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },
            Instruction{ name: "???", operate: Opcode::Nop, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "STA", operate: Opcode::Sta,  addrmode: AddressMode::Izx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Nop, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "STY", operate: Opcode::Sty,   addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "STA", operate: Opcode::Sta,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "STX", operate: Opcode::Stx,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 3 },Instruction{ name: "DEY", operate: Opcode::Dey,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Nop,  addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "TXA",  operate: Opcode::Txa,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "STY",  operate: Opcode::Sty,   addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "STA",  operate: Opcode::Sta, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "STX",  operate: Opcode::Stx, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 4 },
            Instruction{ name: "BCC", operate: Opcode::Bcc, addrmode: AddressMode::Rel, cycles: 2 },Instruction{ name: "STA", operate: Opcode::Sta,  addrmode: AddressMode::Izy, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "STY", operate: Opcode::Sty,   addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "STA", operate: Opcode::Sta,  addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "STX", operate: Opcode::Stx,  addrmode: AddressMode::Zpy, cycles: 4 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "TYA", operate: Opcode::Tya,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "STA",  operate: Opcode::Sta,  addrmode: AddressMode::Aby, cycles: 5 },Instruction{ name: "TXS",  operate: Opcode::Txs,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "STA",  operate: Opcode::Sta, addrmode: AddressMode::Abx, cycles: 5 },Instruction{ name: "???",  operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },
            Instruction{ name: "LDY", operate: Opcode::Ldy, addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "LDA", operate: Opcode::Lda,  addrmode: AddressMode::Izx, cycles: 6 },Instruction{ name: "LDX", operate: Opcode::Ldx, addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "LDY", operate: Opcode::Ldy,   addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "LDA", operate: Opcode::Lda,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "LDX", operate: Opcode::Ldx,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 3 },Instruction{ name: "TAY", operate: Opcode::Tay,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "LDA",  operate: Opcode::Lda,  addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "TAX",  operate: Opcode::Tax,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "LDY",  operate: Opcode::Ldy,   addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "LDA",  operate: Opcode::Lda, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "LDX",  operate: Opcode::Ldx, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 4 },
            Instruction{ name: "BCS", operate: Opcode::Bcs, addrmode: AddressMode::Rel, cycles: 2 },Instruction{ name: "LDA", operate: Opcode::Lda,  addrmode: AddressMode::Izy, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "LDY", operate: Opcode::Ldy,   addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "LDA", operate: Opcode::Lda,  addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "LDX", operate: Opcode::Ldx,  addrmode: AddressMode::Zpy, cycles: 4 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "CLV", operate: Opcode::Clv,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "LDA",  operate: Opcode::Lda,  addrmode: AddressMode::Aby, cycles: 4 },Instruction{ name: "TSX",  operate: Opcode::Tsx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "LDY",  operate: Opcode::Ldy,   addrmode: AddressMode::Abx, cycles: 4 },Instruction{ name: "LDA",  operate: Opcode::Lda, addrmode: AddressMode::Abx, cycles: 4 },Instruction{ name: "LDX",  operate: Opcode::Ldx, addrmode: AddressMode::Aby, cycles: 4 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 4 },
            Instruction{ name: "CPY", operate: Opcode::Cpy, addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "CMP", operate: Opcode::Cmp,  addrmode: AddressMode::Izx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Nop, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "CPY", operate: Opcode::Cpy,   addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "CMP", operate: Opcode::Cmp,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "DEC", operate: Opcode::Dec,  addrmode: AddressMode::Zp0, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "INY", operate: Opcode::Iny,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "CMP",  operate: Opcode::Cmp,  addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "DEX",  operate: Opcode::Dex,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "CPY",  operate: Opcode::Cpy,   addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "CMP",  operate: Opcode::Cmp, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "DEC",  operate: Opcode::Dec, addrmode: AddressMode::Abs, cycles: 6 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },
            Instruction{ name: "BNE", operate: Opcode::Bne, addrmode: AddressMode::Rel, cycles: 2 },Instruction{ name: "CMP", operate: Opcode::Cmp,  addrmode: AddressMode::Izy, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "CMP", operate: Opcode::Cmp,  addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "DEC", operate: Opcode::Dec,  addrmode: AddressMode::Zpx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "CLD", operate: Opcode::Cld,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "CMP",  operate: Opcode::Cmp,  addrmode: AddressMode::Aby, cycles: 4 },Instruction{ name: "NOP",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "CMP",  operate: Opcode::Cmp, addrmode: AddressMode::Abx, cycles: 4 },Instruction{ name: "DEC",  operate: Opcode::Dec, addrmode: AddressMode::Abx, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },
            Instruction{ name: "CPX", operate: Opcode::Cpx, addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "SBC", operate: Opcode::Sbc,  addrmode: AddressMode::Izx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Nop, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "CPX", operate: Opcode::Cpx,   addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "SBC", operate: Opcode::Sbc,  addrmode: AddressMode::Zp0, cycles: 3 },Instruction{ name: "INC", operate: Opcode::Inc,  addrmode: AddressMode::Zp0, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 5 },Instruction{ name: "INX", operate: Opcode::Inx,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "SBC",  operate: Opcode::Sbc,  addrmode: AddressMode::Imm, cycles: 2 },Instruction{ name: "NOP",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Sbc,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "CPX",  operate: Opcode::Cpx,   addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "SBC",  operate: Opcode::Sbc, addrmode: AddressMode::Abs, cycles: 4 },Instruction{ name: "INC",  operate: Opcode::Inc, addrmode: AddressMode::Abs, cycles: 6 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },
            Instruction{ name: "BEQ", operate: Opcode::Beq, addrmode: AddressMode::Rel, cycles: 2 },Instruction{ name: "SBC", operate: Opcode::Sbc,  addrmode: AddressMode::Izy, cycles: 5 },Instruction{ name: "???", operate: Opcode::Xxx, addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 8 },Instruction{ name: "???", operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "SBC", operate: Opcode::Sbc,  addrmode: AddressMode::Zpx, cycles: 4 },Instruction{ name: "INC", operate: Opcode::Inc,  addrmode: AddressMode::Zpx, cycles: 6 },Instruction{ name: "???", operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 6 },Instruction{ name: "SED", operate: Opcode::Sed,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "SBC",  operate: Opcode::Sbc,  addrmode: AddressMode::Aby, cycles: 4 },Instruction{ name: "NOP",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 2 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Nop,   addrmode: AddressMode::Imp, cycles: 4 },Instruction{ name: "SBC",  operate: Opcode::Sbc, addrmode: AddressMode::Abx, cycles: 4 },Instruction{ name: "INC",  operate: Opcode::Inc, addrmode: AddressMode::Abx, cycles: 7 },Instruction{ name: "???",  operate: Opcode::Xxx,   addrmode: AddressMode::Imp, cycles: 7 },
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
        let lo = self.bus.read(self.addr_abs) as u16;
        // get hi byte of the address
        let hi = self.bus.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = Flags6502::U as u8;

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
            let lo = self.bus.read(self.addr_abs) as u16;
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
        let lo = self.bus.read(self.addr_abs) as u16;
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
        if self.lookup[self.opcode as usize].addrmode != AddressMode::Imp {
            // set fetched to the contents of the address
            self.fetched = self.bus.read(self.addr_abs);
        }
        // returned the fetched data
        self.fetched
    }

    // Convenience functions to access status register
    /// Returns the value of a specific bit of the status register
    pub fn get_flag(&self, f: Flags6502) -> u8 {
        if (self.status & f as u8) > 0 {
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

        let mut value: u8;
        let mut lo: u16;
        let mut hi: u16;
        let mut map_lines = BTreeMap::new();

        while addr <= end {
            let line_addr = addr;

            // prefix line with instruction address
            let mut s_inst = format!("${:X}: ", addr);

            // read instruction and get its readable name
            let opcode = self.bus.read(addr as u16);
            addr += 1;
            s_inst = format!("{}{} ", s_inst, self.lookup[opcode as usize].name); 

            if self.lookup[opcode as usize].addrmode == AddressMode::Imp {
                s_inst = format!("{} {{IMP}}", s_inst);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Imm {
                value = self.bus.read(addr as u16);
                addr += 1;
                s_inst = format!("{}#${:02X} {{IMM}}", s_inst, value);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Zp0 {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:02X} {{ZP0}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Zpx {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:02X}, X {{ZPX}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Zpy {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:02X}, Y {{ZPY}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Izx {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}(${:02X}, X) {{IZX}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Izy {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}(${:02X}), Y {{IZY}}", s_inst, lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Abs {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:04X} {{ABS}}", s_inst, (hi << 8) | lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Abx {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:04X}, X {{ABX}}", s_inst, (hi << 8) | lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Aby {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}${:04X}, Y {{ABY}}", s_inst, (hi << 8) | lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Ind {
                lo = self.bus.read(addr as u16) as u16;
                addr += 1;
                hi = self.bus.read(addr as u16) as u16;
                addr += 1;
                s_inst = format!("{}(${:04X}) {{IND}}", s_inst, (hi << 8) | lo);
            }
            else if self.lookup[opcode as usize].addrmode == AddressMode::Rel {
                value = self.bus.read(addr as u16);
                addr += 1;
                s_inst = format!("{}${:X} [${:04X}] {{REL}}", s_inst, value, addr + value as u16);
            }

            map_lines.insert(line_addr, s_inst);
        }

        map_lines
    }
}