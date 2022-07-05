use crate::nes_6502::Nes6502;

pub const MEM_SIZE: usize = 64 * 1024;

#[derive(Debug)]
pub enum BusError {
    WriteOutOfBounds(u16, u8),
    ReadOutOfBounds(u16),
}

/// Contains bus devices
#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; MEM_SIZE],
}

impl Bus {
    pub fn write(&mut self, addr: u16, data: u8) -> Result<(), BusError> {
        // if the address is greater than 0x0000 and 0xFFFF is greater than address ...
        if addr >= 0x0000 && addr <= 0xFFFF {
            // allow a write to thavaluet address
            self.ram[addr as usize] = data;
            return Ok(());
        }

        Err(BusError::WriteOutOfBounds(addr, data))
    }

    pub fn read(&self, addr: u16) -> Result<u8, BusError> {
        // if the address is greater than 0x0000 and 0xFFFF is greater than address ...
        if addr >= 0x0000 && addr <= 0xFFFF {
            // allow a read of that address
            return Ok(self.ram[addr as usize]);
        }

        Err(BusError::ReadOutOfBounds(addr))
    }
}