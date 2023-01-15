use super::{Color, Position};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Rook(Color, Position),
    Bishop(Color, Position),
    Knight(Color, Position),
    Pawn(Color, Position),
}

impl Piece {
    /// Constructs a [`Vec<Piece>`] of intial white pieces.
    pub fn white_pieces() -> Vec<Piece> {
        Self::initial_pieces(Color::White)
    }

    /// Constructs a [`Vec<Piece>`] of initial black pieces.
    pub fn black_pieces() -> Vec<Piece> {
        Self::initial_pieces(Color::Black)
    }

    fn initial_pieces(color: Color) -> Vec<Piece> {
        let mut pieces = Vec::new();

        let back_rank = if color == Color::White { 0 } else { 7 };
        let pawn_rank = if color == Color::White { 1 } else { 6 };

        for i in 0..8 {
            let position = Position::new(back_rank, i);
            let piece = Self::get_piece_by_index(i);

            pieces.push(piece(color, position));
        }

        for i in 0..8 {
            let position = Position::new(pawn_rank, i);

            pieces.push(Self::Pawn(color, position));
        }

        pieces
    }

    fn get_piece_by_index(index: u8) -> fn(Color, Position) -> Piece {
        match index {
            0 => Self::Rook,
            1 => Self::Knight,
            2 => Self::Bishop,
            3 => Self::Queen,
            4 => Self::King,
            5 => Self::Bishop,
            6 => Self::Knight,
            7 => Self::Rook,
            _ => panic!("Index cannot be higher than 7"),
        }
    }

    pub fn get_color(&self) -> &Color {
        match self {
            Self::King(c, _)
            | Self::Queen(c, _)
            | Self::Knight(c, _)
            | Self::Rook(c, _)
            | Self::Bishop(c, _)
            | Self::Pawn(c, _) => c,
        }
    }

    pub fn get_position(&self) -> &Position {
        match self {
            Self::King(_, p)
            | Self::Queen(_, p)
            | Self::Knight(_, p)
            | Self::Rook(_, p)
            | Self::Bishop(_, p)
            | Self::Pawn(_, p) => p,
        }
    }
}

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let color = match self.get_color() {
            Color::White => match self {
                Self::King(_, _) => "♔",
                Self::Queen(_, _) => "♕",
                Self::Knight(_, _) => "♘",
                Self::Rook(_, _) => "♖",
                Self::Bishop(_, _) => "♗",
                Self::Pawn(_, _) => "♙",
            },
            Color::Black => match self {
                Self::King(_, _) => "♚",
                Self::Queen(_, _) => "♛",
                Self::Knight(_, _) => "♞",
                Self::Rook(_, _) => "♜",
                Self::Bishop(_, _) => "♝",
                Self::Pawn(_, _) => "♟",
            },
        };

        write!( f, "{}", color)
    }
}
