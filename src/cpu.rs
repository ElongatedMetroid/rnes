use std::process;

use crate::{registers::Registers, NegativeSet, opcodes::{OPCODES, OpcodeName, AddressingMode}};

pub struct Cpu {
    pub registers: Registers,
    memory: [u8; 0xFFFF],
}

impl Cpu {
    pub fn new() -> Self {
        Self { 
            registers: Registers::default(), 
            memory: [0; 0xFFFF],
        }
    }
    pub fn mem_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn mem_read_u16(&self, address: u16) -> u16 {
        let lo = self.mem_read(address) as u16;
        let hi = self.mem_read(address + 1) as u16;

        (hi << 8) | lo
    }
    pub fn mem_write_u16(&mut self, address: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;

        self.mem_write(address, lo);
        self.mem_write(address + 1, hi)
    }
    pub fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
    pub fn fetch_address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Implied => {
                panic!("addressing mode is implied, no data to be read");
            },
            // An address in the program rom 
            AddressingMode::Immediate => self.registers.program_counter,
            // An address in zero page
            AddressingMode::ZeroPage => self.mem_read(self.registers.program_counter) as u16,
            AddressingMode::ZeroPageX => todo!(),
            AddressingMode::ZeroPageY => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::Absolute => todo!(),
            AddressingMode::AbsoluteX => todo!(),
            AddressingMode::AbsoluteY => todo!(),
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndexedIndirectX => todo!(),
            AddressingMode::IndirectIndexedY => todo!(),
        }
    }
    pub fn load_and_run(&mut self, program: &Vec<u8>) {
        self.load(program);
        self.run();
    }
    pub fn load(&mut self, program: &Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program);
        self.registers.program_counter = 0x8000;
    }
    pub fn run(&mut self) {
        loop {
            let opcode_byte = self.mem_read(self.registers.program_counter);
            let opcode = OPCODES[opcode_byte as usize].unwrap_or_else(|| {
                eprintln!("Invalid opcode: {opcode_byte}");
                process::exit(1);
            });
            self.registers.program_counter += 1;

            match opcode.name() {
                OpcodeName::LDA => {
                    let addr = self.fetch_address(opcode.addressing_mode());
                    let load = self.mem_read(addr);
                    self.registers.program_counter += 1;
                    self.registers.a = load;

                    // when a is zero set the zero register (otherwise it will be reset)
                    self.registers.status.zero = self.registers.a == 0;

                    // if a is "negative", set the negative register (otherwise it will be reset)
                    self.registers.status.negative = self.registers.a.negative_set();

                }
                OpcodeName::BRK => {
                    return;
                }
                OpcodeName::STA => {
                    let store_at = self.fetch_address(opcode.addressing_mode());
                    self.registers.program_counter += 1;
                    self.mem_write(store_at, self.registers.a);
                }
                _ => {
                    todo!("{} ({opcode_byte:#x})", opcode.name());
                }
            }
        }
    }
}
