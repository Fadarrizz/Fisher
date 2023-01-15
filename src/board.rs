use super::{Square, Piece, Position};

#[derive(Debug)]
pub struct Board {
    squares: [Square; 64],
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::empty();

        Piece::white_pieces()
            .into_iter()
            .chain(Piece::black_pieces())
            .for_each(|piece| board.add_piece(piece));

        board
    }
}

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let header = "\n  ┌───┬───┬───┬───┬───┬───┬───┬───┐  ";
        let middle = "\n  ├───┼───┼───┼───┼───┼───┼───┼───┤  ";
        let footer = "\n  └───┴───┴───┴───┴───┴───┴───┴───┘  \n";
        let divider = " │ ";
        let columns = "    A   B   C   D   E   F   G   H    ";

        write!(f, "{}", header)?;
        let height = 8;
        let width = 8;

        for row in 0..height {
            writeln!(f)?;

            let print_row = height - row - 1;
            write!(f, "{}", print_row + 1)?;

            for col in 0..width {
                write!(f, "{divider}")?;

                let print_col = col;
                let pos = Position::new(print_row, print_col);

                let s = if let Some(piece) = self.get_piece(&pos) {
                    piece.to_string()
                } else {
                    String::from(" ")
                };

                write!(f, "{}", s)?;
            }
            write!(f, "{}", divider)?;
            
            if row == 7 {
                write!(f, "{}", footer)?;
                write!(f, "{}", columns)?;
            } else {
                write!(f, "{}", middle)?;
            }
        }

        Ok(())
    }
}

impl Board {
    fn empty() -> Self {
        Self {
            squares: [Square::empty(); 64],
        }
    }

    fn get_square_index(&self, position: &Position) -> usize {
        ((7 - position.get_row()) * 8 + position.get_col()) as usize
    }

    fn get_mut_square(&mut self, position: &Position) -> &mut Square {
        let index = self.get_square_index(position);
        &mut self.squares[index]
    }

    fn add_piece(&mut self, piece: Piece) {
        let position = piece.get_position();
        *self.get_mut_square(position) = Square::from(piece);
    }

    fn get_piece(&self, position: &Position) -> Option<Piece> {
        let index = self.get_square_index(position);
        self.squares[index].get_piece()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Position, Color};

    #[test]
    fn test_get_square_index() -> Result<(), String> {
        let board = Board::empty();

        let pos = Position::try_from("A8")?;
        assert_eq!(0, board.get_square_index(&pos));

        let pos = Position::try_from("H8")?;
        assert_eq!(7, board.get_square_index(&pos));
        
        let pos = Position::try_from("A1")?;
        assert_eq!(56, board.get_square_index(&pos));

        let pos = Position::try_from("H1")?;
        assert_eq!(63, board.get_square_index(&pos));

        Ok(())
    }

    #[test]
    fn test_get_add_piece() {
        let mut board = Board::empty();
        
        assert_eq!(Square::empty(), board.squares[0]);

        let piece = Piece::Pawn(Color::White, Position::new(7, 0));
        board.add_piece(piece);

        assert_eq!(Square::from(piece), board.squares[0]);
    }
}
