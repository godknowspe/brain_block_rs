use crate::shape::Piece;
use dlx_rs::Solver;

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

    pub fn solve(&self, pre_placed: &[Placement], max_solutions: usize) -> Vec<Vec<Placement>> {
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
        let mut results = Vec::new();
        while let Some(sol) = solver.solve() {
            results.push(sol);
            if results.len() >= max_solutions {
                break;
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_solutions(puzzle: &Puzzle, sols: &[Vec<Placement>]) {
        for sol in sols {
            assert_eq!(sol.len(), puzzle.pieces.len(), "Each solution must use all pieces");
            let mut piece_used = vec![false; puzzle.pieces.len()];
            let mut cell_covered = vec![vec![false; puzzle.width as usize]; puzzle.height as usize];
            for p in sol {
                assert!(!piece_used[p.piece_index], "Each piece must be used exactly once");
                piece_used[p.piece_index] = true;
                for pt in &p.piece.coords {
                    let cx = (pt.x + p.dx) as usize;
                    let cy = (pt.y + p.dy) as usize;
                    assert!(cx < puzzle.width as usize, "Cell x out of bounds");
                    assert!(cy < puzzle.height as usize, "Cell y out of bounds");
                    assert!(!cell_covered[cy][cx], "Each cell must be covered exactly once");
                    cell_covered[cy][cx] = true;
                }
            }
            // Check that all cells are covered
            for y in 0..puzzle.height as usize {
                for x in 0..puzzle.width as usize {
                    assert!(cell_covered[y][x], "Cell ({}, {}) must be covered", x, y);
                }
            }
        }
    }

    #[test]
    fn test_solve_set_1() {
        let puzzle = Puzzle::load_puzzle(1);
        let sols = puzzle.solve(&[], 10);
        assert!(!sols.is_empty(), "Set 1 should have solutions");
        verify_solutions(&puzzle, &sols);
        println!("Set 1 solutions: {}", sols.len());
    }

    #[test]
    fn test_solve_set_2() {
        let puzzle = Puzzle::load_puzzle(2);
        let sols = puzzle.solve(&[], 10);
        assert!(!sols.is_empty(), "Set 2 should have solutions");
        verify_solutions(&puzzle, &sols);
        println!("Set 2 solutions: {}", sols.len());
    }

    #[test]
    fn test_solve_set_3() {
        let puzzle = Puzzle::load_puzzle(3);
        let sols = puzzle.solve(&[], 1);
        assert!(!sols.is_empty(), "Set 3 should have solutions");
        verify_solutions(&puzzle, &sols);
        println!("Set 3 solutions: {}", sols.len());
    }
}
