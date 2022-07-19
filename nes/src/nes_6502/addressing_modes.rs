use super::Nes6502;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AddressMode {
    Imp,
    Zp0,
    Zpx,
    Zpy,
    Abs,
    Abx,
    Aby,
    Imm,
    Ind,
    Izx,
    Izy,
    Rel,
}

impl AddressMode {
    pub fn execute(mode: AddressMode, nes: &mut Nes6502) -> u8 {
        match mode {
            AddressMode::Imp => nes.imp(),
            AddressMode::Zp0 => nes.zp0(),
            AddressMode::Zpx => nes.zpx(),
            AddressMode::Zpy => nes.zpy(),
            AddressMode::Abs => nes.abs(),
            AddressMode::Abx => nes.abx(),
            AddressMode::Aby => nes.aby(),
            AddressMode::Imm => nes.imm(),
            AddressMode::Ind => nes.ind(),
            AddressMode::Izx => nes.izx(),
            AddressMode::Izy => nes.izy(),
            AddressMode::Rel => nes.rel(),
        }
    }
}

impl Nes6502 {
    // Addressing modes, the 6502 has many addressing modes which are used
    // to acess data in memory, some are direct and some are indirect. Each
    // opcode contains information on which addressing mode should be used
    // to execute the instruction, in regards where it reads and or writes
    // the data it uses. The addressing mode changes the number of bytes
    // that makes up the instruction, so we handle addressing before executing
    // the instruction (so we can insure the program counter is at the correct
    // position). The instruction is primed with the addresses it needs,
    // and the number of clock cycles the instruction needs to be executed.
    // These functions may ajust the number of cycles required depending on
    // where the memory is accessed.

    // All of the address modes will set the addr_abs variable so the instruction
    // knows where to read the data from when it needs to.

    /// No data as part of the instruction
    /// It could be operating upon the accumulator though
    pub(super) fn imp(&mut self) -> u8 {
        self.fetched = self.a;
        0
    }
    /// Zero page addressing: the byte of data we are interesting in reading
    /// for this instruction can be located somewhere in page zero of memory
    /// zero page is where the high byte is zero (ex. 0x00FF)
    pub(super) fn zp0(&mut self) -> u8 {
        // Read the address from the program counter 
        self.addr_abs = self.bus.cpu_read(self.pc, false) as u16;

        // increment the program counter
        self.pc += 1;

        // set addr_abs to the lower byte of itself 
        self.addr_abs &= 0x00FF;

        0
    }
    /// Zero page addressing with x register offset
    pub(super) fn zpx(&mut self) -> u8 {
        self.addr_abs = (self.bus.cpu_read(self.pc, false) + self.x) as u16;

        self.pc += 1;
        self.addr_abs &= 0x00FF;

        0
    }
    /// Zero page addressing with y register offset
    pub(super) fn zpy(&mut self) -> u8 {
        self.addr_abs = (self.bus.cpu_read(self.pc, false) + self.y) as u16;

        self.pc += 1;
        self.addr_abs &= 0x00FF;

        0
    }
    /// Absolute addressing a full 16-bit address is loaded and used
    /// The instruction for this has to be 3 bytes long to store
    /// (1) the opcode, (2) the lo byte of the absolute address, and
    /// (3) the hi byte of the absolute address.
    pub(super) fn abs(&mut self) -> u8 {
        // Get lo byte of the instruction
        let lo = self.bus.cpu_read(self.pc, false) as u16;

        // increment pc so we can get the hi byte
        self.pc += 1;
        
        let hi = self.bus.cpu_read(self.pc, false) as u16;
        
        self.pc += 1;

        // combine the lo and hi byte
        self.addr_abs = (hi << 8) | lo;

        0
    }
    /// Absolute addressing with x register offset
    pub(super) fn abx(&mut self) -> u8 {
        // Get lo byte of the instruction
        let lo = self.bus.cpu_read(self.pc, false) as u16;

        // increment pc so we can get the hi byte
        self.pc += 1;
        
        let hi = self.bus.cpu_read(self.pc, false) as u16;

        self.pc += 1;

        // combine the lo and hi byte
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;
        // if after incrementing with the x register the whole address has
        // changed to a different page, we need to indicate to the system
        // that we need an additional clock cycle.
        // We can check this by seeing if the high byte has changed after
        // adding x to it. Because if it has changed it changed due to 
        // overflow.
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }
    /// Absolute addressing with y register offset
    pub(super) fn aby(&mut self) -> u8 {
        // Get lo byte of the instruction
        let lo = self.bus.cpu_read(self.pc, false) as u16;

        // increment pc so we can get the hi byte
        self.pc += 1;
        
        // Get the high byte of the instruction
        let hi = self.bus.cpu_read(self.pc, false) as u16;
        
        // move pc to next instruction
        self.pc += 1;

        // combine the lo and hi byte
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {  
            1
        } else {
            0
        }
    }
    /// Immediate mode addressing means the data is immediatly supplied
    /// as part of the instruction; its going to be the next byte
    pub(super) fn imm(&mut self) -> u8 {
        // set addr_abs to self.pc because the data is the next byte
        // of the instruction (pc is already set to the next byte), 
        // so the instruction knows to read the data from there
        self.addr_abs = self.pc;
        self.pc += 1;
        0
    }
    /// Indirect, the 16-bit address is read to get the actual 16-bit address.
    /// This addressing mode is weird because it has a bug in the hardware. To
    /// emulate this acurratly, we need to also emulate this bug. If the low
    /// byte of the supplied address is 0xFF, then to read the high byte of the
    /// actual address we need to cross a page boundary. This doesnt actually
    /// work on the chip as designed, instead it wraps back around in the same
    /// page, yeilding an invailid actual address
    pub(super) fn ind(&mut self) -> u8 {
        // get lo byte of the pointer
        let ptr_lo = self.bus.cpu_read(self.pc, false) as u16;
        // increment program counter to get hi byte of the pointer
        self.pc += 1;
        // get hi byte of the pointer
        let ptr_hi = self.bus.cpu_read(self.pc, false) as u16;
        // move the program counter to the next instruction
        self.pc += 1;

        // combine lo and hi
        let ptr = (ptr_hi << 8) | ptr_lo;

        // Simulate hardware bug
        if ptr_lo == 0x00FF {
            self.addr_abs = ((self.bus.cpu_read(ptr & 0xFF00, false) as u16) << 8) | self.bus.cpu_read(ptr, false) as u16;
        } else { // Behave normally
            // Read the address the pointer contains
            self.addr_abs = ((self.bus.cpu_read(ptr + 1, false) as u16) << 8) | self.bus.cpu_read(ptr, false) as u16;
        }
        
        0
    }
    /// Indirect addressing of zero page with x offset (the 16-bit address is stored in 0 page)
    /// The supplied 8-bit address is offset by X Register to index
    /// a location in page 0x00. The actual 16-bit address is read 
    /// from this location
    pub(super) fn izx(&mut self) -> u8 {
        // The supplied address located in zero page references somewhere in memory
        let t = self.bus.cpu_read(self.pc, false) as u16;
        // increment program counter to position it at next instruction
        self.pc += 1;

        // Read the 16-bit address from zero page
        // read the data (because the address contains another address) of the lo byte of address + the x register
        let lo = self.bus.cpu_read(((t + self.x as u16) as u16) & 0x00FF, false) as u16;
        // read the data the hi byte of the address + the x register
        let hi = self.bus.cpu_read(((t + (self.x + 1) as u16) as u16) & 0x00FF, false) as u16;
        
        // combine lo and hi
        self.addr_abs = (hi << 8) | lo;

        0
    }
    /// Indirect addressing of zero page with y offset
    /// This is different from Indirect addressing with x offset;
    /// if the offset causes a change in page then an additional 
    /// clock cycle if required
    pub(super) fn izy(&mut self) -> u8 {
        // The supplied address located in zero page references somewhere in memory
        let t = self.bus.cpu_read(self.pc, false) as u16;
        // increment program counter to position it at next instruction
        self.pc += 1;

        // Read the 16-bit address from zero page
        // read the data (because the address contains another address) of the lo byte of address + the y register
        let lo = self.bus.cpu_read((t as u16) & 0x00FF, false) as u16;
        // read the data the hi byte of the address + the y register
        let hi = self.bus.cpu_read((t + 1) & 0x00FF, false) as u16;
        
        // combine lo and hi
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        // check if the page has changed from the y offset
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0   
        }
    }
    /// Relative addressing, this mode is exclusive to branch
    /// instructions, the address must reside within -128 to 
    /// +127 of the branch instruction, i.e. you cant directly
    /// branch to any address in the addressable range
    pub(super) fn rel(&mut self) -> u8 {
        // Read the address contained in the program counter
        self.addr_rel = self.bus.cpu_read(self.pc, false) as u16;
        // move program counter to next insturction
        self.pc += 1;
        // Check if the address is signed
        if (self.addr_rel & 0x80) != 0 {
            // if it is signed, set the high byte of the address
            // to all ones
            self.addr_rel |= 0xFF00;
        }

        0
    }
}