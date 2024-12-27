use cgmath::Vector2;

pub(super) struct Piece {
    pub kind: Kind,
    pub position: Vector2<usize>,
    pub rotation: Rotation,
}

pub enum Kind {
    Square,
    Line,
    T,
    L,
    J,
    S,
    Z,
}
impl Piece {}

impl Kind {}

pub enum Rotation {
    N,
    S,
    E,
    W,
}
