mod field;
use field::*;

mod sudoku_easy_1;



fn main() {

    // Init
    let mut sud: Sudoku = Default::default();
    print_sudoku(&sud, false);
    sudoku_easy_1::fill_sudoku(&mut sud);
    sud.check_validity();
    print_sudoku(&sud, false);
    print_sudoku(&sud, true);




    // is_valid(&playground);
}
