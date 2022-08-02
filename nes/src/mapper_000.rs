use crate::Mapper;

pub struct Mapper000 {
    prg_banks: u8,
    chr_banks: u8,
}

impl Mapper for Mapper000 {
    fn cpu_map_read(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        if (0x8000..0xFFFF).contains(&addr) {
            // The prg_banks variable contains how many 16kb chunks were 
            // loaded as program rom so if there is more than one we know we
            // are working with the 32kb rom.
            // So if it is a 32kb rom and the address is in the allowable range
            // we will mask the address to asure we are offsetting from 0
            // But if there is one bank of program rom this is a 16kb rom
            // and if it is 16kb we mirror it within the address range
            
            // if PRGROM is 16KB
	        //     CPU Address Bus          PRG ROM
	        //     0x8000 -> 0xBFFF: Map    0x0000 -> 0x3FFF
	        //     0xC000 -> 0xFFFF: Mirror 0x0000 -> 0x3FFF
	        // if PRGROM is 32KB
	        //     CPU Address Bus          PRG ROM
	        //     0x8000 -> 0xFFFF: Map    0x0000 -> 0x7FFF	
            *mapped_addr = (addr & (if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32; 
            return true;
        }
        
        false
    }

    fn cpu_map_write(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        if (0x8000..0xFFFF).contains(&addr) {
            *mapped_addr = (addr & (if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32; 
            return true;
        }
        
        false
    }

    fn ppu_map_read(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        if (0x8000..0x1FFF).contains(&addr) {
            // No bank switching
            *mapped_addr = addr as u32;
            return true;
        }
        
        false
    }

    fn ppu_map_write(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        if (0x8000..0x1FFF).contains(&addr) {
            // chr memory, ROM
            
            return true;
        }
    
        false
    }

    fn new(prg_banks: u8, chr_banks: u8) -> Self {
        Self {
            prg_banks,
            chr_banks,
        }
    }
}