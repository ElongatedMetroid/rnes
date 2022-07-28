use std::{ops::Range, rc::Rc, cell::RefCell};

use crate::Nes2C02;

/// Contains bus devices
pub struct Bus {
    pub cpu_ram: [u8; 2048],
    pub ppu: Nes2C02,
}

impl Default for Bus {
    fn default() -> Self {
        Self { 
            cpu_ram: [0; 2048],
            ppu: Nes2C02 {},
        }
    }
}

impl Bus {
    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        if (0x0000..=0x1FFF).contains(&addr) {    
            self.cpu_ram[(addr & 0x07FF) as usize] = data;
        } else if (0x2000..=0x3FFF).contains(&addr) {
            self.ppu.cpu_write(addr & 0x0007, data);
        }
    }

    pub fn cpu_read(&self, addr: u16, _read_only: bool) -> u8 {
        let mut data: u8 = 0x00;
        
        if (0x0000..=0x1FFF).contains(&addr) {
            data = self.cpu_ram[(addr & 0x07FF) as usize];
        }

        data
    }
}