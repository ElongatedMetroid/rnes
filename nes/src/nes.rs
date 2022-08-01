//! Contains the CPU itself, and functions like reset, insert_cartridge,
//! and clock. Most methods in here require access to the CPU struct 
//! and Bus struct, and since the Bus is contained in the CPU, CPU methods
//! cannot be called from the Bus; the reason there in here

use std::rc::Rc;

use crate::{cartridge::Cartridge, Nes6502};

pub struct Nes {
    pub cpu: Nes6502,
}

impl Default for Nes {
    fn default() -> Self {
        Self { cpu: Default::default() }
    }
}

impl Nes {
    // System interface

    /// Insert a cartridge
    fn insert_cartridge(&mut self, cartridge: Rc<Cartridge>) {
        self.cpu.bus.cart = Some(Rc::clone(&cartridge));
        self.cpu.bus.ppu.connect_cartridge(Rc::clone(&cartridge));
    }
    /// Reset button pressed
    fn reset(&mut self) {
        self.cpu.reset();
        self.cpu.bus.system_clock_counter = 0;
    }
    /// Call one system tick
    fn clock() {

    }
}