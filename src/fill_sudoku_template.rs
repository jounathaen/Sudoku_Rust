// Template File for a Sudoku.
// Remove the lines which do not insert a number to the Sudoku
//
// The Code below should generate the following Sudoku:
// +---+---+---+
// |  4| 83|  2|
// | 51|  4|3  |
// |   | 96|71 |
// +---+---+---+
// |12 |8  |  6|
// | 4 |   |5  |
// |83 |6 7|9  |
// +---+---+---+
// | 6 |3 9| 4 |
// |  7|   |2 5|
// | 9 | 5 |8 3|
// +---+---+---+


use field::*;

pub fn fill_sudoku (mut sud: &mut Sudoku){

    sud.insert_number( , 0, 0);
    sud.insert_number( , 1, 0);
    sud.insert_number( , 2, 0);
    sud.insert_number(4, 3, 0);
    sud.insert_number( , 4, 0);
    sud.insert_number(8, 5, 0);
    sud.insert_number(3, 6, 0);
    sud.insert_number( , 7, 0);
    sud.insert_number( , 8, 0);
    sud.insert_number(2, 0, 1);
    sud.insert_number( , 1, 1);
    sud.insert_number(5, 2, 1);
    sud.insert_number(1, 3, 1);
    sud.insert_number( , 4, 1);
    sud.insert_number( , 5, 1);
    sud.insert_number(4, 6, 1);
    sud.insert_number(3, 7, 1);
    sud.insert_number( , 8, 1);
    sud.insert_number( , 0, 2);
    sud.insert_number( , 1, 2);
    sud.insert_number( , 2, 2);
    sud.insert_number( , 3, 2);
    sud.insert_number( , 4, 2);
    sud.insert_number(9, 5, 2);
    sud.insert_number(6, 6, 2);
    sud.insert_number(7, 7, 2);
    sud.insert_number(1, 8, 2);
    sud.insert_number( , 0, 3);
    sud.insert_number(1, 1, 3);
    sud.insert_number(2, 2, 3);
    sud.insert_number( , 3, 3);
    sud.insert_number(8, 4, 3);
    sud.insert_number( , 5, 3);
    sud.insert_number( , 6, 3);
    sud.insert_number( , 7, 3);
    sud.insert_number( , 8, 3);
    sud.insert_number(6, 0, 4);
    sud.insert_number( , 1, 4);
    sud.insert_number(4, 2, 4);
    sud.insert_number( , 3, 4);
    sud.insert_number( , 4, 4);
    sud.insert_number( , 5, 4);
    sud.insert_number( , 6, 4);
    sud.insert_number(5, 7, 4);
    sud.insert_number( , 8, 4);
    sud.insert_number( , 0, 5);
    sud.insert_number(8, 1, 5);
    sud.insert_number(3, 2, 5);
    sud.insert_number( , 3, 5);
    sud.insert_number(6, 4, 5);
    sud.insert_number( , 5, 5);
    sud.insert_number(7, 6, 5);
    sud.insert_number(9, 7, 5);
    sud.insert_number( , 8, 5);
    sud.insert_number( , 0, 6);
    sud.insert_number( , 1, 6);
    sud.insert_number(6, 2, 6);
    sud.insert_number( , 3, 6);
    sud.insert_number(3, 4, 6);
    sud.insert_number( , 5, 6);
    sud.insert_number(9, 6, 6);
    sud.insert_number( , 7, 6);
    sud.insert_number(4, 8, 6);
    sud.insert_number( , 0, 7);
    sud.insert_number( , 1, 7);
    sud.insert_number( , 2, 7);
    sud.insert_number(7, 3, 7);
    sud.insert_number( , 4, 7);
    sud.insert_number( , 5, 7);
    sud.insert_number( , 6, 7);
    sud.insert_number(2, 7, 7);
    sud.insert_number( , 8, 7);
    sud.insert_number(5, 0, 8);
    sud.insert_number( , 1, 8);
    sud.insert_number(9, 2, 8);
    sud.insert_number( , 3, 8);
    sud.insert_number( , 4, 8);
    sud.insert_number(5, 5, 8);
    sud.insert_number( , 6, 8);
    sud.insert_number(8, 7, 8);
    sud.insert_number( , 8, 8);

}

#[test]
fn test_fill_sudoku(){
    let mut sud: Sudoku = Default::default();
    fill_sudoku(&mut sud);
    sud.check_validity();
}
