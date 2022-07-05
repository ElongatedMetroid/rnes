use crate::{bus::Bus, nes_6502::Nes6502};

mod bus;
mod nes_6502;

fn main() {
    let cpu = Nes6502::new();

    println!("Hello, world!");
}
