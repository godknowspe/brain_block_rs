mod shape;
mod puzzle;

use shape::Piece;
use puzzle::Puzzle;

fn main() {
    println!("Brain Block Puzzle Solver (Rust Edition)");

    // Test Puzzle Set 1
    // board: (3, 2)
    // pieces:
    //  [(0, 0), (0, 1), (1, 0)]
    //  [(0, 0), (1, 0)]
    //  [(0, 0)]
    let pieces = vec![
        Piece::new(vec![(0, 0), (0, 1), (1, 0)]),
        Piece::new(vec![(0, 0), (1, 0)]),
        Piece::new(vec![(0, 0)]),
    ];

    let puzzle = Puzzle::new(3, 2, pieces);
    
    println!("Solving puzzle Set 1...");
    if let Some(solution) = puzzle.solve() {
        println!("Solution found! ({} pieces placed)", solution.len());
        
        // Print text representation
        let mut board = vec![vec!['.'; 3]; 2];
        for placement in solution {
            let ch = (b'A' + placement.piece_index as u8) as char;
            for p in &placement.piece.coords {
                let x = (p.x + placement.dx) as usize;
                let y = (p.y + placement.dy) as usize;
                board[y][x] = ch;
            }
        }
        
        for row in board {
            let s: String = row.into_iter().collect();
            println!("{}", s);
        }
    } else {
        println!("No solution found.");
    }
}
