mod truck;
mod game;
mod resource;
mod building;
mod enemy;

use eframe::egui;
use game::GameApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Factory Tank Miner",
        options,
        Box::new(|_cc| Ok(Box::new(GameApp::default()))),
    )
}
