// use std::error::Error;

mod field;
use field::*;


fn main() {
    // Solve Sudokus from csv file
    let file_path = "data/sudoku_small.csv";
    if let Err(err) = solve_sudokus_from_csv(&String::from(file_path)) {
        println!("{}", err);
    }

    // Solve Sudokus from String
    let mut sud: Sudoku;
    sud = Sudoku::from(String::from("904008010060092007001040802243900080090000206006030700000405000009000068705610304"));

    // sud.print_lvl = Lvl::Verbose;
    // sud.print_lvl = Lvl::Interactive;
    sud.print_lvl = Lvl::Solution;

    sud.easy_solve().unwrap();

    // Solution:
    // 924 578 613
    // 368 192 457
    // 571 346 892

    // 243 967 185
    // 197 854 236
    // 856 231 749

    // 632 485 971
    // 419 723 568
    // 785 619 324

}
