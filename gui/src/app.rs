extern crate nes;

use std::{collections::BTreeMap, fs::File, io::Write};

use eframe::{
    egui::{self, Key},
};

use egui::{Color32, github_link_file};
use nes::{Nes6502, Flags6502};

/// NES's native output width
const NES_WIDTH: f32 = 256.0;
/// NES's native output height
const NES_HEIGHT: f32 = 240.0;

/// Holds booleans for toggling on and off gui elements
struct GuiToggles {
    mem_view: bool,
    disasm_view: bool,
    reg_view: bool,
    info_view: bool,
}

/// Holds data related the the application, including the NES,
/// map of assembly, and gui toggles
pub struct App {
    nes: Nes6502,
    map_asm: BTreeMap<u16, String>,
    viewable_ram_offset: u16,
    gui_toggles: GuiToggles,
}

// Create a default setting to initialize our app structure
impl Default for App {
    fn default() -> Self {
        let mut app = Self { 
            nes: Nes6502::default(), 
            map_asm: BTreeMap::new(),
            viewable_ram_offset: 0x8000,
            gui_toggles: GuiToggles {
                mem_view: false,
                disasm_view: false,
                reg_view: false,
                info_view: false,
            },
        };

        // If we are in debug mode have all gui_toggles on
        if cfg!(debug_assertions) {
            app.gui_toggles.mem_view = true;
            app.gui_toggles.disasm_view = true;
            app.gui_toggles.reg_view = true;
            app.gui_toggles.info_view = true;
        }

        // Load program
        let test_prog: Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];
        app.nes.bus.cpu_ram[0x8000..0x8000 + test_prog.len()].copy_from_slice(&test_prog);

        // Set vectors
        app.nes.bus.cpu_ram[0xFFFC] = 0x00;
        app.nes.bus.cpu_ram[0xFFFD] = 0x80;

        // Put the disassembled code into the map
        app.map_asm = app.nes.disassemble(0x4000..=0x8FFF);

        // Put the NES into a known state
        app.nes.reset();

        // Return the instance of the app we created
        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set visuals to dark "theme"
        ctx.set_visuals(egui::Visuals::dark());

        // Clock the CPU when space is pressed
        if ctx.input().key_pressed(Key::Space) {
            loop {
                self.nes.clock();
                if self.nes.complete() {
                    break;
                }
            }
        }

        if ctx.input().key_pressed(Key::R) {
            self.nes.reset();
        }

        if ctx.input().key_pressed(Key::I) {
            self.nes.irq();
        }

        if ctx.input().key_pressed(Key::N) {
            self.nes.nmi();
        }

        // The central panel will hold buttons to access debug options
        // it will also hold arguably the most important thing, the
        // display!
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
                    if ui.button("View Info").clicked() {
                        self.gui_toggles.info_view = true;
                    }
                });
            });

        // If the memory viewer is toggled on
        if self.gui_toggles.mem_view {
            // Check for input from the V key to change the viewable ram offset
            if ctx.input().key_pressed(Key::V) {
                self.viewable_ram_offset = if self.viewable_ram_offset == 0x0000 {
                    // program bank 1
                    0x8000
                } else {
                    // zero page
                    0x0000
                };
            }
            
            use std::fmt::Write;
            // Create a vector to store the contents of ram (at from the offset)
            let mut mem = Vec::new();
            // Set the address we will be starting at to the offset
            let mut addr = self.viewable_ram_offset;
            // loop 16 times (for each column)
            for _ in 0..16 {
                // Set the string to the first address of the row
                let mut str = format!("{:04X}:", addr);
    
                // loop 16 times (for each row)
                for _ in 0..16 {
                    // Write the value of addr onto the end of the string
                    write!(&mut str, " {:02X}", self.nes.bus.cpu_read(addr, true)).unwrap();
                    // Increment addr
                    addr += 1;
                }
    
                // Push the rows entire string onto the Vector before continuing to the next row
                mem.push(str);
            }    

            // Create memory view window
            egui::Window::new("Memory View") 
                .open(&mut self.gui_toggles.mem_view)
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Press V to Change the Viewable Memory Range");

                    ui.separator();

                    // Iterate through each row of memory and create a label
                    mem.into_iter().for_each(|s| {
                        ui.label(s);
                    });
                });
        }
        if self.gui_toggles.disasm_view {
            egui::Window::new("Disassembly View")
                .open(&mut self.gui_toggles.disasm_view)
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| {
                    if ui.button("Dump to file").clicked() {
                        let mut dump_file = match File::create("disassembly_dump.txt") {
                            Ok(v) => v,
                            Err(e) => {
                                self.nes.info.push(format!("Could not create file: {}", e));
                                return;
                            }
                        };

                        self.map_asm.iter().for_each(|(_, s)| {
                            writeln!(dump_file, "{}", s).unwrap();
                        })
                    }

                    self.map_asm
                        .range(self.nes.pc.saturating_sub(8)..)
                        .take(10)
                        .for_each(|(addr, str)| {
                            ui.colored_label(if *addr == self.nes.pc {Color32::GREEN} else {Color32::WHITE}, str);
                        })
                });
        }
        if self.gui_toggles.reg_view {
            egui::Window::new("Register View")
                .open(&mut self.gui_toggles.reg_view)
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| {
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
        if self.gui_toggles.info_view {
            egui::Window::new("Info and Log Viewer")
                .open(&mut self.gui_toggles.info_view)
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| {
                    if ui.button("Dump to file").clicked() {
                        let mut dump_file = match File::create("info_dump.txt") {
                            Ok(v) => v,
                            Err(e) => { 
                                self.nes.info.push(format!("Could not create file: {}", e));
                                return;
                            },
                        };

                        for line in &self.nes.info {
                            writeln!(dump_file, "{}", line.as_str()).unwrap();
                        }
                    }

                    ui.separator();

                    // Credit this rando for creating the project
                    ui.label("Created by Nate, GPLv3 2022-2022");
                    // Add a link to the hub
                    ui.add(github_link_file!("https://github.com/NateNoNameSOFT/rnes/blob/main/", "(source code)"));

                    ui.separator();

                    for info in &self.nes.info[self.nes.info.len().saturating_sub(10)..self.nes.info.len()] {
                        ui.label(info);
                    }
                });
        }
        
    }
}