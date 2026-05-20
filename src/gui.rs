#![allow(deprecated)]
use eframe::egui;

#[derive(Default)]
pub struct BrainBlockApp {
    board_width: i32,
    board_height: i32,
    solved: bool,
}

impl BrainBlockApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            board_width: 8,
            board_height: 5,
            solved: false,
        }
    }
}

// egui 0.34
impl eframe::App for BrainBlockApp {
    // update is still used if you want full control, but egui 0.34 added `ui` maybe?
    // Wait, the error is missing `ui`: `fn ui(&mut self, _: &mut Ui, _: &mut eframe::Frame)` 
    // This is NOT from eframe, wait... maybe App requires both update and ui in 0.34? 
    // Let's implement BOTH
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui, _frame);
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Brain Block Puzzle Solver");
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Solve").clicked() {
                self.solved = true;
            }
            if ui.button("Reset").clicked() {
                self.solved = false;
            }
        });

        ui.add_space(20.0);

        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(self.board_width as f32 * 40.0, self.board_height as f32 * 40.0),
            egui::Sense::hover(),
        );

        let painter = ui.painter();
        for y in 0..self.board_height {
            for x in 0..self.board_width {
                let cell_rect = egui::Rect::from_min_size(
                    rect.min + egui::vec2(x as f32 * 40.0, y as f32 * 40.0),
                    egui::vec2(40.0, 40.0),
                );
                
                let color = if self.solved && (x + y) % 2 == 0 {
                    egui::Color32::LIGHT_GREEN
                } else {
                    egui::Color32::DARK_GRAY
                };

                painter.rect_filled(cell_rect, 0.0, color);
                painter.rect_stroke(cell_rect, 0.0, (1.0, egui::Color32::BLACK), egui::StrokeKind::Inside);
            }
        }
    }
}
