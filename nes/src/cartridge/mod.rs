use std::{fs::File, io::{BufReader, Read}};

use crate::bus::{PpuBusDevice, CpuBusDevice};

pub struct Cartridge {
    prg_memory: Vec<u8>,
    chr_memory: Vec<u8>,

    /// Which mapper are we using
    mapper_id: u8,
    /// How many prg banks there are
    prg_banks: u8,
    /// How many chr banks there are
    chr_banks: u8,

}

// The cartridge has access to both the ppu's bus and
// the main bus.

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

impl Cartridge {
    pub fn new(file_name: &str) -> Result<Cartridge, std::io::Error> {
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

        let file = File::open(file_name)?;
        let mut buffer = BufReader::new(file);
        // Stores the raw iNes header data
        let mut ines_data: [u8; 16] = [0; 16];
        // Read the iNes header into the array
        buffer.read_exact(&mut ines_data).unwrap();

        let mut name: [char; 4] = ['a'; 4];
        for (i, byte) in ines_data[0..4].bytes().enumerate() {
            name[i] = byte.unwrap() as char;
        }

        let prg_rom_chunks: u8 = ines_data[4];
        let chr_rom_chunks: u8 = ines_data[5];
        let mapper1: u8 = ines_data[6];
        let mapper2: u8 = ines_data[7];
        let prg_ram_size: u8 = ines_data[8];
        let tv_system1: u8 = ines_data[9];
        let tv_system2: u8 = ines_data[10];

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

        todo!()
    }
}