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

pub fn fill_sudoku (mut sud: &mut Sudoku){

    insert_number(4, &mut sud, 2, 0);
    insert_number(8, &mut sud, 4, 0);
    insert_number(3, &mut sud, 5, 0);
    insert_number(2, &mut sud, 8, 0);
    insert_number(5, &mut sud, 1, 1);
    insert_number(1, &mut sud, 2, 1);
    insert_number(4, &mut sud, 5, 1);
    insert_number(3, &mut sud, 6, 1);
    insert_number(9, &mut sud, 4, 2);
    insert_number(6, &mut sud, 5, 2);
    insert_number(7, &mut sud, 6, 2);
    insert_number(1, &mut sud, 7, 2);
    insert_number(1, &mut sud, 0, 3);
    insert_number(2, &mut sud, 1, 3);
    insert_number(8, &mut sud, 3, 3);
    insert_number(6, &mut sud, 8, 3);
    insert_number(4, &mut sud, 1, 4);
    insert_number(5, &mut sud, 6, 4);
    insert_number(8, &mut sud, 0, 5);
    insert_number(3, &mut sud, 1, 5);
    insert_number(6, &mut sud, 3, 5);
    insert_number(7, &mut sud, 5, 5);
    insert_number(9, &mut sud, 6, 5);
    insert_number(6, &mut sud, 1, 6);
    insert_number(3, &mut sud, 3, 6);
    insert_number(9, &mut sud, 5, 6);
    insert_number(4, &mut sud, 7, 6);
    insert_number(7, &mut sud, 2, 7);
    insert_number(2, &mut sud, 6, 7);
    insert_number(5, &mut sud, 8, 7);
    insert_number(9, &mut sud, 1, 8);
    insert_number(5, &mut sud, 4, 8);
    insert_number(8, &mut sud, 6, 8);
    insert_number(3, &mut sud, 8, 8);
    sud.set_difficulty(2);

}
#[test]
fn test_fill_sudoku(){
    let mut sud: Sudoku = Default::default();
    fill_sudoku(&mut sud);
    sud.check_validity();
}
