extern crate nes;

use std::collections::BTreeMap;

use eframe::{
    egui::{self, Key},
};

use egui::Color32;
use nes::{Nes6502, Flags6502};

struct GuiToggles {
    mem_view: bool,
    disasm_view: bool,
    reg_view: bool,
}

pub struct App {
    nes: Nes6502,
    map_asm: BTreeMap<u16, String>,
    viewable_ram_offset: u16,
    gui_toggles: GuiToggles,

}

impl Default for App {
    fn default() -> Self {
        
        let mut app = Self { 
            nes: Nes6502::new(), 
            map_asm: BTreeMap::new(),
            viewable_ram_offset: 0x8000,
            gui_toggles: GuiToggles {
                mem_view: false,
                disasm_view: false,
                reg_view: false,
            },
        };

        if cfg!(debug_assertions) {
            app.gui_toggles.mem_view = true;
            app.gui_toggles.disasm_view = true;
            app.gui_toggles.reg_view = true;
        }

        let test_prog: Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];
        app.nes.bus.ram[0x8000..0x8000 + test_prog.len()].copy_from_slice(&test_prog);

        app.nes.bus.ram[0xFFFC] = 0x00;
        app.nes.bus.ram[0xFFFD] = 0x80;

        app.map_asm = app.nes.disassemble(0x4000..=0x8FFF);

        app.nes.reset();

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        if ctx.input().key_pressed(Key::Space) {
            loop {
                self.nes.clock();
                if self.nes.complete() {
                    break;
                }
            }
        }

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("View Memory").clicked() {
                        self.gui_toggles.mem_view = true;
                    }
                    if ui.button("View Dissassemly").clicked() {
                        self.gui_toggles.disasm_view = true;
                    }
                    if ui.button("View Registers").clicked() {
                        self.gui_toggles.reg_view = true;
                    }
                });
            });

        egui::Window::new("Game")
            .collapsible(true)
            .resizable(false)
            .default_size([256.0 * 2.0, 240.0 * 2.0])
            .show(ctx, |ui| {
                ui.label("af");
            });

        if self.gui_toggles.mem_view == true {
            if ctx.input().key_pressed(Key::V) {
                self.viewable_ram_offset = if self.viewable_ram_offset == 0x0000 {
                    0x8000
                } else {
                    0x0000
                };
            }

            use std::fmt::Write;
            let mut mem = Vec::new();
            let mut addr = self.viewable_ram_offset;
            for _ in 0..16 {
                let mut str = format!("{:04X}:", addr);
    
                for _ in 0..16 {
                    write!(&mut str, " {:02X}", self.nes.bus.read(addr)).unwrap();
                    addr += 1;
                }
    
                mem.push(str);
            }    

            egui::Window::new("Memory View") 
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            self.gui_toggles.mem_view = false;
                        }

                        ui.label("Press V to Change the Viewable Memory Range");
                    });

                    ui.separator();

                    for line in mem.into_iter() {
                        ui.label(line);
                    }
                });
        }
        if self.gui_toggles.disasm_view == true {
            egui::Window::new("Disassembly View")
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            self.gui_toggles.disasm_view = false;
                        }
                    });

                    self.map_asm
                        .range(self.nes.pc.checked_sub(8).unwrap_or(0)..)
                        .take(10)
                        .for_each(|(addr, str)| {
                            ui.colored_label(if *addr == self.nes.pc {Color32::GREEN} else {Color32::WHITE}, str);
                        })
                });
        }
        if self.gui_toggles.reg_view == true {
            egui::Window::new("Register View")
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            self.gui_toggles.reg_view = false;
                        }
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.colored_label(if self.nes.get_flag(Flags6502::N) == 1 {Color32::GREEN} else { Color32:: RED }, "N");
                        ui.colored_label(if self.nes.get_flag(Flags6502::V) == 1 {Color32::GREEN} else { Color32:: RED }, "V");
                        ui.colored_label(if self.nes.get_flag(Flags6502::U) == 1 {Color32::GREEN} else { Color32:: RED }, "-");
                        ui.colored_label(if self.nes.get_flag(Flags6502::B) == 1 {Color32::GREEN} else { Color32:: RED }, "B");
                        ui.colored_label(if self.nes.get_flag(Flags6502::D) == 1 {Color32::GREEN} else { Color32:: RED }, "D");
                        ui.colored_label(if self.nes.get_flag(Flags6502::I) == 1 {Color32::GREEN} else { Color32:: RED }, "I");
                        ui.colored_label(if self.nes.get_flag(Flags6502::Z) == 1 {Color32::GREEN} else { Color32:: RED }, "Z");
                        ui.colored_label(if self.nes.get_flag(Flags6502::C) == 1 {Color32::GREEN} else { Color32:: RED }, "C");
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label(format!("PC: ${:04X}", self.nes.pc));
                        ui.label(format!("A: ${:02X} [{:03}]", self.nes.a, self.nes.a));
                        ui.label(format!("X: ${:02X} [{:03}]", self.nes.x, self.nes.x));
                        ui.label(format!("Y: ${:02X} [{:03}]", self.nes.y, self.nes.y));
                        ui.label(format!("Stack Ptr ${:04X}", self.nes.stkp));
                    }) 
                });
        }
        
    }
}