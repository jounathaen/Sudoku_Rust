mod field;
use field::*;


fn main() {


    let mut playground: [[field::Entry; 9]; 9] = Default::default();
    insert_number(1, &mut playground, 0, 0);
    insert_number(2, &mut playground, 0, 1);
    insert_number(3, &mut playground, 0, 2);
    insert_number(4, &mut playground, 1, 0);
    insert_number(5, &mut playground, 1, 1);


    print_playground(&playground);

    remove_from_guesses_line(&mut playground[0], 5);



    // is_valid(&playground);
}
