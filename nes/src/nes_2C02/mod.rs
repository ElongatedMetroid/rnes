pub struct Nes2C02 {

}

impl Nes2C02 {
    // Communications with main bus

    /// Read from main bus
    pub fn cpu_read(&mut self, addr: u16, read_only: bool) -> u8 {
        todo!()
    }
    /// Write to main bus
    pub fn cpu_write(&mut self, addr: u16, data: u8) {

    }

    // Communications with PPU bus

    /// Read from ppu bus
    fn ppu_read(&mut self, addr: u16, read_only: bool) -> u8 {
        todo!()
    }
    /// Write to ppu bus
    fn ppu_write(&mut self, addr: u16, data: u8) {
        
    }
}