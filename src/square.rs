use super::Piece;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Square {
    piece: Option<Piece>,
}

impl Square {
    /// Creates an empty [`Square`].
    pub fn empty() -> Self {
        Self { piece: None }
    }

    /// Creates a [`Square`] from a [`Piece`].
    pub fn from(piece: Piece) -> Self {
        Self { piece: Some(piece) }
    }

    pub fn is_empty(&self) -> bool {
        self.piece.is_none()
    }

    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }
}
