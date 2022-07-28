use crate::bus::{CpuBusDevice, PpuBusDevice};

pub struct Nes2C02 {

}

impl CpuBusDevice for Nes2C02 {
    fn cpu_read(&self, addr: u16, read_only: bool) -> u8 {
        let data: u8 = 0x00;

        match addr {
            // Control
            0x0000 => {
                
            }
            // Mask
            0x0001 => {
                
            }
            // Status
            0x0002 => {
                
            }
            // OAM Address
            0x0003 => {
                
            }
            // OAM Data
            0x0004 => {
                
            }
            // Scroll
            0x0005 => {
                
            }
            // PPU Address
            0x0006 => {
                
            }
            // PPU Data
            0x0007 => {
                
            }
            _ => {

            }
        }

        data
    }

    fn cpu_write(&mut self, addr: u16, data: u8) {
        let data: u8 = 0x00;

        match addr {
            // Control
            0x0000 => {
                
            }
            // Mask
            0x0001 => {
                
            }
            // Status
            0x0002 => {
                
            }
            // OAM Address
            0x0003 => {
                
            }
            // OAM Data
            0x0004 => {
                
            }
            // Scroll
            0x0005 => {
                
            }
            // PPU Address
            0x0006 => {
                
            }
            // PPU Data
            0x0007 => {
                
            }
            _ => {

            }
        }
    }
}

impl PpuBusDevice for Nes2C02 {
    fn ppu_read(&mut self, mut addr: u16, read_only: bool) -> u8 {
        let data: u8 = 0x00;

        addr &= 0x3FFF;

        data
    }

    fn ppu_write(&mut self, mut addr: u16, data: u8) {
        addr &= 0x3FFF;
    }
}