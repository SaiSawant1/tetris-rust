pub mod piece;
use cgmath::Vector2;
use piece::{Kind as PieceKind, Piece};
use rand::{rngs::ThreadRng, seq::SliceRandom};

type Coordinate = Vector2<usize>;
type Offset = Vector2<isize>;

pub struct Engine {
    board: Board,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::blank(),
            bag: Vec::new(),
            rng: rand::thread_rng(),
            cursor: None,
        }
    }
    fn fill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.bag.extend_from_slice(&PieceKind::ALL);
        self.bag.shuffle(&mut self.rng);
    }
    fn place_cursor() {}
}

struct Board {
    cells: [bool; Board::SIZE],
}

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Board::WIDTH * Board::HEIGHT;

    fn in_bounds(Coordinate { x, y }: Coordinate) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    fn blank() -> Self {
        Self {
            cells: [false; Self::SIZE],
        }
    }
}
