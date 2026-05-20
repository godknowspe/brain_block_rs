mod shape;
mod puzzle;
mod gui;

use shape::Piece;
use puzzle::Puzzle;

fn main() -> eframe::Result<()> {
    println!("Starting Brain Block GUI...");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Brain Block"),
        ..Default::default()
    };

    eframe::run_native(
        "Brain Block",
        options,
        Box::new(|cc| Ok(Box::new(gui::BrainBlockApp::new(cc)))),
    )
}
