#![allow(deprecated)]
use eframe::egui;
use crate::puzzle::{Puzzle, Placement};
use crate::shape::Piece;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};

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
    current_set: usize,
    puzzle: Puzzle,
    board_width: i32,
    board_height: i32,
    grid: Vec<Vec<Option<usize>>>,
    pre_placed: Vec<Placement>,
    solutions: Vec<Vec<Placement>>,
    solution_idx: usize,
    held_piece: Option<(usize, Piece)>,
    solving: bool,
    solution_receiver: Option<Receiver<(Vec<Vec<Placement>>, Duration)>>,
    solve_time: Option<Duration>,
}

impl Default for BrainBlockApp {
    fn default() -> Self {
        let set = 2;
        let puzzle = Puzzle::load_puzzle(set);
        let width = puzzle.width;
        let height = puzzle.height;
        let grid = vec![vec![None; width as usize]; height as usize];
        
        Self {
            current_set: set,
            puzzle,
            board_width: width,
            board_height: height,
            grid,
            pre_placed: Vec::new(),
            solutions: Vec::new(),
            solution_idx: 0,
            held_piece: None,
            solving: false,
            solution_receiver: None,
            solve_time: None,
        }
    }
}

impl BrainBlockApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn change_set(&mut self, set: usize) {
        self.current_set = set;
        self.puzzle = Puzzle::load_puzzle(set);
        self.board_width = self.puzzle.width;
        self.board_height = self.puzzle.height;
        self.pre_placed.clear();
        self.solutions.clear();
        self.solution_idx = 0;
        self.held_piece = None;
        self.solving = false;
        self.solution_receiver = None;
        self.solve_time = None;
        self.update_grid();
    }

    fn update_grid(&mut self) {
        self.grid = vec![vec![None; self.board_width as usize]; self.board_height as usize];
        let placements = if !self.solutions.is_empty() { &self.solutions[self.solution_idx] } else { &self.pre_placed };
        for p in placements {
            for pt in &p.piece.coords {
                let cx = (pt.x + p.dx) as usize;
                let cy = (pt.y + p.dy) as usize;
                if cx < self.board_width as usize && cy < self.board_height as usize {
                    self.grid[cy][cx] = Some(p.piece_index);
                }
            }
        }
    }
}

impl eframe::App for BrainBlockApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Handle incoming background solve results
        if let Some(rx) = &self.solution_receiver {
            if let Ok((sols, duration)) = rx.try_recv() {
                self.solving = false;
                self.solve_time = Some(duration);
                self.solution_receiver = None;
                self.solutions = sols;
                self.solution_idx = 0;
                self.update_grid();
            }
        }

        
        if ctx.input(|i| i.key_pressed(egui::Key::R)) {
            if let Some((_, ref mut piece)) = self.held_piece {
                *piece = piece.rotate_90();
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::M)) {
            if let Some((_, ref mut piece)) = self.held_piece {
                *piece = piece.mirror_horizontal();
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.held_piece = None;
        self.solving = false;
        self.solution_receiver = None;
        self.solve_time = None;
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Brain Block Puzzle Solver");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Puzzle Set:");
                let mut new_set = self.current_set;
                ui.selectable_value(&mut new_set, 1, "Set 1 (3x2)");
                ui.selectable_value(&mut new_set, 2, "Set 2 (8x5)");
                ui.selectable_value(&mut new_set, 3, "Set 3 (10x6)");
                
                if new_set != self.current_set {
                    self.change_set(new_set);
                }
            });
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Solve").clicked() && !self.solving {
                    self.solving = true;
                    self.solve_time = None;
                    
                    let puzzle = self.puzzle.clone();
                    let pre_placed = self.pre_placed.clone();
                    let (tx, rx) = channel();
                    self.solution_receiver = Some(rx);
                    
                    let ctx_clone = ctx.clone();
                    thread::spawn(move || {
                        let start = Instant::now();
                        // 找最多 10 個解
                        let sols = puzzle.solve(&pre_placed, 10);
                        let duration = start.elapsed();
                        let _ = tx.send((sols, duration));
                        ctx_clone.request_repaint(); // Wake up UI thread
                    });
                }
                if ui.button("Reset / Clear").clicked() && !self.solving {
                    self.pre_placed.clear();
                    self.solutions.clear();
        self.solution_idx = 0;
                    self.update_grid();
                }

                if self.solving {
                    ui.spinner();
                    ui.label("Solving in background...");
                } else if let Some(t) = self.solve_time {
                    if !self.solutions.is_empty() {
                        ui.label(egui::RichText::new(format!("Found {} solution(s) in {:?}", self.solutions.len(), t)).color(egui::Color32::GREEN));
                        
                        ui.separator();
                        if ui.button("<").clicked() {
                            if self.solution_idx > 0 {
                                self.solution_idx -= 1;
                                self.update_grid();
                            }
                        }
                        ui.label(format!(" {} / {} ", self.solution_idx + 1, self.solutions.len()));
                        if ui.button(">").clicked() {
                            if self.solution_idx + 1 < self.solutions.len() {
                                self.solution_idx += 1;
                                self.update_grid();
                            }
                        }
                    } else {
                        ui.label(egui::RichText::new(format!("No solution (took {:?})", t)).color(egui::Color32::RED));
                    }
                }

            });

            ui.add_space(20.0);

            // Draw Board
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(self.board_width as f32 * 40.0, self.board_height as f32 * 40.0),
                egui::Sense::click(),
            );

            if (response.clicked() || response.secondary_clicked()) && !self.solving {
                if let Some(pos) = response.hover_pos() {
                    let cx = ((pos.x - rect.min.x) / 40.0).floor() as i32;
                    let cy = ((pos.y - rect.min.y) / 40.0).floor() as i32;

                    if response.secondary_clicked() {
                        if cx >= 0 && cx < self.board_width && cy >= 0 && cy < self.board_height {
                            if let Some(idx) = self.grid[cy as usize][cx as usize] {
                                self.pre_placed.retain(|p| p.piece_index != idx);
                                self.solutions.clear();
                                self.solution_idx = 0;
                                self.update_grid();
                            }
                        }
                    } 
                    else if response.clicked() {
                        if !self.solutions.is_empty() {
                            // If a solution is displayed, clear it to return to editing mode
                            self.solutions.clear();
                            self.solution_idx = 0;
                            self.update_grid();
                        } else if self.held_piece.is_none() {
                            // Pick up a placed piece from the board
                            if cx >= 0 && cx < self.board_width && cy >= 0 && cy < self.board_height {
                                if let Some(idx) = self.grid[cy as usize][cx as usize] {
                                    if let Some(pos_idx) = self.pre_placed.iter().position(|p| p.piece_index == idx) {
                                        let placement = self.pre_placed.remove(pos_idx);
                                        self.held_piece = Some((idx, placement.piece));
                                        self.solutions.clear();
                                        self.solution_idx = 0;
                                        self.update_grid();
                                    }
                                }
                            }
                        } else if let Some((idx, piece)) = self.held_piece.clone() {
                            // Place the held piece on the board
                            let mut valid = true;
                            for pt in &piece.coords {
                                let nx = pt.x + cx;
                                let ny = pt.y + cy;
                                if nx < 0 || nx >= self.board_width || ny < 0 || ny >= self.board_height {
                                    valid = false;
                                    break;
                                }
                                if self.grid[ny as usize][nx as usize].is_some() {
                                    valid = false;
                                    break;
                                }
                            }
                            if valid {
                                self.pre_placed.push(Placement {
                                    piece_index: idx,
                                    piece: piece.clone(),
                                    dx: cx,
                                    dy: cy,
                                });
                                self.held_piece = None;
                                self.solving = false;
                                self.solution_receiver = None;
                                self.solve_time = None;
                                self.solutions.clear();
                                self.solution_idx = 0;
                                self.update_grid();
                            }
                        }
                    }
                }
            }

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

            ui.add_space(20.0);
            ui.heading("Inventory (Click to pick up, click on board to place)");
            ui.label("Controls: [R] = Rotate 90° | [M] = Mirror | [Esc] = Cancel hold | [Right Click] = Remove placed");
            ui.add_space(10.0);

            // Inventory
            let placed_indices: std::collections::HashSet<_> = self.pre_placed.iter().map(|p| p.piece_index).collect();
            
            let mut newly_held = None;
            ui.horizontal_wrapped(|ui| {
                for (i, piece) in self.puzzle.pieces.iter().enumerate() {
                    // if part of solution, pre-placed, or currently held, skip
                    let is_held = self.held_piece.as_ref().map_or(false, |(idx, _)| *idx == i);
                    if is_held || placed_indices.contains(&i) || (!self.solutions.is_empty() && self.solutions[self.solution_idx].iter().any(|p| p.piece_index == i)) {
                        continue;
                    }
                    
                    let max_x = piece.coords.iter().map(|p| p.x).max().unwrap_or(0);
                    let max_y = piece.coords.iter().map(|p| p.y).max().unwrap_or(0);
                    
                    let (inv_rect, inv_resp) = ui.allocate_exact_size(
                        egui::vec2((max_x + 1) as f32 * 25.0, (max_y + 1) as f32 * 25.0),
                        egui::Sense::click()
                    );
                    
                    if inv_resp.clicked() {
                        newly_held = Some((i, piece.clone()));
                    }
                    
                    let inv_painter = ui.painter();
                    let color = PIECE_COLORS[i % PIECE_COLORS.len()];
                    for pt in &piece.coords {
                        let cell_rect = egui::Rect::from_min_size(
                            inv_rect.min + egui::vec2(pt.x as f32 * 25.0, pt.y as f32 * 25.0),
                            egui::vec2(25.0, 25.0),
                        );
                        inv_painter.rect_filled(cell_rect, 0.0, color);
                        inv_painter.rect_stroke(cell_rect, 0.0, (1.0, egui::Color32::BLACK), egui::StrokeKind::Inside);
                    }
                    
                    ui.add_space(15.0);
                }
            });
            
            if let Some(held) = newly_held {
                self.held_piece = Some(held);
                self.solutions.clear();
        self.solution_idx = 0; 
                self.update_grid();
            }

            // Held piece preview
            if let Some((idx, piece)) = &self.held_piece {
                if let Some(pos) = ctx.pointer_hover_pos() {
                    ctx.request_repaint();

                    let preview_painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("held")));
                    let mut color = PIECE_COLORS[*idx % PIECE_COLORS.len()];
                    color = color.gamma_multiply(0.7); 
                    
                    let is_over_board = rect.contains(pos);
                    
                    let (base_pos, cell_size) = if is_over_board {
                        let cx = ((pos.x - rect.min.x) / 40.0).floor();
                        let cy = ((pos.y - rect.min.y) / 40.0).floor();
                        (rect.min + egui::vec2(cx * 40.0, cy * 40.0), 40.0)
                    } else {
                        (pos, 25.0)
                    };
                    
                    for pt in &piece.coords {
                        let cell_rect = egui::Rect::from_min_size(
                            base_pos + egui::vec2(pt.x as f32 * cell_size, pt.y as f32 * cell_size),
                            egui::vec2(cell_size, cell_size),
                        );
                        preview_painter.rect_filled(cell_rect, 0.0, color);
                        preview_painter.rect_stroke(cell_rect, 0.0, (1.0, egui::Color32::BLACK), egui::StrokeKind::Inside);
                    }
                }
            }
        });
    }
}
