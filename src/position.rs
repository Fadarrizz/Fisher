#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Position {
    row: u8,
    col: u8,
}

impl TryFrom<&str> for Position {
    type Error = String;

    /// Converts a `&str` into a [`Position`].
    fn try_from(s: &str) -> Result<Position, Self::Error> {
        assert!(s.len() == 2, "Position can only be two characters, e.g. 'A1', 'H7'"); 
        let mut chars = s.chars();
        let (col, row) = (chars.next().unwrap(), chars.next().unwrap());

        let col = match col {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => return Err(String::from("Invalid position")),
        };

        let row = row.to_digit(10).unwrap() - 1;

        Ok(Position { row: row as u8, col })
    }
}

impl Position {
    /// Creates a [`Position`] for its respective row and column number.
    ///
    /// # Examples:
    /// ```
    /// let position = chess::Position::new(0, 7);
    ///
    /// assert_eq!(0, position.get_row());
    /// assert_eq!(7, position.get_col());
    /// ```
    pub fn new(row: u8, col: u8) -> Self {
        assert!(col <= 7, "Col must be a number from 0-7.");
        assert!(row <= 7, "Row must be a number from 0-7.");

        Self { row, col }
    }

    pub fn get_row(&self) -> u8 {
        self.row
    }

    pub fn get_col(&self) -> u8 {
        self.col
    }
}

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let col = match self.col {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            _ => '?',
        };

        write!(f, "{}{}", self.row + 1, col)
    }
}

#[cfg(test)]
mod tests {
    use crate::Position;

    #[test]
    fn test_try_from() -> Result<(), String> {
        assert_eq!(Position::new(0, 0), Position::try_from("A1")?);
        assert_eq!(Position::new(7, 0), Position::try_from("A8")?);
        assert_eq!(Position::new(0, 7), Position::try_from("H1")?);
        assert_eq!(Position::new(7, 7), Position::try_from("H8")?);

        Ok(())
    }
}
