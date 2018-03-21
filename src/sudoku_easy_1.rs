// +---+---+---+
// |004|083|002|
// |051|004|300|
// |000|096|710|
// +---+---+---+
// |120|800|006|
// |040|000|500|
// |830|607|900|
// +---+---+---+
// |060|309|040|
// |007|000|205|
// |090|050|803|
// +---+---+---+
use field::*;

pub fn fill_sudoku (sud: &mut Sudoku){

    sud.insert_number(4,  2, 0);
    sud.insert_number(8,  4, 0);
    sud.insert_number(3,  5, 0);
    sud.insert_number(2,  8, 0);
    sud.insert_number(5,  1, 1);
    sud.insert_number(1,  2, 1);
    sud.insert_number(4,  5, 1);
    sud.insert_number(3,  6, 1);
    sud.insert_number(9,  4, 2);
    sud.insert_number(6,  5, 2);
    sud.insert_number(7,  6, 2);
    sud.insert_number(1,  7, 2);
    sud.insert_number(1,  0, 3);
    sud.insert_number(2,  1, 3);
    sud.insert_number(8,  3, 3);
    sud.insert_number(6,  8, 3);
    sud.insert_number(4,  1, 4);
    sud.insert_number(5,  6, 4);
    sud.insert_number(8,  0, 5);
    sud.insert_number(3,  1, 5);
    sud.insert_number(6,  3, 5);
    sud.insert_number(7,  5, 5);
    sud.insert_number(9,  6, 5);
    sud.insert_number(6,  1, 6);
    sud.insert_number(3,  3, 6);
    sud.insert_number(9,  5, 6);
    sud.insert_number(4,  7, 6);
    sud.insert_number(7,  2, 7);
    sud.insert_number(2,  6, 7);
    sud.insert_number(5,  8, 7);
    sud.insert_number(9,  1, 8);
    sud.insert_number(5,  4, 8);
    sud.insert_number(8,  6, 8);
    sud.insert_number(3,  8, 8);
    sud.set_difficulty(2);

}
#[test]
fn test_fill_sudoku(){
    let mut sud: Sudoku = Default::default();
    fill_sudoku(&mut sud);
    sud.check_validity();
}
