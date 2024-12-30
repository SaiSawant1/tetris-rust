pub mod piece;
use std::usize;

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
    fn place_cursor(&mut self) {
        let cursor = self
            .cursor
            .take()
            .expect("Called place_cursor Without cursor");
        for coord in cursor.cells().expect("Cursor was outof bounds") {
            let cell: &mut bool = self.board.get_mut(coord).unwrap();
            debug_assert_eq!(*cell, false);
            *cell = true;
        }
    }
}

struct Board([bool; Board::SIZE]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Board::WIDTH * Board::HEIGHT;

    fn in_bounds(Coordinate { x, y }: Coordinate) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    fn blank() -> Self {
        Self([false; Self::SIZE])
    }

    fn indexing(Coordinate { x, y }: Coordinate) -> usize {
        y * Self::WIDTH + x
    }

    fn get_mut(&mut self, coord: Coordinate) -> Option<&mut bool> {
        Self::in_bounds(coord).then(|| &mut self.0[Self::indexing(coord)])
    }
}
