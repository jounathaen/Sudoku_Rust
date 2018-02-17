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

pub fn fill_playground (mut playground: &mut [[Entry ; 9] ; 9]){

    insert_number(4, &mut playground, 2, 0);
    insert_number(8, &mut playground, 4, 0);
    insert_number(3, &mut playground, 5, 0);
    insert_number(2, &mut playground, 8, 0);
    insert_number(5, &mut playground, 1, 1);
    insert_number(1, &mut playground, 2, 1);
    insert_number(4, &mut playground, 5, 1);
    insert_number(3, &mut playground, 6, 1);
    insert_number(9, &mut playground, 4, 2);
    insert_number(6, &mut playground, 5, 2);
    insert_number(7, &mut playground, 6, 2);
    insert_number(1, &mut playground, 7, 2);
    insert_number(1, &mut playground, 0, 3);
    insert_number(2, &mut playground, 1, 3);
    insert_number(8, &mut playground, 3, 3);
    insert_number(6, &mut playground, 8, 3);
    insert_number(4, &mut playground, 1, 4);
    insert_number(5, &mut playground, 6, 4);
    insert_number(8, &mut playground, 0, 5);
    insert_number(3, &mut playground, 1, 5);
    insert_number(6, &mut playground, 3, 5);
    insert_number(7, &mut playground, 5, 5);
    insert_number(9, &mut playground, 6, 5);
    insert_number(6, &mut playground, 1, 6);
    insert_number(3, &mut playground, 3, 6);
    insert_number(9, &mut playground, 5, 6);
    insert_number(4, &mut playground, 7, 6);
    insert_number(7, &mut playground, 2, 7);
    insert_number(2, &mut playground, 6, 7);
    insert_number(5, &mut playground, 8, 7);
    insert_number(9, &mut playground, 1, 8);
    insert_number(5, &mut playground, 4, 8);
    insert_number(8, &mut playground, 6, 8);
    insert_number(3, &mut playground, 8, 8);

}
