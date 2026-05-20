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

impl eframe::App for BrainBlockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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

            // 繪製一個簡單的棋盤佔位
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
        });
    }
    
    // egui 0.34 需要實作 clear_color
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}
