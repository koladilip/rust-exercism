#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        match (
            (self.position.rank - other.position.rank).abs(),
            (self.position.file - other.position.file).abs(),
        ) {
            (0, _) => true,
            (_, 0) => true,
            (a, b) if a == b => true,
            _ => false,
        }
    }
}
