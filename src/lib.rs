mod position;
pub use position::Position;

mod piece;
pub use piece::Piece;

mod square;
pub use square::Square;

mod board;
pub use board::Board;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Color {
    White,
    Black,
}
