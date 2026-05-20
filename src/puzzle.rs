use crate::shape::{Piece, Point};
use dlx_rs::Solver;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Puzzle {
    pub width: i32,
    pub height: i32,
    pub pieces: Vec<Piece>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    pub piece_index: usize,
    pub piece: Piece,
    pub dx: i32,
    pub dy: i32,
}

impl Puzzle {
    pub fn new(width: i32, height: i32, pieces: Vec<Piece>) -> Self {
        Self { width, height, pieces }
    }

    pub fn load_puzzle(set: usize) -> Self {
        match set {
            1 => Self::new(3, 2, vec![
                Piece::new(vec![(0, 0), (0, 1), (1, 0)]),
                Piece::new(vec![(0, 0), (1, 0)]),
                Piece::new(vec![(0, 0)]),
            ]),
            2 => Self::new(8, 5, vec![
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
            ]),
            3 | _ => Self::new(10, 6, vec![
                Piece::new(vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 0)]),
                Piece::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
                Piece::new(vec![(0, 0), (1, 0), (2, 0), (0, 1), (2, 1)]),
                Piece::new(vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)]),
                Piece::new(vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
                Piece::new(vec![(0, 0), (1, 0), (2, 0), (3, 0), (1, 1)]),
                Piece::new(vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)]),
                Piece::new(vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]),
                Piece::new(vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 1)]),
                Piece::new(vec![(0, 0), (1, 0), (1, 1), (2, 1), (2, 2)]),
                Piece::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (3, 1)]),
                Piece::new(vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)]),
            ]),
        }
    }

    fn cell_to_col(&self, x: i32, y: i32) -> usize {
        let cell_idx = (y * self.width + x) as usize;
        self.pieces.len() + 1 + cell_idx
    }

    pub fn solve(&self, pre_placed: &[Placement]) -> Option<Vec<Placement>> {
        let num_pieces = self.pieces.len();
        let num_cells = (self.width * self.height) as usize;
        let total_cols = num_pieces + num_cells;

        let mut solver: Solver<Placement> = Solver::new(total_cols);

        for (piece_idx, piece) in self.pieces.iter().enumerate() {
            // Check if this piece is pre-placed
            if let Some(pre) = pre_placed.iter().find(|p| p.piece_index == piece_idx) {
                let mut columns = vec![piece_idx + 1];
                let mut valid = true;
                for p in &pre.piece.coords {
                    let cx = p.x + pre.dx;
                    let cy = p.y + pre.dy;
                    if cx < 0 || cx >= self.width || cy < 0 || cy >= self.height {
                        valid = false;
                        break;
                    }
                    columns.push(self.cell_to_col(cx, cy));
                }
                // Pre-placed must be valid, otherwise unsolvable
                if valid {
                    solver.add_option(pre.clone(), &columns);
                }
                continue;
            }

            // Not pre-placed, generate all valid options
            let variations = piece.generate_variations();
            for var in variations {
                let max_x = var.coords.iter().map(|p| p.x).max().unwrap_or(0);
                let max_y = var.coords.iter().map(|p| p.y).max().unwrap_or(0);

                for dx in 0..=(self.width - 1 - max_x) {
                    for dy in 0..=(self.height - 1 - max_y) {
                        let mut columns = vec![piece_idx + 1];
                        let mut valid = true;
                        
                        for p in &var.coords {
                            let cx = p.x + dx;
                            let cy = p.y + dy;
                            if cx < 0 || cx >= self.width || cy < 0 || cy >= self.height {
                                valid = false;
                                break;
                            }
                            columns.push(self.cell_to_col(cx, cy));
                        }

                        if valid {
                            let placement = Placement {
                                piece_index: piece_idx,
                                piece: var.clone(),
                                dx,
                                dy,
                            };
                            solver.add_option(placement, &columns);
                        }
                    }
                }
            }
        }
        solver.solve()
    }
}
