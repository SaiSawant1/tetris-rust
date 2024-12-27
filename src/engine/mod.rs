pub mod piece;
use piece::Kind as PieceKind;
pub struct Engine {
    board: Board,
    bag: Vec<PieceKind>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::blank(),
            bag: Vec::new(),
        }
    }
}

struct Board {
    cells: [bool; Board::SIZE],
}

impl Board {
    const WIDTH: usize = 10;
    const LENGTH: usize = 20;
    const SIZE: usize = Board::WIDTH * Board::LENGTH;

    fn blank() -> Self {
        Self {
            cells: [false; Self::SIZE],
        }
    }
}
