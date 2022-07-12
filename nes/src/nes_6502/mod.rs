// Opcodes are better in capitals change my mind
#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::bus::{Bus , MEM_SIZE, self};
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
    bus: Bus,

    /// Accumulator Register
    a: u8,
    /// X Register
    x: u8,
    /// Y Register
    y: u8,
    /// Stack pointer (pointers to a location on the bus)
    stkp: u8,
    /// Program counter
    pc: u16,
    /// Status Register
    status: u8,

    // Variables bellow assiste to facilitate emulation

    /// Represents the working input value to the ALU
    fetched: u8,
    /// All used memory addresses end up in here
    addr_abs: u16,
    /// Represents absolute address following a branch
    addr_rel: u16,
    /// The instruction byte
    opcode: u8,
    /// Counts how many cycles the instruction has remaining
    cycles: u8,
    lookup: Vec<Instruction>,
}

struct Instruction {
    name: &'static str,
    operate: fn(&mut Nes6502) -> u8,
    addrmode: fn(&mut Nes6502) -> u8,
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
            Instruction{ name: "BRK", operate: |nes| nes.BRK(), addrmode: |nes| nes.IMM(), cycles: 7 },Instruction{ name: "ORA", operate: |nes| nes.ORA(),  addrmode: |nes| nes.IZX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 3 },Instruction{ name: "ORA", operate: |nes| nes.ORA(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "ASL", operate: |nes| nes.ASL(),  addrmode: |nes| nes.ZP0(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "PHP", operate: |nes| nes.PHP(),   addrmode: |nes| nes.IMP(), cycles: 3 },Instruction{ name: "ORA",  operate: |nes| nes.ORA(),  addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "ASL",  operate: |nes| nes.ASL(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "ORA",  operate: |nes| nes.ORA(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "ASL",  operate: |nes| nes.ASL(), addrmode: |nes| nes.ABS(), cycles: 6 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },
            Instruction{ name: "BPL", operate: |nes| nes.BPL(), addrmode: |nes| nes.REL(), cycles: 2 },Instruction{ name: "ORA", operate: |nes| nes.ORA(),  addrmode: |nes| nes.IZY(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "ORA", operate: |nes| nes.ORA(),  addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "ASL", operate: |nes| nes.ASL(),  addrmode: |nes| nes.ZPX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "CLC", operate: |nes| nes.CLC(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "ORA",  operate: |nes| nes.ORA(),  addrmode: |nes| nes.ABY(), cycles: 4 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "ORA",  operate: |nes| nes.ORA(), addrmode: |nes| nes.ABX(), cycles: 4 },Instruction{ name: "ASL",  operate: |nes| nes.ASL(), addrmode: |nes| nes.ABX(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },
            Instruction{ name: "JSR", operate: |nes| nes.JSR(), addrmode: |nes| nes.ABS(), cycles: 6 },Instruction{ name: "AND", operate: |nes| nes.AND(),  addrmode: |nes| nes.IZX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "BIT", operate: |nes| nes.BIT(),   addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "AND", operate: |nes| nes.AND(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "ROL", operate: |nes| nes.ROL(),  addrmode: |nes| nes.ZP0(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "PLP", operate: |nes| nes.PLP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "AND",  operate: |nes| nes.AND(),  addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "ROL",  operate: |nes| nes.ROL(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "BIT",  operate: |nes| nes.BIT(),   addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "AND",  operate: |nes| nes.AND(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "ROL",  operate: |nes| nes.ROL(), addrmode: |nes| nes.ABS(), cycles: 6 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },
            Instruction{ name: "BMI", operate: |nes| nes.BMI(), addrmode: |nes| nes.REL(), cycles: 2 },Instruction{ name: "AND", operate: |nes| nes.AND(),  addrmode: |nes| nes.IZY(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "AND", operate: |nes| nes.AND(),  addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "ROL", operate: |nes| nes.ROL(),  addrmode: |nes| nes.ZPX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "SEC", operate: |nes| nes.SEC(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "AND",  operate: |nes| nes.AND(),  addrmode: |nes| nes.ABY(), cycles: 4 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "AND",  operate: |nes| nes.AND(), addrmode: |nes| nes.ABX(), cycles: 4 },Instruction{ name: "ROL",  operate: |nes| nes.ROL(), addrmode: |nes| nes.ABX(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },
            Instruction{ name: "RTI", operate: |nes| nes.RTI(), addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "EOR", operate: |nes| nes.EOR(),  addrmode: |nes| nes.IZX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 3 },Instruction{ name: "EOR", operate: |nes| nes.EOR(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "LSR", operate: |nes| nes.LSR(),  addrmode: |nes| nes.ZP0(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "PHA", operate: |nes| nes.PHA(),   addrmode: |nes| nes.IMP(), cycles: 3 },Instruction{ name: "EOR",  operate: |nes| nes.EOR(),  addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "LSR",  operate: |nes| nes.LSR(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "JMP",  operate: |nes| nes.JMP(),   addrmode: |nes| nes.ABS(), cycles: 3 },Instruction{ name: "EOR",  operate: |nes| nes.EOR(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "LSR",  operate: |nes| nes.LSR(), addrmode: |nes| nes.ABS(), cycles: 6 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },
            Instruction{ name: "BVC", operate: |nes| nes.BVC(), addrmode: |nes| nes.REL(), cycles: 2 },Instruction{ name: "EOR", operate: |nes| nes.EOR(),  addrmode: |nes| nes.IZY(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "EOR", operate: |nes| nes.EOR(),  addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "LSR", operate: |nes| nes.LSR(),  addrmode: |nes| nes.ZPX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "CLI", operate: |nes| nes.CLI(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "EOR",  operate: |nes| nes.EOR(),  addrmode: |nes| nes.ABY(), cycles: 4 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "EOR",  operate: |nes| nes.EOR(), addrmode: |nes| nes.ABX(), cycles: 4 },Instruction{ name: "LSR",  operate: |nes| nes.LSR(), addrmode: |nes| nes.ABX(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },
            Instruction{ name: "RTS", operate: |nes| nes.RTS(), addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "ADC", operate: |nes| nes.ADC(),  addrmode: |nes| nes.IZX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 3 },Instruction{ name: "ADC", operate: |nes| nes.ADC(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "ROR", operate: |nes| nes.ROR(),  addrmode: |nes| nes.ZP0(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "PLA", operate: |nes| nes.PLA(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "ADC",  operate: |nes| nes.ADC(),  addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "ROR",  operate: |nes| nes.ROR(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "JMP",  operate: |nes| nes.JMP(),   addrmode: |nes| nes.IND(), cycles: 5 },Instruction{ name: "ADC",  operate: |nes| nes.ADC(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "ROR",  operate: |nes| nes.ROR(), addrmode: |nes| nes.ABS(), cycles: 6 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },
            Instruction{ name: "BVS", operate: |nes| nes.BVS(), addrmode: |nes| nes.REL(), cycles: 2 },Instruction{ name: "ADC", operate: |nes| nes.ADC(),  addrmode: |nes| nes.IZY(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "ADC", operate: |nes| nes.ADC(),  addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "ROR", operate: |nes| nes.ROR(),  addrmode: |nes| nes.ZPX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "SEI", operate: |nes| nes.SEI(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "ADC",  operate: |nes| nes.ADC(),  addrmode: |nes| nes.ABY(), cycles: 4 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "ADC",  operate: |nes| nes.ADC(), addrmode: |nes| nes.ABX(), cycles: 4 },Instruction{ name: "ROR",  operate: |nes| nes.ROR(), addrmode: |nes| nes.ABX(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },
            Instruction{ name: "???", operate: |nes| nes.NOP(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "STA", operate: |nes| nes.STA(),  addrmode: |nes| nes.IZX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.NOP(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "STY", operate: |nes| nes.STY(),   addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "STA", operate: |nes| nes.STA(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "STX", operate: |nes| nes.STX(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 3 },Instruction{ name: "DEY", operate: |nes| nes.DEY(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.NOP(),  addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "TXA",  operate: |nes| nes.TXA(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "STY",  operate: |nes| nes.STY(),   addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "STA",  operate: |nes| nes.STA(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "STX",  operate: |nes| nes.STX(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 4 },
            Instruction{ name: "BCC", operate: |nes| nes.BCC(), addrmode: |nes| nes.REL(), cycles: 2 },Instruction{ name: "STA", operate: |nes| nes.STA(),  addrmode: |nes| nes.IZY(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "STY", operate: |nes| nes.STY(),   addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "STA", operate: |nes| nes.STA(),  addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "STX", operate: |nes| nes.STX(),  addrmode: |nes| nes.ZPY(), cycles: 4 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "TYA", operate: |nes| nes.TYA(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "STA",  operate: |nes| nes.STA(),  addrmode: |nes| nes.ABY(), cycles: 5 },Instruction{ name: "TXS",  operate: |nes| nes.TXS(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "STA",  operate: |nes| nes.STA(), addrmode: |nes| nes.ABX(), cycles: 5 },Instruction{ name: "???",  operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },
            Instruction{ name: "LDY", operate: |nes| nes.LDY(), addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "LDA", operate: |nes| nes.LDA(),  addrmode: |nes| nes.IZX(), cycles: 6 },Instruction{ name: "LDX", operate: |nes| nes.LDX(), addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "LDY", operate: |nes| nes.LDY(),   addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "LDA", operate: |nes| nes.LDA(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "LDX", operate: |nes| nes.LDX(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 3 },Instruction{ name: "TAY", operate: |nes| nes.TAY(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "LDA",  operate: |nes| nes.LDA(),  addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "TAX",  operate: |nes| nes.TAX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "LDY",  operate: |nes| nes.LDY(),   addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "LDA",  operate: |nes| nes.LDA(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "LDX",  operate: |nes| nes.LDX(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 4 },
            Instruction{ name: "BCS", operate: |nes| nes.BCS(), addrmode: |nes| nes.REL(), cycles: 2 },Instruction{ name: "LDA", operate: |nes| nes.LDA(),  addrmode: |nes| nes.IZY(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "LDY", operate: |nes| nes.LDY(),   addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "LDA", operate: |nes| nes.LDA(),  addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "LDX", operate: |nes| nes.LDX(),  addrmode: |nes| nes.ZPY(), cycles: 4 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "CLV", operate: |nes| nes.CLV(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "LDA",  operate: |nes| nes.LDA(),  addrmode: |nes| nes.ABY(), cycles: 4 },Instruction{ name: "TSX",  operate: |nes| nes.TSX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "LDY",  operate: |nes| nes.LDY(),   addrmode: |nes| nes.ABX(), cycles: 4 },Instruction{ name: "LDA",  operate: |nes| nes.LDA(), addrmode: |nes| nes.ABX(), cycles: 4 },Instruction{ name: "LDX",  operate: |nes| nes.LDX(), addrmode: |nes| nes.ABY(), cycles: 4 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 4 },
            Instruction{ name: "CPY", operate: |nes| nes.CPY(), addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "CMP", operate: |nes| nes.CMP(),  addrmode: |nes| nes.IZX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.NOP(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "CPY", operate: |nes| nes.CPY(),   addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "CMP", operate: |nes| nes.CMP(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "DEC", operate: |nes| nes.DEC(),  addrmode: |nes| nes.ZP0(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "INY", operate: |nes| nes.INY(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "CMP",  operate: |nes| nes.CMP(),  addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "DEX",  operate: |nes| nes.DEX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "CPY",  operate: |nes| nes.CPY(),   addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "CMP",  operate: |nes| nes.CMP(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "DEC",  operate: |nes| nes.DEC(), addrmode: |nes| nes.ABS(), cycles: 6 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },
            Instruction{ name: "BNE", operate: |nes| nes.BNE(), addrmode: |nes| nes.REL(), cycles: 2 },Instruction{ name: "CMP", operate: |nes| nes.CMP(),  addrmode: |nes| nes.IZY(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "CMP", operate: |nes| nes.CMP(),  addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "DEC", operate: |nes| nes.DEC(),  addrmode: |nes| nes.ZPX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "CLD", operate: |nes| nes.CLD(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "CMP",  operate: |nes| nes.CMP(),  addrmode: |nes| nes.ABY(), cycles: 4 },Instruction{ name: "NOP",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "CMP",  operate: |nes| nes.CMP(), addrmode: |nes| nes.ABX(), cycles: 4 },Instruction{ name: "DEC",  operate: |nes| nes.DEC(), addrmode: |nes| nes.ABX(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },
            Instruction{ name: "CPX", operate: |nes| nes.CPX(), addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "SBC", operate: |nes| nes.SBC(),  addrmode: |nes| nes.IZX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.NOP(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "CPX", operate: |nes| nes.CPX(),   addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "SBC", operate: |nes| nes.SBC(),  addrmode: |nes| nes.ZP0(), cycles: 3 },Instruction{ name: "INC", operate: |nes| nes.INC(),  addrmode: |nes| nes.ZP0(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 5 },Instruction{ name: "INX", operate: |nes| nes.INX(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "SBC",  operate: |nes| nes.SBC(),  addrmode: |nes| nes.IMM(), cycles: 2 },Instruction{ name: "NOP",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.SBC(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "CPX",  operate: |nes| nes.CPX(),   addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "SBC",  operate: |nes| nes.SBC(), addrmode: |nes| nes.ABS(), cycles: 4 },Instruction{ name: "INC",  operate: |nes| nes.INC(), addrmode: |nes| nes.ABS(), cycles: 6 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },
            Instruction{ name: "BEQ", operate: |nes| nes.BEQ(), addrmode: |nes| nes.REL(), cycles: 2 },Instruction{ name: "SBC", operate: |nes| nes.SBC(),  addrmode: |nes| nes.IZY(), cycles: 5 },Instruction{ name: "???", operate: |nes| nes.XXX(), addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 8 },Instruction{ name: "???", operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "SBC", operate: |nes| nes.SBC(),  addrmode: |nes| nes.ZPX(), cycles: 4 },Instruction{ name: "INC", operate: |nes| nes.INC(),  addrmode: |nes| nes.ZPX(), cycles: 6 },Instruction{ name: "???", operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 6 },Instruction{ name: "SED", operate: |nes| nes.SED(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "SBC",  operate: |nes| nes.SBC(),  addrmode: |nes| nes.ABY(), cycles: 4 },Instruction{ name: "NOP",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 2 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.NOP(),   addrmode: |nes| nes.IMP(), cycles: 4 },Instruction{ name: "SBC",  operate: |nes| nes.SBC(), addrmode: |nes| nes.ABX(), cycles: 4 },Instruction{ name: "INC",  operate: |nes| nes.INC(), addrmode: |nes| nes.ABX(), cycles: 7 },Instruction{ name: "???",  operate: |nes| nes.XXX(),   addrmode: |nes| nes.IMP(), cycles: 7 },
        ];

        nes
    }

    /// Perform one clock cycle's worth of update
    fn clock(&mut self) {
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
            self.opcode = self.read(self.pc);
            // Increment program counter, we read the part we needed (the opcode byte)
            self.pc += 1;

            // get number of cycles needed for the instruction
            self.cycles = self.lookup[self.opcode as usize].cycles;

            // Perform a fetch of the intermmediate data using the required addressing mode
            let additional_cycle1 = (self.lookup[self.opcode as usize].addrmode)(self);
            // Perform the operation
            let additional_cycle2 = (self.lookup[self.opcode as usize].operate)(self);

            // The addressing mode and opcode may have altered the number of cycles
            // required for this instruction to complete
            self.cycles += additional_cycle1 & additional_cycle2;
        }
        // everytime we call the clock function one cycle has elapsed
        // so we decrement the number of cycles remaining for this instruction
        self.cycles -= 1;
    }

    // The functions below represent external event functions, In the NES hardware,
    // these represent pins that are asserted to produce a change in state.

    /// Reset Interrupt - forces the CPU into a known state
    fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00 | Flags6502::U as u8;

        // this address contains the address we want to set our program counter to
        self.addr_abs = 0xFFFC;
        // get lo byte of the address
        let lo = self.read(self.addr_abs + 0) as u16;
        // get hi byte of the address
        let hi = self.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        self.cycles = 8;
    }
    /// Interrupt Request - Executes an instruction at a specific location
    fn irq(&mut self) {
        // if interrupts are enabled
        if self.get_flag(Flags6502::I) == 0 {
            // push the program counter to the stack
            self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.stkp -= 1;
            self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
            self.stkp -= 1;

            self.set_flag(Flags6502::B, false);
            self.set_flag(Flags6502::U, true);
            self.set_flag(Flags6502::I, true);
            // write the status to the stack
            self.write(0x0100 + self.stkp as u16, self.status);
            self.stkp -= 1;

            // get the value of the new program counter; forces the program to jump to a 
            // known location set by the programmer to handle the interupt.
            self.addr_abs = 0xFFFE;
            let lo = self.read(self.addr_abs + 0) as u16;
            let hi = self.read(self.addr_abs + 1) as u16;
            self.pc = (hi << 8) | lo;

            self.cycles = 7;
        }
    }
    /// Non-Maskable Interrupt Request - Same as irq but cannot be disabled
    fn nmi(&mut self) {
        // push the program counter to the stack
        self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, true);
        self.set_flag(Flags6502::I, true);
        // write the status to the stack
        self.write(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;

        // get the value of the new program counter; forces the program to jump to a 
        // known location set by the programmer to handle the interupt.
        self.addr_abs = 0xFFFA;
        let lo = self.read(self.addr_abs + 0) as u16;
        let hi = self.read(self.addr_abs + 1) as u16;
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
        if !((self.lookup[self.opcode as usize].addrmode)(self) == self.IMP()) {
            // set fetched to the contents of the address
            self.fetched = self.read(self.addr_abs);
        }
        // returned the fetched data
        self.fetched
    }

    // Linkage to the communication bus
    fn read(&self, addr: u16) -> u8 {
        self.bus.read(addr)
    }
    fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    // Convenience functions to access status register
    /// Returns the value of a specific bit of the status register
    fn get_flag(&self, f: Flags6502) -> u8 {
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
    fn disassemble(&mut self, n_start: &u16, n_stop: &u16) -> HashMap<u16, String> {
        let mut addr= *n_start as u32;
        let mut value: u8 = 0x00;
        let mut lo: u8 = 0x00;
        let mut hi: u8 = 0x00;
        let mut map_lines: HashMap<u16, String> = HashMap::new();
        let mut line_addr: u16 = 0;

        while addr <= *n_stop as u32 {
            line_addr = addr as u16;

            // prefix line with instruction address
            let mut s_inst = format!("${:X}: ", addr);

            // read instruction and get its readable name
            let opcode = self.read(addr as u16);
            addr += 1;
            s_inst = format!("{}{} ", s_inst, self.lookup[opcode as usize].name); 

            if (self.lookup[opcode as usize].addrmode)(self) == self.IMP() {
                s_inst = format!("{} {{IMP}}", s_inst);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.IMM() {
                value = self.read(addr as u16);
                addr += 1;
                s_inst = format!("{}#${:X} {{IMM}}", s_inst, value);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.ZP0() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}${:X} {{ZP0}}", s_inst, lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.ZPX() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}${:X}, X {{ZPX}}", s_inst, lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.ZPY() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}${:X}, Y {{ZPY}}", s_inst, lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.IZX() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}(${:X}, X) {{IZX}}", s_inst, lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.IZY() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = 0x00;
                s_inst = format!("{}(${:X}), Y {{IZY}}", s_inst, lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.ABS() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                s_inst = format!("{}${:X} {{ABS}}", s_inst, (hi << 8) | lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.ABX() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                s_inst = format!("{}${:X}, X {{ABS}}", s_inst, (hi << 8) | lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.ABY() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                s_inst = format!("{}${:X}, Y {{ABS}}", s_inst, (hi << 8) | lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.IND() {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                s_inst = format!("{}(${:X}) {{IND}}", s_inst, (hi << 8) | lo);
            }
            else if (self.lookup[opcode as usize].addrmode)(self) == self.REL() {
                value = self.read(addr as u16);
                addr += 1;
                s_inst = format!("{}${:X} [${:X}] {{REL}}", s_inst, value, addr + value as u32);
            }

            map_lines.insert(line_addr, s_inst);
        }

        map_lines
    }
}