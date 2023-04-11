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
            println!("{:x}", self.registers.program_counter);
            let opcode = OPCODES[opcode_byte as usize].unwrap_or_else(|| {
                eprintln!("Invalid opcode: {opcode_byte}");
                process::exit(1);
            });
            self.registers.program_counter += 1;

            match opcode.name() {
                OpcodeName::ADC => todo!(),
                OpcodeName::AND => todo!(),
                OpcodeName::ASL => todo!(),
                OpcodeName::BCC => todo!(),
                OpcodeName::BCS => todo!(),
                OpcodeName::BEQ => todo!(),
                OpcodeName::BIT => todo!(),
                OpcodeName::BMI => todo!(),
                OpcodeName::BNE => todo!(),
                OpcodeName::BPL => todo!(),
                OpcodeName::BRK => return,
                OpcodeName::BVC => todo!(),
                OpcodeName::BVS => todo!(),
                OpcodeName::CLC => todo!(),
                OpcodeName::CLD => todo!(),
                OpcodeName::CLI => todo!(),
                OpcodeName::CLV => todo!(),
                OpcodeName::CMP => todo!(),
                OpcodeName::CPX => todo!(),
                OpcodeName::CPY => todo!(),
                OpcodeName::DEC => todo!(),
                OpcodeName::DEX => todo!(),
                OpcodeName::DEY => todo!(),
                OpcodeName::EOR => todo!(),
                OpcodeName::INC => todo!(),
                OpcodeName::INX => todo!(),
                OpcodeName::INY => todo!(),
                OpcodeName::JMP => todo!(),
                OpcodeName::JSR => todo!(),
                OpcodeName::LDA => self.lda(opcode.addressing_mode()),
                OpcodeName::LDX => todo!(),
                OpcodeName::LDY => todo!(),
                OpcodeName::LSR => todo!(),
                OpcodeName::NOP => todo!(),
                OpcodeName::ORA => todo!(),
                OpcodeName::PHA => todo!(),
                OpcodeName::PHP => todo!(),
                OpcodeName::PLA => todo!(),
                OpcodeName::PLP => todo!(),
                OpcodeName::ROL => todo!(),
                OpcodeName::ROR => todo!(),
                OpcodeName::RTI => todo!(),
                OpcodeName::RTS => todo!(),
                OpcodeName::SBC => todo!(),
                OpcodeName::SEC => todo!(),
                OpcodeName::SED => todo!(),
                OpcodeName::SEI => todo!(),
                OpcodeName::STA => self.sta(opcode.addressing_mode()),
                OpcodeName::STX => todo!(),
                OpcodeName::STY => todo!(),
                OpcodeName::TAX => todo!(),
                OpcodeName::TAY => todo!(),
                OpcodeName::TSX => todo!(),
                OpcodeName::TXA => todo!(),
                OpcodeName::TXS => todo!(),
                OpcodeName::TYA => todo!(),
            }
        }
    }

    fn adc(&mut self, addressing_mode: AddressingMode) {

    }
    fn and(&mut self, addressing_mode: AddressingMode) {

    }
    fn asl(&mut self, addressing_mode: AddressingMode) {

    }
    fn bcc(&mut self, addressing_mode: AddressingMode) {

    }
    fn bcs(&mut self, addressing_mode: AddressingMode) {

    }
    fn beq(&mut self, addressing_mode: AddressingMode) {

    }
    fn bit(&mut self, addressing_mode: AddressingMode) {

    }
    fn bmi(&mut self, addressing_mode: AddressingMode) {

    }
    fn bne(&mut self, addressing_mode: AddressingMode) {

    }
    fn bpl(&mut self, addressing_mode: AddressingMode) {

    }
    fn brk(&mut self) {
        return;
    }
    fn bvc(&mut self, addressing_mode: AddressingMode) {

    }
    fn bvs(&mut self, addressing_mode: AddressingMode) {

    }
    fn clc(&mut self, addressing_mode: AddressingMode) {

    }
    fn cld(&mut self, addressing_mode: AddressingMode) {

    }
    fn cli(&mut self, addressing_mode: AddressingMode) {

    }
    fn clv(&mut self, addressing_mode: AddressingMode) {

    }
    fn cmp(&mut self, addressing_mode: AddressingMode) {

    }
    fn cpx(&mut self, addressing_mode: AddressingMode) {

    }
    fn cpy(&mut self, addressing_mode: AddressingMode) {

    }
    fn dec(&mut self, addressing_mode: AddressingMode) {

    }
    fn dex(&mut self, addressing_mode: AddressingMode) {

    }
    fn dey(&mut self, addressing_mode: AddressingMode) {

    }
    fn eor(&mut self, addressing_mode: AddressingMode) {

    }
    fn inc(&mut self, addressing_mode: AddressingMode) {

    }
    fn inx(&mut self, addressing_mode: AddressingMode) {

    }
    fn iny(&mut self, addressing_mode: AddressingMode) {

    }
    fn jmp(&mut self, addressing_mode: AddressingMode) {

    }
    fn jsr(&mut self, addressing_mode: AddressingMode) {

    }
    fn lda(&mut self, addressing_mode: AddressingMode) {
        let addr = self.fetch_address(addressing_mode);
        let load = self.mem_read(addr);
        self.registers.program_counter += 1;
        self.registers.a = load;

        // when a is zero set the zero register (otherwise it will be reset)
        self.registers.status.zero = self.registers.a == 0;

        // if a is "negative", set the negative register (otherwise it will be reset)
        self.registers.status.negative = self.registers.a.negative_set();
    }
    fn ldx(&mut self, addressing_mode: AddressingMode) {

    }
    fn ldy(&mut self, addressing_mode: AddressingMode) {

    }
    fn lsr(&mut self, addressing_mode: AddressingMode) {

    }
    fn nop(&mut self, addressing_mode: AddressingMode) {

    }
    fn ora(&mut self, addressing_mode: AddressingMode) {

    }
    fn pha(&mut self, addressing_mode: AddressingMode) {

    }
    fn php(&mut self, addressing_mode: AddressingMode) {

    }
    fn pla(&mut self, addressing_mode: AddressingMode) {

    }
    fn plp(&mut self, addressing_mode: AddressingMode) {

    }
    fn rol(&mut self, addressing_mode: AddressingMode) {

    }
    fn ror(&mut self, addressing_mode: AddressingMode) {

    }
    fn rti(&mut self, addressing_mode: AddressingMode) {

    }
    fn rts(&mut self, addressing_mode: AddressingMode) {

    }
    fn sbc(&mut self, addressing_mode: AddressingMode) {

    }
    fn sec(&mut self, addressing_mode: AddressingMode) {

    }
    fn sed(&mut self, addressing_mode: AddressingMode) {

    }
    fn sei(&mut self, addressing_mode: AddressingMode) {

    }
    fn sta(&mut self, addressing_mode: AddressingMode) {
        let store_at = self.fetch_address(addressing_mode);
        self.registers.program_counter += 1;
        self.mem_write(store_at, self.registers.a);
    }
    fn stx(&mut self, addressing_mode: AddressingMode) {

    }
    fn sty(&mut self, addressing_mode: AddressingMode) {

    }
    fn tax(&mut self, addressing_mode: AddressingMode) {

    }
    fn tay(&mut self, addressing_mode: AddressingMode) {

    }
    fn tsx(&mut self, addressing_mode: AddressingMode) {

    }
    fn txa(&mut self, addressing_mode: AddressingMode) {

    }
    fn txs(&mut self, addressing_mode: AddressingMode) {

    }
    fn tya(&mut self, addressing_mode: AddressingMode) {

    }

}
