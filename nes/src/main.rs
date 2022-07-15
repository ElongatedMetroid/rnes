extern crate olc_pixel_game_engine;

use std::{collections::HashMap, process};

use nes_6502::{Nes6502, Flags6502};

use crate::olc_pixel_game_engine as olc;

mod bus;
mod nes_6502;

struct Test6502 {
    nes: Nes6502,
    map_asm: Vec<String>,
}

impl Test6502 {
    fn draw_ram(&self, x: i32, y: i32, mut n_addr: u16, n_rows: i32, n_columns: i32) {
        let n_ram_x = x;
        let mut n_ram_y = y;

        for row in 0..n_rows {
            let mut s_offset = format!("${:04X}:", n_addr);
            for col in 0..n_columns {
                s_offset = format!("{} {:02X}", s_offset, self.nes.read(n_addr));
                n_addr += 1;
            }
            olc::draw_string(n_ram_x, n_ram_y, &s_offset, olc::BLACK).unwrap();
            n_ram_y += 10;
        }
    }

    fn draw_cpu(&self, x: i32, y: i32) {
        olc::draw_string(x, y, "STATUS:", olc::WHITE).unwrap();
        olc::draw_string(x + 64, y, "N", if self.nes.get_flag(Flags6502::N) == 1 { olc::GREEN } else { olc::RED }).unwrap();
        olc::draw_string(x + 80, y, "V", if self.nes.get_flag(Flags6502::V) == 1 { olc::GREEN } else { olc::RED }).unwrap();
        olc::draw_string(x + 96, y, "-", if self.nes.get_flag(Flags6502::U) == 1 { olc::GREEN } else { olc::RED }).unwrap();
        olc::draw_string(x + 112, y, "B", if self.nes.get_flag(Flags6502::B) == 1 { olc::GREEN } else { olc::RED }).unwrap();
        olc::draw_string(x + 128, y, "D", if self.nes.get_flag(Flags6502::D) == 1 { olc::GREEN } else { olc::RED }).unwrap();
        olc::draw_string(x + 144, y, "I", if self.nes.get_flag(Flags6502::I) == 1 { olc::GREEN } else { olc::RED }).unwrap();
        olc::draw_string(x + 160, y, "Z", if self.nes.get_flag(Flags6502::Z) == 1 { olc::GREEN } else { olc::RED }).unwrap();
        olc::draw_string(x + 178, y, "C", if self.nes.get_flag(Flags6502::C) == 1 { olc::GREEN } else { olc::RED }).unwrap();
        olc::draw_string(x, y + 10, &format!("PC: ${:04X}", self.nes.pc), olc::GREEN).unwrap();
        olc::draw_string(x, y + 20, &format!("A: ${:02X} [{:03}]", self.nes.a, self.nes.a), olc::GREEN).unwrap();
        olc::draw_string(x, y + 30, &format!("X: ${:02X} [{:03}]", self.nes.x, self.nes.x), olc::GREEN).unwrap();
        olc::draw_string(x, y + 40, &format!("Y: ${:02X} [{:03}]", self.nes.y, self.nes.y), olc::GREEN).unwrap();
        olc::draw_string(x, y + 50, &format!("Stack Ptr: ${:04X}", self.nes.stkp), olc::GREEN).unwrap();
    }

    fn draw_asm(&self, x: i32, y: i32) {
        let mut i = 0;
        for line in &self.map_asm {
            olc::draw_string(x, y + i as i32, line.as_str(), olc::BLACK);
            i += 10;
        }
    }
}

impl olc::Application for Test6502 {
    fn on_user_create(&mut self) -> Result<(), olc_pixel_game_engine::Error> {
        let test_prog: Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];
        let n_offset = 0x8000;

        self.nes.bus.ram[n_offset..n_offset + test_prog.len()].copy_from_slice(&test_prog);
        
        self.nes.bus.ram[0xFFFC] = 0x00;
        self.nes.bus.ram[0xFFFD] = 0x80;

        self.map_asm = self.nes.disassemble(0x0000, 0xFFFF);

        self.nes.reset();

        Ok(())
    }
    fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc_pixel_game_engine::Error> {
        olc::clear(olc::WHITE);

        if olc::get_key(olc::Key::SPACE).pressed  {
            loop {
                self.nes.clock();
                if self.nes.complete() {
                    break;
                }
            }
        }

        if olc::get_key(olc::Key::I).pressed {
            self.nes.irq();
        }

        self.draw_ram(2, 2, 0x0000, 16, 16);
        self.draw_ram(2, 182, 0x8000, 16, 16);
        self.draw_cpu(448, 2);

        self.draw_asm(448, 72);

        Ok(())
    }
    fn on_user_destroy(&mut self) -> Result<(), olc_pixel_game_engine::Error> {
        Ok(())
    }
}

fn main() {
    let mut test = Test6502 {
        nes: Nes6502::new(),
        map_asm: Vec::new()
    };

    match olc::start("rnes", &mut test, 680, 480, 2, 2) {
        Ok(_) => (),
        Err(e) => {
            println!("An error occured while trying to start the program: {}", e);
            process::exit(1);
        }
    }
}
