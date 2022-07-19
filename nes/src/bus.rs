pub const MEM_SIZE: usize = 64 * 1024;

/// Contains bus devices
#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; MEM_SIZE],
}

impl Bus {
    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        if (0x0000..=0xFFFF).contains(&addr) {    
            self.ram[addr as usize] = data;
        }
    }

    pub fn cpu_read(&self, addr: u16, _read_only: bool) -> u8 {
        if (0x0000..=0xFFFF).contains(&addr) {
            self.ram[addr as usize]
        } else {
            0
        }
    }
}