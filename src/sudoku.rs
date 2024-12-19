use crate::board::Board;
use std::cell::RefCell;

#[derive(Debug)]
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

        while ind < 81 && !self.board.borrow().empty(ind / 9, ind % 9) {
            ind += 1;
        }

        // Base case for recursion
        if ind >= 81 {
            return true;
        }

        let row = ind / 9;
        let col = ind % 9;

        for guess in 1..=9 {
            if !self.board.borrow_mut().guess(row, col, guess) {
                continue;
            }

            let success = self.solve_by_backtrack(Some(ind + 1));
            if success {
                return true;
            }
        }

        self.board.borrow_mut().reset(row, col);

        return false;
    }

    pub fn guesses(&self) -> u32 {
        self.board.borrow().guesses()
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.board.borrow())
    }
}
