pub const MEM_SIZE: usize = 64 * 1024;

/// Contains bus devices
#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; MEM_SIZE],
}

impl Bus {
    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }
}