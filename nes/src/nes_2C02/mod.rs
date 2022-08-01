use std::{rc::Rc};

use crate::{bus::{CpuBusDevice, PpuBusDevice}, cartridge::Cartridge};

pub struct Nes2C02 {
    /// VRAM
    table_name: [[u8; 1024]; 2],
    /// RAM
    table_palette: [u8; 32],

    pub cart: Option<Rc<Cartridge>>,
}

impl Default for Nes2C02 {
    fn default() -> Self {
        Self { 
            table_name: [[0; 1024]; 2],
            table_palette: [0; 32],
            cart: None,
        }
    }
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

        // Mask address incase the ppu ever tries to 
        // write to its bus past its addressable range
        addr &= 0x3FFF;

        data
    }

    fn ppu_write(&mut self, mut addr: u16, data: u8) {
        // Mask address incase the ppu ever tries to 
        // write to its bus past its addressable range
        addr &= 0x3FFF;
    }
}

impl Nes2C02 {
    pub fn connect_cartridge(&mut self, cartridge: Rc<Cartridge>) {
        self.cart = Some(cartridge);
    }

    fn clock(&mut self) {

    }
}