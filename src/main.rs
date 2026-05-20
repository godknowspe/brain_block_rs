mod shape;

use shape::Piece;

fn main() {
    println!("Brain Block Puzzle Solver (Rust Edition)");

    // Test a basic piece (L-shape)
    // Python origin: [(0, 0), (0, 1), (1, 0)]
    let piece = Piece::new(vec![(0, 0), (0, 1), (1, 0)]);
    
    println!("Original Piece: {:?}", piece.coords);
    
    let variations = piece.generate_variations();
    println!("Total unique variations: {}", variations.len());
    for (i, v) in variations.iter().enumerate() {
        println!("Variation {}: {:?}", i + 1, v.coords);
    }
}
