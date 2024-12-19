use crate::board::Board;
use std::cell::RefCell;

pub struct Sudoku {
    board: RefCell<Board>,
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Sudoku {
        Sudoku {
            board: RefCell::new(Board::new(board)),
        }
    }

    pub fn solve_by_backtrack(&self, ind: Option<usize>) -> bool {
        let mut ind = ind.unwrap_or(0);

        // Base case for recursion
        if ind >= 81 {
            return true;
        }

        while !self.board.borrow().empty(ind / 9, ind % 9) {
            ind += 1;
        }

        for guess in 1..=9 {
            let row = ind / 9;
            let col = ind % 9;

            if !self.board.borrow_mut().guess(row, col, guess) {
                continue;
            }

            let success = self.solve_by_backtrack(Some(ind + 1));
            if success {
                return true;
            }
        }

        return false;
    }
}
