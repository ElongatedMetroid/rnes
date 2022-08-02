use std::{fs::File, io::{BufReader, Read}, rc::Rc, cell::RefCell};

use crate::{bus::{PpuBusDevice, CpuBusDevice}, Mapper, Mapper000};

pub struct Cartridge<T: Mapper> {
    prg_memory: Vec<u8>,
    chr_memory: Vec<u8>,

    /// Which mapper are we using
    mapper_id: u8,
    /// How many prg banks there are
    prg_banks: u8,
    /// How many chr banks there are
    chr_banks: u8,

    mapper: Option<Rc<RefCell<T>>>

}

impl<T> Cartridge<T> {
    pub fn new() -> Cartridge<T> {
        Cartridge {
            prg_memory: Vec::new(),
            chr_memory: Vec::new(),
            mapper_id: 0,
            prg_banks: 0,
            chr_banks: 0,
        
            mapper: None,
        }
    }

    fn initialize(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        /// iNES format header
        #[derive(Debug)]
        struct Header {
            name: [char; 4],
            prg_rom_chunks: u8,
            chr_rom_chunks: u8,
            mapper1: u8,
            mapper2: u8,
            prg_ram_size: u8,
            tv_system1: u8,
            tv_system2: u8,
        }

        let mut file = File::open(file_name)?;
        // Stores the raw iNes file data
        let mut ines_data: Vec<u8> = Vec::new();
        // Read the iNes file into the vector
        file.read_to_end(&mut ines_data).unwrap();
        // The file is no longer needed so we drop it early
        drop(file);

        let mut name: [char; 4] = ['a'; 4];
        // Extract name from ines_data
        for (i, byte) in ines_data[0..4].bytes().enumerate() {
            name[i] = byte.unwrap() as char;
        }

        // Extract bytes from ines_data
        let prg_rom_chunks: u8 = ines_data[4];
        let chr_rom_chunks: u8 = ines_data[5];
        let mapper1: u8 = ines_data[6];
        let mapper2: u8 = ines_data[7];
        let prg_ram_size: u8 = ines_data[8];
        let tv_system1: u8 = ines_data[9];
        let tv_system2: u8 = ines_data[10];

        // Store into a header structure
        let header = Header {
            name,
            prg_rom_chunks,
            chr_rom_chunks,
            mapper1,
            mapper2,
            prg_ram_size,
            tv_system1,
            tv_system2,
        };

        println!("{:#?}", header);

        let mut ines_data_byte_offset = 17;
        if (header.mapper1 & 0x04) != 0 {
            ines_data_byte_offset = 513;
        }

        // Determine Mapper ID
        self.mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);

        // "Discover" file format
        let file_type: u8 = 1;

        if file_type == 0 {

        }

        if file_type == 1 {
            // Read in how many banks of memory are in the ROM for the program memory
            self.prg_banks = header.prg_rom_chunks;
            // Resize program memory vector to that size
            // A single bank of program memory is 16KB
            self.prg_memory.resize(self.prg_banks as usize * 16384, 0);
            // Read the data from the file into the vector
            self.prg_memory = ines_data[ines_data_byte_offset..=self.prg_memory.len()].to_vec();

            // Repeat for character memory

            self.chr_banks = header.chr_rom_chunks;
            self.chr_memory.resize(self.chr_banks as usize * 8192, 0);
            self.chr_memory = ines_data[self.prg_memory.len() + 1..=self.chr_memory.len()].to_vec();
        }

        if file_type == 2 {

        }

        // Load correct mapper
        match self.mapper_id {
            0 => self.mapper = Some(Rc::new(RefCell::new(Mapper000::new(self.prg_banks, self.chr_banks)))),
            _ => (),
        }


        Ok(())
    }

    pub fn handle_cpu_read(&self, addr: u16, read_only: bool) -> bool {
        false
    }
    pub fn handle_cpu_write(&mut self, addr: u16, data: u8) -> bool {
        false
    }
    pub fn handle_ppu_read(&mut self, addr: u16, read_only: bool) -> bool {
        false
    }
    pub fn handle_ppu_write(&mut self, addr: u16, data: u8) -> bool {
        false
    }
}