extern crate nes;

use std::collections::BTreeMap;

use eframe::{
    egui::{self, Color32, FontDefinitions, FontFamily, Frame, Key, Rect, TextStyle, TextureId, Vec2},
};

use nes::Nes6502;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
struct GuiToggles {
    mem_view: bool,
}

pub struct App {
    nes: Nes6502,
    //map_asm: Option<BTreeMap<u16, String>>,
    viewable_ram_offset: u16,
    gui_toggles: GuiToggles,

}

impl Default for App {
    fn default() -> Self {
        let mut app = Self { 
            nes: Nes6502::new(), 
            viewable_ram_offset: 0x8000,
            gui_toggles: GuiToggles {
                mem_view: false,
            },
        };

        let test_prog: Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];
        app.nes.bus.ram[0x8000..0x8000 + test_prog.len()].copy_from_slice(&test_prog);

        app.nes.bus.ram[0xFFFC] = 0x00;
        app.nes.bus.ram[0xFFFD] = 0x80;

        //app.map_asm

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
                write!(&mut str, " {:02X}", self.nes.bus.read(addr));
                addr += 1;
            }

            mem.push(str);
        }

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("View Memory").clicked() {
                        self.gui_toggles.mem_view = true;
                    }
                    if ui.button("View Dissassemly").clicked() {

                    }
                    if ui.button("View Registers").clicked() {

                    }
                });
            });

        if self.gui_toggles.mem_view == true {
            egui::Window::new("Memory View") 
                .collapsible(true)
                .resizable(true)
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
        
    }
}