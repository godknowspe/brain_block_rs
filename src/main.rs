mod shape;
mod puzzle;
mod gui;

fn main() -> eframe::Result<()> {
    println!("Starting Brain Block GUI...");

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([850.0, 700.0])
            .with_title("Brain Block"),
        ..Default::default()
    };

    eframe::run_native(
        "Brain Block",
        options,
        Box::new(|cc| Ok(Box::new(gui::BrainBlockApp::new(cc)))),
    )
}
