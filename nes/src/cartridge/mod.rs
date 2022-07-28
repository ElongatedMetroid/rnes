use crate::bus::{PpuBusDevice, CpuBusDevice};

struct Cartridge {

}

impl CpuBusDevice for Cartridge {
    fn cpu_read(&self, addr: u16, read_only: bool) -> u8 {
        todo!()
    }

    fn cpu_write(&mut self, addr: u16, data: u8) {
        todo!()
    }
}

impl PpuBusDevice for Cartridge {
    fn ppu_read(&mut self, addr: u16, read_only: bool) -> u8 {
        todo!()
    }

    fn ppu_write(&mut self, addr: u16, data: u8) {
        todo!()
    }
}