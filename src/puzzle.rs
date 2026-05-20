use crate::shape::{Piece, Point};
use dlx_rs::Solver;
use std::collections::HashSet;

pub struct Puzzle {
    width: i32,
    height: i32,
    pieces: Vec<Piece>,
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
        Self {
            width,
            height,
            pieces,
        }
    }

    /// Converts board (x, y) coordinates to a DLX column index for the cell.
    /// DLX columns are 1-based in our usage:
    /// Pieces: 1 to num_pieces
    /// Cells: num_pieces + 1 to num_pieces + width * height
    fn cell_to_col(&self, x: i32, y: i32) -> usize {
        let cell_idx = (y * self.width + x) as usize;
        self.pieces.len() + 1 + cell_idx
    }

    pub fn solve(&self) -> Option<Vec<Placement>> {
        let num_pieces = self.pieces.len();
        let num_cells = (self.width * self.height) as usize;
        let total_cols = num_pieces + num_cells;

        let mut solver: Solver<Placement> = Solver::new(total_cols);

        // Generate all valid placements for all pieces
        for (piece_idx, piece) in self.pieces.iter().enumerate() {
            let variations = piece.generate_variations();

            for var in variations {
                // Try to place this variation at every possible (dx, dy)
                let max_x = var.coords.iter().map(|p| p.x).max().unwrap_or(0);
                let max_y = var.coords.iter().map(|p| p.y).max().unwrap_or(0);

                for dx in 0..=(self.width - 1 - max_x) {
                    for dy in 0..=(self.height - 1 - max_y) {
                        let mut columns = Vec::new();
                        // 1. Piece constraint: this piece must be used exactly once
                        columns.push(piece_idx + 1);

                        // 2. Cell constraints: the cells covered by this placement
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

        // Return the first solution
        solver.solve()
    }
}
