use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub coords: Vec<Point>,
}

impl Piece {
    pub fn new(coords: Vec<(i32, i32)>) -> Self {
        let mut piece = Self {
            coords: coords.into_iter().map(|(x, y)| Point::new(x, y)).collect(),
        };
        piece.normalize();
        piece
    }

    /// Rotate 90 degrees counter-clockwise
    pub fn rotate_90(&self) -> Self {
        let rotated = self.coords.iter().map(|p| Point::new(-p.y, p.x)).collect();
        let mut piece = Self { coords: rotated };
        piece.normalize();
        piece
    }

    /// Mirror horizontally (flip left-right)
    pub fn mirror_horizontal(&self) -> Self {
        let mirrored = self.coords.iter().map(|p| Point::new(-p.x, p.y)).collect();
        let mut piece = Self { coords: mirrored };
        piece.normalize();
        piece
    }

    /// Translate minimum x and y to 0,0 and sort coordinates
    pub fn normalize(&mut self) {
        if self.coords.is_empty() {
            return;
        }
        let min_x = self.coords.iter().map(|p| p.x).min().unwrap_or(0);
        let min_y = self.coords.iter().map(|p| p.y).min().unwrap_or(0);

        for p in &mut self.coords {
            p.x -= min_x;
            p.y -= min_y;
        }
        self.coords.sort();
    }

    /// Generate all unique variations (rotations + mirrors)
    pub fn generate_variations(&self) -> Vec<Piece> {
        let mut variations = HashSet::new();
        let mut current = self.clone();

        for _ in 0..4 {
            variations.insert(current.clone());
            variations.insert(current.mirror_horizontal());
            current = current.rotate_90();
        }

        variations.into_iter().collect()
    }
}
