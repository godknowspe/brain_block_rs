#![allow(deprecated)]
use eframe::egui;
use crate::puzzle::{Puzzle, Placement};
use crate::shape::{Piece, Point};

const PIECE_COLORS: &[egui::Color32] = &[
    egui::Color32::from_rgb(255, 107, 107), // #FF6B6B
    egui::Color32::from_rgb(78, 205, 196),  // #4ECDC4
    egui::Color32::from_rgb(69, 183, 209),  // #45B7D1
    egui::Color32::from_rgb(255, 160, 122), // #FFA07A
    egui::Color32::from_rgb(152, 216, 200), // #98D8C8
    egui::Color32::from_rgb(247, 220, 111), // #F7DC6F
    egui::Color32::from_rgb(187, 143, 206), // #BB8FCE
    egui::Color32::from_rgb(133, 193, 226), // #85C1E2
    egui::Color32::from_rgb(248, 183, 57),  // #F8B739
    egui::Color32::from_rgb(82, 183, 136),  // #52B788
    egui::Color32::from_rgb(230, 57, 70),   // #E63946
    egui::Color32::from_rgb(6, 255, 165),   // #06FFA5
];

pub struct BrainBlockApp {
    puzzle: Puzzle,
    board_width: i32,
    board_height: i32,
    grid: Vec<Vec<Option<usize>>>, // 儲存 (x, y) 上的 piece_index
}

impl Default for BrainBlockApp {
    fn default() -> Self {
        // 載入 Set 2 (8x5) 
        let width = 8;
        let height = 5;
        let pieces = vec![
            Piece::new(vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
            Piece::new(vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
            Piece::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            Piece::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            Piece::new(vec![(0, 0), (0, 1), (0, 2), (1, 0)]),
            Piece::new(vec![(0, 0), (0, 1), (0, 2), (1, 0)]),
            Piece::new(vec![(0, 0), (1, 0), (1, 1), (2, 0)]),
            Piece::new(vec![(0, 0), (1, 0), (1, 1), (2, 0)]),
            Piece::new(vec![(0, 0), (1, 0), (1, 1), (2, 1)]),
            Piece::new(vec![(0, 0), (1, 0), (1, 1), (2, 1)]),
        ];
        
        let puzzle = Puzzle::new(width, height, pieces);
        let grid = vec![vec![None; width as usize]; height as usize];
        
        Self {
            puzzle,
            board_width: width,
            board_height: height,
            grid,
        }
    }
}

impl BrainBlockApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for BrainBlockApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Brain Block Puzzle Solver");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Solve").clicked() {
                    if let Some(solution) = self.puzzle.solve() {
                        // 清空並更新 Grid
                        self.grid = vec![vec![None; self.board_width as usize]; self.board_height as usize];
                        for placement in solution {
                            for p in &placement.piece.coords {
                                let cx = (p.x + placement.dx) as usize;
                                let cy = (p.y + placement.dy) as usize;
                                self.grid[cy][cx] = Some(placement.piece_index);
                            }
                        }
                    } else {
                        println!("No solution found!");
                    }
                }
                if ui.button("Reset").clicked() {
                    self.grid = vec![vec![None; self.board_width as usize]; self.board_height as usize];
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
                    
                    let color = match self.grid[y as usize][x as usize] {
                        Some(idx) => PIECE_COLORS[idx % PIECE_COLORS.len()],
                        None => egui::Color32::DARK_GRAY,
                    };

                    painter.rect_filled(cell_rect, 0.0, color);
                    painter.rect_stroke(cell_rect, 0.0, (1.0, egui::Color32::BLACK), egui::StrokeKind::Inside);
                }
            }
        });
    }
}
