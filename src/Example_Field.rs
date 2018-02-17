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

    insert_number(0, &mut playground, 0, 0);
    insert_number(0, &mut playground, 1, 0);
    insert_number(0, &mut playground, 2, 0);
    insert_number(4, &mut playground, 3, 0);
    insert_number(0, &mut playground, 4, 0);
    insert_number(8, &mut playground, 5, 0);
    insert_number(3, &mut playground, 6, 0);
    insert_number(0, &mut playground, 7, 0);
    insert_number(0, &mut playground, 8, 0);
    insert_number(2, &mut playground, 0, 1);
    insert_number(0, &mut playground, 1, 1);
    insert_number(5, &mut playground, 2, 1);
    insert_number(1, &mut playground, 3, 1);
    insert_number(0, &mut playground, 4, 1);
    insert_number(0, &mut playground, 5, 1);
    insert_number(4, &mut playground, 6, 1);
    insert_number(3, &mut playground, 7, 1);
    insert_number(0, &mut playground, 8, 1);
    insert_number(0, &mut playground, 0, 2);
    insert_number(0, &mut playground, 1, 2);
    insert_number(0, &mut playground, 2, 2);
    insert_number(0, &mut playground, 3, 2);
    insert_number(0, &mut playground, 4, 2);
    insert_number(9, &mut playground, 5, 2);
    insert_number(6, &mut playground, 6, 2);
    insert_number(7, &mut playground, 7, 2);
    insert_number(1, &mut playground, 8, 2);
    insert_number(0, &mut playground, 0, 3);
    insert_number(1, &mut playground, 1, 3);
    insert_number(2, &mut playground, 2, 3);
    insert_number(0, &mut playground, 3, 3);
    insert_number(8, &mut playground, 4, 3);
    insert_number(0, &mut playground, 5, 3);
    insert_number(0, &mut playground, 6, 3);
    insert_number(0, &mut playground, 7, 3);
    insert_number(0, &mut playground, 8, 3);
    insert_number(6, &mut playground, 0, 4);
    insert_number(0, &mut playground, 1, 4);
    insert_number(4, &mut playground, 2, 4);
    insert_number(0, &mut playground, 3, 4);
    insert_number(0, &mut playground, 4, 4);
    insert_number(0, &mut playground, 5, 4);
    insert_number(0, &mut playground, 6, 4);
    insert_number(5, &mut playground, 7, 4);
    insert_number(0, &mut playground, 8, 4);
    insert_number(0, &mut playground, 0, 5);
    insert_number(8, &mut playground, 1, 5);
    insert_number(3, &mut playground, 2, 5);
    insert_number(0, &mut playground, 3, 5);
    insert_number(6, &mut playground, 4, 5);
    insert_number(0, &mut playground, 5, 5);
    insert_number(7, &mut playground, 6, 5);
    insert_number(9, &mut playground, 7, 5);
    insert_number(0, &mut playground, 8, 5);
    insert_number(0, &mut playground, 0, 6);
    insert_number(0, &mut playground, 1, 6);
    insert_number(6, &mut playground, 2, 6);
    insert_number(0, &mut playground, 3, 6);
    insert_number(3, &mut playground, 4, 6);
    insert_number(0, &mut playground, 5, 6);
    insert_number(9, &mut playground, 6, 6);
    insert_number(0, &mut playground, 7, 6);
    insert_number(4, &mut playground, 8, 6);
    insert_number(0, &mut playground, 0, 7);
    insert_number(0, &mut playground, 1, 7);
    insert_number(0, &mut playground, 2, 7);
    insert_number(7, &mut playground, 3, 7);
    insert_number(0, &mut playground, 4, 7);
    insert_number(0, &mut playground, 5, 7);
    insert_number(0, &mut playground, 6, 7);
    insert_number(2, &mut playground, 7, 7);
    insert_number(0, &mut playground, 8, 7);
    insert_number(5, &mut playground, 0, 8);
    insert_number(0, &mut playground, 1, 8);
    insert_number(9, &mut playground, 2, 8);
    insert_number(0, &mut playground, 3, 8);
    insert_number(0, &mut playground, 4, 8);
    insert_number(5, &mut playground, 5, 8);
    insert_number(0, &mut playground, 6, 8);
    insert_number(8, &mut playground, 7, 8);
    insert_number(0, &mut playground, 8, 8);

}
