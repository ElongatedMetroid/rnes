//! The bus will represent the core of the emulation;
//! the bus will represent the NES itself.

use crate::Nes2C02;

pub trait CpuBusDevice {
    // Communications with main bus

    /// Read from main bus
    fn cpu_read(&self, addr: u16, read_only: bool) -> u8;
    /// Write to main bus
    fn cpu_write(&mut self, addr: u16, data: u8);
}

pub trait PpuBusDevice {
    // Communications with PPU bus

    /// Read from ppu bus
    fn ppu_read(&mut self, addr: u16, read_only: bool) -> u8;
    /// Write to ppu bus
    fn ppu_write(&mut self, addr: u16, data: u8);
}

/// Contains bus devices
pub struct Bus {
    /// Count how many times clock has been called
    system_clock_counter: u32,

    pub cpu_ram: [u8; 2048],
    pub ppu: Nes2C02,
}

impl Default for Bus {
    fn default() -> Self {
        let bus = Self { 
            system_clock_counter: 0,

            cpu_ram: [0; 2048],
            ppu: Nes2C02 {},
        };

        bus
    }
}

impl Bus {
    // Bus read & write

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
        } else if (0x2000..=0x3FFF).contains(&addr) {
            data = self.ppu.cpu_read(addr & 0x0007, _read_only);
        }

        data
    }
}