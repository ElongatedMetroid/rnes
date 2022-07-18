extern crate nes;

mod app;

const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1080.0;

fn main() {
    let app = app::App::default();

    let native_options = eframe::NativeOptions {
        always_on_top: false,
        decorated: true,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_size: Some((SCREEN_WIDTH, SCREEN_HEIGHT).into()),
        resizable: false,
        transparent: false,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native("rnes", native_options, Box::new( |_cc| Box::new(app)));
}