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

    insert_number( , &mut sud, 0, 0);
    insert_number( , &mut sud, 1, 0);
    insert_number( , &mut sud, 2, 0);
    insert_number(4, &mut sud, 3, 0);
    insert_number( , &mut sud, 4, 0);
    insert_number(8, &mut sud, 5, 0);
    insert_number(3, &mut sud, 6, 0);
    insert_number( , &mut sud, 7, 0);
    insert_number( , &mut sud, 8, 0);
    insert_number(2, &mut sud, 0, 1);
    insert_number( , &mut sud, 1, 1);
    insert_number(5, &mut sud, 2, 1);
    insert_number(1, &mut sud, 3, 1);
    insert_number( , &mut sud, 4, 1);
    insert_number( , &mut sud, 5, 1);
    insert_number(4, &mut sud, 6, 1);
    insert_number(3, &mut sud, 7, 1);
    insert_number( , &mut sud, 8, 1);
    insert_number( , &mut sud, 0, 2);
    insert_number( , &mut sud, 1, 2);
    insert_number( , &mut sud, 2, 2);
    insert_number( , &mut sud, 3, 2);
    insert_number( , &mut sud, 4, 2);
    insert_number(9, &mut sud, 5, 2);
    insert_number(6, &mut sud, 6, 2);
    insert_number(7, &mut sud, 7, 2);
    insert_number(1, &mut sud, 8, 2);
    insert_number( , &mut sud, 0, 3);
    insert_number(1, &mut sud, 1, 3);
    insert_number(2, &mut sud, 2, 3);
    insert_number( , &mut sud, 3, 3);
    insert_number(8, &mut sud, 4, 3);
    insert_number( , &mut sud, 5, 3);
    insert_number( , &mut sud, 6, 3);
    insert_number( , &mut sud, 7, 3);
    insert_number( , &mut sud, 8, 3);
    insert_number(6, &mut sud, 0, 4);
    insert_number( , &mut sud, 1, 4);
    insert_number(4, &mut sud, 2, 4);
    insert_number( , &mut sud, 3, 4);
    insert_number( , &mut sud, 4, 4);
    insert_number( , &mut sud, 5, 4);
    insert_number( , &mut sud, 6, 4);
    insert_number(5, &mut sud, 7, 4);
    insert_number( , &mut sud, 8, 4);
    insert_number( , &mut sud, 0, 5);
    insert_number(8, &mut sud, 1, 5);
    insert_number(3, &mut sud, 2, 5);
    insert_number( , &mut sud, 3, 5);
    insert_number(6, &mut sud, 4, 5);
    insert_number( , &mut sud, 5, 5);
    insert_number(7, &mut sud, 6, 5);
    insert_number(9, &mut sud, 7, 5);
    insert_number( , &mut sud, 8, 5);
    insert_number( , &mut sud, 0, 6);
    insert_number( , &mut sud, 1, 6);
    insert_number(6, &mut sud, 2, 6);
    insert_number( , &mut sud, 3, 6);
    insert_number(3, &mut sud, 4, 6);
    insert_number( , &mut sud, 5, 6);
    insert_number(9, &mut sud, 6, 6);
    insert_number( , &mut sud, 7, 6);
    insert_number(4, &mut sud, 8, 6);
    insert_number( , &mut sud, 0, 7);
    insert_number( , &mut sud, 1, 7);
    insert_number( , &mut sud, 2, 7);
    insert_number(7, &mut sud, 3, 7);
    insert_number( , &mut sud, 4, 7);
    insert_number( , &mut sud, 5, 7);
    insert_number( , &mut sud, 6, 7);
    insert_number(2, &mut sud, 7, 7);
    insert_number( , &mut sud, 8, 7);
    insert_number(5, &mut sud, 0, 8);
    insert_number( , &mut sud, 1, 8);
    insert_number(9, &mut sud, 2, 8);
    insert_number( , &mut sud, 3, 8);
    insert_number( , &mut sud, 4, 8);
    insert_number(5, &mut sud, 5, 8);
    insert_number( , &mut sud, 6, 8);
    insert_number(8, &mut sud, 7, 8);
    insert_number( , &mut sud, 8, 8);

}

#[test]
fn test_fill_sudoku(){
    let mut sud: Sudoku = Default::default();
    fill_sudoku(&mut sud);
    sud.check_validity();
}
