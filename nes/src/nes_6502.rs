#![allow(non_snake_case)]

use crate::bus::{Bus , MEM_SIZE};

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

    }
    /// Interrupt Request - Executes an instruction at a specific location
    fn irq(&mut self) {

    }
    /// Non-Maskable Interrupt Request - Same as irq but cannot be disabled
    fn nmi(&mut self) {

    }

    /// The read location of data can come from two sources, a memory address, or
    /// its immediately available as part of the instruction. This function decides
    /// depending on address mode of the instruction byte
    fn fetch(&self) -> u8 {
        todo!()
    }

    // Linkage to the communication bus
    fn read(&self, addr: u16) -> u8 {
        self.bus.read(addr)
    }
    fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    // Convenience functions to access status register
    fn get_flag(&self, f: Flags6502) {

    }
    fn set_flag(&self, f: Flags6502, b: bool) {

    }    

    // Addressing modes, the 6502 has many addressing modes which are used
    // to acess data in memory, some are direct and some are indirect. Each
    // opcode contains information on which addressing mode should be used
    // to execute the instruction, in regards where it reads and or writes
    // the data it uses. The addressing mode changes the number of bytes
    // that makes up the instruction, so we handle addressing before executing
    // the instruction (so we can insure the program counter is at the correct
    // position). The instruction is primed with the addresses it needs,
    // and the number of clock cycles the instruction needs to be executed.
    // These functions may ajust the number of cycles required depending on
    // where the memory is accessed.

    // All of the address modes will set the addr_abs variable so the instruction
    // knows where to read the data from when it needs to.

    /// No data as part of the instruction
    /// It could be operating upon the accumulator though
    fn IMP(&mut self) -> u8 {
        self.fetched = self.a;
        0
    }
    /// Zero page addressing: the byte of data we are interesting in reading
    /// for this instruction can be located somewhere in page zero of memory
    /// zero page is where the high byte is zero (ex. 0x00FF)
    fn ZP0(&mut self) -> u8 {
        // Read the address from the program counter 
        self.addr_abs = self.read(self.pc) as u16;

        // increment the program counter
        self.pc += 1;

        // set addr_abs to the lower byte of itself 
        self.addr_abs &= 0x00FF;

        0
    }
    /// Zero page addressing with x register offset
    fn ZPX(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc + self.x as u16) as u16;

        self.pc += 1;
        self.addr_abs &= 0x00FF;

        0
    }
    /// Zero page addressing with y register offset
    fn ZPY(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc + self.y as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;

        0
    }
    /// Absolute addressing a full 16-bit address is loaded and used
    /// The instruction for this has to be 3 bytes long to store
    /// (1) the opcode, (2) the lo byte of the absolute address, and
    /// (3) the hi byte of the absolute address.
    fn ABS(&mut self) -> u8 {
        // Get lo byte of the instruction
        let lo = self.read(self.pc) as u16;

        // increment pc so we can get the hi byte
        self.pc += 1;
        
        let hi = self.read(self.pc) as u16;
        
        self.pc += 1;

        // combine the lo and hi byte
        self.addr_abs = (hi << 8) | lo;

        0
    }
    /// Absolute addressing with x register offset
    fn ABX(&mut self) -> u8 {
        // Get lo byte of the instruction
        let lo = self.read(self.pc) as u16;

        // increment pc so we can get the hi byte
        self.pc += 1;
        
        let hi = self.read(self.pc) as u16;

        self.pc += 1;

        // combine the lo and hi byte
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;
        // if after incrementing with the x register the whole address has
        // changed to a different page, we need to indicate to the system
        // that we need an additional clock cycle.
        // We can check this by seeing if the high byte has changed after
        // adding x to it. Because if it has changed it changed due to 
        // overflow.
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        } else {
            0
        }
    }
    /// Absolute addressing with y register offset
    fn ABY(&mut self) -> u8 {
        // Get lo byte of the instruction
        let lo = self.read(self.pc) as u16;

        // increment pc so we can get the hi byte
        self.pc += 1;
        
        // Get the high byte of the instruction
        let hi = self.read(self.pc) as u16;
        
        // move pc to next instruction
        self.pc += 1;

        // combine the lo and hi byte
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {  
            return 1;
        } else {
            0
        }
    }
    /// Immediate mode addressing means the data is immediatly supplied
    /// as part of the instruction; its going to be the next byte
    fn IMM(&mut self) -> u8 {
        // set addr_abs to self.pc because the data is the next byte
        // of the instruction (pc is already set to the next byte), 
        // so the instruction knows to read the data from there
        self.addr_abs = self.pc;
        self.pc += 1;
        0
    }
    /// Indirect, the 16-bit address is read to get the actual 16-bit address.
    /// This addressing mode is weird because it has a bug in the hardware. To
    /// emulate this acurratly, we need to also emulate this bug. If the low
    /// byte of the supplied address is 0xFF, then to read the high byte of the
    /// actual address we need to cross a page boundary. This doesnt actually
    /// work on the chip as designed, instead it wraps back around in the same
    /// page, yeilding an invailid actual address
    fn IND(&mut self) -> u8 {
        // get lo byte of the pointer
        let ptr_lo = self.read(self.pc) as u16;
        // increment program counter to get hi byte of the pointer
        self.pc += 1;
        // get hi byte of the pointer
        let ptr_hi = self.read(self.pc) as u16;
        // move the program counter to the next instruction
        self.pc += 1;

        // combine lo and hi
        let ptr = (ptr_hi << 8) | ptr_lo;

        // Simulate hardware bug
        if ptr_lo == 0x00FF {
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr + 0) as u16;
        } else { // Behave normally
            // Read the address the pointer contains
            self.addr_abs = ((self.read(ptr + 1) as u16) << 8) | self.read(ptr + 0) as u16;
        }
        
        0
    }
    /// Indirect addressing of zero page with x offset (the 16-bit address is stored in 0 page)
    /// The supplied 8-bit address is offset by X Register to index
    /// a location in page 0x00. The actual 16-bit address is read 
    /// from this location
    fn IZX(&mut self) -> u8 {
        // The supplied address located in zero page references somewhere in memory
        let t = self.read(self.pc) as u16;
        // increment program counter to position it at next instruction
        self.pc += 1;

        // Read the 16-bit address from zero page
        // read the data (because the address contains another address) of the lo byte of address + the x register
        let lo = self.read(((t + self.x as u16) as u16) & 0x00FF) as u16;
        // read the data the hi byte of the address + the x register
        let hi = self.read(((t + (self.x + 1) as u16) as u16) & 0x00FF) as u16;
        
        // combine lo and hi
        self.addr_abs = (hi << 8) | lo;

        0
    }
    /// Indirect addressing of zero page with y offset
    /// This is different from Indirect addressing with x offset;
    /// if the offset causes a change in page then an additional 
    /// clock cycle if required
    fn IZY(&mut self) -> u8 {
        // The supplied address located in zero page references somewhere in memory
        let t = self.read(self.pc) as u16;
        // increment program counter to position it at next instruction
        self.pc += 1;

        // Read the 16-bit address from zero page
        // read the data (because the address contains another address) of the lo byte of address + the y register
        let lo = self.read((t as u16) & 0x00FF) as u16;
        // read the data the hi byte of the address + the y register
        let hi = self.read((t + 1 as u16) & 0x00FF) as u16;
        
        // combine lo and hi
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        // check if the page has changed from the y offset
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        } else {
            0   
        }
    }
    /// Relative addressing, this mode is exclusive to branch
    /// instructions, the address must reside within -128 to 
    /// +127 of the branch instruction, i.e. you cant directly
    /// branch to any address in the addressable range
    fn REL(&mut self) -> u8 {
        // Read the address contained in the program counter
        self.addr_rel = self.read(self.pc) as u16;
        // move program counter to next insturction
        self.pc += 1;
        // Check if the address is signed
        if self.addr_rel & 0x80 == 1 {
            // if it is signed, set the high byte of the address
            // to all ones
            self.addr_rel |= 0xFF00;
        }

        0
    }

    // Opcodes
    // The 6502 has 56 legal opcodes, illegal opcodes are not modeled
    // in this emulator. Most of these opcodes return 0 but some are
    // capable of requiring more cycles when executing under certain
    // conditions combined with certain addressing modes.

    /// add with carry
    fn ADC(&self) -> u8 {
        todo!()
    }
    /// and (with accumulator)
    fn AND(&self) -> u8 {
        todo!()
    }
    /// arithmetic shift left
    fn ASL(&self) -> u8 {
        todo!()
    }
    /// branch on carry clear
    fn BCC(&self) -> u8 {
        todo!()
    }
    /// branch on carry set
    fn BCS(&self) -> u8 {
        todo!()
    }
    /// branch on equal (zero set)
    fn BEQ(&self) -> u8 {
        todo!()
    }
    /// bit test
    fn BIT(&self) -> u8 {
        todo!()
    }
    /// branch on minus (negative set)
    fn BMI(&self) -> u8 {
        todo!()
    }
    /// branch on not equal (zero clear)
    fn BNE(&self) -> u8 {
        todo!()
    }
    /// branch on plus (negative clear)
    fn BPL(&self) -> u8 {
        todo!()
    }
    /// break / interrupt
    fn BRK(&self) -> u8 {
        todo!()
    }
    /// branch on overflow clear
    fn BVC(&self) -> u8 {
        todo!()
    }
    /// branch on overflow set
    fn BVS(&self) -> u8 {
        todo!()
    }
    /// clear carry
    fn CLC(&self) -> u8 {
        todo!()
    }
    /// clear decimal
    fn CLD(&self) -> u8 {
        todo!()
    }
    /// clear interrupt disable
    fn CLI(&self) -> u8 {
        todo!()
    }
    /// clear overflow
    fn CLV(&self) -> u8 {
        todo!()
    }
    /// compare (with accumulator)
    fn CMP(&self) -> u8 {
        todo!()
    }
    /// compare with X
    fn CPX(&self) -> u8 {
        todo!()
    }
    /// compare with Y
    fn CPY(&self) -> u8 {
        todo!()
    }
    // decrement
    fn DEC(&self) -> u8 {
        todo!()
    }
    /// decrement X
    fn DEX(&self) -> u8 {
        todo!()
    }
    /// decrement Y
    fn DEY(&self) -> u8 {
        todo!()
    }
    /// exclusive or (with accumulator)
    fn EOR(&self) -> u8 {
        todo!()
    }
    /// increment
    fn INC(&self) -> u8 {
        todo!()
    }
    /// increment X
    fn INX(&self) -> u8 {
        todo!()
    }
    /// increment Y
    fn INY(&self) -> u8 {
        todo!()
    }
    /// jump
    fn JMP(&self) -> u8 {
        todo!()
    }
    /// jump subroutine
    fn JSR(&self) -> u8 {
        todo!()
    }
    /// load accumulator
    fn LDA(&self) -> u8 {
        todo!()
    }
    /// load X
    fn LDX(&self) -> u8 {
        todo!()
    }
    /// load Y
    fn LDY(&self) -> u8 {
        todo!()
    }
    /// logical shift right
    fn LSR(&self) -> u8 {
        todo!()
    }
    /// no operation
    fn NOP(&self) -> u8 {
        todo!()
    }
    /// or with accumulator
    fn ORA(&self) -> u8 {
        todo!()
    }
    /// push accumulator
    fn PHA(&self) -> u8 {
        todo!()
    }
    /// push processor status (SR)
    fn PHP(&self) -> u8 {
        todo!()
    }
    /// pull accumulator
    fn PLA(&self) -> u8{
        todo!()
    }
    /// pull processor status (SR)
    fn PLP(&self) -> u8 {
        todo!()
    }
    /// rotate left
    fn ROL(&self) -> u8 {
        todo!()
    }
    /// rotate right
    fn ROR(&self) -> u8 {
        todo!()
    }
    /// return from interrupt
    fn RTI(&self) -> u8 {
        todo!()
    }
    /// return from subroutine
    fn RTS(&self) -> u8 {
        todo!()
    }
    /// subtract with carry
    fn SBC(&self) -> u8 {
        todo!()
    }
    /// set carry
    fn SEC(&self) -> u8 {
        todo!()
    }
    /// set decimal
    fn SED(&self) -> u8 {
        todo!()
    }
    /// set interrupt disable
    fn SEI(&self) -> u8 {
        todo!()
    }
    /// store accumulator
    fn STA(&self) -> u8 {
        todo!()
    }
    /// store X
    fn STX(&self) -> u8 {
        todo!()
    }
    /// store Y
    fn STY(&self) -> u8 {
        todo!()
    }
    /// transfer accumulator to X
    fn TAX(&self) -> u8 {
        todo!()
    }
    /// transfer accumulator to Y
    fn TAY(&self) -> u8 {
        todo!()
    }
    /// transfer stack pointer to X
    fn TSX(&self) -> u8 {
        todo!()
    }
    /// transfer X to accumulator
    fn TXA(&self) -> u8 {
        todo!()
    }
    /// transfer X to stack pointer
    fn TXS(&self) -> u8 {
        todo!()
    }
    /// transfer Y to accumulator 
    fn TYA(&self) -> u8 {
        todo!()
    }
    fn XXX(&self) -> u8 {
        todo!()
    }
}