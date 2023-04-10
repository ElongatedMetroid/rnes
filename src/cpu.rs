use crate::{registers::Registers, NegativeSet};

pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self { 
            registers: Registers::default(), 
        }
    }
    pub fn interpret(&mut self, program: &Vec<u8>) {
        self.registers.program_counter = 0;

        loop {
            let opcode = program[self.registers.program_counter as usize];
            self.registers.program_counter += 1;

            match opcode {
                0xA9 => {
                    let load = program[self.registers.program_counter as usize];
                    self.registers.program_counter += 1;
                    self.registers.a = load;

                    // when a is zero set the zero register (otherwise it will be reset)
                    self.registers.status.zero = self.registers.a == 0;

                    // if a is "negative", set the negative register (otherwise it will be reset)
                    self.registers.status.negative = self.registers.a.negative_set();

                }
                0x00 => {
                    return;
                }
                _ => todo!()
            }
        }
    }
}
