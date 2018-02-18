mod field;
use field::*;

mod sudoku_easy_1;



fn main() {

    // Init
    let mut sud: Sudoku = Default::default();
    sud.print(false);
    sudoku_easy_1::fill_sudoku(&mut sud);
    sud.check_validity();
    sud.print(false);
    // sud.print(true);
    let mut count = 0;
    while {sud.solve_obvious() > 0 } {
        count = count + 1;
        println!("Round {}", count);
        sud.print(false);
    }

    sud.check_validity();
}
