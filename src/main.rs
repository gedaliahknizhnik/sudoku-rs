use std::time::Instant;
use sudoku_rs::sudoku::Sudoku;

fn main() {
    let sudoku = Sudoku::new([
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]);

    println!("Original:\n{}", sudoku);

    let now = Instant::now();
    {
        sudoku.solve_by_backtrack(None);
    }
    let elapsed = now.elapsed();

    println!("Solved:\n{}", sudoku);
    println!(
        "Solution took {}ms and {} guesses",
        elapsed.as_millis(),
        sudoku.guesses()
    );
}
