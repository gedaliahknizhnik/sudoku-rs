use crate::element::Element;

#[derive(Debug)]
pub struct Board {
    board: [[u8; 9]; 9],
    guesses: u32,
}

impl Board {
    pub fn new(board: [[u8; 9]; 9]) -> Board {
        Board { board, guesses: 0 }
    }

    pub fn empty(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == 0
    }

    pub fn reset(&mut self, row: usize, col: usize) {
        self.board[row][col] = 0;
    }

    pub fn guess(&mut self, row: usize, col: usize, guess: u8) -> bool {
        self.board[row][col] = guess;
        self.guesses += 1;

        Element::evaluate(&self.row(row))
            && Element::evaluate(&self.column(col))
            && Element::evaluate(&self.subsquare(row / 3, col / 3))
    }

    // Accessors ***************************************************************

    pub fn guesses(&self) -> u32 {
        self.guesses
    }

    fn row_iter(&self, ind: usize) -> impl Iterator<Item = &u8> {
        self.board[ind].iter()
    }

    pub fn row(&self, ind: usize) -> Vec<u8> {
        if !(0..9).contains(&ind) {
            panic!("Values must be in the range [0,8] for sudoku.")
        }
        self.row_iter(ind).cloned().collect()
    }

    fn column_iter(&self, ind: usize) -> impl Iterator<Item = u8> + '_ {
        self.board.iter().map(move |row| row[ind])
    }

    pub fn column(&self, ind: usize) -> Vec<u8> {
        if !(0..9).contains(&ind) {
            panic!("Values must be in the range [0,8] for sudoku.")
        }
        self.column_iter(ind).collect()
    }

    fn subsquare_iter(&self, row: usize, col: usize) -> impl Iterator<Item = u8> + '_ {
        if !(0..3).contains(&row) || !(0..3).contains(&col) {
            panic!("Row and column indices must be in the range [0,2] for subsquares.")
        }
        let row_start = row * 3;
        let col_start = col * 3;
        self.board[row_start..(row_start + 3) as usize]
            .iter()
            .flat_map(move |r| r[col_start..(col_start + 3) as usize].iter().cloned())
    }

    pub fn subsquare(&self, row: usize, col: usize) -> Vec<u8> {
        self.subsquare_iter(row, col).collect()
    }
}

fn valtoprint(val: u8) -> String {
    if val > 0 {
        val.to_string()
    } else {
        " ".to_string()
    }
}
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.board.iter() {
            write!(f, "[ ")?;
            for val in row[..row.len() - 1].iter() {
                write!(f, "{}, ", valtoprint(*val))?;
            }
            writeln!(f, "{} ]", valtoprint(row[row.len() - 1]))?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static board: Board = Board {
        board: [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ],
    };

    #[test]
    #[should_panic]
    fn too_large() {
        board.row(9);
    }

    #[test]
    fn get_row() {
        assert_eq!(board.row(0), vec![5, 3, 4, 6, 7, 8, 9, 1, 2]);
        assert_eq!(board.row(3), vec![8, 5, 9, 7, 6, 1, 4, 2, 3]);
        assert_ne!(board.row(4), vec![4, 3, 6, 8, 5, 3, 7, 9, 1]);
    }

    #[test]
    fn get_column() {
        assert_eq!(board.column(0), vec![5, 6, 1, 8, 4, 7, 9, 2, 3]);
        assert_eq!(board.column(3), vec![6, 1, 3, 7, 8, 9, 5, 4, 2]);
    }

    #[test]
    fn get_subsquare() {
        assert_eq!(board.subsquare(0, 0), vec![5, 3, 4, 6, 7, 2, 1, 9, 8]);
        assert_eq!(board.subsquare(1, 1), vec![7, 6, 1, 8, 5, 3, 9, 2, 4]);
    }
}
